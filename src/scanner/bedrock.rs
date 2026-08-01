use std::net::SocketAddr;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::time::timeout;
use super::types::{Edition, ServerInfo};

const MAGIC: [u8; 16] = [
    0x00, 0xFF, 0xFF, 0x00, 0xFE, 0xFE, 0xFE, 0xFE,
    0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56, 0x78,
];

pub async fn probe(addr: SocketAddr, timeout_ms: u64) -> Option<ServerInfo> {
    let dur = Duration::from_millis(timeout_ms);
    let start = Instant::now();

    let socket = UdpSocket::bind(super::local_bind_addr(&addr)).await.ok()?;
    socket.connect(addr).await.ok()?;

    timeout(dur, socket.send(&build_ping())).await.ok()?.ok()?;

    let mut buf = [0u8; 4096];
    let n = timeout(dur, socket.recv(&mut buf)).await.ok()?.ok()?;
    let latency_ms = start.elapsed().as_millis() as u64;

    parse_pong(&buf[..n], addr, latency_ms)
}

fn build_ping() -> [u8; 33] {
    let mut buf = [0u8; 33];
    buf[0] = 0x01;
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    buf[1..9].copy_from_slice(&ts.to_be_bytes());
    buf[9..25].copy_from_slice(&MAGIC);
    buf
}

fn parse_pong(data: &[u8], addr: SocketAddr, latency_ms: u64) -> Option<ServerInfo> {
    if data.first()? != &0x1C || data.len() < 35 {
        return None;
    }
    let str_len = u16::from_be_bytes([data[33], data[34]]) as usize;
    let end = 35 + str_len;
    if data.len() < end {
        return None;
    }
    let raw = std::str::from_utf8(&data[35..end]).ok()?;
    parse_motd(raw, addr, latency_ms)
}

fn parse_motd(raw: &str, addr: SocketAddr, latency_ms: u64) -> Option<ServerInfo> {
    // Format: edition;motd1;protocol;version;online;max;guid;motd2;gamemode;gamemode_num;portV4;portV6
    let parts: Vec<&str> = raw.split(';').collect();
    if parts.len() < 6 {
        return None;
    }
    let get = |i: usize| parts.get(i).map(|s| s.to_string()).filter(|s| !s.is_empty());

    let mut info = ServerInfo::base(addr, Edition::Bedrock);
    info.motd = super::strip_section_codes(parts[1]);
    info.protocol = parts[2].parse().unwrap_or(0);
    info.version = parts[3].to_string();
    info.online = parts[4].parse().unwrap_or(0);
    info.max_players = parts[5].parse().unwrap_or(0);
    info.latency_ms = latency_ms;
    info.ping_history = vec![latency_ms];
    info.bedrock_edition = get(0);
    info.server_guid = get(6);
    info.sub_motd = get(7).map(|s| super::strip_section_codes(&s));
    info.gamemode = get(8);
    info.port_v4 = parts.get(10).and_then(|s| s.trim().parse().ok());
    info.port_v6 = parts.get(11).and_then(|s| s.trim().parse().ok());
    Some(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr() -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], 19132))
    }

    /// Wrap a MOTD string in a well-formed unconnected pong:
    /// `0x1C` + timestamp(8) + GUID(8) + MAGIC(16) + len(u16) + motd.
    fn pong(motd: &str) -> Vec<u8> {
        let mut p = vec![0x1C];
        p.extend_from_slice(&0u64.to_be_bytes());
        p.extend_from_slice(&0u64.to_be_bytes());
        p.extend_from_slice(&MAGIC);
        p.extend_from_slice(&(motd.len() as u16).to_be_bytes());
        p.extend_from_slice(motd.as_bytes());
        p
    }

    #[test]
    fn parses_full_pong() {
        let motd = "MCPE;§eDedicated Server;390;1.14.60;5;10;1234567890;Bedrock level;Survival;1;19132;19133";
        let info = parse_pong(&pong(motd), addr(), 7).expect("should parse");
        assert_eq!(info.edition, Edition::Bedrock);
        assert_eq!(info.motd, "Dedicated Server"); // section code stripped
        assert_eq!(info.protocol, 390);
        assert_eq!(info.version, "1.14.60");
        assert_eq!(info.online, 5);
        assert_eq!(info.max_players, 10);
        assert_eq!(info.latency_ms, 7);
        assert_eq!(info.bedrock_edition.as_deref(), Some("MCPE"));
        assert_eq!(info.server_guid.as_deref(), Some("1234567890"));
        assert_eq!(info.sub_motd.as_deref(), Some("Bedrock level"));
        assert_eq!(info.gamemode.as_deref(), Some("Survival"));
        assert_eq!(info.port_v4, Some(19132));
        assert_eq!(info.port_v6, Some(19133));
    }

    #[test]
    fn parses_minimal_pong_without_optional_fields() {
        // Only the six required fields; optionals stay None.
        let info = parse_pong(&pong("MCPE;Hi;390;1.14.60;0;20"), addr(), 0).expect("should parse");
        assert_eq!(info.motd, "Hi");
        assert_eq!(info.max_players, 20);
        assert_eq!(info.server_guid, None);
        assert_eq!(info.gamemode, None);
        assert_eq!(info.port_v4, None);
    }

    #[test]
    fn rejects_wrong_packet_id() {
        let mut bytes = pong("MCPE;Hi;390;1.0;0;1");
        bytes[0] = 0x00;
        assert!(parse_pong(&bytes, addr(), 0).is_none());
    }

    #[test]
    fn rejects_truncated_packet() {
        assert!(parse_pong(&[0x1C; 10], addr(), 0).is_none());
    }

    #[test]
    fn rejects_length_past_end_of_buffer() {
        let mut bytes = pong("MCPE;Hi;390;1.0;0;1");
        // Claim a MOTD far longer than what follows.
        bytes[33..35].copy_from_slice(&9999u16.to_be_bytes());
        assert!(parse_pong(&bytes, addr(), 0).is_none());
    }

    #[test]
    fn rejects_motd_with_too_few_fields() {
        assert!(parse_motd("MCPE;Hi;390", addr(), 0).is_none());
    }
}
