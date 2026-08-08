use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::i18n::Language;

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
    pub theme: ThemePref,
    pub language: LangPref,
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
            theme: ThemePref::default(),
            language: LangPref::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemePref {
    Dark,
    Light,
    #[default]
    #[serde(other)]
    System,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum LangPref {
    En,
    Ru,
    Zh,
    Ja,
    #[default]
    #[serde(other)]
    Auto,
}

impl LangPref {
    pub fn resolve(self) -> Language {
        match self {
            LangPref::En => Language::English,
            LangPref::Ru => Language::Russian,
            LangPref::Zh => Language::Chinese,
            LangPref::Ja => Language::Japanese,
            LangPref::Auto => Language::detect(),
        }
    }
}

impl From<Language> for LangPref {
    fn from(lang: Language) -> Self {
        match lang {
            Language::English => LangPref::En,
            Language::Russian => LangPref::Ru,
            Language::Chinese => LangPref::Zh,
            Language::Japanese => LangPref::Ja,
        }
    }
}

fn config_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "mc-scan").map(|d| d.config_dir().join("config.json"))
}

pub fn load() -> Option<Config> {
    let bytes = std::fs::read(config_path()?).ok()?;
    serde_json::from_slice(&bytes).ok()
}

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
    fn theme_pref_round_trips() {
        for pref in [ThemePref::System, ThemePref::Dark, ThemePref::Light] {
            let json = serde_json::to_string(&pref).unwrap();
            assert_eq!(serde_json::from_str::<ThemePref>(&json).unwrap(), pref);
        }
        assert_eq!(
            serde_json::from_str::<ThemePref>("\"nonsense\"").unwrap(),
            ThemePref::System
        );
        assert_eq!(ThemePref::default(), ThemePref::System);
    }

    #[test]
    fn lang_pref_round_trips() {
        for (pref, lang) in [
            (LangPref::En, Language::English),
            (LangPref::Ru, Language::Russian),
            (LangPref::Zh, Language::Chinese),
            (LangPref::Ja, Language::Japanese),
        ] {
            let json = serde_json::to_string(&pref).unwrap();
            assert_eq!(serde_json::from_str::<LangPref>(&json).unwrap(), pref);
            assert_eq!(LangPref::from(lang), pref);
            assert_eq!(pref.resolve(), lang);
        }
        assert_eq!(
            serde_json::from_str::<LangPref>("\"xx\"").unwrap(),
            LangPref::Auto
        );
        assert_eq!(
            serde_json::from_str::<LangPref>("\"\"").unwrap(),
            LangPref::Auto
        );
        assert_eq!(LangPref::default(), LangPref::Auto);
    }

    #[test]
    fn config_survives_a_json_round_trip() {
        let cfg = Config {
            ranges: vec!["10.0.0.0/24".into(), "1.2.3.4/32".into()],
            concurrency: "2048".into(),
            theme: ThemePref::Light,
            language: LangPref::Ru,
            ..Config::default()
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(back.ranges, cfg.ranges);
        assert_eq!(back.concurrency, "2048");
        assert_eq!(back.theme, ThemePref::Light);
        assert_eq!(back.language, LangPref::Ru);
    }

    #[test]
    fn partial_json_fills_defaults() {
        let back: Config = serde_json::from_str(r#"{"query_enabled": false}"#).unwrap();
        assert!(!back.query_enabled);
        assert_eq!(back.java_ports, "25565");
        assert_eq!(back.theme, ThemePref::System);
        assert_eq!(back.language, LangPref::Auto);
    }
}
