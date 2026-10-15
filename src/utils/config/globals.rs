//! Convenience global getters/setters and local config file operations.
//! Provides `impl Config` blocks for the convenience methods that read and
//! mutate the global singleton, plus local ntconfig.toml file management.

use crate::config::{validate_alias_name, Config, LocalConfig};

use crate::output::{print_error, print_success};
use serde::Deserialize;
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
    pub fn global_get_watch_limit() -> usize {
        Self::read_global().watch_limit
    }
    /// No upper bound by design; only the lower bound is guarded.
    pub fn global_set_watch_limit(n: usize) {
        let mut cfg = Self::write_global();
        cfg.watch_limit = n.max(1);
        cfg.save();
    }

    pub fn global_get_color_enabled() -> bool {
        Self::read_global().color_enabled
    }
    pub fn global_set_color_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.color_enabled = enabled;
        cfg.save();
        // Never force colors on when the console can't render them —
        // otherwise `setc ON` on legacy conhost brings back `[91m` garbage.
        let vt_ok = crate::utils::terminal::detect().color;
        colored::control::set_override(enabled && vt_ok);
    }

    pub fn global_get_autosuggest_enabled() -> bool {
        Self::read_global().autosuggest_enabled
    }
    pub fn global_set_autosuggest_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.autosuggest_enabled = enabled;
        cfg.save();
    }

    pub fn global_get_git_integration_enabled() -> bool {
        Self::read_global().git_integration_enabled
    }
    pub fn global_set_git_integration_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.git_integration_enabled = enabled;
        cfg.save();
        crate::utils::git::invalidate();
    }

    pub fn global_get_language() -> String {
        Self::read_global().language.clone()
    }
    pub fn global_set_language(lang: &str) {
        let mut cfg = Self::write_global();
        cfg.language = lang.to_string();
        cfg.save();
    }

    pub fn global_get_language_active() -> bool {
        Self::read_global().language_active
    }
    pub fn global_set_language_active(active: bool) {
        let mut cfg = Self::write_global();
        cfg.language_active = active;
        cfg.save();
    }

    pub fn global_reload_lang() {
        crate::utils::config::lang::reload();
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
        cfg.run_aliases
            .insert(name.to_lowercase(), command.to_string());
        cfg.save();
    }

    pub fn global_remove_run_alias(name: &str) {
        let mut cfg = Self::write_global();
        cfg.run_aliases.remove(&name.to_lowercase());
        cfg.save();
    }

    pub fn global_update_run_alias(name: &str, command: &str) -> bool {
        let mut cfg = Self::write_global();
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            cfg.run_aliases.entry(name.to_lowercase())
        {
            e.insert(command.to_string());
            cfg.save();
            true
        } else {
            false
        }
    }

    // ---- Run alias completion hints (`#param --hint-*` specs) ----
    //
    // Mirrors the `run_aliases` map one-to-one (same local-or-global
    // routing, same lowercased keys). Values are serialized hint specs
    // (`#x --hint-dir …`) produced by `serialize_hint_spec`.

    pub fn global_get_run_alias_hints() -> HashMap<String, String> {
        Self::read_global().run_alias_hints.clone()
    }

    pub fn global_add_run_alias_hint(name: &str, spec: &str) {
        let mut cfg = Self::write_global();
        cfg.run_alias_hints
            .insert(name.to_lowercase(), spec.to_string());
        cfg.save();
    }

    pub fn global_remove_run_alias_hint(name: &str) {
        let mut cfg = Self::write_global();
        cfg.run_alias_hints.remove(&name.to_lowercase());
        cfg.save();
    }

    pub fn global_update_run_alias_hint(name: &str, spec: &str) -> bool {
        let mut cfg = Self::write_global();
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            cfg.run_alias_hints.entry(name.to_lowercase())
        {
            e.insert(spec.to_string());
            cfg.save();
            true
        } else {
            false
        }
    }

    pub fn global_rename_run_alias_hint(old_name: &str, new_name: &str) {
        let mut cfg = Self::write_global();
        if let Some(spec) = cfg.run_alias_hints.remove(&old_name.to_lowercase()) {
            cfg.run_alias_hints.insert(new_name.to_lowercase(), spec);
            cfg.save();
        }
    }

    pub fn global_clear_run_alias_hints() {
        let mut cfg = Self::write_global();
        cfg.run_alias_hints.clear();
        cfg.save();
    }

    // ---- Named hint value lists (`ral hintn` dimension) ----
    //
    // Values for `#param --hint-n <name>` / `--select-n <name>`, keyed by
    // lowercased list name. Same local-or-global routing as `run_aliases`:
    // writes land in ntconfig.toml when one exists in the cwd, reads merge
    // global + local (local wins, like `db_aliases`).

    /// Merged hint lists: global entries overlaid by local ones.
    pub fn get_run_hint_lists_merged() -> HashMap<String, Vec<String>> {
        let mut merged = Self::read_global().run_hint_lists.clone();
        if let Ok(local) = Self::read_local_config() {
            if let Some(map) = local.run_hint_lists {
                merged.extend(map);
            }
        }
        merged
    }

    /// Merged hint lists plus whether the cwd has a local ntconfig.toml.
    pub fn get_run_hint_lists_with_source() -> (HashMap<String, Vec<String>>, bool) {
        let is_local = Self::get_local_config_path().is_some();
        (Self::get_run_hint_lists_merged(), is_local)
    }

    pub fn global_get_run_hint_lists() -> HashMap<String, Vec<String>> {
        Self::read_global().run_hint_lists.clone()
    }

    pub fn global_add_run_hint_list(name: &str, values: &[String]) {
        let mut cfg = Self::write_global();
        cfg.run_hint_lists
            .insert(name.to_lowercase(), values.to_vec());
        cfg.save();
    }

    /// Remove a hint list from the global config. Returns true when removed.
    pub fn global_remove_run_hint_list(name: &str) -> bool {
        let mut cfg = Self::write_global();
        let removed = cfg.run_hint_lists.remove(&name.to_lowercase()).is_some();
        cfg.save();
        removed
    }

    pub fn global_rename_run_hint_list(old_name: &str, new_name: &str) {
        let mut cfg = Self::write_global();
        if let Some(values) = cfg.run_hint_lists.remove(&old_name.to_lowercase()) {
            cfg.run_hint_lists.insert(new_name.to_lowercase(), values);
            cfg.save();
        }
    }

    /// Add a hint list to local config (or global if no local config).
    pub fn local_add_run_hint_list(
        name: &str,
        values: &[String],
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.run_hint_lists.is_none() {
                local.run_hint_lists = Some(std::collections::HashMap::new());
            }
            if let Some(ref mut map) = local.run_hint_lists {
                map.insert(name_lower, values.to_vec());
            }
            Self::write_local_config(&local)?;
        } else {
            Config::global_add_run_hint_list(name, values);
        }
        Ok(())
    }

    /// Remove a hint list from local config (or global if no local config).
    /// Returns true when an entry was actually removed.
    pub fn local_remove_run_hint_list(
        name: &str,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            let removed = match local.run_hint_lists.as_mut() {
                Some(map) => {
                    let removed = map.remove(&name_lower).is_some();
                    if map.is_empty() {
                        local.run_hint_lists = None;
                    }
                    removed
                }
                None => false,
            };
            Self::write_local_config(&local)?;
            Ok(removed)
        } else {
            Ok(Config::global_remove_run_hint_list(name))
        }
    }

    /// Move a hint list from old name to new name (for `ral hintn rnm`).
    pub fn local_rename_run_hint_list(old_name: &str, new_name: &str) {
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_hint_lists {
                if let Some(values) = map.remove(&old_name.to_lowercase()) {
                    map.insert(new_name.to_lowercase(), values);
                    let _ = Self::write_local_config(&local);
                }
            }
        } else {
            Config::global_rename_run_hint_list(old_name, new_name);
        }
    }

    // ---- Database aliases (`dbal` dimension for the ndb shell) ----
    //
    // Reads merge global + local (local wins); writes go to the local
    // ntconfig.toml when one exists in the cwd, else to the global config.
    // Unlike `ral`, these helpers never print — the `ndb` layer owns its
    // localized messages.

    /// Merged `dbal` map: global entries overlaid by local ones.
    pub fn get_db_aliases() -> HashMap<String, String> {
        let mut merged = Self::read_global().db_aliases.clone();
        if let Ok(local) = Self::read_local_config() {
            if let Some(map) = local.db_aliases {
                merged.extend(map);
            }
        }
        merged
    }

    /// Whether the cwd has a local ntconfig.toml (write routing target).
    pub fn db_aliases_are_local() -> bool {
        Self::get_local_config_path().is_some()
    }

    /// Insert or overwrite a `dbal` entry. Returns `true` when it landed in
    /// the local config, `false` for the global config.
    pub fn add_db_alias(
        name: &str,
        sql: &str,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.db_aliases.is_none() {
                local.db_aliases = Some(HashMap::new());
            }
            if let Some(ref mut map) = local.db_aliases {
                map.insert(name_lower, sql.to_string());
            }
            Self::write_local_config(&local)?;
            Ok(true)
        } else {
            let mut cfg = Self::write_global();
            cfg.db_aliases.insert(name_lower, sql.to_string());
            cfg.save();
            Ok(false)
        }
    }

    /// Remove a `dbal` entry from the routed store. Returns `true` when an
    /// entry was actually removed.
    pub fn remove_db_alias(name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            match local
                .db_aliases
                .as_mut()
                .and_then(|m| m.remove(&name_lower))
            {
                Some(_) => {
                    if local.db_aliases.as_ref().is_some_and(|m| m.is_empty()) {
                        local.db_aliases = None;
                    }
                    Self::write_local_config(&local)?;
                    Ok(true)
                }
                None => Ok(false),
            }
        } else {
            let mut cfg = Self::write_global();
            let removed = cfg.db_aliases.remove(&name_lower).is_some();
            cfg.save();
            Ok(removed)
        }
    }

    /// Replace the SQL of an existing `dbal` entry in the routed store.
    /// Returns `true` when the entry existed and was updated.
    pub fn update_db_alias(
        name: &str,
        sql: &str,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            let entry = local
                .db_aliases
                .as_mut()
                .and_then(|m| m.get_mut(&name_lower));
            match entry {
                Some(slot) => {
                    *slot = sql.to_string();
                    Self::write_local_config(&local)?;
                    Ok(true)
                }
                None => Ok(false),
            }
        } else {
            let mut cfg = Self::write_global();
            if let std::collections::hash_map::Entry::Occupied(mut e) =
                cfg.db_aliases.entry(name_lower)
            {
                e.insert(sql.to_string());
                cfg.save();
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    /// Rename a `dbal` entry, keeping it in whichever store holds it.
    /// Returns `false` when `old` is unknown or `new` is taken/invalid.
    pub fn rename_db_alias(old_name: &str, new_name: &str) -> bool {
        let old_lower = old_name.to_lowercase();
        let new_lower = new_name.to_lowercase();
        if !validate_alias_name(&new_lower) {
            return false;
        }
        let merged = Self::get_db_aliases();
        let Some(sql) = merged.get(&old_lower).cloned() else {
            return false;
        };
        if merged.contains_key(&new_lower) {
            return false;
        }
        // Remove from both stores, then re-add through the normal routing.
        {
            let mut cfg = Self::write_global();
            cfg.db_aliases.remove(&old_lower);
            cfg.save();
        }
        if Self::get_local_config_path().is_some() {
            if let Ok(mut local) = Self::read_local_config() {
                let mut touched = false;
                if let Some(ref mut map) = local.db_aliases {
                    touched = map.remove(&old_lower).is_some();
                    if map.is_empty() {
                        local.db_aliases = None;
                    }
                }
                if touched {
                    let _ = Self::write_local_config(&local);
                }
            }
        }
        Self::add_db_alias(&new_lower, &sql).is_ok()
    }

    /// Clear the routed store's `dbal` entries. Returns the count removed.
    pub fn clear_db_aliases() -> std::result::Result<usize, Box<dyn std::error::Error>> {
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            let count = local.db_aliases.as_ref().map(|m| m.len()).unwrap_or(0);
            local.db_aliases = None;
            Self::write_local_config(&local)?;
            Ok(count)
        } else {
            let mut cfg = Self::write_global();
            let count = cfg.db_aliases.len();
            cfg.db_aliases.clear();
            cfg.save();
            Ok(count)
        }
    }

    // ---- Math functions (global only) ----

    pub fn global_get_math_fns() -> HashMap<String, String> {
        Self::read_global().math_functions.clone()
    }

    pub fn global_add_math_fn(name: &str, def: &str) {
        let mut cfg = Self::write_global();
        cfg.math_functions
            .insert(name.to_lowercase(), def.to_string());
        cfg.save();
    }

    pub fn global_remove_math_fn(name: &str) {
        let mut cfg = Self::write_global();
        cfg.math_functions.remove(&name.to_lowercase());
        cfg.save();
    }

    pub fn global_update_math_fn(name: &str, def: &str) -> bool {
        let mut cfg = Self::write_global();
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            cfg.math_functions.entry(name.to_lowercase())
        {
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
    fn write_local_config(
        local: &LocalConfig,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    pub fn local_add_ignored_dir(
        name: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::modify_local_vec(
            |lc| &mut lc.ignored_directory_names,
            Config::global_add_ignored_dir,
            name,
            &format!("Now ignoring directory: {}", name),
        );
        Ok(())
    }

    /// Remove ignored directory from local config (or global if no local config)
    pub fn local_remove_ignored_dir(
        name: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::remove_from_local_vec(
            |lc| &mut lc.ignored_directory_names,
            Config::global_remove_ignored_dir,
            name,
            &format!("No longer ignoring directory: {}", name),
        );
        Ok(())
    }

    /// Add ignored extension to local config (or global if no local config)
    pub fn local_add_ignored_extension(
        ext: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    pub fn local_add_extra_supported_extension(
        ext: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let ext_lower = ext.to_lowercase();
        if Self::get_local_config_path().is_none() {
            Config::global_add_extra_supported_extension(&ext_lower);
            return Ok(());
        }
        let mut local = Self::read_local_config().unwrap_or_default();
        let vec = local
            .extra_supported_extensions
            .get_or_insert_with(Vec::new);
        if !vec.contains(&ext_lower) {
            vec.push(ext_lower.clone());
        }
        Self::write_local_config(&local)?;
        print_success(&format!("Now caring about .{} files (local config)", ext));
        Ok(())
    }

    /// Add ignored file to local config (or global if no local config)
    pub fn local_add_ignored_file(
        name: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::modify_local_vec(
            |lc| &mut lc.ignored_files,
            Config::global_add_ignored_file,
            name,
            &format!("Now ignoring file: {}", name),
        );
        Ok(())
    }

    /// Add extra supported file to local config (or global if no local config)
    pub fn local_add_extra_supported_file(
        name: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Self::modify_local_vec(
            |lc| &mut lc.extra_supported_files,
            Config::global_add_extra_supported_file,
            name,
            &format!("Now caring about file: {}", name),
        );
        Ok(())
    }

    /// Add run alias to local config (or global if no local config)
    pub fn local_add_run_alias(
        name: &str,
        command: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
            print_success(&format!(
                "Alias '{}' -> '{}' (global config)",
                name, command
            ));
        }
        Ok(())
    }

    /// Remove run alias from local config (or global if no local config)
    pub fn local_remove_run_alias(
        name: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    pub fn local_update_run_alias(
        name: &str,
        command: &str,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_aliases {
                if let std::collections::hash_map::Entry::Occupied(mut e) = map.entry(name_lower) {
                    e.insert(command.to_string());
                    Self::write_local_config(&local)?;
                    print_success(&format!(
                        "Updated alias '{}' -> '{}' (local config)",
                        name, command
                    ));
                    return Ok(true);
                }
            }
            print_error(&format!("Alias '{}' not found in local config", name));
            Ok(false)
        } else {
            let result = Config::global_update_run_alias(name, command);
            if result {
                print_success(&format!(
                    "Updated alias '{}' -> '{}' (global config)",
                    name, command
                ));
            } else {
                print_error(&format!("Alias '{}' not found in global config", name));
            }
            Ok(result)
        }
    }

    /// Add run alias hint spec to local config (or global if no local config).
    /// Pass `None` as spec to just remove any existing hint (used on plain edits).
    pub fn local_add_run_alias_hint(
        name: &str,
        spec: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if local.run_alias_hints.is_none() {
                local.run_alias_hints = Some(std::collections::HashMap::new());
            }
            if let Some(ref mut map) = local.run_alias_hints {
                map.insert(name_lower, spec.to_string());
            }
            Self::write_local_config(&local)?;
        } else {
            Config::global_add_run_alias_hint(name, spec);
        }
        Ok(())
    }

    /// Remove a run alias hint spec from local config (or global if no local config).
    pub fn local_remove_run_alias_hint(
        name: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let name_lower = name.to_lowercase();
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_alias_hints {
                if map.remove(&name_lower).is_some() && map.is_empty() {
                    local.run_alias_hints = None;
                }
                Self::write_local_config(&local)?;
            }
        } else {
            Config::global_remove_run_alias_hint(name);
        }
        Ok(())
    }

    /// Move a hint spec from old name to new name (for `ral rnm`).
    pub fn local_rename_run_alias_hint(old_name: &str, new_name: &str) {
        if Self::get_local_config_path().is_some() {
            let mut local = Self::read_local_config().unwrap_or_default();
            if let Some(ref mut map) = local.run_alias_hints {
                if let Some(spec) = map.remove(&old_name.to_lowercase()) {
                    map.insert(new_name.to_lowercase(), spec);
                    let _ = Self::write_local_config(&local);
                }
            }
        } else {
            Config::global_rename_run_alias_hint(old_name, new_name);
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
            local.run_alias_hints = None;
            Self::write_local_config(&local)?;
            print_success(&format!(
                "Cleared {} run alias(es) from local config.",
                count
            ));
        } else {
            // Drop WRITE guard before printing: print_success() re-reads
            // Config (via ansi_enabled) and would deadlock on same-thread
            // WRITE->READ re-acquire.
            let count = {
                let mut cfg = Config::write_global();
                let count = cfg.run_aliases.len();
                cfg.run_aliases.clear();
                cfg.run_alias_hints.clear();
                cfg.save();
                count
            };
            print_success(&format!(
                "Cleared {} run alias(es) from global config.",
                count
            ));
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

    // ---- QSC (Quick Short Command) ----

    pub fn global_get_ascii_fallback() -> bool {
        Self::read_global().ascii_fallback
    }

    pub fn global_set_ascii_fallback(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.ascii_fallback = enabled;
        cfg.save();
    }

    // ---- tmprt (OpenWeather) ----

    pub fn global_get_openweather_api_key() -> Option<String> {
        Self::read_global().openweather_api_key.clone()
    }

    pub fn global_set_openweather_api_key(key: Option<String>) {
        let mut cfg = Self::write_global();
        cfg.openweather_api_key = key;
        cfg.save();
    }

    pub fn global_get_openweather_default_city() -> Option<String> {
        Self::read_global().openweather_default_city.clone()
    }

    pub fn global_set_openweather_default_city(city: Option<String>) {
        let mut cfg = Self::write_global();
        cfg.openweather_default_city = city;
        cfg.save();
    }

    pub fn global_get_dir_view_enabled() -> bool {
        Self::read_global().dir_view_enabled
    }

    pub fn global_set_dir_view_enabled(enabled: bool) {
        let mut cfg = Self::write_global();
        cfg.dir_view_enabled = enabled;
        cfg.save();
    }

    pub fn global_get_qsc() -> Option<crate::config::QscHotkey> {
        Self::read_global().qsc.clone()
    }

    pub fn global_set_qsc(hotkey: Option<crate::config::QscHotkey>) {
        let mut cfg = Self::write_global();
        cfg.qsc = hotkey;
        cfg.save();
    }

    pub fn global_get_serve_port(svc: crate::config::ServeService) -> u16 {
        Self::read_global().serve.for_service(svc).port
    }

    pub fn global_set_serve_port(svc: crate::config::ServeService, port: u16) {
        let mut cfg = Self::write_global();
        cfg.serve.for_service_mut(svc).port = port;
        cfg.save();
    }

    pub fn global_get_serve_bind(svc: crate::config::ServeService) -> String {
        Self::read_global().serve.for_service(svc).bind.clone()
    }

    pub fn global_set_serve_bind(svc: crate::config::ServeService, bind: &str) {
        let mut cfg = Self::write_global();
        cfg.serve.for_service_mut(svc).bind = bind.to_string();
        cfg.save();
    }

    /// Parse a QSC hotkey string like `"#c #s #a 0"` or `"#c a"` or `"off"`.
    /// Returns None for "off", Some(hotkey) on success.
    pub fn parse_qsc(input: &str) -> Option<crate::config::QscHotkey> {
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("off") || trimmed.is_empty() {
            return None;
        }
        let mut ctrl = false;
        let mut shift = false;
        let mut alt = false;
        let mut key: Option<String> = None;

        for token in trimmed.split_whitespace() {
            match token {
                "#c" => ctrl = true,
                "#s" => shift = true,
                "#a" => alt = true,
                _ => {
                    if token.starts_with('#') {
                        return None;
                    }
                    key = Some(token.to_uppercase());
                }
            }
        }

        let key = key?;
        // Validate: single char or F1-F12
        if key.len() == 1 {
            let c = key.chars().next().unwrap();
            if !c.is_alphanumeric() {
                return None;
            }
        } else if ![
            "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
        ]
        .contains(&key.as_str())
        {
            return None;
        }
        // Require at least one modifier
        if !ctrl && !shift && !alt {
            return None;
        }

        Some(crate::config::QscHotkey {
            ctrl,
            shift,
            alt,
            key,
        })
    }

    // ---- Import helpers ----

    /// Import run aliases from a `.ntc.ral` file and merge into local or global config.
    /// Batches writes: reads the target config once, applies all changes, writes once.
    pub fn import_run_aliases_from_file(
        path: &Path,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        #[derive(Deserialize)]
        struct RalFile {
            run_aliases: Option<std::collections::HashMap<String, String>>,
            run_alias_hints: Option<std::collections::HashMap<String, String>>,
            run_hint_lists: Option<std::collections::HashMap<String, Vec<String>>>,
        }
        let ral: RalFile = toml::from_str(&content)?;
        let aliases = match ral.run_aliases {
            Some(a) => a,
            None => return Ok(()),
        };
        let hints = ral.run_alias_hints.unwrap_or_default();
        let lists = ral.run_hint_lists.unwrap_or_default();

        if Self::get_local_config_path().is_some() {
            // Batch into local config: read once, apply all, write once
            let mut local = Self::read_local_config().unwrap_or_default();
            let map = local
                .run_aliases
                .get_or_insert_with(std::collections::HashMap::new);
            for (name, cmd) in &aliases {
                map.insert(name.to_lowercase(), cmd.clone());
            }
            if !hints.is_empty() {
                let hmap = local
                    .run_alias_hints
                    .get_or_insert_with(std::collections::HashMap::new);
                for (name, spec) in &hints {
                    hmap.insert(name.to_lowercase(), spec.clone());
                }
            }
            if !lists.is_empty() {
                let lmap = local
                    .run_hint_lists
                    .get_or_insert_with(std::collections::HashMap::new);
                for (name, values) in &lists {
                    lmap.insert(name.to_lowercase(), values.clone());
                }
            }
            Self::write_local_config(&local)?;
        } else {
            // Batch into global config: write lock once, apply all, save once
            let mut cfg = Self::write_global();
            for (name, cmd) in &aliases {
                cfg.run_aliases.insert(name.to_lowercase(), cmd.clone());
            }
            for (name, spec) in &hints {
                cfg.run_alias_hints
                    .insert(name.to_lowercase(), spec.clone());
            }
            for (name, values) in &lists {
                cfg.run_hint_lists
                    .insert(name.to_lowercase(), values.clone());
            }
            cfg.save();
        }
        if lists.is_empty() {
            print_success(&format!("Imported {} alias(es).", aliases.len()));
        } else {
            print_success(&format!(
                "Imported {} alias(es) and {} hint list(s).",
                aliases.len(),
                lists.len()
            ));
        }
        Ok(())
    }

    /// Import ignore/care settings from a `.ntc.igcare` file and merge into local or global config.
    /// Batches writes: reads the target config once, applies all changes, writes once.
    pub fn import_igcare_from_file(
        path: &Path,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_support::TempCwd;

    fn local_ntconfig() -> TempCwd {
        TempCwd::with_fixture(&[("ntconfig.toml", "")], &[])
    }

    fn local_hints() -> HashMap<String, String> {
        Config::read_local_config()
            .unwrap_or_default()
            .run_alias_hints
            .unwrap_or_default()
    }

    #[test]
    fn hint_specs_follow_alias_lifecycle_locally() {
        let _cwd = local_ntconfig();
        // add
        Config::local_add_run_alias_hint("tld", "#x --hint-dir").unwrap();
        assert_eq!(
            local_hints().get("tld").map(String::as_str),
            Some("#x --hint-dir")
        );
        // rename moves the spec
        Config::local_rename_run_alias_hint("tld", "tlx");
        let hints = local_hints();
        assert!(!hints.contains_key("tld"));
        assert_eq!(hints.get("tlx").map(String::as_str), Some("#x --hint-dir"));
        // remove deletes it
        Config::local_remove_run_alias_hint("tlx").unwrap();
        assert!(!local_hints().contains_key("tlx"));
    }

    #[test]
    fn ral_import_brings_hint_section() {
        let _cwd = local_ntconfig();
        let dir = std::env::current_dir().unwrap();
        let path = dir.join("hints_test.ntc.ral");
        std::fs::write(
            &path,
            "[run_aliases]\ntld = \"tl dir ${x:-.}\"\n\n[run_alias_hints]\ntld = \"#x --hint-dir\"\n",
        )
        .unwrap();
        Config::import_run_aliases_from_file(&path).unwrap();
        let local = Config::read_local_config().unwrap_or_default();
        assert_eq!(
            local
                .run_aliases
                .unwrap_or_default()
                .get("tld")
                .map(String::as_str),
            Some("tl dir ${x:-.}")
        );
        assert_eq!(
            local_hints().get("tld").map(String::as_str),
            Some("#x --hint-dir")
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn ral_import_without_hints_section_still_works() {
        let _cwd = local_ntconfig();
        let dir = std::env::current_dir().unwrap();
        let path = dir.join("nohints_test.ntc.ral");
        std::fs::write(&path, "[run_aliases]\nbtr = \"cargo build\"\n").unwrap();
        Config::import_run_aliases_from_file(&path).unwrap();
        assert!(local_hints().is_empty());
        let _ = std::fs::remove_file(&path);
    }

    fn local_lists() -> HashMap<String, Vec<String>> {
        Config::read_local_config()
            .unwrap_or_default()
            .run_hint_lists
            .unwrap_or_default()
    }

    #[test]
    fn ral_import_brings_hint_lists_section() {
        let _cwd = local_ntconfig();
        let dir = std::env::current_dir().unwrap();
        let path = dir.join("lists_test.ntc.ral");
        std::fs::write(
            &path,
            "[run_aliases]\nsrv = \"ssh $p\"\n\n[run_alias_hints]\nsrv = \"#p --hint-n port\"\n\n[run_hint_lists]\nport = [\"8080\", \"5050\"]\n",
        )
        .unwrap();
        Config::import_run_aliases_from_file(&path).unwrap();
        assert_eq!(
            local_lists().get("port"),
            Some(&vec!["8080".to_string(), "5050".to_string()])
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn hint_lists_follow_routed_lifecycle() {
        let _cwd = local_ntconfig();
        Config::local_add_run_hint_list(
            "port",
            &["8080".to_string(), "5050".to_string()],
        )
        .unwrap();
        assert_eq!(
            local_lists().get("port"),
            Some(&vec!["8080".to_string(), "5050".to_string()])
        );
        Config::local_rename_run_hint_list("port", "ports");
        let lists = local_lists();
        assert!(!lists.contains_key("port"));
        assert!(lists.contains_key("ports"));
        assert!(Config::local_remove_run_hint_list("ports").unwrap());
        assert!(!local_lists().contains_key("ports"));
        assert!(!Config::local_remove_run_hint_list("ports").unwrap());
    }
}
