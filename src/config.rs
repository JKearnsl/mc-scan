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
    /// Theme preference code ("system"/"dark"/"light").
    pub theme: String,
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
            theme: ThemePref::System.code().into(),
            language: String::new(),
        }
    }
}

/// The user's theme choice. `System` follows the OS color scheme; the other two
/// pin it regardless of the OS.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ThemePref {
    #[default]
    System,
    Dark,
    Light,
}

impl ThemePref {
    pub fn code(self) -> &'static str {
        match self {
            ThemePref::System => "system",
            ThemePref::Dark => "dark",
            ThemePref::Light => "light",
        }
    }

    /// Parse a persisted code, defaulting to `System` for anything unrecognized.
    pub fn from_code(code: &str) -> Self {
        match code {
            "dark" => ThemePref::Dark,
            "light" => ThemePref::Light,
            _ => ThemePref::System,
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
    fn theme_pref_code_round_trips() {
        for pref in [ThemePref::System, ThemePref::Dark, ThemePref::Light] {
            assert_eq!(ThemePref::from_code(pref.code()), pref);
        }
        assert_eq!(ThemePref::from_code("nonsense"), ThemePref::System);
        assert_eq!(ThemePref::default(), ThemePref::System);
    }

    #[test]
    fn config_survives_a_json_round_trip() {
        let cfg = Config {
            ranges: vec!["10.0.0.0/24".into(), "1.2.3.4/32".into()],
            concurrency: "2048".into(),
            theme: "light".into(),
            language: "ru".into(),
            ..Config::default()
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(back.ranges, cfg.ranges);
        assert_eq!(back.concurrency, "2048");
        assert_eq!(back.theme, "light");
        assert_eq!(back.language, "ru");
    }

    #[test]
    fn partial_json_fills_defaults() {
        // A file written by an older build with only some keys still loads.
        let back: Config = serde_json::from_str(r#"{"query_enabled": false}"#).unwrap();
        assert!(!back.query_enabled);
        assert_eq!(back.java_ports, "25565"); // default filled in
        assert_eq!(back.theme, "system"); // default filled in
    }
}
