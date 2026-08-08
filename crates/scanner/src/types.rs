use super::limits::{Concurrency, Ports, TimeoutMs};
use ipnet::IpNet;
use std::net::SocketAddr;

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
pub struct ModInfo {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub uuid: [u8; 16],
    pub access_token: String,
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
    pub samples: Vec<String>,
    pub ping_history: Vec<u64>,

    // Java extras (SLP status)
    pub favicon: Option<String>,
    pub sample_ids: Vec<String>,
    pub secure_chat: Option<bool>,
    pub mods: Vec<ModInfo>,

    // Query full stat (enable-query=true)
    pub world: Option<String>,
    pub plugins: Vec<String>,

    // Some(true)=online, Some(false)=cracked, None=undetermined
    pub online_mode: Option<bool>,
    pub whitelist: Option<bool>,

    // Bedrock extras (unconnected pong)
    pub bedrock_edition: Option<String>,
    pub server_guid: Option<String>,
    pub sub_motd: Option<String>,
    pub gamemode: Option<String>,
    pub port_v4: Option<u16>,
    pub port_v6: Option<u16>,
}

impl ServerInfo {
    pub fn base(addr: SocketAddr, edition: Edition) -> Self {
        Self {
            addr,
            edition,
            motd: String::new(),
            version: String::new(),
            protocol: 0,
            online: 0,
            max_players: 0,
            latency_ms: 0,
            samples: Vec::new(),
            ping_history: Vec::new(),
            favicon: None,
            sample_ids: Vec::new(),
            secure_chat: None,
            mods: Vec::new(),
            world: None,
            plugins: Vec::new(),
            online_mode: None,
            whitelist: None,
            bedrock_edition: None,
            server_guid: None,
            sub_motd: None,
            gamemode: None,
            port_v4: None,
            port_v6: None,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ScanConfig {
    pub ranges: Vec<IpNet>,
    pub java_ports: Ports,
    pub bedrock_ports: Ports,
    pub concurrency: Concurrency,
    pub timeout_ms: TimeoutMs,
}

impl ScanConfig {
    pub fn target_count(&self) -> usize {
        let ports = (self.java_ports.len() + self.bedrock_ports.len()) as u128;
        let hosts: u128 = self.ranges.iter().map(host_count).sum();
        hosts.saturating_mul(ports).min(usize::MAX as u128) as usize
    }
}

// Matches IpNet::hosts(): IPv4 excludes network+broadcast (except /31, /32).
pub fn host_count(net: &IpNet) -> u128 {
    match net {
        IpNet::V4(n) => match n.prefix_len() {
            32 => 1,
            31 => 2,
            p => (1u128 << (32 - p)) - 2,
        },
        IpNet::V6(n) => {
            let bits = 128 - n.prefix_len() as u32;
            if bits >= 128 {
                u128::MAX
            } else {
                1u128 << bits
            }
        }
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            ranges: vec![],
            java_ports: Ports::from_input("25565"),
            bedrock_ports: Ports::from_input("19132"),
            concurrency: Concurrency::default(),
            timeout_ms: TimeoutMs::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(ranges: &[&str]) -> ScanConfig {
        ScanConfig {
            ranges: ranges.iter().map(|s| s.parse().unwrap()).collect(),
            java_ports: Ports::from_input("25565"),
            bedrock_ports: Ports::from_input("19132"),
            concurrency: Concurrency::default(),
            timeout_ms: TimeoutMs::default(),
        }
    }

    #[test]
    fn host_count_ipv4_matches_hosts_iter() {
        assert_eq!(host_count(&"10.0.0.0/24".parse().unwrap()), 254);
        assert_eq!(host_count(&"10.0.0.0/30".parse().unwrap()), 2);
        assert_eq!(host_count(&"10.0.0.0/31".parse().unwrap()), 2);
        assert_eq!(host_count(&"10.0.0.5/32".parse().unwrap()), 1);
    }

    #[test]
    fn host_count_ipv6_includes_all() {
        assert_eq!(host_count(&"2001:db8::/126".parse().unwrap()), 4);
    }

    #[test]
    fn target_count_multiplies_ports_and_saturates() {
        assert_eq!(cfg(&["10.0.0.0/24"]).target_count(), 254 * 2);
        assert_eq!(cfg(&["2001:db8::/32"]).target_count(), usize::MAX);
    }
}
