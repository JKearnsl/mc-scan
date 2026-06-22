mod java;
mod bedrock;
pub mod parse;
pub mod types;

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use futures::{stream, Stream, StreamExt};
use types::{Edition, ScanConfig, ServerInfo};

pub fn scan(config: Arc<ScanConfig>) -> impl Stream<Item = Option<ServerInfo>> + Send + 'static {
    let timeout_ms = config.timeout_ms;
    let concurrency = config.concurrency;
    let java_ports = config.java_ports.clone();
    let bedrock_ports = config.bedrock_ports.clone();

    let targets: Vec<(IpAddr, u16, Edition)> = config
        .ranges
        .iter()
        .flat_map(|r| r.hosts())
        .flat_map(|ip| {
            let j = java_ports.iter().map(move |&p| (ip, p, Edition::Java));
            let b = bedrock_ports.iter().map(move |&p| (ip, p, Edition::Bedrock));
            j.chain(b)
        })
        .collect();

    stream::iter(targets)
        .map(move |(ip, port, edition)| async move {
            let addr = SocketAddr::new(ip, port);
            match edition {
                Edition::Java => java::probe(addr, timeout_ms).await,
                Edition::Bedrock => bedrock::probe(addr, timeout_ms).await,
            }
        })
        .buffer_unordered(concurrency)
}

pub(super) fn strip_section_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\u{00A7}' {
            chars.next();
        } else {
            result.push(c);
        }
    }
    result
}
