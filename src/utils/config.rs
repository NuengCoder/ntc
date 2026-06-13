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

    /// Whether color output is enabled
    #[serde(default = "default_color_enabled")]
    pub color_enabled: bool,

    #[serde(default)]
    pub teleports: HashMap<String, PathBuf>,

    #[serde(default)]
    pub run_aliases: HashMap<String, String>,

    #[serde(default)]
    pub math_functions: HashMap<String, String>,

    /// Set via `watch trigger <alias>` or `watch trigger off`.
    #[serde(default)]
    pub watch_trigger_alias: Option<String>,

    /// Whether autosuggest (ghost text) is enabled in the shell
    #[serde(default = "default_autosuggest_enabled")]
    pub autosuggest_enabled: bool,
}

// Single source of truth for alias name validation.
// Used by both config.rs and commands.rs — keep this list in sync with
// all shell commands that users should not be able to override.
pub fn validate_alias_name(name: &str) -> bool {
    let reserved_commands = [
        "go", "cd", "godrive", "god", "back", "b", "view", "ls", "txt", "txtc", "txtf",
        "html", "json", "md", "pdf", "docx", "xlsx",
        "seto", "setd", "setl", "sett", "seth", "setc", "seta",
        "watch", "clear", "version", "where",
        "gos", "gosc", "ral", "run", "r", "showcg", "help", "exit", "quit",
        "ignored", "ignore", "cared", "ignoref", "caref", "ignoren", "caren",
        "ignores", "ignoresc", "cares", "caresc",
        "size", "tp", "opencg", "resetcg", "restorecg", "local",
        "ne", "ntceditor", "igcare",
        "esc", "bkup", "pldw", "unpd", "gs", "fs", "ds", "diff", "fgo", "fsc", "locate",
        "dino", "math", "init", "deinit",
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
fn default_max_depth() -> usize { 2 }
fn default_num_threads() -> usize { 4 }
fn default_history_enabled() -> bool { false }
fn default_color_enabled() -> bool { false }
fn default_autosuggest_enabled() -> bool { true }
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
            color_enabled: default_color_enabled(),
            autosuggest_enabled: default_autosuggest_enabled(),
            teleports: HashMap::new(),
            run_aliases: HashMap::new(),
            math_functions: HashMap::new(),
            watch_trigger_alias: None,
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

        // Apply color override at startup
        colored::control::set_override(cfg.color_enabled);

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
        let mut cfg = Self::write_global();
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
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
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
        Self::read_global().output_path.clone()
    }
    pub fn global_set_output_path(path: &Path) {
        let mut cfg = Self::write_global();
        cfg.output_path = path.to_path_buf();
        cfg.save(); // safe: cfg is &Config, no lock re-acquired
    }
    pub fn global_get_max_depth() -> usize {
        Self::read_global().max_depth
    }
    pub fn global_set_max_depth(depth: usize) {
        let mut cfg = Self::write_global();
        cfg.max_depth = depth.clamp(1, 20);
        cfg.save();
    }
    pub fn global_get_show_line_numbers() -> bool {
        Self::read_global().show_line_numbers
    }
    pub fn global_set_show_line_numbers(show: bool) {
        let mut cfg = Self::write_global();
        cfg.show_line_numbers = show;
        cfg.save();
    }
    pub fn global_get_num_threads() -> usize {
        Self::read_global().num_threads
    }
    pub fn global_set_num_threads(threads: usize) {
        let mut cfg = Self::write_global();
        cfg.num_threads = threads.clamp(1, 16);
        cfg.save();
    }

    // ---- History ----
    pub fn global_get_history_path() -> Option<PathBuf> {
        Self::read_global().history_path.clone()
    }
    pub fn global_get_history_enabled() -> bool {
        Self::read_global().history_enabled
    }
    pub fn global_set_history_path(path: Option<PathBuf>) {
        let mut cfg = Self::write_global();
        cfg.history_path = path;
        cfg.save();
    }
    pub fn global_set_history_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.history_enabled = enabled;
        cfg.save();
    }

    // ---- Ignore / care helpers ----
    pub fn global_get_ignored_dirs() -> HashSet<String> {
        Self::read_global().ignored_directory_names.clone()
    }
    pub fn global_add_ignored_dir(name: &str) {
        let mut cfg = Self::write_global();
        cfg.ignored_directory_names.insert(name.to_string());
        cfg.save();
    }
    pub fn global_remove_ignored_dir(name: &str) {
        let mut cfg = Self::write_global();
        cfg.ignored_directory_names.remove(name);
        cfg.save();
    }
    pub fn global_get_ignored_extensions() -> HashSet<String> {
        Self::read_global().ignored_extensions.clone()
    }
    pub fn global_add_ignored_extension(ext: &str) {
        let mut cfg = Self::write_global();
        cfg.ignored_extensions.insert(ext.to_lowercase());
        // Remove from extra_supported to fix conflict
        cfg.extra_supported_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }
    pub fn global_remove_ignored_extension(ext: &str) {
        let mut cfg = Self::write_global();
        cfg.ignored_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }
    pub fn global_get_extra_supported_extensions() -> HashSet<String> {
        Self::read_global().extra_supported_extensions.clone()
    }
    pub fn global_add_extra_supported_extension(ext: &str) {
        let mut cfg = Self::write_global();
        cfg.extra_supported_extensions.insert(ext.to_lowercase());
        // Remove from ignored to fix conflict
        cfg.ignored_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }
    pub fn global_remove_extra_supported_extension(ext: &str) {
        let mut cfg = Self::write_global();
        cfg.extra_supported_extensions.remove(&ext.to_lowercase());
        cfg.save();
    }

    // ---- Specific files (caren / ignoren) ----
    pub fn global_get_ignored_files() -> HashSet<String> {
        Self::read_global().ignored_files.clone()
    }
    pub fn global_add_ignored_file(name: &str) {
        let mut cfg = Self::write_global();
        cfg.ignored_files.insert(name.to_string());
        cfg.extra_supported_files.remove(name);
        cfg.save();
    }
    pub fn global_remove_ignored_file(name: &str) {
        let mut cfg = Self::write_global();
        cfg.ignored_files.remove(name);
        cfg.save();
    }
    pub fn global_get_extra_supported_files() -> HashSet<String> {
        Self::read_global().extra_supported_files.clone()
    }
    pub fn global_add_extra_supported_file(name: &str) {
        let mut cfg = Self::write_global();
        cfg.extra_supported_files.insert(name.to_string());
        cfg.ignored_files.remove(name);
        cfg.save();
    }
    pub fn global_remove_extra_supported_file(name: &str) {
        let mut cfg = Self::write_global();
        cfg.extra_supported_files.remove(name);
        cfg.save();
    }

    pub fn global_get_file_watcher_enabled() -> bool {
        Self::read_global().file_watcher_enabled
    }
    pub fn global_set_file_watcher_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.file_watcher_enabled = enabled;
        cfg.save();
    }

    pub fn global_get_color_enabled() -> bool {
        Self::read_global().color_enabled
    }
    pub fn global_set_color_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.color_enabled = enabled;
        cfg.save();
        colored::control::set_override(enabled);
    }

    pub fn global_get_autosuggest_enabled() -> bool {
        Self::read_global().autosuggest_enabled
    }
    pub fn global_set_autosuggest_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.autosuggest_enabled = enabled;
        cfg.save();
    }

    // Parse helpers
    pub fn parse_on_off(state: &str) -> Option<bool> {
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
        Self::read_global().teleports.clone()
    }
    
    pub fn global_set_teleports(teleports: HashMap<String, PathBuf>) {
        let mut cfg = Self::write_global();
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
        
        let mut cfg = Self::write_global();
        
        // Check if old alias exists
        if !cfg.run_aliases.contains_key(&old_lower) {
            return false;
        }
        
        // Check if new name already exists
        if cfg.run_aliases.contains_key(&new_lower) {
            return false;
        }
        
        // Perform rename
        if let Some(command) = cfg.run_aliases.remove(&old_lower) {
            cfg.run_aliases.insert(new_lower, command);
        } else {
            return false;
        }
        cfg.save();
        
        true
    }

    pub fn global_get_run_aliases() -> HashMap<String, String> {
        Self::read_global().run_aliases.clone()
    }
    
    pub fn global_add_run_alias(name: &str, command: &str) {
        let mut cfg = Self::write_global();
        cfg.run_aliases.insert(name.to_lowercase(), command.to_string());
        cfg.save();
    }
    
    pub fn global_remove_run_alias(name: &str) {
        let mut cfg = Self::write_global();
        cfg.run_aliases.remove(&name.to_lowercase());
        cfg.save();
    }
    
    pub fn global_update_run_alias(name: &str, command: &str) -> bool {
        let mut cfg = Self::write_global();
        if let std::collections::hash_map::Entry::Occupied(mut e) = cfg.run_aliases.entry(name.to_lowercase()) {
            e.insert(command.to_string());
            cfg.save();
            true
        } else {
            false
        }
    }

    // ---- Math functions (global only) ----

    pub fn global_get_math_fns() -> HashMap<String, String> {
        Self::read_global().math_functions.clone()
    }

    pub fn global_add_math_fn(name: &str, def: &str) {
        let mut cfg = Self::write_global();
        cfg.math_functions.insert(name.to_lowercase(), def.to_string());
        cfg.save();
    }

    pub fn global_remove_math_fn(name: &str) {
        let mut cfg = Self::write_global();
        cfg.math_functions.remove(&name.to_lowercase());
        cfg.save();
    }

    pub fn global_update_math_fn(name: &str, def: &str) -> bool {
        let mut cfg = Self::write_global();
        if let std::collections::hash_map::Entry::Occupied(mut e) = cfg.math_functions.entry(name.to_lowercase()) {
            e.insert(def.to_string());
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
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
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_aliases {
                if let std::collections::hash_map::Entry::Occupied(mut e) = map.entry(name_lower) {
                    e.insert(command.to_string());
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
        if Self::get_local_config_path().is_some() {
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
            let mut cfg = Config::write_global();
            let count = cfg.run_aliases.len();
            cfg.run_aliases.clear();
            cfg.save();
            print_success(&format!("Cleared {} run alias(es) from global config.", count));
        }
        Ok(())
    }

    pub fn global_get_watch_trigger_alias() -> Option<String> {
        Self::read_global().watch_trigger_alias.clone()
    }
    
    pub fn global_set_watch_trigger_alias(alias: Option<String>) {
        let mut cfg = Self::write_global();
        cfg.watch_trigger_alias = alias;
        cfg.save();
    }

    // ---- Import helpers ----

    /// Import run aliases from a `.ntc.ral` file and merge into local or global config.
    pub fn import_run_aliases_from_file(path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        #[derive(Deserialize)]
        struct RalFile {
            run_aliases: Option<std::collections::HashMap<String, String>>,
        }
        let ral: RalFile = toml::from_str(&content)?;
        if let Some(aliases) = ral.run_aliases {
            for (name, cmd) in &aliases {
                Self::local_add_run_alias(name, cmd)?;
            }
        }
        Ok(())
    }

    /// Import ignore/care settings from a `.ntc.igcare` file and merge into local or global config.
    pub fn import_igcare_from_file(path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        #[derive(Deserialize)]
        struct IgcareFile {
            ignored_directory_names: Option<Vec<String>>,
            ignored_extensions: Option<Vec<String>>,
            extra_supported_extensions: Option<Vec<String>>,
            ignored_files: Option<Vec<String>>,
            extra_supported_files: Option<Vec<String>>,
        }
        let igc: IgcareFile = toml::from_str(&content)?;

        // Merge each category — add each item to the current config
        if let Some(ref dirs) = igc.ignored_directory_names {
            for dir in dirs {
                Self::local_add_ignored_dir(dir)?;
            }
        }
        if let Some(ref exts) = igc.ignored_extensions {
            for ext in exts {
                Self::local_add_ignored_extension(ext)?;
            }
        }
        if let Some(ref exts) = igc.extra_supported_extensions {
            for ext in exts {
                Self::local_add_extra_supported_extension(ext)?;
            }
        }
        if let Some(ref files) = igc.ignored_files {
            for file in files {
                Self::local_add_ignored_file(file)?;
            }
        }
        if let Some(ref files) = igc.extra_supported_files {
            for file in files {
                Self::local_add_extra_supported_file(file)?;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_alias_name_rejects_reserved() {
        for cmd in [
            "go", "cd", "help", "exit", "quit", "run", "r", "ral",
            "dino", "math", "setc", "diff", "bkup", "view", "txt",
        ] {
            assert!(!validate_alias_name(cmd), "reserved '{}' should be rejected", cmd);
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
        // The check compares the literal name against lowercase reserved words,
        // so uppercase versions of reserved words are accepted as aliases.
        assert!(validate_alias_name("GO"), "uppercase 'GO' is not in reserved list");
        assert!(validate_alias_name("DINO"), "uppercase 'DINO' is not in reserved list");
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
        assert!(cfg.teleports.is_empty());
        assert!(cfg.run_aliases.is_empty());
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