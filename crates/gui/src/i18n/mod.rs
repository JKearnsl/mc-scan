mod ru;
mod en;
mod zh;
mod ja;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Russian,
    Chinese,
    Japanese,
}

impl Language {
    pub fn from_locale(locale: &str) -> Self {
        let l = locale.to_lowercase();
        if l.starts_with("ru") {
            Language::Russian
        } else if l.starts_with("zh") {
            Language::Chinese
        } else if l.starts_with("ja") {
            Language::Japanese
        } else {
            Language::English
        }
    }

    pub fn detect() -> Self {
        sys_locale::get_locale()
            .as_deref()
            .map(Self::from_locale)
            .unwrap_or(Language::English)
    }
}

pub struct Tr {
    pub theme: &'static str,
    pub dark: &'static str,
    pub light: &'static str,
    pub system: &'static str,
    pub language: &'static str,
    pub ports: &'static str,
    pub parameters: &'static str,
    pub threads: &'static str,
    pub timeout_ms: &'static str,
    pub subtitle: &'static str,
    pub found: &'static str,
    pub ip_ranges: &'static str,
    pub total_addresses: &'static str,
    pub scan: &'static str,
    pub stop: &'static str,
    pub add_ranges_title: &'static str,
    pub add_ranges_hint: &'static str,
    pub ranges_rejected: &'static str,
    pub add: &'static str,
    pub cancel: &'static str,
    pub scanning: &'static str,
    pub settings: &'static str,
    pub results_empty: &'static str,
    pub players: &'static str,
    pub ping: &'static str,
    pub version: &'static str,
    pub online: &'static str,
    pub address: &'static str,
    pub copy: &'static str,
    pub copied: &'static str,
    pub protocol: &'static str,
    pub software: &'static str,
    pub edition: &'static str,
    pub motd: &'static str,
    pub players_online: &'static str,
    pub latency: &'static str,
    pub java_edition: &'static str,
    pub bedrock_edition: &'static str,
    pub mods: &'static str,
    pub gamemode: &'static str,
    pub secure_chat: &'static str,
    pub enabled: &'static str,
    pub disabled: &'static str,
    pub world: &'static str,
    pub plugins: &'static str,
    pub enrichment: &'static str,
    pub query_label: &'static str,
    pub online_mode_label: &'static str,
    pub online_mode: &'static str,
    pub online_yes: &'static str,
    pub online_no: &'static str,
    pub search: &'static str,
    pub no_matches: &'static str,
    pub sort_recent: &'static str,
    pub sort_players: &'static str,
    pub sort_ping: &'static str,
    pub filter_all: &'static str,
    pub filter_java: &'static str,
    pub filter_bedrock: &'static str,
    pub sort_by: &'static str,
    pub direction: &'static str,
    pub dir_asc: &'static str,
    pub dir_desc: &'static str,
    pub filters_title: &'static str,
    pub reset: &'static str,
    pub online_any: &'static str,
    pub version_hint: &'static str,
    pub plugin_hint: &'static str,
}

pub fn tr(lang: Language) -> &'static Tr {
    match lang {
        Language::English => &en::EN,
        Language::Russian => &ru::RU,
        Language::Chinese => &zh::ZH,
        Language::Japanese => &ja::JA,
    }
}
