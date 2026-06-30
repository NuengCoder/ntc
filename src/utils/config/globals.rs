//! Convenience global getters/setters and local config file operations.
//! Provides `impl Config` blocks for the convenience methods that read and
//! mutate the global singleton, plus local ntconfig.toml file management.

use crate::config::{Config, LocalConfig, validate_alias_name};

use serde::Deserialize;
use crate::output::{print_error, print_success};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

impl Config {
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

    // ---- Generic local config mutation helpers ----
    //
    // These consolidate the repetitive "read-local-or-fall-back-to-global" pattern
    // that appeared 8+ times in the codebase.

    /// Modify a `Vec<String>` field on the local config (or global `HashSet` as fallback).
    fn modify_local_vec(
        local_field: fn(&mut LocalConfig) -> &mut Option<Vec<String>>,
        global_add: fn(&str),
        item: &str,
        success_msg: &str,
    ) {
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            let vec = local_field(&mut local);
            if vec.is_none() {
                *vec = Some(Vec::new());
            }
            if let Some(ref mut v) = vec {
                if !v.contains(&item.to_string()) {
                    v.push(item.to_string());
                }
            }
            if Self::write_local_config(&local).is_ok() {
                print_success(&format!("{} (local config)", success_msg));
            }
        } else {
            global_add(item);
        }
    }

    /// Remove from a `Vec<String>` field on the local config (or global `HashSet` as fallback).
    fn remove_from_local_vec(
        local_field: fn(&mut LocalConfig) -> &mut Option<Vec<String>>,
        global_remove: fn(&str),
        item: &str,
        success_msg: &str,
    ) {
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut vec) = local_field(&mut local) {
                vec.retain(|x| x != item);
                if vec.is_empty() {
                    *local_field(&mut local) = None;
                }
            }
            if Self::write_local_config(&local).is_ok() {
                print_success(&format!("{} (local config)", success_msg));
            }
        } else {
            global_remove(item);
        }
    }

    /// Add ignored directory to local config (or global if no local config)
    pub fn local_add_ignored_dir(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::modify_local_vec(
            |lc| &mut lc.ignored_directory_names,
            Config::global_add_ignored_dir,
            name,
            &format!("Now ignoring directory: {}", name),
        );
        Ok(())
    }

    /// Remove ignored directory from local config (or global if no local config)
    pub fn local_remove_ignored_dir(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::remove_from_local_vec(
            |lc| &mut lc.ignored_directory_names,
            Config::global_remove_ignored_dir,
            name,
            &format!("No longer ignoring directory: {}", name),
        );
        Ok(())
    }

    /// Add ignored extension to local config (or global if no local config)
    pub fn local_add_ignored_extension(ext: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let ext_lower = ext.to_lowercase();
        // Pre-add to global's `ignored_extensions` before going through the helper
        // so the fn pointer path can be used.
        if Self::get_local_config_path().is_none() {
            Config::global_add_ignored_extension(&ext_lower);
            return Ok(());
        }
        let mut local = Self::read_local_config().unwrap_or_default();
        let vec = local.ignored_extensions.get_or_insert_with(Vec::new);
        if !vec.contains(&ext_lower) {
            vec.push(ext_lower.clone());
        }
        Self::write_local_config(&local)?;
        print_success(&format!("Now ignoring .{} files (local config)", ext));
        Ok(())
    }

    /// Add extra supported extension to local config (or global if no local config)
    pub fn local_add_extra_supported_extension(ext: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let ext_lower = ext.to_lowercase();
        if Self::get_local_config_path().is_none() {
            Config::global_add_extra_supported_extension(&ext_lower);
            return Ok(());
        }
        let mut local = Self::read_local_config().unwrap_or_default();
        let vec = local.extra_supported_extensions.get_or_insert_with(Vec::new);
        if !vec.contains(&ext_lower) {
            vec.push(ext_lower.clone());
        }
        Self::write_local_config(&local)?;
        print_success(&format!("Now caring about .{} files (local config)", ext));
        Ok(())
    }

    /// Add ignored file to local config (or global if no local config)
    pub fn local_add_ignored_file(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::modify_local_vec(
            |lc| &mut lc.ignored_files,
            Config::global_add_ignored_file,
            name,
            &format!("Now ignoring file: {}", name),
        );
        Ok(())
    }

    /// Add extra supported file to local config (or global if no local config)
    pub fn local_add_extra_supported_file(name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::modify_local_vec(
            |lc| &mut lc.extra_supported_files,
            Config::global_add_extra_supported_file,
            name,
            &format!("Now caring about file: {}", name),
        );
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
    /// Batches writes: reads the target config once, applies all changes, writes once.
    pub fn import_run_aliases_from_file(path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        #[derive(Deserialize)]
        struct RalFile {
            run_aliases: Option<std::collections::HashMap<String, String>>,
        }
        let ral: RalFile = toml::from_str(&content)?;
        let aliases = match ral.run_aliases {
            Some(a) => a,
            None => return Ok(()),
        };

        if Self::get_local_config_path().is_some() {
            // Batch into local config: read once, apply all, write once
            let mut local = Self::read_local_config().unwrap_or_default();
            let map = local.run_aliases.get_or_insert_with(std::collections::HashMap::new);
            for (name, cmd) in &aliases {
                map.insert(name.to_lowercase(), cmd.clone());
            }
            Self::write_local_config(&local)?;
        } else {
            // Batch into global config: write lock once, apply all, save once
            let mut cfg = Self::write_global();
            for (name, cmd) in &aliases {
                cfg.run_aliases.insert(name.to_lowercase(), cmd.clone());
            }
            cfg.save();
        }
        print_success(&format!("Imported {} alias(es).", aliases.len()));
        Ok(())
    }

    /// Import ignore/care settings from a `.ntc.igcare` file and merge into local or global config.
    /// Batches writes: reads the target config once, applies all changes, writes once.
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

        let mut total = 0usize;

        if Self::get_local_config_path().is_some() {
            // Batch into local config: read once, apply all, write once
            let mut local = Self::read_local_config().unwrap_or_default();

            let mut extend = |target: &mut Option<Vec<String>>, items: &[String]| {
                let vec = target.get_or_insert_with(Vec::new);
                for item in items {
                    if !vec.contains(item) {
                        vec.push(item.clone());
                        total += 1;
                    }
                }
            };

            if let Some(ref dirs) = igc.ignored_directory_names {
                extend(&mut local.ignored_directory_names, dirs);
            }
            if let Some(ref exts) = igc.ignored_extensions {
                extend(&mut local.ignored_extensions, exts);
            }
            if let Some(ref exts) = igc.extra_supported_extensions {
                extend(&mut local.extra_supported_extensions, exts);
            }
            if let Some(ref files) = igc.ignored_files {
                extend(&mut local.ignored_files, files);
            }
            if let Some(ref files) = igc.extra_supported_files {
                extend(&mut local.extra_supported_files, files);
            }

            Self::write_local_config(&local)?;
        } else {
            // Batch into global config: write lock once, apply all, save once
            let mut cfg = Self::write_global();

            if let Some(ref dirs) = igc.ignored_directory_names {
                for dir in dirs {
                    if cfg.ignored_directory_names.insert(dir.clone()) {
                        total += 1;
                    }
                }
            }
            if let Some(ref exts) = igc.ignored_extensions {
                for ext in exts {
                    if cfg.ignored_extensions.insert(ext.to_lowercase()) {
                        total += 1;
                    }
                }
            }
            if let Some(ref exts) = igc.extra_supported_extensions {
                for ext in exts {
                    if cfg.extra_supported_extensions.insert(ext.to_lowercase()) {
                        total += 1;
                    }
                }
            }
            if let Some(ref files) = igc.ignored_files {
                for file in files {
                    if cfg.ignored_files.insert(file.clone()) {
                        total += 1;
                    }
                }
            }
            if let Some(ref files) = igc.extra_supported_files {
                for file in files {
                    if cfg.extra_supported_files.insert(file.clone()) {
                        total += 1;
                    }
                }
            }

            cfg.save();
        }
        print_success(&format!("Imported {} ignore/care setting(s).", total));
        Ok(())
    }
}