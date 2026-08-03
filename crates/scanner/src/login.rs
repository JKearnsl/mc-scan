//! Detect online-mode / offline-mode (cracked) based on the server's response to Login Start.
//!
//! Send a handshake (next state = 2) and Login Start, read the FIRST packet
//! of the login phase and immediately terminate the connection—before any authentication:
//! - `0x01` Encryption Request => online-mode (Mojang authorization required)
//! - `0x02` Login Success / `0x03` Set Compression => offline-mode (cracked)
//! - `0x00` Disconnect / other => unknown (whitelist / ban / version)
//!

use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn probe(addr: SocketAddr, protocol: i32, timeout_ms: u64) -> Option<bool> {
    let dur = Duration::from_millis(timeout_ms);
    let mut stream = timeout(dur, TcpStream::connect(addr)).await.ok()?.ok()?;

    let handshake = {
        let mut payload = Vec::new();
        write_varint(&mut payload, 0x00); // packet id: handshake
        write_varint(&mut payload, protocol);
        write_string(&mut payload, &addr.ip().to_string());
        payload.extend_from_slice(&addr.port().to_be_bytes());
        write_varint(&mut payload, 2); // next state = login
        frame(payload)
    };
    timeout(dur, stream.write_all(&handshake))
        .await
        .ok()?
        .ok()?;

    let ls = build_login_start(env!("CARGO_PKG_NAME"), protocol);
    timeout(dur, stream.write_all(&ls)).await.ok()?.ok()?;

    let _len = timeout(dur, read_varint_stream(&mut stream)).await.ok()??;
    let id = timeout(dur, read_varint_stream(&mut stream)).await.ok()??;
    classify_login_packet(id)
}

fn classify_login_packet(id: i32) -> Option<bool> {
    match id {
        0x01 => Some(true),         // Encryption Request => online-mode
        0x02 | 0x03 => Some(false), // Login Success / Set Compression => offline-mode
        _ => None,                  // 0x00 Disconnect etc.
    }
}

async fn read_varint_stream(stream: &mut TcpStream) -> Option<i32> {
    let mut result = 0i32;
    let mut shift = 0u32;
    loop {
        let b = stream.read_u8().await.ok()?;
        result |= ((b & 0x7F) as i32) << shift;
        if b & 0x80 == 0 {
            return Some(result);
        }
        shift += 7;
        if shift >= 35 {
            return None;
        }
    }
}

fn build_login_start(name: &str, protocol: i32) -> Vec<u8> {
    let mut payload = Vec::new();
    write_varint(&mut payload, 0x00); // packet id: Login Start
    write_string(&mut payload, name);

    if protocol >= 764 {
        // 1.20.2+ : имя + UUID (без флага)
        payload.extend_from_slice(&offline_uuid(name));
    } else if protocol >= 761 {
        // 1.19.3 – 1.20.1 : имя + Bool(has_uuid) + UUID
        payload.push(0x01);
        payload.extend_from_slice(&offline_uuid(name));
    } else if protocol >= 759 {
        // 1.19 – 1.19.2 : имя + Bool(has_sig=false) + Bool(has_uuid) + UUID
        payload.push(0x00);
        payload.push(0x01);
        payload.extend_from_slice(&offline_uuid(name));
    }
    // < 1.19 : только имя

    frame(payload)
}

fn offline_uuid(name: &str) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut h: u64 = 0xcbf29ce484222325;
    for (i, b) in name.bytes().enumerate() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
        out[i % 16] ^= (h >> (8 * (i % 8))) as u8;
    }
    out[6] = (out[6] & 0x0F) | 0x40;
    out[8] = (out[8] & 0x3F) | 0x80;
    out
}

fn frame(payload: Vec<u8>) -> Vec<u8> {
    let mut packet = Vec::with_capacity(payload.len() + 3);
    write_varint(&mut packet, payload.len() as i32);
    packet.extend_from_slice(&payload);
    packet
}

fn write_varint(buf: &mut Vec<u8>, value: i32) {
    let mut v = value as u32;
    loop {
        if v & !0x7F == 0 {
            buf.push(v as u8);
            return;
        }
        buf.push((v & 0x7F | 0x80) as u8);
        v >>= 7;
    }
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    write_varint(buf, s.len() as i32);
    buf.extend_from_slice(s.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_maps_packet_ids() {
        assert_eq!(classify_login_packet(0x01), Some(true)); // Encryption Request
        assert_eq!(classify_login_packet(0x02), Some(false)); // Login Success
        assert_eq!(classify_login_packet(0x03), Some(false)); // Set Compression
        assert_eq!(classify_login_packet(0x00), None); // Disconnect
    }

    #[tokio::test]
    async fn reads_varints_arriving_in_separate_segments() {
        use tokio::io::AsyncWriteExt;
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await.unwrap();
            for byte in [0x01u8, 0x01] {
                let _ = s.write_all(&[byte]).await;
                let _ = s.flush().await;
            }
        });
        let mut client = TcpStream::connect(addr).await.unwrap();
        let len = read_varint_stream(&mut client).await;
        let id = read_varint_stream(&mut client).await;
        let _ = server.await;

        assert_eq!(len, Some(1));
        assert_eq!(classify_login_packet(id.unwrap()), Some(true));
    }

    #[test]
    fn login_start_shapes_per_version() {
        // 1.20.2
        let p764 = build_login_start("Scanner", 764);
        // frame: len varint + [id + strlen + "Scanner"(7) + uuid(16)]
        // payload = 1 + 1 + 7 + 16 = 25
        assert_eq!(p764[0], 25);
        // 1.19.3
        let p761 = build_login_start("Scanner", 761);
        assert_eq!(p761[0], 26);
        // < 1.19
        let p47 = build_login_start("Scanner", 47);
        assert_eq!(p47[0], 9);
    }
}
