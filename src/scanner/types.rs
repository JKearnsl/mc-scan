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
pub struct ModInfo {
    pub id: String,
    pub version: String,
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
    
    // --- Java extras (SLP status) ---
    /// data-URI base64 PNG 64×64.
    pub favicon: Option<String>,
    /// UUID players
    pub sample_ids: Vec<String>,
    /// secure chat (1.19+).
    pub secure_chat: Option<bool>,
    /// Mods Forge/NeoForge with versions.
    pub mods: Vec<ModInfo>,

    // --- Query full stat (enable-query=true) ---
    /// World name (`map`).
    pub world: Option<String>,
    /// Plugins list.
    pub plugins: Vec<String>,

    // --- Login ---
    /// online-mode (Some(true)) / offline-mode aka cracked (Some(false)) / undefined (None).
    pub online_mode: Option<bool>,

    // --- Bedrock extras (unconnected pong) ---
    /// Edition-string Bedrock (MCPE/MCEE).
    pub bedrock_edition: Option<String>,
    /// Server GUID.
    pub server_guid: Option<String>,
    /// Sub MOTD.
    pub sub_motd: Option<String>,
    /// Gamemode (Survival/Creative/…).
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
