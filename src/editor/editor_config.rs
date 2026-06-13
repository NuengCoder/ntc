use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Persistent configuration for ntcEditor, stored at `{config_dir}/ntc/editor.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default)]
    pub auto_save: bool,
    #[serde(default = "default_true")]
    pub syntax_enabled: bool,
    #[serde(default)]
    pub color_enabled: bool,
}

fn default_true() -> bool {
    true
}

impl EditorConfig {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("editor.toml"))
    }

    /// Load from disk, or return defaults if file doesn't exist.
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str::<Self>(&content) {
                        return cfg;
                    }
                }
            }
        }
        Self::default()
    }

    /// Save to disk.
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(toml) = toml::to_string_pretty(self) {
                let _ = fs::write(&path, toml);
            }
        }
    }
}
