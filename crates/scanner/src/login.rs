use crate::types::Credentials;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

const MAX_DISCONNECT_LEN: usize = 32 * 1024;

#[derive(Debug, Default, PartialEq)]
pub struct LoginOutcome {
    pub online_mode: Option<bool>,
    pub whitelist: Option<bool>,
}

pub async fn probe(
    addr: SocketAddr,
    protocol: i32,
    timeout_ms: u64,
    creds: Option<&Credentials>,
) -> LoginOutcome {
    probe_inner(addr, protocol, timeout_ms, creds)
        .await
        .unwrap_or_default()
}

async fn probe_inner(
    addr: SocketAddr,
    protocol: i32,
    timeout_ms: u64,
    _creds: Option<&Credentials>,
) -> Option<LoginOutcome> {
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

    let len = timeout(dur, read_varint_stream(&mut stream)).await.ok()??;
    let payload = read_packet_payload(&mut stream, len, dur).await?;
    let mut cur = &payload[..];
    let id = read_varint_buf(&mut cur)?;
    Some(classify_login_packet(id, cur))
}

fn classify_login_packet(id: i32, payload: &[u8]) -> LoginOutcome {
    match id {
        0x01 => LoginOutcome {
            online_mode: Some(true),
            whitelist: None,
        },
        0x02 | 0x03 => LoginOutcome {
            online_mode: Some(false),
            whitelist: Some(false),
        },
        0x00 if disconnect_is_whitelist(payload) => LoginOutcome {
            online_mode: Some(false),
            whitelist: Some(true),
        },
        _ => LoginOutcome::default(),
    }
}

fn disconnect_is_whitelist(payload: &[u8]) -> bool {
    let mut cur = payload;
    let Some(len) = read_varint_buf(&mut cur) else {
        return false;
    };
    let len = len.max(0) as usize;
    let bytes = &cur[..len.min(cur.len())];
    let msg = String::from_utf8_lossy(bytes).to_lowercase();
    const NEEDLES: [&str; 5] = [
        "not_whitelisted",
        "whitelist",
        "white-list",
        "white list",
        "белом списке",
    ];
    NEEDLES.iter().any(|n| msg.contains(n))
}

async fn read_packet_payload(stream: &mut TcpStream, len: i32, dur: Duration) -> Option<Vec<u8>> {
    let len = usize::try_from(len).ok()?.min(MAX_DISCONNECT_LEN);
    let mut buf = vec![0u8; len];
    timeout(dur, stream.read_exact(&mut buf)).await.ok()?.ok()?;
    Some(buf)
}

fn read_varint_buf(buf: &mut &[u8]) -> Option<i32> {
    let mut result = 0i32;
    let mut shift = 0u32;
    loop {
        let (&b, rest) = buf.split_first()?;
        *buf = rest;
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

    fn disconnect_payload(reason: &str) -> Vec<u8> {
        let mut p = Vec::new();
        write_string(&mut p, reason);
        p
    }

    #[test]
    fn classify_maps_packet_ids() {
        assert_eq!(
            classify_login_packet(0x01, &[]),
            LoginOutcome {
                online_mode: Some(true),
                whitelist: None
            }
        );
        let offline = LoginOutcome {
            online_mode: Some(false),
            whitelist: Some(false),
        };
        assert_eq!(classify_login_packet(0x02, &[]), offline);
        assert_eq!(classify_login_packet(0x03, &[]), offline);
    }

    #[test]
    fn disconnect_without_whitelist_reason_is_undetermined() {
        let p = disconnect_payload("{\"text\":\"Server is full\"}");
        assert_eq!(classify_login_packet(0x00, &p), LoginOutcome::default());
        assert_eq!(classify_login_packet(0x00, &[]), LoginOutcome::default());
    }

    #[test]
    fn whitelist_disconnect_marks_offline_and_whitelisted() {
        let vanilla = disconnect_payload("{\"translate\":\"multiplayer.disconnect.not_whitelisted\"}");
        let plugin = disconnect_payload("{\"text\":\"You are not white-listed on this server!\"}");
        let russian = disconnect_payload("{\"text\":\"Вас нет в белом списке\"}");
        let expected = LoginOutcome {
            online_mode: Some(false),
            whitelist: Some(true),
        };
        assert_eq!(classify_login_packet(0x00, &vanilla), expected);
        assert_eq!(classify_login_packet(0x00, &plugin), expected);
        assert_eq!(classify_login_packet(0x00, &russian), expected);
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
        assert_eq!(classify_login_packet(id.unwrap(), &[]).online_mode, Some(true));
    }

    #[test]
    fn login_start_shapes_per_version() {
        // 1.20.2+ payload = id(1) + strlen(1) + "Scanner"(7) + uuid(16) = 25
        let p764 = build_login_start("Scanner", 764);
        assert_eq!(p764[0], 25);
        let p761 = build_login_start("Scanner", 761);
        assert_eq!(p761[0], 26);
        let p47 = build_login_start("Scanner", 47);
        assert_eq!(p47[0], 9);
    }
}
