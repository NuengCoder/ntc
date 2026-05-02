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
}

fn default_output_path() -> PathBuf {
    dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("."))
}
fn default_max_depth() -> usize { 10 }
fn default_num_threads() -> usize { 4 }
fn default_ignored_dirs() -> HashSet<String> {
    let mut s = HashSet::new();
    s.insert("target".to_string());
    s.insert("build".to_string());
    s
}

impl Config {
    pub fn new() -> Self {
        Self {
            output_path: default_output_path(),
            max_depth: 10,
            show_line_numbers: false,
            num_threads: 4,
            ignored_directory_names: default_ignored_dirs(),
            ignored_extensions: HashSet::new(),
            extra_supported_extensions: HashSet::new(),
        }
    }

    // ---- Persistence ----
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("config.toml"))
    }

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
        Self::new()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            let _ = fs::create_dir_all(path.parent().unwrap());
            if let Ok(toml) = toml::to_string_pretty(self) {
                let _ = fs::write(&path, toml);
            }
        }
    }

    // ---- Global singleton ----
    pub fn global() -> &'static RwLock<Config> {
        static CONFIG: std::sync::LazyLock<RwLock<Config>> =
            std::sync::LazyLock::new(|| RwLock::new(Config::load()));
        &CONFIG
    }

    pub fn save_global() {
        Self::global().read().unwrap().save();
    }

    // ---- Convenience global methods ----
    pub fn global_get_output_path() -> PathBuf {
        Self::global().read().unwrap().output_path.clone()
    }
    pub fn global_set_output_path(path: &Path) {
        Self::global().write().unwrap().output_path = path.to_path_buf();
        Self::save_global();
    }
    pub fn global_get_max_depth() -> usize {
        Self::global().read().unwrap().max_depth
    }
    pub fn global_set_max_depth(depth: usize) {
        let mut cfg = Self::global().write().unwrap();
        cfg.max_depth = depth.clamp(2, cfg.max_depth.max(10));
        cfg.save();
    }
    pub fn global_get_show_line_numbers() -> bool {
        Self::global().read().unwrap().show_line_numbers
    }
    pub fn global_set_show_line_numbers(show: bool) {
        Self::global().write().unwrap().show_line_numbers = show;
        Self::save_global();
    }
    pub fn global_get_num_threads() -> usize {
        Self::global().read().unwrap().num_threads
    }
    pub fn global_set_num_threads(threads: usize) {
        let mut cfg = Self::global().write().unwrap();
        cfg.num_threads = threads.clamp(1, 64);
        cfg.save();
    }

    // Ignore / care helpers
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
        cfg.save();
    }
    pub fn global_remove_extra_supported_extension(ext: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.extra_supported_extensions.remove(&ext.to_lowercase());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = Config::new();
        assert!(!cfg.output_path.as_os_str().is_empty());
        assert_eq!(cfg.max_depth, 10);
        assert!(!cfg.show_line_numbers);
        assert_eq!(cfg.num_threads, 4);
        assert!(cfg.ignored_directory_names.contains("target"));
    }
}