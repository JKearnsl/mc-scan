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

    let socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;
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
    let parts: Vec<&str> = raw.split(';').collect();
    if parts.len() < 6 {
        return None;
    }
    Some(ServerInfo {
        addr,
        edition: Edition::Bedrock,
        motd: super::strip_section_codes(parts[1]),
        protocol: parts[2].parse().unwrap_or(0),
        version: parts[3].to_string(),
        online: parts[4].parse().unwrap_or(0),
        max_players: parts[5].parse().unwrap_or(0),
        latency_ms,
    })
}
