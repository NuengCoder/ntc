use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Persistent configuration for ntcEditor, stored at `{config_dir}/ntc/editor.toml`.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_true")]
    pub auto_save: bool,
    #[serde(default = "default_true")]
    pub syntax_enabled: bool,
    #[serde(default)]
    pub color_enabled: bool,
    /// Number of spaces per indentation level (default: 4).
    #[serde(default = "default_tab_size")]
    pub tab_size: usize,
    /// Cursor blink rate in milliseconds (0 = no blinking). Default: 500.
    #[serde(default = "default_cursor_blink_rate")]
    pub cursor_blink_rate_ms: u64,
    /// Maximum undo history depth (default: 200). Higher = more memory, more undo.
    #[serde(default = "default_undo_depth")]
    pub undo_depth: usize,
    /// Maximum recently opened files buffer stack (default: 16).
    #[serde(default = "default_max_buffers")]
    pub max_buffers: usize,
    /// Maximum open tabs (default: 32).
    #[serde(default = "default_max_tabs")]
    pub max_tabs: usize,
    /// Maximum lines retained in the run output buffer (default: 10000).
    #[serde(default = "default_max_run_lines")]
    pub max_run_lines: usize,
    /// Number of lines to pre-tokenize beyond the visible area (default: 50).
    /// Higher values use more RAM but make scrolling smoother.
    #[serde(default = "default_pre_tokenize_lines")]
    pub pre_tokenize_lines: usize,
    /// Event polling rate in milliseconds (default: 16 ≈ 60fps).
    /// Lower values feel more responsive but use more CPU.
    #[serde(default = "default_poll_rate_ms")]
    pub poll_rate_ms: u64,
    /// Auto-save session interval in seconds (default: 30).
    #[serde(default = "default_session_auto_save_secs")]
    pub session_auto_save_secs: u64,
    /// Maximum session age in hours before it's discarded (default: 24).
    #[serde(default = "default_session_max_age_hours")]
    pub session_max_age_hours: u64,
    /// Mouse support in text field and terminal panel only
    /// (tab bar, sidebar, scrollbars ignore mouse). Default: on.
    #[serde(default = "default_true")]
    pub mouse_enabled: bool,
    /// Sidebar width as percent of terminal width (1-100).
    /// None = legacy fixed 30 columns (`nfe:size def`).
    #[serde(default)]
    pub sidebar_width_pct: Option<u8>,
}

fn default_tab_size() -> usize {
    4
}
fn default_cursor_blink_rate() -> u64 {
    500
}
fn default_undo_depth() -> usize {
    200
}
fn default_max_buffers() -> usize {
    16
}
fn default_max_tabs() -> usize {
    32
}
fn default_max_run_lines() -> usize {
    100_000
}
fn default_pre_tokenize_lines() -> usize {
    150
}
fn default_poll_rate_ms() -> u64 {
    8
}
fn default_session_auto_save_secs() -> u64 {
    30
}
fn default_session_max_age_hours() -> u64 {
    24
}

fn default_true() -> bool {
    true
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            auto_save: default_true(),
            syntax_enabled: default_true(),
            color_enabled: false,
            tab_size: default_tab_size(),
            cursor_blink_rate_ms: default_cursor_blink_rate(),
            undo_depth: default_undo_depth(),
            max_buffers: default_max_buffers(),
            max_tabs: default_max_tabs(),
            max_run_lines: default_max_run_lines(),
            pre_tokenize_lines: default_pre_tokenize_lines(),
            poll_rate_ms: default_poll_rate_ms(),
            session_auto_save_secs: default_session_auto_save_secs(),
            session_max_age_hours: default_session_max_age_hours(),
            mouse_enabled: default_true(),
            sidebar_width_pct: None,
        }
    }
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
