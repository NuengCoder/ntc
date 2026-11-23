use std::collections::BTreeMap;
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
    /// Indentation width for file types not listed in `tabs` (default: 4).
    #[serde(default = "default_tabs_other")]
    pub tabs_other: usize,
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
    /// Mouse support in the text field, sidebar, tab bar and terminal
    /// panel. Panel borders ignore mouse. Default: on.
    #[serde(default = "default_true")]
    pub mouse_enabled: bool,
    /// Sidebar width as percent of terminal width (1-100).
    /// None = legacy fixed 30 columns (`nfe:size def`).
    #[serde(default)]
    pub sidebar_width_pct: Option<u8>,
    /// Per-extension indentation overrides, e.g. `yml = 2`, `py = 4`.
    /// Keys are lowercase extensions without the dot. Kept last so TOML
    /// writes it as a trailing `[tabs]` table.
    #[serde(default)]
    pub tabs: BTreeMap<String, usize>,
}

fn default_tabs_other() -> usize {
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
            tabs_other: default_tabs_other(),
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
            tabs: BTreeMap::new(),
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
                    if let Ok(mut cfg) = toml::from_str::<Self>(&content) {
                        // Legacy `tab_size` (the old single global width)
                        // stands in for `tabs_other` when the new key is
                        // absent, so existing configs keep their indent.
                        if let Ok(raw) = toml::from_str::<toml::Value>(&content) {
                            cfg.migrate_legacy_tab_size(&raw);
                        }
                        return cfg;
                    }
                }
            }
        }
        Self::default()
    }

    /// Adopt the old single `tab_size` key when `tabs_other` is absent.
    fn migrate_legacy_tab_size(&mut self, raw: &toml::Value) {
        if raw.get("tabs_other").is_some() {
            return;
        }
        if let Some(n) = raw.get("tab_size").and_then(|v| v.as_integer()) {
            self.tabs_other = n.clamp(1, 16) as usize;
        }
    }

    /// Effective indentation width for `path`.
    ///
    /// Resolution order: an explicit `[tabs]` entry for the lowercased
    /// extension, then the built-in default for the detected language,
    /// then `tabs_other`. Clamped to 1-16 so a bad config cannot break
    /// the guide/column math.
    pub fn indent_width_for(&self, path: &std::path::Path) -> usize {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase());
        if let Some(w) = ext.as_deref().and_then(|e| self.tabs.get(e)) {
            return (*w).clamp(1, 16);
        }
        if let Some(w) =
            crate::utils::syntax::detect_language_for_path(path).and_then(language_indent)
        {
            return w.clamp(1, 16);
        }
        self.tabs_other.clamp(1, 16)
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

/// Built-in indentation style per language, applied when the extension is
/// not overridden in `[tabs]`. Mirrors the common convention for each
/// language (2-space YAML/JSON/JS/Markdown, 4-space Python/Rust/C/Java/...).
fn language_indent(lang: crate::utils::syntax::SyntaxLanguage) -> Option<usize> {
    use crate::utils::syntax::SyntaxLanguage as L;
    Some(match lang {
        L::Yaml
        | L::Json
        | L::JavaScript
        | L::TypeScript
        | L::Html
        | L::Css
        | L::Markdown
        | L::Ruby
        | L::Lua
        | L::Haskell
        | L::Elixir
        | L::Nim
        | L::Scala
        | L::Clojure
        | L::OCaml
        | L::Dart
        | L::Graphql
        // `.ntc` scripts are written in the two-space style the language
        // service documents, so Tab and the indent guides both step by 2.
        | L::NtcScr => 2,
        L::Python
        | L::Rust
        | L::C
        | L::Cpp
        | L::CSharp
        | L::Java
        | L::Kotlin
        | L::Swift
        | L::Go
        | L::Php
        | L::Ps1
        | L::Zig
        | L::Julia
        | L::Erlang
        | L::FSharp => 4,
        _ => return None,
    })
}

impl crate::shell::commands::builtin::ne::editor::Editor {
    /// Indentation width for the active file. Reads `self.path`, which is
    /// swapped per tab, so each tab follows its own file type.
    pub(crate) fn indent_width(&self) -> usize {
        self.editor_cfg.indent_width_for(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::EditorConfig;
    use std::path::Path;

    #[test]
    fn indent_width_prefers_config_table() {
        let mut cfg = EditorConfig::default();
        cfg.tabs.insert("py".into(), 2);
        assert_eq!(cfg.indent_width_for(Path::new("a.py")), 2);
        // Extension matching is case-insensitive.
        assert_eq!(cfg.indent_width_for(Path::new("A.PY")), 2);
    }

    #[test]
    fn indent_width_uses_language_defaults() {
        let cfg = EditorConfig::default();
        assert_eq!(cfg.indent_width_for(Path::new("a.yml")), 2);
        assert_eq!(cfg.indent_width_for(Path::new("a.yaml")), 2);
        assert_eq!(cfg.indent_width_for(Path::new("a.py")), 4);
        assert_eq!(cfg.indent_width_for(Path::new("a.rs")), 4);
        assert_eq!(cfg.indent_width_for(Path::new("a.json")), 2);
    }

    #[test]
    fn ntc_scripts_indent_by_two() {
        let cfg = EditorConfig::default();
        assert_eq!(cfg.indent_width_for(Path::new("deploy.ntc")), 2);
        assert_eq!(cfg.indent_width_for(Path::new("DEPLOY.NTC")), 2);
        // An explicit `[tabs]` entry still wins over the default.
        let mut cfg = EditorConfig::default();
        cfg.tabs.insert("ntc".into(), 8);
        assert_eq!(cfg.indent_width_for(Path::new("deploy.ntc")), 8);
    }

    #[test]
    fn indent_width_falls_back_to_tabs_other() {
        let mut cfg = EditorConfig::default();
        cfg.tabs_other = 3;
        assert_eq!(cfg.indent_width_for(Path::new("a.unknownext")), 3);
        // A value that would break column math is clamped.
        cfg.tabs_other = 0;
        assert_eq!(cfg.indent_width_for(Path::new("a.unknownext")), 1);
        cfg.tabs.insert("py".into(), 99);
        assert_eq!(cfg.indent_width_for(Path::new("a.py")), 16);
    }

    #[test]
    fn config_roundtrips_with_tabs_table() {
        let mut cfg = EditorConfig::default();
        cfg.tabs_other = 4;
        cfg.tabs.insert("yml".into(), 2);
        let text = toml::to_string_pretty(&cfg).expect("config serializes");
        assert!(text.contains("[tabs]"), "{text}");
        let back: EditorConfig = toml::from_str(&text).expect("config re-parses");
        assert_eq!(back.tabs_other, 4);
        assert_eq!(back.indent_width_for(Path::new("a.yml")), 2);
        assert_eq!(back.indent_width_for(Path::new("a.py")), 4);
    }

    #[test]
    fn legacy_tab_size_migrates_to_tabs_other() {
        let raw: toml::Value = toml::from_str("tab_size = 2\n").unwrap();
        let mut cfg: EditorConfig = toml::from_str("tab_size = 2\n").unwrap();
        cfg.migrate_legacy_tab_size(&raw);
        assert_eq!(cfg.tabs_other, 2);

        // An explicit `tabs_other` always wins over the legacy key.
        let raw: toml::Value = toml::from_str("tab_size = 2\ntabs_other = 8\n").unwrap();
        let mut cfg: EditorConfig = toml::from_str("tab_size = 2\ntabs_other = 8\n").unwrap();
        cfg.migrate_legacy_tab_size(&raw);
        assert_eq!(cfg.tabs_other, 8);
    }
}
