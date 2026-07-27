
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;


#[derive(Debug, Default)]
pub struct QueryData {
    pub world: Option<String>,
    pub plugins: Vec<String>,
    pub players: Vec<String>,
}

const KV_PREFIX: &[u8] = b"splitnum\x00\x80\x00";
const PLAYER_PREFIX: &[u8] = b"\x01player_\x00\x00";

pub async fn probe(addr: SocketAddr, timeout_ms: u64) -> Option<QueryData> {
    let dur = Duration::from_millis(timeout_ms);
    let socket = UdpSocket::bind(super::local_bind_addr(&addr)).await.ok()?;
    socket.connect(addr).await.ok()?;

    let session_id: i32 = 1;

    // --- Step 1: handshake, challenge-token ---
    let mut hs = Vec::with_capacity(7);
    hs.extend_from_slice(&[0xFE, 0xFD, 0x09]);
    hs.extend_from_slice(&session_id.to_be_bytes());
    timeout(dur, socket.send(&hs)).await.ok()?.ok()?;

    let mut buf = [0u8; 4096];
    let n = timeout(dur, socket.recv(&mut buf)).await.ok()?.ok()?;
    let token = parse_challenge(&buf[..n])?;

    // --- Step 2: full stat req ---
    let mut req = Vec::with_capacity(15);
    req.extend_from_slice(&[0xFE, 0xFD, 0x00]);
    req.extend_from_slice(&session_id.to_be_bytes());
    req.extend_from_slice(&token.to_be_bytes());
    req.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    timeout(dur, socket.send(&req)).await.ok()?.ok()?;

    let n = timeout(dur, socket.recv(&mut buf)).await.ok()?.ok()?;
    parse_full_stat(&buf[..n])
}

/// Resp handshake: type(0x09) + session(4) + ASCII \0.
fn parse_challenge(data: &[u8]) -> Option<i32> {
    if data.len() < 6 || data[0] != 0x09 {
        return None;
    }
    let rest = &data[5..];
    let end = rest.iter().position(|&b| b == 0).unwrap_or(rest.len());
    std::str::from_utf8(&rest[..end]).ok()?.trim().parse::<i32>().ok()
}

/// Full stat: type(0x00) + session(4) + `splitnum\0\x80\0` + KV + players.
fn parse_full_stat(data: &[u8]) -> Option<QueryData> {
    if data.len() < 5 || data[0] != 0x00 {
        return None;
    }
    let body = data[5..].strip_prefix(KV_PREFIX)?;

    // KV
    let mut cur = 0usize;
    let mut world = None;
    let mut plugins_raw = None;
    loop {
        let key = match read_cstr(body, &mut cur) {
            Some(k) if !k.is_empty() => k,
            _ => break,
        };
        let val = match read_cstr(body, &mut cur) {
            Some(v) => v,
            None => break,
        };
        match key.as_str() {
            "map" => world = Some(val),
            "plugins" => plugins_raw = Some(val),
            _ => {}
        }
    }

    // Players `\x01player_\0\0`
    let mut players = Vec::new();
    if let Some(pos) = find(body, PLAYER_PREFIX) {
        let mut pc = pos + PLAYER_PREFIX.len();
        while let Some(name) = read_cstr(body, &mut pc) {
            if name.is_empty() {
                break;
            }
            players.push(name);
        }
    }

    Some(QueryData {
        world: world.filter(|s| !s.is_empty()),
        plugins: parse_plugins(plugins_raw.as_deref()),
        players,
    })
}

fn parse_plugins(raw: Option<&str>) -> Vec<String> {
    let Some(raw) = raw else { return Vec::new() };
    let list = match raw.split_once(": ") {
        Some((_software, rest)) => rest,
        None => return Vec::new(),
    };
    list.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn read_cstr(data: &[u8], cur: &mut usize) -> Option<String> {
    let start = *cur;
    let mut i = start;
    while i < data.len() && data[i] != 0 {
        i += 1;
    }
    if i >= data.len() {
        return None;
    }
    *cur = i + 1;
    Some(String::from_utf8_lossy(&data[start..i]).into_owned())
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet(kv: &[(&str, &str)], players: &[&str]) -> Vec<u8> {
        let mut p = vec![0x00, 0, 0, 0, 1];
        p.extend_from_slice(KV_PREFIX);
        for (k, v) in kv {
            p.extend_from_slice(k.as_bytes());
            p.push(0);
            p.extend_from_slice(v.as_bytes());
            p.push(0);
        }
        p.push(0);
        p.extend_from_slice(PLAYER_PREFIX);
        for name in players {
            p.extend_from_slice(name.as_bytes());
            p.push(0);
        }
        p.push(0);
        p
    }

    #[test]
    fn parses_canonical_response() {
        let data = packet(
            &[
                ("hostname", "A Minecraft Server"),
                ("gametype", "SMP"),
                ("game_id", "MINECRAFT"),
                ("version", "1.4.2"),
                ("plugins", ""),
                ("map", "world"),
                ("numplayers", "2"),
                ("maxplayers", "20"),
                ("hostport", "25565"),
                ("hostip", "127.0.0.1"),
            ],
            &["barneygale", "Vivalahelvig"],
        );
        let q = parse_full_stat(&data).expect("should be parsed");
        assert_eq!(q.world.as_deref(), Some("world"));
        assert!(q.plugins.is_empty());
        assert_eq!(q.players, vec!["barneygale", "Vivalahelvig"]);
    }

    #[test]
    fn parses_plugins_list() {
        let data = packet(
            &[
                ("map", "survival"),
                ("plugins", "Paper on 1.20.1: EssentialsX 2.20; LuckPerms 5.4"),
            ],
            &[],
        );
        let q = parse_full_stat(&data).expect("should be parsed");
        assert_eq!(q.world.as_deref(), Some("survival"));
        assert_eq!(q.plugins, vec!["EssentialsX 2.20", "LuckPerms 5.4"]);
        assert!(q.players.is_empty());
    }

    #[test]
    fn parse_challenge_reads_token() {
        let mut data = vec![0x09, 0, 0, 0, 1];
        data.extend_from_slice(b"9513307\0");
        assert_eq!(parse_challenge(&data), Some(9513307));
    }
}
