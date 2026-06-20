use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use serde_json::Value;
use super::types::{Edition, ServerInfo};

pub async fn probe(addr: SocketAddr, timeout_ms: u64) -> Option<ServerInfo> {
    let dur = Duration::from_millis(timeout_ms);
    let start = Instant::now();

    let mut stream = timeout(dur, TcpStream::connect(addr)).await.ok()?.ok()?;

    let handshake = build_handshake(&addr.ip().to_string(), addr.port());
    stream.write_all(&handshake).await.ok()?;
    stream.write_all(&[0x01, 0x00]).await.ok()?;

    let json: Value = timeout(dur, read_response(&mut stream)).await.ok()??;
    let latency_ms = start.elapsed().as_millis() as u64;

    Some(ServerInfo {
        addr,
        edition: Edition::Java,
        motd: parse_description(&json["description"]),
        version: json["version"]["name"].as_str().unwrap_or("").to_string(),
        protocol: json["version"]["protocol"].as_i64().unwrap_or(0) as i32,
        online: json["players"]["online"].as_u64().unwrap_or(0) as u32,
        max_players: json["players"]["max"].as_u64().unwrap_or(0) as u32,
        latency_ms,
    })
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

async fn read_response(stream: &mut TcpStream) -> Option<Value> {
    let _len = read_varint(stream).await?;
    if read_varint(stream).await? != 0x00 {
        return None;
    }
    let str_len = read_varint(stream).await? as usize;
    let mut buf = vec![0u8; str_len];
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

fn parse_description(v: &Value) -> String {
    match v {
        Value::String(s) => super::strip_section_codes(s),
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
