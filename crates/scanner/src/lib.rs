mod bedrock;
pub mod export;
mod java;
pub mod limits;
mod login;
pub mod parse;
mod query;
pub mod types;

use futures::{Stream, StreamExt, stream};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::trace;
use types::{Edition, ScanConfig, ServerInfo};

#[derive(Debug)]
pub(crate) enum Miss {
    /// No usable connection or response (timeout, refused, unreachable, short read).
    Unreachable(&'static str),
    /// Bytes arrived but did not parse as the expected protocol.
    Unparsed(&'static str),
}

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
            if query_enabled {
                match query::probe(addr, timeout_ms).await {
                    Some(q) => {
                        info.world = q.world;
                        info.plugins = q.plugins;
                        // Prefer the query roster over the truncated SLP sample; the
                        // ids belong to that sample, so drop them with it.
                        if !q.players.is_empty() {
                            info.samples = q.players;
                            info.sample_ids.clear();
                        }
                    }
                    None => trace!(%addr, "query enrichment returned nothing"),
                }
            }
            if online_mode_check {
                info.online_mode = login::probe(addr, info.protocol, timeout_ms).await;
                trace!(%addr, online_mode = ?info.online_mode, "online-mode probe");
            }
            Some(info)
        }
        Edition::Bedrock => bedrock::probe(addr, timeout_ms).await,
    }
}

// A 0.0.0.0 socket can't connect to an IPv6 target, so match the family.
pub(crate) fn local_bind_addr(target: &SocketAddr) -> &'static str {
    if target.is_ipv6() {
        "[::]:0"
    } else {
        "0.0.0.0:0"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_bind_addr_matches_target_family() {
        assert_eq!(
            super::local_bind_addr(&"127.0.0.1:1".parse().unwrap()),
            "0.0.0.0:0"
        );
        assert_eq!(
            super::local_bind_addr(&"[::1]:1".parse().unwrap()),
            "[::]:0"
        );
    }
}
