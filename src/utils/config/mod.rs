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

/// A `serve` sub-service with its own defaults (`files`, `http-server`, `chess`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeService {
    Files,
    Http,
    Chess,
}

impl ServeService {
    /// Parse a user-supplied service name (`files` | `http-server` | `chess`).
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "files" => Some(ServeService::Files),
            "http-server" | "http" => Some(ServeService::Http),
            "chess" => Some(ServeService::Chess),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ServeService::Files => "files",
            ServeService::Http => "http-server",
            ServeService::Chess => "chess",
        }
    }
}

/// Per-service `serve` defaults (port + bind). Managed via
/// `serve config <service> [--port-default N] [--bind-default ADDR]`
/// (alias: `config serve …`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServeServiceConfig {
    #[serde(default = "default_serve_port")]
    pub port: u16,
    #[serde(default = "default_serve_bind")]
    pub bind: String,
}

/// Per-service `serve` defaults container.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServeConfig {
    #[serde(default = "default_serve_service_config")]
    pub files: ServeServiceConfig,
    #[serde(default = "default_serve_service_config")]
    pub http_server: ServeServiceConfig,
    #[serde(default = "default_serve_service_config")]
    pub chess: ServeServiceConfig,
}

/// One column of a `fake` group: the output column name (`alias`) plus the
/// generator `type` that fills it. Serialized as `{ alias = "…", type = "…" }`.
/// A trailing `!` in the CLI spec sets `unique`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FakeColumn {
    pub alias: String,
    #[serde(rename = "type")]
    pub type_name: String,
    /// When true, the generated values for this column must be unique across
    /// all rows (`fake group add … [col as type !]`).
    #[serde(default, skip_serializing_if = "is_false")]
    pub unique: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl FakeColumn {
    pub fn new(alias: impl Into<String>, type_name: impl Into<String>) -> Self {
        FakeColumn {
            alias: alias.into(),
            type_name: type_name.into(),
            unique: false,
        }
    }
}

impl ServeConfig {
    pub fn for_service(&self, svc: ServeService) -> &ServeServiceConfig {
        match svc {
            ServeService::Files => &self.files,
            ServeService::Http => &self.http_server,
            ServeService::Chess => &self.chess,
        }
    }

    pub fn for_service_mut(&mut self, svc: ServeService) -> &mut ServeServiceConfig {
        match svc {
            ServeService::Files => &mut self.files,
            ServeService::Http => &mut self.http_server,
            ServeService::Chess => &mut self.chess,
        }
    }
}

pub(crate) mod globals;
pub(crate) use globals::{AliasStore, ConfigScope};

/// Test-only override for the global config file location.
///
/// Tests that write the global config point it at a temp file so the
/// developer's real `config.toml` is never touched.
#[cfg(test)]
static TEST_CONFIG_PATH: std::sync::OnceLock<std::sync::Mutex<Option<PathBuf>>> =
    std::sync::OnceLock::new();

/// Test-only: redirect the global config file to `path` (`None` restores the
/// per-process temp file).
#[cfg(test)]
pub(crate) fn set_test_config_path(path: Option<PathBuf>) {
    let slot = TEST_CONFIG_PATH.get_or_init(|| std::sync::Mutex::new(None));
    *slot.lock().unwrap_or_else(|e| e.into_inner()) = path;
}

#[cfg(test)]
fn test_config_path() -> Option<PathBuf> {
    let slot = TEST_CONFIG_PATH.get_or_init(|| std::sync::Mutex::new(None));
    slot.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Test-only: the config file every test writes to. It lives in the OS temp
/// directory, so no test run can modify the real `config.toml`.
#[cfg(test)]
pub(crate) fn test_config_file() -> PathBuf {
    std::env::temp_dir().join(format!("ntc-test-config-{}.toml", std::process::id()))
}
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

    /// `#param --hint-*` completion specs for run aliases, keyed by
    /// lowercased alias name (serialized `#x --hint-dir …` form).
    #[serde(default)]
    pub run_alias_hints: HashMap<String, String>,

    /// Named value lists for `#param --hint-n <name>` / `--select-n <name>`
    /// (managed with `ral hintn`), keyed by lowercased list name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub run_hint_lists: HashMap<String, Vec<String>>,

    /// Saved read-only SQL queries for the `ndb` shell (`dbal` dimension:
    /// `qal` lives in the editor, `ral` in the ntc shell, `dbal` in ndb).
    #[serde(default)]
    pub db_aliases: HashMap<String, String>,

    #[serde(default)]
    pub math_functions: HashMap<String, String>,

    /// Set via `watch trigger <alias>` or `watch trigger off`.
    #[serde(default)]
    pub watch_trigger_alias: Option<String>,

    /// Saved `fake` generator groups for the `fake` shell command, keyed by
    /// lowercased group name; each value is an ordered list of columns.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub fake_groups: HashMap<String, Vec<FakeColumn>>,

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

    /// Whether the daily new-release notice is shown at shell startup
    #[serde(default = "default_update_check_enabled")]
    pub update_check_enabled: bool,

    /// Per-service `serve` defaults (port + bind), see `ServeConfig`
    #[serde(default = "default_serve_config")]
    pub serve: ServeConfig,
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
        "viewf",
        "viewd",
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
        "nms",
        "nsheet",
        "nssh",
        "fake",
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

fn default_update_check_enabled() -> bool {
    true
}

fn default_serve_port() -> u16 {
    8080
}

fn default_serve_bind() -> String {
    "0.0.0.0".to_string()
}

fn default_serve_service_config() -> ServeServiceConfig {
    ServeServiceConfig {
        port: default_serve_port(),
        bind: default_serve_bind(),
    }
}

fn default_serve_config() -> ServeConfig {
    ServeConfig {
        files: default_serve_service_config(),
        http_server: default_serve_service_config(),
        chess: default_serve_service_config(),
    }
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
            run_alias_hints: HashMap::new(),
            run_hint_lists: HashMap::new(),
            db_aliases: HashMap::new(),
            math_functions: HashMap::new(),
            watch_trigger_alias: None,
            fake_groups: HashMap::new(),
            qsc: None,
            language: default_language(),
            language_active: default_language_active(),
            ascii_fallback: false,
            openweather_api_key: None,
            openweather_default_city: None,
            dir_view_enabled: default_dir_view_enabled(),
            update_check_enabled: default_update_check_enabled(),
            serve: default_serve_config(),
        }
    }

    // ---- Persistence ----
    fn config_path() -> Option<PathBuf> {
        #[cfg(test)]
        {
            // Tests never touch the developer's real `config.toml`: the path
            // points at a per-process temp file unless a test overrides it.
            Some(test_config_path().unwrap_or_else(test_config_file))
        }
        #[cfg(not(test))]
        {
            dirs::config_dir().map(|d| d.join("ntc").join("config.toml"))
        }
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
        Self::effective_igcare()
            .ignored_directory_names
            .contains(name)
    }

    /// Check if a file extension is ignored by either global or local config.
    pub fn is_extension_ignored(ext: &str) -> bool {
        let ext_lower = ext.to_lowercase();
        Self::effective_igcare()
            .ignored_extensions
            .contains(&ext_lower)
    }

    /// Check if a filename is ignored by either global or local config.
    pub fn is_file_ignored(name: &str) -> bool {
        Self::effective_igcare().ignored_files.contains(name)
    }

    /// Effective ignore/care sets for the current directory.
    ///
    /// Precedence: the local config is applied on top of the global one, so a
    /// local "cared" name is never ignored (even when the global config
    /// ignores it) while every name the local config does not mention keeps
    /// the global behaviour. The global config file is never modified.
    pub fn effective_igcare() -> EffectiveIgcare {
        let cfg = Self::read_global();
        let mut eff = EffectiveIgcare {
            ignored_directory_names: cfg.ignored_directory_names.clone(),
            ignored_extensions: cfg.ignored_extensions.clone(),
            extra_supported_extensions: cfg.extra_supported_extensions.clone(),
            ignored_files: cfg.ignored_files.clone(),
            extra_supported_files: cfg.extra_supported_files.clone(),
        };
        drop(cfg);

        if let Some(local) = Self::get_local_config() {
            if let Some(ref dirs) = local.ignored_directory_names {
                eff.ignored_directory_names.extend(dirs.iter().cloned());
            }
            if let Some(ref exts) = local.ignored_extensions {
                eff.ignored_extensions
                    .extend(exts.iter().map(|e| e.to_lowercase()));
            }
            if let Some(ref exts) = local.extra_supported_extensions {
                eff.extra_supported_extensions
                    .extend(exts.iter().map(|e| e.to_lowercase()));
            }
            if let Some(ref files) = local.ignored_files {
                eff.ignored_files.extend(files.iter().cloned());
            }
            if let Some(ref files) = local.extra_supported_files {
                eff.extra_supported_files.extend(files.iter().cloned());
            }

            // A locally cared name wins over every ignore list.
            if let Some(ref dirs) = local.cared_directory_names {
                for d in dirs {
                    eff.ignored_directory_names.remove(d);
                }
            }
            if let Some(ref exts) = local.cared_extensions {
                for e in exts {
                    let lower = e.to_lowercase();
                    eff.ignored_extensions.remove(&lower);
                    eff.extra_supported_extensions.insert(lower);
                }
            }
            if let Some(ref files) = local.cared_files {
                for f in files {
                    eff.ignored_files.remove(f);
                    eff.extra_supported_files.insert(f.clone());
                }
            }
        }
        eff
    }

    /// Get the effective set of ignored directory names (global + local).
    pub fn get_ignored_dirs_combined() -> HashSet<String> {
        Self::effective_igcare().ignored_directory_names
    }

    /// Get the effective set of ignored extensions (global + local).
    pub fn get_ignored_extensions_combined() -> HashSet<String> {
        Self::effective_igcare().ignored_extensions
    }

    /// Get the effective set of ignored files (global + local).
    pub fn get_ignored_files_combined() -> HashSet<String> {
        Self::effective_igcare().ignored_files
    }

    /// Get the effective set of extra supported extensions (global + local).
    pub fn get_extra_supported_extensions_combined() -> HashSet<String> {
        Self::effective_igcare().extra_supported_extensions
    }

    /// Get the effective set of extra supported files (global + local).
    pub fn get_extra_supported_files_combined() -> HashSet<String> {
        Self::effective_igcare().extra_supported_files
    }

    /// Raw ignore/care sets of the global config (for the `igcared` listing).
    pub fn global_igcare_sets() -> IgcareSets {
        let cfg = Self::read_global();
        IgcareSets {
            ignored_directory_names: cfg.ignored_directory_names.clone(),
            ignored_extensions: cfg.ignored_extensions.clone(),
            extra_supported_extensions: cfg.extra_supported_extensions.clone(),
            ignored_files: cfg.ignored_files.clone(),
            extra_supported_files: cfg.extra_supported_files.clone(),
            cared_directory_names: HashSet::new(),
            cared_extensions: HashSet::new(),
            cared_files: HashSet::new(),
        }
    }

    /// Raw ignore/care sets of the local `ntconfig.toml` (empty when there is
    /// no local config in the current directory).
    pub fn local_igcare_sets() -> IgcareSets {
        let mut sets = IgcareSets::default();
        let Some(local) = Self::get_local_config() else {
            return sets;
        };
        if let Some(v) = local.ignored_directory_names {
            sets.ignored_directory_names = v.into_iter().collect();
        }
        if let Some(v) = local.ignored_extensions {
            sets.ignored_extensions = v.into_iter().map(|e| e.to_lowercase()).collect();
        }
        if let Some(v) = local.extra_supported_extensions {
            sets.extra_supported_extensions = v.into_iter().map(|e| e.to_lowercase()).collect();
        }
        if let Some(v) = local.ignored_files {
            sets.ignored_files = v.into_iter().collect();
        }
        if let Some(v) = local.extra_supported_files {
            sets.extra_supported_files = v.into_iter().collect();
        }
        if let Some(v) = local.cared_directory_names {
            sets.cared_directory_names = v.into_iter().collect();
        }
        if let Some(v) = local.cared_extensions {
            sets.cared_extensions = v.into_iter().map(|e| e.to_lowercase()).collect();
        }
        if let Some(v) = local.cared_files {
            sets.cared_files = v.into_iter().collect();
        }
        sets
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
        // Serialize whole swap-run-restore: without this, two parallel
        // tests each swap the singleton and observe each other's config
        // (flaky CI failures). The guard is held across `f()`.
        static SERIAL: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
        let _serial = SERIAL
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let mut guard = Self::global().write().unwrap_or_else(|e| e.into_inner());
        let old = std::mem::replace(&mut *guard, config);
        drop(guard);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        *Self::global()
            .write()
            .unwrap_or_else(|e| e.into_inner()) = old;
        match result {
            Ok(v) => v,
            Err(e) => std::panic::resume_unwind(e),
        }
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
    /// Names the local config explicitly *cares* about: they are never
    /// ignored, even when the global config ignores them. The global config
    /// itself is never modified — the local list only wins for this folder.
    pub cared_directory_names: Option<Vec<String>>,
    pub cared_extensions: Option<Vec<String>>,
    pub cared_files: Option<Vec<String>>,
    pub run_aliases: Option<HashMap<String, String>>,
    pub run_alias_hints: Option<HashMap<String, String>>,
    pub run_hint_lists: Option<HashMap<String, Vec<String>>>,
    pub db_aliases: Option<HashMap<String, String>>,
    pub fake_groups: Option<HashMap<String, Vec<FakeColumn>>>,
}

/// Raw ignore/care sets of one config store (global or local), used by the
/// `igcared` listing to show each source separately.
#[derive(Debug, Clone, Default)]
pub struct IgcareSets {
    pub ignored_directory_names: HashSet<String>,
    pub ignored_extensions: HashSet<String>,
    pub extra_supported_extensions: HashSet<String>,
    pub ignored_files: HashSet<String>,
    pub extra_supported_files: HashSet<String>,
    /// Only the local store has "cared" lists (un-ignore overrides).
    pub cared_directory_names: HashSet<String>,
    pub cared_extensions: HashSet<String>,
    pub cared_files: HashSet<String>,
}

/// Effective ignore/care sets for the current directory: the global sets with
/// the local sets applied on top (local "cared" names win over every ignore).
#[derive(Debug, Clone, Default)]
pub struct EffectiveIgcare {
    pub ignored_directory_names: HashSet<String>,
    pub ignored_extensions: HashSet<String>,
    pub extra_supported_extensions: HashSet<String>,
    pub ignored_files: HashSet<String>,
    pub extra_supported_files: HashSet<String>,
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

    #[test]
    fn test_serve_service_parse() {
        assert_eq!(ServeService::parse("files"), Some(ServeService::Files));
        assert_eq!(ServeService::parse("FILES"), Some(ServeService::Files));
        assert_eq!(
            ServeService::parse("http-server"),
            Some(ServeService::Http)
        );
        assert_eq!(ServeService::parse("http"), Some(ServeService::Http));
        assert_eq!(ServeService::parse("chess"), Some(ServeService::Chess));
        assert_eq!(ServeService::parse("CHESS"), Some(ServeService::Chess));
        assert_eq!(ServeService::parse("nope"), None);
        assert_eq!(ServeService::parse(""), None);
        assert_eq!(ServeService::Files.name(), "files");
        assert_eq!(ServeService::Http.name(), "http-server");
        assert_eq!(ServeService::Chess.name(), "chess");
    }

    #[test]
    fn test_serve_defaults() {
        let cfg = Config::new();
        assert_eq!(cfg.serve.files.port, 8080);
        assert_eq!(cfg.serve.files.bind, "0.0.0.0");
        assert_eq!(cfg.serve.http_server.port, 8080);
        assert_eq!(cfg.serve.http_server.bind, "0.0.0.0");
        assert_eq!(cfg.serve.chess.port, 8080);
        assert_eq!(cfg.serve.chess.bind, "0.0.0.0");
        assert_eq!(
            cfg.serve.for_service(ServeService::Files).port,
            8080
        );
        assert_eq!(
            cfg.serve.for_service(ServeService::Http).bind,
            "0.0.0.0"
        );
    }

    #[test]
    fn test_serve_serde_missing_table_falls_back_to_defaults() {
        // Old config.toml files have no [serve] table at all.
        let cfg: Config = toml::from_str("max_depth = 3\n").unwrap();
        assert_eq!(cfg.max_depth, 3);
        assert_eq!(cfg.serve.files.port, 8080);
        assert_eq!(cfg.serve.http_server.bind, "0.0.0.0");
    }

    #[test]
    fn test_serve_serde_roundtrip_preserves_values() {
        let mut cfg = Config::new();
        cfg.serve.files.port = 9000;
        cfg.serve.http_server.bind = "127.0.0.1".to_string();
        let s = toml::to_string(&cfg).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert_eq!(back.serve.files.port, 9000);
        assert_eq!(back.serve.http_server.bind, "127.0.0.1");
        assert_eq!(back.serve.http_server.port, 8080);
    }

    #[test]
    fn test_serve_getters_read_swapped_config() {
        let mut cfg = Config::new();
        cfg.serve.files.port = 9000;
        cfg.serve.http_server.bind = "127.0.0.1".to_string();
        Config::with_test_config(cfg, || {
            assert_eq!(Config::global_get_serve_port(ServeService::Files), 9000);
            assert_eq!(
                Config::global_get_serve_bind(ServeService::Http),
                "127.0.0.1"
            );
            assert_eq!(Config::global_get_serve_port(ServeService::Http), 8080);
        });
    }
}
