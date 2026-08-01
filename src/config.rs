//! Persistence of user settings to a JSON file in the OS config directory
//! (e.g. `~/.config/mc-scan/config.json` on Linux).

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::i18n::Language;

/// Persisted settings. `#[serde(default)]` lets an older or partial file load —
/// any missing field falls back to its default rather than failing the whole read.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub ranges: Vec<String>,
    pub java_ports: String,
    pub bedrock_ports: String,
    pub concurrency: String,
    pub timeout_ms: String,
    pub query_enabled: bool,
    pub online_mode_check: bool,
    pub is_dark: bool,
    /// Language code ("en"/"ru"/"zh"/"ja"); empty means "detect from locale".
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ranges: Vec::new(),
            java_ports: "25565".into(),
            bedrock_ports: "19132".into(),
            concurrency: "1024".into(),
            timeout_ms: "1500".into(),
            query_enabled: true,
            online_mode_check: false,
            is_dark: true,
            language: String::new(),
        }
    }
}

pub fn language_code(lang: Language) -> &'static str {
    match lang {
        Language::English => "en",
        Language::Russian => "ru",
        Language::Chinese => "zh",
        Language::Japanese => "ja",
    }
}

pub fn language_from_code(code: &str) -> Option<Language> {
    match code {
        "en" => Some(Language::English),
        "ru" => Some(Language::Russian),
        "zh" => Some(Language::Chinese),
        "ja" => Some(Language::Japanese),
        _ => None,
    }
}

fn config_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "mc-scan").map(|d| d.config_dir().join("config.json"))
}

/// Read the saved config, or `None` if there is no readable/valid file yet.
pub fn load() -> Option<Config> {
    let bytes = std::fs::read(config_path()?).ok()?;
    serde_json::from_slice(&bytes).ok()
}

/// Best-effort write; failures (no config dir, read-only fs) are ignored so a
/// persistence problem never breaks the app.
pub fn save(config: &Config) {
    let Some(path) = config_path() else { return };
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = std::fs::write(path, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_code_round_trips() {
        for lang in [
            Language::English,
            Language::Russian,
            Language::Chinese,
            Language::Japanese,
        ] {
            assert_eq!(language_from_code(language_code(lang)), Some(lang));
        }
        assert_eq!(language_from_code(""), None);
        assert_eq!(language_from_code("xx"), None);
    }

    #[test]
    fn config_survives_a_json_round_trip() {
        let cfg = Config {
            ranges: vec!["10.0.0.0/24".into(), "1.2.3.4/32".into()],
            concurrency: "2048".into(),
            is_dark: false,
            language: "ru".into(),
            ..Config::default()
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(back.ranges, cfg.ranges);
        assert_eq!(back.concurrency, "2048");
        assert!(!back.is_dark);
        assert_eq!(back.language, "ru");
    }

    #[test]
    fn partial_json_fills_defaults() {
        // A file written by an older build with only some keys still loads.
        let back: Config = serde_json::from_str(r#"{"is_dark": false}"#).unwrap();
        assert!(!back.is_dark);
        assert_eq!(back.java_ports, "25565"); // default filled in
        assert!(back.query_enabled);
    }
}
