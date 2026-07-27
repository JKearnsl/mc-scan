mod java;
mod bedrock;
mod query;
mod login;
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

pub async fn probe_server(
    addr: std::net::SocketAddr,
    edition: types::Edition,
    timeout_ms: u64,
    query_enabled: bool,
    online_mode_check: bool,
) -> Option<types::ServerInfo> {
    match edition {
        Edition::Java => {
            let mut info = java::probe(addr, timeout_ms).await?;
            // Query
            if query_enabled {
                if let Some(q) = query::probe(addr, timeout_ms).await {
                    info.world = q.world;
                    info.plugins = q.plugins;
                    // Query отдаёт полный список игроков, а SLP — лишь усечённый sample.
                    if !q.players.is_empty() {
                        info.samples = q.players;
                        info.sample_ids.clear();
                    }
                }
            }
            // Detect online/offline-mode
            if online_mode_check {
                info.online_mode = login::probe(addr, info.protocol, timeout_ms).await;
            }
            Some(info)
        }
        Edition::Bedrock => bedrock::probe(addr, timeout_ms).await,
    }
}

/// Local bind address for an outbound UDP probe, matched to the target's
/// address family. Binding `0.0.0.0` (IPv4) and then connecting to an IPv6
/// target fails, so IPv6 UDP probes must bind `[::]`.
pub(super) fn local_bind_addr(target: &SocketAddr) -> &'static str {
    if target.is_ipv6() { "[::]:0" } else { "0.0.0.0:0" }
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
