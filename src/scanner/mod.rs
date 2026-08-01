mod java;
mod bedrock;
mod query;
mod login;
pub mod limits;
pub mod parse;
pub mod types;

use std::net::SocketAddr;
use std::sync::Arc;
use futures::{stream, Stream, StreamExt};
use types::{Edition, ScanConfig, ServerInfo};

pub fn scan(config: Arc<ScanConfig>) -> impl Stream<Item = Option<ServerInfo>> + Send + 'static {
    let timeout_ms = config.timeout_ms.get();
    let concurrency = config.concurrency.get();

    let mut ports: Vec<(u16, Edition)> =
        Vec::with_capacity(config.java_ports.len() + config.bedrock_ports.len());
    ports.extend(config.java_ports.iter().map(|&p| (p, Edition::Java)));
    ports.extend(config.bedrock_ports.iter().map(|&p| (p, Edition::Bedrock)));
    let ports = Arc::new(ports);

    let ranges = config.ranges.clone();

    // Lazy: a /8 or IPv6 range would be tens of millions of tuples if collected.
    let targets = ranges.into_iter().flat_map(move |net| {
        let ports = ports.clone();
        net.hosts().flat_map(move |ip| {
            let ports = ports.clone();
            (0..ports.len()).map(move |i| {
                let (port, edition) = ports[i].clone();
                (ip, port, edition)
            })
        })
    });

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

// A 0.0.0.0 socket can't connect to an IPv6 target, so match the family.
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
