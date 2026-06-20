use std::net::SocketAddr;
use ipnet::IpNet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Edition {
    Java,
    Bedrock,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edition::Java => write!(f, "J"),
            Edition::Bedrock => write!(f, "B"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub addr: SocketAddr,
    pub edition: Edition,
    pub motd: String,
    pub version: String,
    pub protocol: i32,
    pub online: u32,
    pub max_players: u32,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Hash)]
pub struct ScanConfig {
    pub ranges: Vec<IpNet>,
    pub java_ports: Vec<u16>,
    pub bedrock_ports: Vec<u16>,
    pub concurrency: usize,
    pub timeout_ms: u64,
}

impl ScanConfig {
    pub fn target_count(&self) -> usize {
        let hosts: usize = self.ranges.iter().map(|r| r.hosts().count()).sum();
        hosts * (self.java_ports.len() + self.bedrock_ports.len())
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            ranges: vec![],
            java_ports: vec![25565],
            bedrock_ports: vec![19132],
            concurrency: 1024,
            timeout_ms: 1500,
        }
    }
}
