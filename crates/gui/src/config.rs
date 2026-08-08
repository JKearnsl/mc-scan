use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use crate::APP_NAME;
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

impl Config {
    fn config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("", "", APP_NAME)
            .map(|d| d.config_dir().join("config.json"))
    }

    pub fn load() -> Option<Config> {
        let bytes = std::fs::read(Self::config_path()?).ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    pub fn save(&self) {
        let Some(path) = Self::config_path() else { return };
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
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
