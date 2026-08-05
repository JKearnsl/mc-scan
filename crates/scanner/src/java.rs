use super::Miss;
use super::types::{Edition, ModInfo, ServerInfo};
use serde_json::Value;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::{debug, trace};

pub async fn probe(addr: SocketAddr, timeout_ms: u64) -> Option<ServerInfo> {
    match probe_inner(addr, timeout_ms).await {
        Ok(info) => {
            debug!(%addr, edition = "java", version = %info.version, online = info.online, "found");
            Some(info)
        }
        // Response arrived but didn't parse: notable (parser bug or odd server).
        Err(Miss::Unparsed(stage)) => {
            debug!(%addr, edition = "java", stage, "response did not parse");
            None
        }
        // No/short connection: the common case on a wide scan, kept at trace.
        Err(Miss::Unreachable(stage)) => {
            trace!(%addr, edition = "java", stage, "unreachable");
            None
        }
    }
}

async fn probe_inner(addr: SocketAddr, timeout_ms: u64) -> Result<ServerInfo, Miss> {
    let dur = Duration::from_millis(timeout_ms);
    let start = Instant::now();

    let mut stream = timeout(dur, TcpStream::connect(addr))
        .await
        .map_err(|_| Miss::Unreachable("connect_timeout"))?
        .map_err(|_| Miss::Unreachable("connect"))?;

    let handshake = build_handshake(&addr.ip().to_string(), addr.port());
    stream
        .write_all(&handshake)
        .await
        .map_err(|_| Miss::Unreachable("write"))?;
    stream
        .write_all(&[0x01, 0x00])
        .await
        .map_err(|_| Miss::Unreachable("write"))?;

    let json: Value = timeout(dur, read_response(&mut stream))
        .await
        .map_err(|_| Miss::Unreachable("read_timeout"))??;
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
    Ok(info)
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

/// Hard cap on the SLP status JSON, checked before allocating for it. Bounds a
/// hostile length: a huge one would OOM, a negative VarInt widened via `as usize`
/// would abort on capacity overflow. Real statuses stay far below 4 MiB.
const MAX_STATUS_BYTES: usize = 4 * 1024 * 1024;

async fn read_response(stream: &mut TcpStream) -> Result<Value, Miss> {
    // Buffer the response so each VarInt byte isn't a separate await/syscall.
    let mut reader = BufReader::new(stream);
    let _len = read_varint(&mut reader)
        .await
        .ok_or(Miss::Unreachable("len"))?;
    if read_varint(&mut reader)
        .await
        .ok_or(Miss::Unreachable("packet_id"))?
        != 0x00
    {
        return Err(Miss::Unparsed("packet_id"));
    }
    let str_len = read_varint(&mut reader)
        .await
        .ok_or(Miss::Unreachable("str_len"))?;
    if str_len < 0 || str_len as usize > MAX_STATUS_BYTES {
        return Err(Miss::Unparsed("str_len_bounds"));
    }
    let mut buf = vec![0u8; str_len as usize];
    reader
        .read_exact(&mut buf)
        .await
        .map_err(|_| Miss::Unreachable("body"))?;
    serde_json::from_slice(&buf).map_err(|_| Miss::Unparsed("json"))
}

async fn read_varint<R: AsyncRead + Unpin>(reader: &mut R) -> Option<i32> {
    let mut result = 0i32;
    let mut shift = 0u32;
    loop {
        let byte = reader.read_u8().await.ok()?;
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
                Some(ModInfo {
                    id: id.to_string(),
                    version: version.to_string(),
                })
            })
            .collect();
    }
    if let Some(arr) = json["modinfo"]["modList"].as_array() {
        return arr
            .iter()
            .filter_map(|m| {
                let id = m["modid"].as_str()?;
                let version = m["version"].as_str().unwrap_or("");
                Some(ModInfo {
                    id: id.to_string(),
                    version: version.to_string(),
                })
            })
            .collect();
    }
    Vec::new()
}

/// Flattens the description component tree into plain text, keeping the raw
/// `§` formatting codes intact. Stripping them is a display concern handled by
/// the GUI at render time, so the CSV export retains the original codes.
fn parse_description(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(arr) => arr.iter().map(parse_description).collect(),
        Value::Object(map) => {
            let text = map
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let extras = map
                .get("extra")
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
    async fn read_response_of(bytes: Vec<u8>) -> Result<Value, Miss> {
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
        assert!(matches!(
            read_response_of(bytes).await,
            Err(Miss::Unparsed("str_len_bounds"))
        ));
    }

    #[tokio::test]
    async fn rejects_oversized_status_length() {
        // str_len = VarInt(i32::MAX) = FF FF FF FF 07 (~2 GiB) => rejected by cap.
        let bytes = vec![0x05, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x07];
        assert!(matches!(
            read_response_of(bytes).await,
            Err(Miss::Unparsed("str_len_bounds"))
        ));
    }

    #[tokio::test]
    async fn parses_within_cap() {
        let json = br#"{"players":{"online":3,"max":20}}"#;
        let v = read_response_of(framed_status(json))
            .await
            .expect("should parse");
        assert_eq!(v["players"]["online"].as_u64(), Some(3));
    }

    #[test]
    fn description_array_is_concatenated() {
        use serde_json::json;
        assert_eq!(parse_description(&json!(["a", "b"])), "ab");
        assert_eq!(
            parse_description(&json!([{"text": "x"}, {"text": "y"}])),
            "xy"
        );
    }

    #[test]
    fn description_keeps_raw_codes_and_walks_extra() {
        use serde_json::json;
        // Formatting codes are preserved verbatim; the GUI strips them for display.
        assert_eq!(parse_description(&json!("§aHello")), "§aHello");
        assert_eq!(
            parse_description(&json!({"text": "A", "extra": [{"text": "B"}, "C"]})),
            "ABC"
        );
        assert_eq!(parse_description(&json!(42)), "");
    }

    async fn varint_roundtrip(v: i32) -> Option<i32> {
        let mut buf = Vec::new();
        write_varint(&mut buf, v);
        let mut slice: &[u8] = &buf;
        read_varint(&mut slice).await
    }

    #[tokio::test]
    async fn varint_roundtrips_across_boundaries() {
        for v in [0, 1, 127, 128, 255, 2097151, i32::MAX, -1, i32::MIN] {
            assert_eq!(varint_roundtrip(v).await, Some(v), "roundtrip {v}");
        }
    }

    #[test]
    fn write_varint_matches_protocol_encoding() {
        let enc = |v| {
            let mut b = Vec::new();
            write_varint(&mut b, v);
            b
        };
        assert_eq!(enc(0), [0x00]);
        assert_eq!(enc(1), [0x01]);
        assert_eq!(enc(127), [0x7F]);
        assert_eq!(enc(128), [0x80, 0x01]);
        assert_eq!(enc(300), [0xAC, 0x02]);
        assert_eq!(enc(-1), [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    }

    #[tokio::test]
    async fn read_varint_rejects_overlong_encoding() {
        // Six continuation bytes exceed the 5-byte VarInt limit.
        let mut slice: &[u8] = &[0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        assert!(read_varint(&mut slice).await.is_none());
    }
}
