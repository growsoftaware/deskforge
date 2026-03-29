use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub app: AppConfig,
    #[serde(default)]
    pub popup: PopupConfig,
    #[serde(default)]
    pub keyboard: KeyboardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub start_on_login: bool,
    #[serde(default = "default_true")]
    pub start_minimized: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_position")]
    pub position: String,
    #[serde(default = "default_display_ms")]
    pub display_ms: u64,
    #[serde(default = "default_fade_ms")]
    pub fade_ms: u64,
    #[serde(default = "default_margin_bottom")]
    pub margin_bottom: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub remaps: Vec<Remap>,
    #[serde(default)]
    pub shortcuts: Vec<Shortcut>,
    #[serde(default)]
    pub macros: Vec<TextMacro>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remap {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(default)]
    pub icon: Option<String>,
    pub label_on: String,
    pub label_off: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub id: String,
    pub name: String,
    pub binding: String,
    pub action: ShortcutAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShortcutAction {
    #[serde(rename = "toggle_remap")]
    ToggleRemap { remap_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMacro {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub text: String,
    #[serde(default = "default_method")]
    pub method: String,
}

fn default_true() -> bool { true }
fn default_theme() -> String { "system".into() }
fn default_position() -> String { "bottom-center".into() }
fn default_display_ms() -> u64 { 1500 }
fn default_fade_ms() -> u64 { 500 }
fn default_margin_bottom() -> u32 { 80 }
fn default_method() -> String { "clipboard".into() }

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig::default(),
            popup: PopupConfig::default(),
            keyboard: KeyboardConfig::default(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            start_on_login: false,
            start_minimized: true,
            theme: default_theme(),
        }
    }
}

impl Default for PopupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            position: default_position(),
            display_ms: default_display_ms(),
            fade_ms: default_fade_ms(),
            margin_bottom: default_margin_bottom(),
        }
    }
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            remaps: vec![Remap {
                id: "capslock-escape".into(),
                source: "CapsLock".into(),
                target: "Escape".into(),
                active: false,
                icon: Some("⎋".into()),
                label_on: "Escape".into(),
                label_off: "CapsLock".into(),
            }],
            shortcuts: vec![Shortcut {
                id: "toggle-capslock".into(),
                name: "Toggle CapsLock/Escape".into(),
                binding: "Super+Escape".into(),
                action: ShortcutAction::ToggleRemap {
                    remap_id: "capslock-escape".into(),
                },
            }],
            macros: vec![],
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("deskforge")
        .join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if path.exists() {
        let contents = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&contents).unwrap_or_default()
    } else {
        let config = Config::default();
        save(&config);
        config
    }
}

pub fn save(config: &Config) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(contents) = toml::to_string_pretty(config) {
        let _ = fs::write(&path, contents);
    }
}
