//! Global and local configuration system for ntc.
//! Manages persistent settings (output path, depth, themes, teleports),
//! ignore/care filtering, run aliases, and the global singleton pattern.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

/// QSC (Quick Short Command) hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QscHotkey {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: String,
}

pub(crate) mod globals;
pub mod lang;

/// Global configuration (persisted to disk)
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_output_path")]
    pub output_path: PathBuf,
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    #[serde(default)]
    pub show_line_numbers: bool,
    #[serde(default = "default_num_threads")]
    pub num_threads: usize,
    /// Directories whose name should be ignored during tree/reports
    #[serde(default = "default_ignored_dirs")]
    pub ignored_directory_names: HashSet<String>,
    /// File extensions to ignore (even if otherwise supported)
    #[serde(default)]
    pub ignored_extensions: HashSet<String>,
    /// Extra extensions that should be treated as supported
    #[serde(default)]
    pub extra_supported_extensions: HashSet<String>,
    /// Specific filenames to ignore (e.g., "Cargo.lock")
    #[serde(default)]
    pub ignored_files: HashSet<String>,
    /// Specific filenames to always treat as supported
    #[serde(default)]
    pub extra_supported_files: HashSet<String>,
    /// Custom history file path (None = default location)
    #[serde(default)]
    pub history_path: Option<PathBuf>,
    /// Whether history saving is enabled
    #[serde(default = "default_history_enabled")]
    pub history_enabled: bool,
    /// Whether file watcher is enabled
    #[serde(default)]
    pub file_watcher_enabled: bool,

    /// Max change events shown in the watcher box before "… and N more".
    /// Set via `config watch limit <n>`.
    #[serde(default = "default_watch_limit")]
    pub watch_limit: usize,

    /// Whether color output is enabled
    #[serde(default = "default_color_enabled")]
    pub color_enabled: bool,

    #[serde(default)]
    pub teleports: HashMap<String, PathBuf>,

    #[serde(default)]
    pub run_aliases: HashMap<String, String>,

    /// Saved read-only SQL queries for the `ndb` shell (`dbal` dimension:
    /// `qal` lives in the editor, `ral` in the ntc shell, `dbal` in ndb).
    #[serde(default)]
    pub db_aliases: HashMap<String, String>,

    #[serde(default)]
    pub math_functions: HashMap<String, String>,

    /// Set via `watch trigger <alias>` or `watch trigger off`.
    #[serde(default)]
    pub watch_trigger_alias: Option<String>,

    /// Whether autosuggest (ghost text) is enabled in the shell
    #[serde(default = "default_autosuggest_enabled")]
    pub autosuggest_enabled: bool,

    /// Whether git integration (prompt branch, browser symbols) is enabled
    #[serde(default = "default_git_integration_enabled")]
    pub git_integration_enabled: bool,

    /// QSC (Quick Short Command) global hotkey, if configured
    #[serde(default)]
    pub qsc: Option<QscHotkey>,

    /// Interface language (english, thai, chinese)
    #[serde(default = "default_language")]
    pub language: String,
    /// Whether the selected language is fully active
    #[serde(default = "default_language_active")]
    pub language_active: bool,

    /// Use ASCII-only icons (✓→+ etc) for pipe-safe/legacy terminal output
    #[serde(default)]
    pub ascii_fallback: bool,

    /// OpenWeather API key used by the `tmprt weather` command
    #[serde(default)]
    pub openweather_api_key: Option<String>,

    /// Default city used by the `tmprt weather` command when none is given
    #[serde(default)]
    pub openweather_default_city: Option<String>,

    /// Whether directory tree view is shown in shell prompt
    #[serde(default = "default_dir_view_enabled")]
    pub dir_view_enabled: bool,
}

// Single source of truth for alias name validation.
// Used by both config.rs and commands.rs — keep this list in sync with
// all shell commands that users should not be able to override.
pub fn validate_alias_name(name: &str) -> bool {
    let reserved_commands = [
        "cd",
        "godrive",
        "god",
        "go",
        "back",
        "b",
        "view",
        "ls",
        "report",
        "txtc",
        "txtf",
        "txt",
        "config",
        "clear",
        "version",
        "where",
        "gos",
        "gosc",
        "ral",
        "run",
        "r",
        "showcg",
        "help",
        "exit",
        "quit",
        "backup",
        "ignore",
        "care",
        "igcared",
        "size",
        "tp",
        "opencg",
        "resetcg",
        "restorecg",
        "setc",
        "local",
        "ne",
        "ntceditor",
        "igcare",
        "esc",
        "gs",
        "fs",
        "ds",
        "fgo",
        "fsc",
        "locate",
        "rnmf",
        "rnmd",
        "mv",
        "move",
        "cp",
        "math",
        "init",
        "deinit",
        "diff",
        "bkup",
        "tl",
        "image",
        "img",
        "python_activate",
        "python_deactivate",
        "python_showenv",
        "smtp",
        "clip",
        "qsc",
        "tmprt",
        "git",
        "ndb",
        "nsheet",
        "nssh",
    ];

    if name.trim().is_empty() || name.contains(' ') {
        return false;
    }

    if name.contains('@') || name.contains('#') {
        return false;
    }

    if reserved_commands.contains(&name) {
        return false;
    }

    true
}

fn default_output_path() -> PathBuf {
    dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("."))
}
fn default_max_depth() -> usize {
    2
}
fn default_num_threads() -> usize {
    4
}
fn default_watch_limit() -> usize {
    10
}
fn default_history_enabled() -> bool {
    false
}
fn default_color_enabled() -> bool {
    false
}
fn default_autosuggest_enabled() -> bool {
    true
}

fn default_git_integration_enabled() -> bool {
    true
}

pub const VALID_LANGUAGES: &[&str] = &["english", "thai", "lao", "chinese"];

fn default_language() -> String {
    "english".to_string()
}

fn default_language_active() -> bool {
    true
}

fn default_dir_view_enabled() -> bool {
    true
}

fn default_ignored_dirs() -> HashSet<String> {
    let mut s = HashSet::new();
    s.insert("target".to_string());
    s.insert("build".to_string());
    s.insert("venv".to_string());
    s.insert("node_modules".to_string());
    s.insert("installer".to_string());
    s.insert("logs".to_string());
    s.insert(".git".to_string());
    s
}

impl Config {
    pub fn new() -> Self {
        Self {
            output_path: default_output_path(),
            max_depth: default_max_depth(),
            show_line_numbers: false,
            num_threads: default_num_threads(),
            ignored_directory_names: default_ignored_dirs(),
            ignored_extensions: HashSet::new(),
            extra_supported_extensions: HashSet::new(),
            ignored_files: HashSet::new(),
            extra_supported_files: HashSet::new(),
            history_path: None,
            history_enabled: default_history_enabled(),
            file_watcher_enabled: false,
            watch_limit: default_watch_limit(),
            color_enabled: default_color_enabled(),
            autosuggest_enabled: default_autosuggest_enabled(),
            git_integration_enabled: default_git_integration_enabled(),
            teleports: HashMap::new(),
            run_aliases: HashMap::new(),
            db_aliases: HashMap::new(),
            math_functions: HashMap::new(),
            watch_trigger_alias: None,
            qsc: None,
            language: default_language(),
            language_active: default_language_active(),
            ascii_fallback: false,
            openweather_api_key: None,
            openweather_default_city: None,
            dir_view_enabled: default_dir_view_enabled(),
        }
    }

    // ---- Persistence ----
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("config.toml"))
    }

    /// Load config from disk, then merge .ntconfig if present in current dir
    pub fn load() -> Self {
        let mut cfg = if let Some(path) = Self::config_path() {
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        match toml::from_str::<Self>(&content) {
                            Ok(c) => c,
                            Err(e) => {
                                eprintln!("Warning: config file '{}' has invalid TOML ({}). Using defaults.", path.display(), e);
                                Self::new()
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: could not read config file '{}': {}. Using defaults.",
                            path.display(),
                            e
                        );
                        Self::new()
                    }
                }
            } else {
                Self::new()
            }
        } else {
            Self::new()
        };

        // Auto-detect terminal capabilities. Ensure VT was attempted first
        // (matters when the lib is used without main()'s init call).
        #[cfg(windows)]
        {
            crate::utils::terminal::init();
        }
        let caps = crate::utils::terminal::detect();

        // Auto-disable ANSI colors when VT is unavailable (Win7/8 legacy
        // conhost, Admin legacy conhost) or when NO_COLOR/TERM=dumb is set.
        // `detect()` already folds those into `caps.color`; `set_override`
        // makes `colored` strip codes, and `SHOULD_COLORIZE` gates the
        // raw-ANSI paths (logo, syntax highlight).
        let effective_color = cfg.color_enabled && caps.color;
        colored::control::set_override(effective_color);

        // Auto-enable ASCII fallback when piped or legacy terminal
        if !caps.is_terminal || !caps.color {
            cfg.ascii_fallback = true;
        }
        crate::utils::terminal::set_ascii_mode(cfg.ascii_fallback);

        // Load the language pack directly (no GLOBAL_CONFIG access — avoids deadlock
        // during LazyLock init of the global singleton)
        crate::utils::config::lang::set_map(crate::utils::config::lang::load(&cfg.language));

        // Local ntconfig.toml is NOT merged into global Config.
        // Local settings are read separately when needed (see `get_local_config()`
        // and the combined query helpers below). This keeps global and local
        // completely independent — deleting ntconfig.toml never affects global.

        cfg
    }

    /// Reload config from disk (global and local)
    pub fn reload() -> Self {
        Self::load()
    }

    /// Reload the global config singleton
    pub fn reload_global() {
        let new_config = Self::reload();
        let mut cfg = Self::write_global();
        *cfg = new_config;
    }

    /// Read the local ntconfig.toml from the current directory, if it exists.
    /// This is kept completely separate from the global Config — local settings
    /// are never written into the global struct.
    pub fn get_local_config() -> Option<LocalConfig> {
        if let Ok(cwd) = std::env::current_dir() {
            let local_path = cwd.join("ntconfig.toml");
            if local_path.exists() {
                if let Ok(content) = fs::read_to_string(&local_path) {
                    if let Ok(local) = toml::from_str::<LocalConfig>(&content) {
                        return Some(local);
                    }
                }
            }
        }
        None
    }

    /// Check if a directory name is ignored by either global or local config.
    pub fn is_dir_ignored(name: &str) -> bool {
        let cfg = Self::read_global();
        if cfg.ignored_directory_names.contains(name) {
            return true;
        }
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref dirs) = local.ignored_directory_names {
                if dirs.iter().any(|d| d == name) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if a file extension is ignored by either global or local config.
    pub fn is_extension_ignored(ext: &str) -> bool {
        let ext_lower = ext.to_lowercase();
        let cfg = Self::read_global();
        if cfg.ignored_extensions.contains(&ext_lower) {
            return true;
        }
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref exts) = local.ignored_extensions {
                if exts.iter().any(|e| e.to_lowercase() == ext_lower) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if a filename is ignored by either global or local config.
    pub fn is_file_ignored(name: &str) -> bool {
        let cfg = Self::read_global();
        if cfg.ignored_files.contains(name) {
            return true;
        }
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref files) = local.ignored_files {
                if files.iter().any(|f| f == name) {
                    return true;
                }
            }
        }
        false
    }

    /// Get the combined set of ignored directory names from global + local config.
    pub fn get_ignored_dirs_combined() -> HashSet<String> {
        let cfg = Self::read_global();
        let mut dirs = cfg.ignored_directory_names.clone();
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref local_dirs) = local.ignored_directory_names {
                for d in local_dirs {
                    dirs.insert(d.clone());
                }
            }
        }
        dirs
    }

    /// Get the combined set of ignored extensions from global + local config.
    pub fn get_ignored_extensions_combined() -> HashSet<String> {
        let cfg = Self::read_global();
        let mut exts = cfg.ignored_extensions.clone();
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref local_exts) = local.ignored_extensions {
                for e in local_exts {
                    exts.insert(e.to_lowercase());
                }
            }
        }
        exts
    }

    /// Get the combined set of ignored files from global + local config.
    pub fn get_ignored_files_combined() -> HashSet<String> {
        let cfg = Self::read_global();
        let mut files = cfg.ignored_files.clone();
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref local_files) = local.ignored_files {
                for f in local_files {
                    files.insert(f.clone());
                }
            }
        }
        files
    }

    /// Get the combined set of extra supported extensions from global + local config.
    pub fn get_extra_supported_extensions_combined() -> HashSet<String> {
        let cfg = Self::read_global();
        let mut exts = cfg.extra_supported_extensions.clone();
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref local_exts) = local.extra_supported_extensions {
                for e in local_exts {
                    exts.insert(e.to_lowercase());
                }
            }
        }
        exts
    }

    /// Get the combined set of extra supported files from global + local config.
    pub fn get_extra_supported_files_combined() -> HashSet<String> {
        let cfg = Self::read_global();
        let mut files = cfg.extra_supported_files.clone();
        drop(cfg);
        if let Some(local) = Self::get_local_config() {
            if let Some(ref local_files) = local.extra_supported_files {
                for f in local_files {
                    files.insert(f.clone());
                }
            }
        }
        files
    }

    // NOTE: merge_local() was removed. Local config is never merged into
    // global. Use get_local_config(), is_dir_ignored(), get_ignored_dirs_combined(),
    // etc. to check both global and local independently.

    /// Save this config instance to disk.
    /// Call only when you already hold a reference — do NOT re-acquire the
    /// global lock inside this method (that would deadlock if you hold a write
    /// lock on the same thread).
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!(
                        "Warning: failed to create config directory '{}': {}",
                        parent.display(),
                        e
                    );
                    return;
                }
            }
            match toml::to_string_pretty(self) {
                Ok(toml) => {
                    if let Err(e) = fs::write(&path, toml) {
                        eprintln!(
                            "Warning: failed to write config to '{}': {}",
                            path.display(),
                            e
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Warning: failed to serialize config: {}", e);
                }
            }
        }
    }

    // ---- History path resolution ----
    pub fn resolve_history_path(&self) -> Option<PathBuf> {
        if !self.history_enabled {
            return None;
        }
        if let Some(ref custom) = self.history_path {
            Some(custom.clone())
        } else {
            Some(
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join("ntc_history.txt"),
            )
        }
    }

    // ---- Global singleton ----
    pub fn global() -> &'static RwLock<Config> {
        static CONFIG: std::sync::LazyLock<RwLock<Config>> =
            std::sync::LazyLock::new(|| RwLock::new(Config::load()));
        &CONFIG
    }

    /// Acquire a read lock, recovering from poison if a previous write panicked.
    pub(crate) fn read_global() -> std::sync::RwLockReadGuard<'static, Config> {
        Self::global().read().unwrap_or_else(|e| e.into_inner())
    }

    /// Acquire a write lock, recovering from poison if a previous write panicked.
    pub(crate) fn write_global() -> std::sync::RwLockWriteGuard<'static, Config> {
        Self::global().write().unwrap_or_else(|e| e.into_inner())
    }

    /// Save the global config by acquiring a **read** lock (safe to call from
    /// outside a write-lock section).
    pub fn save_global() {
        Self::read_global().save();
    }

    /// Replace the global config singleton with a custom instance.
    /// Useful for testing or early initialization.
    pub fn set_global(config: Config) {
        *Self::global().write().unwrap() = config;
    }

    /// Temporarily inject a custom config, run `f`, then restore the original.
    /// Returns the result of `f`.
    ///
    /// # Example
    /// ```ignore
    /// let result = Config::with_test_config(custom_cfg, || {
    ///     // code that reads Config::global_get_*()
    /// });
    /// ```
    pub fn with_test_config<R>(config: Config, f: impl FnOnce() -> R) -> R {
        let mut guard = Self::global().write().unwrap();
        let old = std::mem::replace(&mut *guard, config);
        drop(guard);
        let result = f();
        *Self::global().write().unwrap() = old;
        result
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Local config file (ntconfig.toml) - ONLY ignore/care, run_aliases and db_aliases
/// All other settings are always global
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalConfig {
    pub ignored_directory_names: Option<Vec<String>>,
    pub ignored_extensions: Option<Vec<String>>,
    pub extra_supported_extensions: Option<Vec<String>>,
    pub ignored_files: Option<Vec<String>>,
    pub extra_supported_files: Option<Vec<String>>,
    pub run_aliases: Option<HashMap<String, String>>,
    pub db_aliases: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_alias_name_rejects_reserved() {
        for cmd in [
            "go", "cd", "help", "exit", "quit", "run", "r", "ral", "math", "setc", "diff",
            "bkup", "view", "txt",
        ] {
            assert!(
                !validate_alias_name(cmd),
                "reserved '{}' should be rejected",
                cmd
            );
        }
    }

    #[test]
    fn test_validate_alias_name_rejects_at_hash() {
        assert!(!validate_alias_name("@foo"));
        assert!(!validate_alias_name("#bar"));
    }

    #[test]
    fn test_validate_alias_name_accepts_custom() {
        assert!(validate_alias_name("myalias"));
        assert!(validate_alias_name("build_project"));
        assert!(validate_alias_name("x"));
    }

    #[test]
    fn test_validate_alias_name_accepts_uppercase() {
        assert!(
            validate_alias_name("GO"),
            "uppercase 'GO' is not in reserved list"
        );
        assert!(
            validate_alias_name("DINO"),
            "uppercase 'DINO' is not in reserved list"
        );
    }

    #[test]
    fn test_config_default_values() {
        let cfg = Config::new();
        assert_eq!(cfg.max_depth, 2);
        assert_eq!(cfg.num_threads, 4);
        assert!(!cfg.show_line_numbers);
        assert!(!cfg.color_enabled);
        assert!(!cfg.history_enabled);
        assert!(!cfg.file_watcher_enabled);
        assert_eq!(cfg.watch_limit, 10);
        assert!(cfg.teleports.is_empty());
        assert!(cfg.run_aliases.is_empty());
        assert!(cfg.db_aliases.is_empty());
        assert!(cfg.ignored_directory_names.contains("target"));
        assert!(cfg.ignored_directory_names.contains(".git"));
        assert!(cfg.ignored_directory_names.contains("node_modules"));
    }

    #[test]
    fn test_parse_on_off() {
        assert_eq!(Config::parse_on_off("ON"), Some(true));
        assert_eq!(Config::parse_on_off("on"), Some(true));
        assert_eq!(Config::parse_on_off("On"), Some(true));
        assert_eq!(Config::parse_on_off("OFF"), Some(false));
        assert_eq!(Config::parse_on_off("off"), Some(false));
        assert_eq!(Config::parse_on_off(""), None);
        assert_eq!(Config::parse_on_off("invalid"), None);
    }

    #[test]
    fn test_parse_num_threads() {
        assert_eq!(Config::parse_num_threads("4"), Some(4));
        assert_eq!(Config::parse_num_threads("1"), Some(1));
        assert_eq!(Config::parse_num_threads("0"), None);
        assert_eq!(Config::parse_num_threads("-1"), None);
        assert_eq!(Config::parse_num_threads("abc"), None);
    }
}
