use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

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
}

fn default_output_path() -> PathBuf {
    dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("."))
}
fn default_max_depth() -> usize { 2 }
fn default_num_threads() -> usize { 4 }
fn default_history_enabled() -> bool { true }
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
                if let Ok(content) = fs::read_to_string(&path) {
                    toml::from_str::<Self>(&content).unwrap_or_else(|_| Self::new())
                } else {
                    Self::new()
                }
            } else {
                Self::new()
            }
        } else {
            Self::new()
        };

        // Merge .ntconfig from current directory
        if let Ok(cwd) = std::env::current_dir() {
            let local_config = cwd.join("ntconfig.toml");
            if local_config.exists() {
                if let Ok(content) = fs::read_to_string(&local_config) {
                    if let Ok(local) = toml::from_str::<LocalConfig>(&content) {
                        cfg.merge_local(local);
                    }
                }
            }
        }

        cfg
    }

    /// Merge .ntconfig settings (only overrides provided fields)
    fn merge_local(&mut self, local: LocalConfig) {
        if let Some(v) = local.max_depth { self.max_depth = v.clamp(1, 12); }
        if let Some(v) = local.show_line_numbers { self.show_line_numbers = v; }
        if let Some(v) = local.num_threads { self.num_threads = v.clamp(1, 64); }
        if let Some(ref v) = local.output_path { self.output_path = v.clone(); }
        if let Some(ref v) = local.history_path { self.history_path = Some(v.clone()); }
        if let Some(v) = local.history_enabled { self.history_enabled = v; }
        if let Some(ref v) = local.ignored_directory_names {
            self.ignored_directory_names = v.iter().cloned().collect();
        }
        if let Some(ref v) = local.ignored_extensions {
            self.ignored_extensions = v.iter().cloned().collect();
        }
        if let Some(ref v) = local.extra_supported_extensions {
            self.extra_supported_extensions = v.iter().cloned().collect();
        }
        if let Some(ref v) = local.ignored_files {
            self.ignored_files = v.iter().cloned().collect();
        }
    }

    /// Save this config instance to disk.
    /// Call only when you already hold a reference — do NOT re-acquire the
    /// global lock inside this method (that would deadlock if you hold a write
    /// lock on the same thread).
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            let _ = fs::create_dir_all(path.parent().unwrap());
            if let Ok(toml) = toml::to_string_pretty(self) {
                let _ = fs::write(&path, toml);
            }
        }
    }

    // ---- History path resolution ----
    pub fn resolve_history_path(&self) -> PathBuf {
        if !self.history_enabled {
            return PathBuf::new(); // empty = disabled
        }
        if let Some(ref custom) = self.history_path {
            custom.clone()
        } else {
            // Default: next to executable or current dir
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("ntc_history.txt")
        }
    }

    // ---- Global singleton ----
    pub fn global() -> &'static RwLock<Config> {
        static CONFIG: std::sync::LazyLock<RwLock<Config>> =
            std::sync::LazyLock::new(|| RwLock::new(Config::load()));
        &CONFIG
    }

    /// Save the global config by acquiring a **read** lock (safe to call from
    /// outside a write-lock section).
    pub fn save_global() {
        Self::global().read().unwrap().save();
    }

    // ---- Convenience global methods ----
    //
    // Pattern used throughout:
    //   1. Acquire write lock.
    //   2. Mutate the field.
    //   3. Call cfg.save() — uses `&self`, no lock re-acquisition.
    //   4. Write lock is dropped at end of block.
    //
    // Previously some setters called Self::save_global() while holding the
    // write lock, which attempted to acquire a read lock on the same thread —
    // a deadlock on std::sync::RwLock implementations that don't allow
    // read-after-write on the same thread (e.g. pthreads). Fixed by always
    // calling cfg.save() directly on the already-borrowed &mut Config instead.

    pub fn global_get_output_path() -> PathBuf {
        Self::global().read().unwrap().output_path.clone()
    }
    pub fn global_set_output_path(path: &Path) {
        let mut cfg = Self::global().write().unwrap();
        cfg.output_path = path.to_path_buf();
        cfg.save(); // safe: cfg is &Config, no lock re-acquired
    }
    pub fn global_get_max_depth() -> usize {
        Self::global().read().unwrap().max_depth
    }
    pub fn global_set_max_depth(depth: usize) {
        let mut cfg = Self::global().write().unwrap();
        cfg.max_depth = depth.clamp(1, 12);
        cfg.save();
    }
    pub fn global_get_show_line_numbers() -> bool {
        Self::global().read().unwrap().show_line_numbers
    }
    pub fn global_set_show_line_numbers(show: bool) {
        let mut cfg = Self::global().write().unwrap();
        cfg.show_line_numbers = show;
        cfg.save();
    }
    pub fn global_get_num_threads() -> usize {
        Self::global().read().unwrap().num_threads
    }
    pub fn global_set_num_threads(threads: usize) {
        let mut cfg = Self::global().write().unwrap();
        cfg.num_threads = threads.clamp(1, 64);
        cfg.save();
    }

    // ---- History ----
    pub fn global_get_history_path() -> Option<PathBuf> {
        Self::global().read().unwrap().history_path.clone()
    }
    pub fn global_get_history_enabled() -> bool {
        Self::global().read().unwrap().history_enabled
    }
    pub fn global_set_history_path(path: Option<PathBuf>) {
        let mut cfg = Self::global().write().unwrap();
        cfg.history_path = path;
        cfg.save();
    }
    pub fn global_set_history_enabled(enabled: bool) {
        let mut cfg = Self::global().write().unwrap();
        cfg.history_enabled = enabled;
        cfg.save();
    }

    // ---- Ignore / care helpers ----
    pub fn global_get_ignored_dirs() -> HashSet<String> {
        Self::global().read().unwrap().ignored_directory_names.clone()
    }
    pub fn global_add_ignored_dir(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.ignored_directory_names.insert(name.to_string());
        cfg.save();
    }
    pub fn global_remove_ignored_dir(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.ignored_directory_names.remove(name);
        cfg.save();
    }
    pub fn global_get_ignored_extensions() -> HashSet<String> {
        Self::global().read().unwrap().ignored_extensions.clone()
    }
    pub fn global_add_ignored_extension(ext: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.ignored_extensions.insert(ext.to_lowercase());
        // Remove from extra_supported to fix conflict
        cfg.extra_supported_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }
    pub fn global_remove_ignored_extension(ext: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.ignored_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }
    pub fn global_get_extra_supported_extensions() -> HashSet<String> {
        Self::global().read().unwrap().extra_supported_extensions.clone()
    }
    pub fn global_add_extra_supported_extension(ext: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.extra_supported_extensions.insert(ext.to_lowercase());
        // Remove from ignored to fix conflict
        cfg.ignored_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }
    pub fn global_remove_extra_supported_extension(ext: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.extra_supported_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }

    // ---- Specific files (caren / ignoren) ----
    pub fn global_get_ignored_files() -> HashSet<String> {
        Self::global().read().unwrap().ignored_files.clone()
    }
    pub fn global_add_ignored_file(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.ignored_files.insert(name.to_string());
        cfg.extra_supported_files.remove(name);
        cfg.save();
    }
    pub fn global_remove_ignored_file(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.ignored_files.remove(name);
        cfg.save();
    }
    pub fn global_get_extra_supported_files() -> HashSet<String> {
        Self::global().read().unwrap().extra_supported_files.clone()
    }
    pub fn global_add_extra_supported_file(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.extra_supported_files.insert(name.to_string());
        cfg.ignored_files.remove(name);
        cfg.save();
    }
    pub fn global_remove_extra_supported_file(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.extra_supported_files.remove(name);
        cfg.save();
    }

    pub fn global_get_file_watcher_enabled() -> bool {
        Self::global().read().unwrap().file_watcher_enabled
    }
    pub fn global_set_file_watcher_enabled(enabled: bool) {
        let mut cfg = Self::global().write().unwrap();
        cfg.file_watcher_enabled = enabled;
        cfg.save();
    }

    // Parse helpers
    pub fn parse_line_numbers_state(state: &str) -> Option<bool> {
        match state.to_uppercase().as_str() {
            "ON" => Some(true),
            "OFF" => Some(false),
            _ => None,
        }
    }
    pub fn parse_num_threads(input: &str) -> Option<usize> {
        input.parse::<usize>().ok().filter(|&n| n > 0)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Local config file (.ntconfig) - all fields optional
#[derive(Debug, Deserialize, Default)]
struct LocalConfig {
    pub output_path: Option<PathBuf>,
    pub max_depth: Option<usize>,
    pub show_line_numbers: Option<bool>,
    pub num_threads: Option<usize>,
    pub history_path: Option<PathBuf>,
    pub history_enabled: Option<bool>,
    pub ignored_directory_names: Option<Vec<String>>,
    pub ignored_extensions: Option<Vec<String>>,
    pub extra_supported_extensions: Option<Vec<String>>,
    pub ignored_files: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = Config::new();
        assert!(!cfg.output_path.as_os_str().is_empty());
        assert_eq!(cfg.max_depth, default_max_depth());
        assert!(!cfg.show_line_numbers);
        assert_eq!(cfg.num_threads, default_num_threads());
        assert!(cfg.ignored_directory_names.contains("target"));
        assert_eq!(cfg.history_enabled, default_history_enabled());
        assert!(cfg.history_path.is_none());
    }

    #[test]
    fn test_default_trait_matches_new() {
        let a = Config::new();
        let b = Config::default();
        assert_eq!(a.max_depth, b.max_depth);
        assert_eq!(a.num_threads, b.num_threads);
        assert_eq!(a.show_line_numbers, b.show_line_numbers);
        assert_eq!(a.history_enabled, b.history_enabled);
    }
}