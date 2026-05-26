use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::output::{print_error, print_success};

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

    #[serde(default)]
    pub teleports: HashMap<String, PathBuf>,

    #[serde(default)]
    pub run_aliases: HashMap<String, String>,
}

// Add this function to Config impl
pub fn validate_alias_name(name: &str) -> bool {
    let reserved_commands = [
        "go", "cd", "godrive", "god", "back", "b", "view", "ls", "txt", "html", "json", "md",
        "seto", "setd", "setl", "sett", "seth", "watch", "clear", "version", "where",
        "gos", "gosc", "ral", "run", "r", "showcg", "help", "exit", "quit", "ignored",
        "ignore", "cared", "ignoref", "caref", "ignoren", "caren", "size", "tp", "opencg",
        "resetcg", "restorecg", "gencg" 
    ];
    
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
fn default_max_depth() -> usize { 2 }
fn default_num_threads() -> usize { 4 }
fn default_history_enabled() -> bool { false }
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
            teleports: HashMap::new(),
            run_aliases: HashMap::new(),
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

        // Merge .ntconfig from current directory (only ignore/care and run_aliases)
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

    /// Reload config from disk (global and local)
    pub fn reload() -> Self {
        Self::load()
    }

    /// Reload the global config singleton
    pub fn reload_global() {
        let new_config = Self::reload();
        let mut cfg = Self::global().write().unwrap();
        *cfg = new_config;
    }

    /// Merge .ntconfig settings (ONLY ignore/care and run_aliases)
    /// Everything else (output_path, max_depth, teleports, etc.) stays global
    fn merge_local(&mut self, local: LocalConfig) {
        // ONLY merge ignore/care settings and run_aliases
        // All other settings (output_path, max_depth, show_line_numbers, num_threads,
        // history_path, history_enabled, file_watcher_enabled, teleports) are NEVER
        // overridden by local config - they always come from global config
        
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
        if let Some(ref v) = local.extra_supported_files {
            self.extra_supported_files = v.iter().cloned().collect();
        }
        // Merge run aliases (local overrides global if same name)
        if let Some(ref v) = local.run_aliases {
            for (name, cmd) in v {
                self.run_aliases.insert(name.clone(), cmd.clone());
            }
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
    pub fn resolve_history_path(&self) -> Option<PathBuf> {
        if !self.history_enabled {
            return None;
        }
        if let Some(ref custom) = self.history_path {
            Some(custom.clone())
        } else {
            Some(std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("ntc_history.txt"))
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
        cfg.max_depth = depth.clamp(1, 20);
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
        cfg.num_threads = threads.clamp(1, 16);
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

    pub fn global_get_teleports() -> HashMap<String, PathBuf> {
        Self::global().read().unwrap().teleports.clone()
    }
    
    pub fn global_set_teleports(teleports: HashMap<String, PathBuf>) {
        let mut cfg = Self::global().write().unwrap();
        cfg.teleports = teleports;
        cfg.save();
    }

    pub fn global_rename_run_alias(old_name: &str, new_name: &str) -> bool {
        let old_lower = old_name.to_lowercase();
        let new_lower = new_name.to_lowercase();
        
        // Validate new name
        if !validate_alias_name(&new_lower) {
            return false;
        }
        
        let mut cfg = Self::global().write().unwrap();
        
        // Check if old alias exists
        if !cfg.run_aliases.contains_key(&old_lower) {
            return false;
        }
        
        // Check if new name already exists
        if cfg.run_aliases.contains_key(&new_lower) {
            return false;
        }
        
        // Perform rename
        let command = cfg.run_aliases.remove(&old_lower).unwrap();
        cfg.run_aliases.insert(new_lower, command);
        cfg.save();
        
        true
    }

    pub fn global_get_run_aliases() -> HashMap<String, String> {
        Self::global().read().unwrap().run_aliases.clone()
    }
    
    pub fn global_add_run_alias(name: &str, command: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.run_aliases.insert(name.to_lowercase(), command.to_string());
        cfg.save();
    }
    
    pub fn global_remove_run_alias(name: &str) {
        let mut cfg = Self::global().write().unwrap();
        cfg.run_aliases.remove(&name.to_lowercase());
        cfg.save();
    }
    
    pub fn global_update_run_alias(name: &str, command: &str) -> bool {
        let mut cfg = Self::global().write().unwrap();
        if cfg.run_aliases.contains_key(&name.to_lowercase()) {
            cfg.run_aliases.insert(name.to_lowercase(), command.to_string());
            cfg.save();
            true
        } else {
            false
        }
    }

    // ---- Local config file operations ----

    /// Get the path to ntconfig.toml in current directory if it exists
    pub fn get_local_config_path() -> Option<PathBuf> {
        if let Ok(cwd) = std::env::current_dir() {
            let local_config = cwd.join("ntconfig.toml");
            if local_config.exists() {
                return Some(local_config);
            }
        }
        None
    }

    /// Read current local config file
    fn read_local_config() -> Result<LocalConfig, toml::de::Error> {
        if let Some(path) = Self::get_local_config_path() {
            let content = std::fs::read_to_string(&path).ok();
            if let Some(content) = content {
                return toml::from_str(&content);
            }
        }
        Ok(LocalConfig::default())
    }

    /// Write local config file
    fn write_local_config(local: &LocalConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::get_local_config_path() {
            let toml = toml::to_string_pretty(local)?;
            std::fs::write(&path, toml)?;
        }
        Ok(())
    }

    /// Add ignored directory to local config (or global if no local config)
    pub fn local_add_ignored_dir(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.ignored_directory_names.is_none() {
                local.ignored_directory_names = Some(Vec::new());
            }
            if let Some(ref mut vec) = local.ignored_directory_names {
                if !vec.contains(&name.to_string()) {
                    vec.push(name.to_string());
                }
            }
            Self::write_local_config(&local)?;
            print_success(&format!("Now ignoring directory: {} (local config)", name));
        } else {
            Config::global_add_ignored_dir(name);
            print_success(&format!("Now ignoring directory: {} (global config)", name));
        }
        Ok(())
    }

    /// Remove ignored directory from local config (or global if no local config)
    pub fn local_remove_ignored_dir(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut vec) = local.ignored_directory_names {
                vec.retain(|x| x != name);
                if vec.is_empty() {
                    local.ignored_directory_names = None;
                }
            }
            Self::write_local_config(&local)?;
            print_success(&format!("No longer ignoring directory: {} (local config)", name));
        } else {
            Config::global_remove_ignored_dir(name);
            print_success(&format!("No longer ignoring directory: {} (global config)", name));
        }
        Ok(())
    }

    /// Add ignored extension to local config (or global if no local config)
    pub fn local_add_ignored_extension(ext: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let ext_lower = ext.to_lowercase();
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.ignored_extensions.is_none() {
                local.ignored_extensions = Some(Vec::new());
            }
            if let Some(ref mut vec) = local.ignored_extensions {
                if !vec.contains(&ext_lower) {
                    vec.push(ext_lower);
                }
            }
            Self::write_local_config(&local)?;
            print_success(&format!("Now ignoring .{} files (local config)", ext));
        } else {
            Config::global_add_ignored_extension(ext);
            print_success(&format!("Now ignoring .{} files (global config)", ext));
        }
        Ok(())
    }

    /// Add extra supported extension to local config (or global if no local config)
    pub fn local_add_extra_supported_extension(ext: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let ext_lower = ext.to_lowercase();
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.extra_supported_extensions.is_none() {
                local.extra_supported_extensions = Some(Vec::new());
            }
            if let Some(ref mut vec) = local.extra_supported_extensions {
                if !vec.contains(&ext_lower) {
                    vec.push(ext_lower);
                }
            }
            Self::write_local_config(&local)?;
            print_success(&format!("Now caring about .{} files (local config)", ext));
        } else {
            Config::global_add_extra_supported_extension(ext);
            print_success(&format!("Now caring about .{} files (global config)", ext));
        }
        Ok(())
    }

    /// Add ignored file to local config (or global if no local config)
    pub fn local_add_ignored_file(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.ignored_files.is_none() {
                local.ignored_files = Some(Vec::new());
            }
            if let Some(ref mut vec) = local.ignored_files {
                if !vec.contains(&name.to_string()) {
                    vec.push(name.to_string());
                }
            }
            Self::write_local_config(&local)?;
            print_success(&format!("Now ignoring file: {} (local config)", name));
        } else {
            Config::global_add_ignored_file(name);
            print_success(&format!("Now ignoring file: {} (global config)", name));
        }
        Ok(())
    }

    /// Add extra supported file to local config (or global if no local config)
    pub fn local_add_extra_supported_file(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.extra_supported_files.is_none() {
                local.extra_supported_files = Some(Vec::new());
            }
            if let Some(ref mut vec) = local.extra_supported_files {
                if !vec.contains(&name.to_string()) {
                    vec.push(name.to_string());
                }
            }
            Self::write_local_config(&local)?;
            print_success(&format!("Now caring about file: {} (local config)", name));
        } else {
            Config::global_add_extra_supported_file(name);
            print_success(&format!("Now caring about file: {} (global config)", name));
        }
        Ok(())
    }

    /// Add run alias to local config (or global if no local config)
    pub fn local_add_run_alias(name: &str, command: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.run_aliases.is_none() {
                local.run_aliases = Some(std::collections::HashMap::new());
            }
            if let Some(ref mut map) = local.run_aliases {
                map.insert(name_lower, command.to_string());
            }
            Self::write_local_config(&local)?;
            print_success(&format!("Alias '{}' -> '{}' (local config)", name, command));
        } else {
            Config::global_add_run_alias(name, command);
            print_success(&format!("Alias '{}' -> '{}' (global config)", name, command));
        }
        Ok(())
    }

    /// Remove run alias from local config (or global if no local config)
    pub fn local_remove_run_alias(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_aliases {
                if map.remove(&name_lower).is_some() {
                    if map.is_empty() {
                        local.run_aliases = None;
                    }
                    Self::write_local_config(&local)?;
                    print_success(&format!("Removed alias '{}' (local config)", name));
                } else {
                    print_error(&format!("Alias '{}' not found in local config", name));
                }
            } else {
                print_error(&format!("Alias '{}' not found in local config", name));
            }
        } else {
            Config::global_remove_run_alias(name);
            print_success(&format!("Removed alias '{}' (global config)", name));
        }
        Ok(())
    }

    /// Update run alias in local config (or global if no local config)
    pub fn local_update_run_alias(name: &str, command: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_aliases {
                if map.contains_key(&name_lower) {
                    map.insert(name_lower, command.to_string());
                    Self::write_local_config(&local)?;
                    print_success(&format!("Updated alias '{}' -> '{}' (local config)", name, command));
                    return Ok(true);
                }
            }
            print_error(&format!("Alias '{}' not found in local config", name));
            Ok(false)
        } else {
            let result = Config::global_update_run_alias(name, command);
            if result {
                print_success(&format!("Updated alias '{}' -> '{}' (global config)", name, command));
            } else {
                print_error(&format!("Alias '{}' not found in global config", name));
            }
            Ok(result)
        }
    }

    /// Get all run aliases (already handled by global_get_run_aliases which merges)
    /// But for display, we might want to show source
    pub fn get_run_aliases_with_source() -> (HashMap<String, String>, bool) {
        let is_local = Self::get_local_config_path().is_some();
        let aliases = Config::global_get_run_aliases();
        (aliases, is_local)
    }

    /// Clear all run aliases from local config (or global if no local config)
    pub fn local_clear_run_aliases() -> std::result::Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = Self::get_local_config_path() {
            let mut local = Self::read_local_config().unwrap_or_default();
            let count = if let Some(ref map) = local.run_aliases {
                map.len()
            } else {
                0
            };
            local.run_aliases = None;
            Self::write_local_config(&local)?;
            print_success(&format!("Cleared {} run alias(es) from local config.", count));
        } else {
            let mut cfg = Config::global().write().unwrap();
            let count = cfg.run_aliases.len();
            cfg.run_aliases.clear();
            cfg.save();
            print_success(&format!("Cleared {} run alias(es) from global config.", count));
        }
        Ok(())
    }
    
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Local config file (.ntconfig) - ONLY ignore/care and run_aliases
/// All other settings are always global
#[derive(Debug, Serialize, Deserialize, Default)]
struct LocalConfig {
    pub ignored_directory_names: Option<Vec<String>>,
    pub ignored_extensions: Option<Vec<String>>,
    pub extra_supported_extensions: Option<Vec<String>>,
    pub ignored_files: Option<Vec<String>>,
    pub extra_supported_files: Option<Vec<String>>,
    pub run_aliases: Option<HashMap<String, String>>,
}