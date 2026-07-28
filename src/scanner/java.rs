use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use serde_json::Value;
use super::types::{Edition, ModInfo, ServerInfo};

pub async fn probe(addr: SocketAddr, timeout_ms: u64) -> Option<ServerInfo> {
    let dur = Duration::from_millis(timeout_ms);
    let start = Instant::now();

    let mut stream = timeout(dur, TcpStream::connect(addr)).await.ok()?.ok()?;

    let handshake = build_handshake(&addr.ip().to_string(), addr.port());
    stream.write_all(&handshake).await.ok()?;
    stream.write_all(&[0x01, 0x00]).await.ok()?;

    let json: Value = timeout(dur, read_response(&mut stream)).await.ok()??;
    let latency_ms = start.elapsed().as_millis() as u64;

    let (samples, sample_ids) = parse_samples(&json["players"]["sample"]);

    let mut info = ServerInfo::base(addr, Edition::Java);
    info.motd = parse_description(&json["description"]);
    info.version = json["version"]["name"].as_str().unwrap_or("").to_string();
    info.protocol = json["version"]["protocol"].as_i64().unwrap_or(0) as i32;
    info.online = json["players"]["online"].as_u64().unwrap_or(0) as u32;
    info.max_players = json["players"]["max"].as_u64().unwrap_or(0) as u32;
    info.latency_ms = latency_ms;
    info.samples = samples;
    info.sample_ids = sample_ids;
    info.ping_history = vec![latency_ms];
    info.favicon = json["favicon"].as_str().map(|s| s.to_string());
    info.secure_chat = json["enforcesSecureChat"].as_bool();
    info.mods = parse_mods(&json);
    Some(info)
}

fn build_handshake(host: &str, port: u16) -> Vec<u8> {
    let mut payload = Vec::new();
    write_varint(&mut payload, 0x00);
    write_varint(&mut payload, -1);
    write_string(&mut payload, host);
    payload.extend_from_slice(&port.to_be_bytes());
    write_varint(&mut payload, 1);

    let mut packet = Vec::new();
    write_varint(&mut packet, payload.len() as i32);
    packet.extend_from_slice(&payload);
    packet
}

/// Hard cap on the SLP status JSON. A well-formed status (even with a 64×64
/// favicon and a full player sample) stays well under this. The cap protects
/// against a malicious/broken server sending a huge or negative length, which
/// would otherwise allocate multiple gigabytes (OOM) or, for a negative VarInt
/// widened via `as usize`, abort the process on a capacity overflow.
const MAX_STATUS_BYTES: usize = 4 * 1024 * 1024;

async fn read_response(stream: &mut TcpStream) -> Option<Value> {
    let _len = read_varint(stream).await?;
    if read_varint(stream).await? != 0x00 {
        return None;
    }
    let str_len = read_varint(stream).await?;
    if str_len < 0 || str_len as usize > MAX_STATUS_BYTES {
        return None;
    }
    let mut buf = vec![0u8; str_len as usize];
    stream.read_exact(&mut buf).await.ok()?;
    serde_json::from_slice(&buf).ok()
}

async fn read_varint(stream: &mut TcpStream) -> Option<i32> {
    let mut result = 0i32;
    let mut shift = 0u32;
    loop {
        let byte = stream.read_u8().await.ok()?;
        result |= ((byte & 0x7F) as i32) << shift;
        if byte & 0x80 == 0 {
            return Some(result);
        }
        shift += 7;
        if shift >= 35 {
            return None;
        }
    }
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
    let bytes = s.as_bytes();
    write_varint(buf, bytes.len() as i32);
    buf.extend_from_slice(bytes);
}

fn parse_samples(v: &Value) -> (Vec<String>, Vec<String>) {
    let mut names = Vec::new();
    let mut ids = Vec::new();
    if let Some(arr) = v.as_array() {
        for p in arr {
            if let Some(name) = p["name"].as_str() {
                names.push(name.to_string());
                ids.push(p["id"].as_str().unwrap_or("").to_string());
            }
        }
    }
    (names, ids)
}

/// Mods list from `forgeData.mods[]` (FML2/NeoForge) or `modinfo.modList[]` (old FML).
fn parse_mods(json: &Value) -> Vec<ModInfo> {
    if let Some(arr) = json["forgeData"]["mods"].as_array() {
        return arr
            .iter()
            .filter_map(|m| {
                let id = m["modId"].as_str()?;
                let version = m["modmarker"]
                    .as_str()
                    .or_else(|| m["version"].as_str())
                    .unwrap_or("");
                Some(ModInfo { id: id.to_string(), version: version.to_string() })
            })
            .collect();
    }
    if let Some(arr) = json["modinfo"]["modList"].as_array() {
        return arr
            .iter()
            .filter_map(|m| {
                let id = m["modid"].as_str()?;
                let version = m["version"].as_str().unwrap_or("");
                Some(ModInfo { id: id.to_string(), version: version.to_string() })
            })
            .collect();
    }
    Vec::new()
}

fn parse_description(v: &Value) -> String {
    match v {
        Value::String(s) => super::strip_section_codes(s),
        Value::Array(arr) => arr.iter().map(parse_description).collect(),
        Value::Object(map) => {
            let text = map.get("text")
                .and_then(|v| v.as_str())
                .map(super::strip_section_codes)
                .unwrap_or_default();
            let extras = map.get("extra")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(parse_description).collect::<String>())
                .unwrap_or_default();
            format!("{}{}", text, extras)
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    /// Serve `bytes` to a single client and run `read_response` against it.
    async fn read_response_of(bytes: Vec<u8>) -> Option<Value> {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await.unwrap();
            let _ = s.write_all(&bytes).await;
        });
        let mut client = TcpStream::connect(addr).await.unwrap();
        let out = read_response(&mut client).await;
        let _ = server.await;
        out
    }

    fn framed_status(json: &[u8]) -> Vec<u8> {
        let mut inner = vec![0x00]; // packet id
        write_varint(&mut inner, json.len() as i32);
        inner.extend_from_slice(json);
        let mut packet = Vec::new();
        write_varint(&mut packet, inner.len() as i32);
        packet.extend_from_slice(&inner);
        packet
    }

    #[tokio::test]
    async fn rejects_negative_status_length() {
        // len=5, id=0x00, str_len = VarInt(-1) = FF FF FF FF 0F.
        // Without the guard this aborts the process on `vec!` capacity overflow.
        let bytes = vec![0x05, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F];
        assert!(read_response_of(bytes).await.is_none());
    }

    #[tokio::test]
    async fn rejects_oversized_status_length() {
        // str_len = VarInt(i32::MAX) = FF FF FF FF 07 (~2 GiB) => rejected by cap.
        let bytes = vec![0x05, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x07];
        assert!(read_response_of(bytes).await.is_none());
    }

    #[tokio::test]
    async fn parses_within_cap() {
        let json = br#"{"players":{"online":3,"max":20}}"#;
        let v = read_response_of(framed_status(json)).await.expect("should parse");
        assert_eq!(v["players"]["online"].as_u64(), Some(3));
    }

    #[test]
    fn description_array_is_concatenated() {
        use serde_json::json;
        assert_eq!(parse_description(&json!(["a", "b"])), "ab");
        assert_eq!(parse_description(&json!([{"text": "x"}, {"text": "y"}])), "xy");
    }
}
