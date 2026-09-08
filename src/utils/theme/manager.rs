//! Global theme manager singleton.
//! Handles theme loading, switching, CRUD, import/export, and default
//! theme installation. Accessed via `ThemeManager::current()` and
//! `ThemeManager::set_theme()`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::RwLock;

use super::{Theme, THEME_CHANGED};

pub struct ThemeManager {
    current_theme: RwLock<Theme>,
    available_themes: RwLock<HashMap<String, Theme>>,
}

impl ThemeManager {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("theme.toml"))
    }

    pub fn themes_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ntc").join("themes"))
    }

    fn load_themes() -> HashMap<String, Theme> {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), Theme::default_theme());
        themes.insert("light".to_string(), Theme::light_theme());

        if let Some(themes_dir) = Self::themes_dir() {
            if let Ok(entries) = fs::read_dir(themes_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ext == "ntc_theme" || ext == "toml" {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(theme) = toml::from_str::<Theme>(&content) {
                                    themes.insert(theme.name.clone(), theme);
                                }
                            }
                        }
                    }
                }
            }
        }
        themes
    }

    fn load_current() -> Theme {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str::<ThemeConfig>(&content) {
                        let themes = Self::load_themes();
                        if let Some(theme) = themes.get(&cfg.current_theme) {
                            return theme.clone();
                        }
                    }
                }
            }
        }
        Theme::default_theme()
    }

    fn save_current(theme_name: &str) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let cfg = ThemeConfig {
                current_theme: theme_name.to_string(),
            };
            if let Ok(content) = toml::to_string_pretty(&cfg) {
                let _ = fs::write(&path, content);
            }
        }
    }

    pub fn global() -> &'static ThemeManager {
        static INSTANCE: std::sync::LazyLock<ThemeManager> =
            std::sync::LazyLock::new(|| ThemeManager {
                current_theme: RwLock::new(ThemeManager::load_current()),
                available_themes: RwLock::new(ThemeManager::load_themes()),
            });
        &INSTANCE
    }

    /// Replace the global ThemeManager's state with a custom instance.
    /// Useful for testing or startup customization.
    pub fn set_global(tm: ThemeManager) {
        let g = Self::global();
        let current = tm.current_theme.into_inner().unwrap();
        let available = tm.available_themes.into_inner().unwrap();
        *g.current_theme.write().unwrap() = current;
        *g.available_themes.write().unwrap() = available;
    }

    /// Temporarily inject a custom ThemeManager, run `f`, then restore the
    /// original. Returns the result of `f`.
    pub fn with_test_theme<R>(tm: ThemeManager, f: impl FnOnce() -> R) -> R {
        let g = Self::global();
        let mut cur = g.current_theme.write().unwrap();
        let mut av = g.available_themes.write().unwrap();
        let old_cur = std::mem::replace(&mut *cur, tm.current_theme.into_inner().unwrap());
        let old_av = std::mem::replace(&mut *av, tm.available_themes.into_inner().unwrap());
        drop((cur, av));
        let result = f();
        let mut cur = g.current_theme.write().unwrap();
        let mut av = g.available_themes.write().unwrap();
        *cur = old_cur;
        *av = old_av;
        result
    }

    pub fn current() -> std::sync::RwLockReadGuard<'static, Theme> {
        Self::global()
            .current_theme
            .read()
            .unwrap_or_else(|e| e.into_inner())
    }

    pub fn set_theme(name: &str) -> bool {
        let themes = Self::global()
            .available_themes
            .read()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(theme) = themes.get(name) {
            let mut current = Self::global()
                .current_theme
                .write()
                .unwrap_or_else(|e| e.into_inner());
            *current = theme.clone();
            Self::save_current(name);
            THEME_CHANGED.store(true, Ordering::Release);
            #[cfg(not(target_os = "android"))]
            {
                colored::control::set_override(crate::config::Config::global_get_color_enabled());
            }
            true
        } else {
            false
        }
    }

    pub fn list_themes() -> Vec<String> {
        let themes = Self::global()
            .available_themes
            .read()
            .unwrap_or_else(|e| e.into_inner());
        let mut names: Vec<String> = themes.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn ensure_default_themes() {
        let themes_dir = match Self::themes_dir() {
            Some(d) => d,
            None => return,
        };

        if !themes_dir.exists() {
            let _ = fs::create_dir_all(&themes_dir);
        }

        // Only write files that don't already exist on disk.
        // Previously this checked list_themes() which always returned
        // ["default", "light"] from the hardcoded map — causing writes every startup.
        let default_path = themes_dir.join("default.ntc_theme");
        if !default_path.exists() {
            let _ = fs::write(&default_path, Self::get_default_theme_content());
        }

        let light_path = themes_dir.join("light.ntc_theme");
        if !light_path.exists() {
            let _ = fs::write(&light_path, Self::get_light_theme_content());
        }

        let example_path = themes_dir.join("example.ntc_theme");
        if !example_path.exists() {
            let _ = fs::write(&example_path, Self::get_example_theme_template());
        }

        Self::reload_themes();
    }

    fn get_default_theme_content() -> String {
        r#"# ntc Default Theme
# File extension: .ntc.theme
name = "default"
author = "NuengCoder"
description = "Default ntc theme (VS Code Dark+ inspired)"

[syntax]
keyword = "bright_cyan"
string = "bright_green"
comment = "bright_black"
number = "bright_green"
type = "cyan"
builtin = "bright_red"
function = "bright_yellow"
operator = "yellow"
punctuation = "yellow"
attribute = "bright_cyan"
macro_token = "bright_magenta"
regex = "bright_red"
tag = "bright_cyan"
constant = "cyan"
normal = "bright_white"

[shell]
prompt_bracket = "bright_cyan"
prompt_path = "cyan"
prompt_watcher = "yellow"
prompt_arrow = "green"
success = "green"
error = "red"
warning = "yellow"
info = "cyan"
tree_branch = { r = 30, g = 220, b = 220 }
tree_dir = "bright_magenta"
tree_file = "yellow"
tree_ignored = "bright_black"
tree_size = "yellow"
command_output = "yellow"
separator = "cyan"
help_header = "cyan"
help_section = "cyan"
help_example = "green"
teleport_name = "bright_blue"
alias_name = "blue"
alias_command = "bright_black"
git_branch = "bright_cyan"
git_added = "green"
git_modified = "yellow"
git_deleted = "red"
git_untracked = "bright_black"
git_conflict = "bright_red"

[editor]
editor_bg = { r = 21, g = 19, b = 26 }
gutter_bg = { r = 24, g = 22, b = 30 }
status_bg = { r = 130, g = 100, b = 210 }
hint_bg = { r = 28, g = 26, b = 34 }
sidebar_bg = { r = 21, g = 19, b = 26 }
sidebar_selected_bg = { r = 50, g = 45, b = 65 }
run_panel_bg = { r = 26, g = 24, b = 32 }
gutter_text = { r = 100, g = 95, b = 118 }
status_text = "white"
status_modified = "yellow"
hint_text = { r = 140, g = 222, b = 255 }
cursor_bg = "white"
cursor_text = "black"
extra_cursor_bg = { r = 255, g = 140, b = 154 }
selection_bg = { r = 58, g = 50, b = 85 }
search_match_bg = { r = 48, g = 43, b = 63 }
search_current_bg = { r = 130, g = 100, b = 210 }
border = { r = 50, g = 46, b = 62 }
scrollbar = { r = 38, g = 35, b = 48 }
scrollbar_thumb = { r = 130, g = 100, b = 210 }
sidebar_dir = { r = 134, g = 215, b = 235 }
sidebar_file = { r = 200, g = 200, b = 215 }
sidebar_current = { r = 255, g = 208, b = 145 }
sidebar_selected = "white"
run_header_fg = { r = 95, g = 145, b = 205 }
run_header_unfocused_fg = { r = 45, g = 85, b = 120 }
run_output_fg = { r = 185, g = 198, b = 212 }
"#
        .to_string()
    }

    fn get_light_theme_content() -> String {
        r#"# ntc Light Theme
name = "light"
author = "NuengCoder"
description = "Light theme for bright environments"

[syntax]
keyword = { r = 120, g = 60, b = 180 }
string = { r = 30, g = 140, b = 50 }
comment = { r = 120, g = 120, b = 120 }
number = { r = 200, g = 120, b = 40 }
type = { r = 40, g = 140, b = 160 }
builtin = { r = 180, g = 60, b = 80 }
function = { r = 50, g = 140, b = 180 }
operator = { r = 80, g = 80, b = 80 }
punctuation = { r = 100, g = 100, b = 100 }
attribute = { r = 130, g = 80, b = 200 }
macro_token = { r = 150, g = 100, b = 210 }
regex = { r = 200, g = 120, b = 60 }
tag = { r = 40, g = 160, b = 140 }
constant = { r = 180, g = 130, b = 60 }
normal = { r = 30, g = 30, b = 30 }

[shell]
prompt_bracket = "bright_blue"
prompt_path = "blue"
prompt_watcher = "yellow"
prompt_arrow = "green"
success = "green"
error = "red"
warning = "yellow"
info = "blue"
tree_branch = "green"
tree_dir = "blue"
tree_file = "black"
tree_ignored = "bright_black"
tree_size = "yellow"
command_output = "black"
separator = "bright_blue"
help_header = "bright_blue"
help_section = "blue"
help_example = "green"
teleport_name = "blue"
alias_name = "blue"
alias_command = "bright_black"
git_branch = "blue"
git_added = "green"
git_modified = "yellow"
git_deleted = "red"
git_untracked = "bright_black"
git_conflict = "bright_red"

[editor]
editor_bg = { r = 245, g = 245, b = 245 }
gutter_bg = { r = 240, g = 240, b = 240 }
status_bg = { r = 100, g = 80, b = 200 }
hint_bg = { r = 235, g = 235, b = 235 }
sidebar_bg = { r = 240, g = 240, b = 240 }
sidebar_selected_bg = { r = 210, g = 210, b = 220 }
run_panel_bg = { r = 235, g = 235, b = 235 }
gutter_text = { r = 120, g = 120, b = 120 }
status_text = "white"
status_modified = "yellow"
hint_text = { r = 80, g = 140, b = 180 }
cursor_bg = "black"
cursor_text = "white"
extra_cursor_bg = { r = 220, g = 100, b = 120 }
selection_bg = { r = 200, g = 190, b = 220 }
search_match_bg = { r = 210, g = 210, b = 220 }
search_current_bg = { r = 160, g = 120, b = 220 }
border = { r = 180, g = 180, b = 200 }
scrollbar = { r = 180, g = 180, b = 200 }
scrollbar_thumb = { r = 140, g = 100, b = 200 }
sidebar_dir = { r = 60, g = 140, b = 160 }
sidebar_file = { r = 60, g = 60, b = 60 }
sidebar_current = { r = 200, g = 120, b = 40 }
sidebar_selected = "black"
run_header_fg = { r = 60, g = 100, b = 140 }
run_header_unfocused_fg = { r = 35, g = 65, b = 95 }
run_output_fg = { r = 40, g = 50, b = 60 }
"#
        .to_string()
    }

    pub fn get_example_theme_template() -> String {
        r#"# ntc Theme Template
name = "mytheme"
author = "Your Name"
description = "My custom theme"

[syntax]
keyword = "magenta"
string = "green"
comment = "bright_black"
number = "yellow"
type = "cyan"
builtin = "red"
function = "bright_blue"
operator = "white"
punctuation = "bright_black"
attribute = "bright_magenta"
macro_token = "bright_magenta"
regex = "bright_yellow"
tag = "bright_cyan"
constant = "yellow"
normal = "white"

[shell]
prompt_bracket = "cyan"
prompt_path = "blue"
prompt_watcher = "yellow"
prompt_arrow = "green"
success = "green"
error = "red"
warning = "yellow"
info = "cyan"
tree_branch = "green"
tree_dir = "blue"
tree_file = "white"
tree_ignored = "bright_black"
tree_size = "yellow"
command_output = "white"
separator = "cyan"
help_header = "cyan"
help_section = "cyan"
help_example = "green"
teleport_name = "bright_blue"
alias_name = "blue"
alias_command = "bright_black"
git_branch = "bright_cyan"
git_added = "green"
git_modified = "yellow"
git_deleted = "red"
git_untracked = "bright_black"
git_conflict = "bright_red"

[editor]
editor_bg = { r = 13, g = 13, b = 21 }
gutter_bg = { r = 13, g = 13, b = 21 }
status_bg = { r = 162, g = 119, b = 255 }
hint_bg = { r = 21, g = 21, b = 29 }
sidebar_bg = { r = 13, g = 13, b = 21 }
sidebar_selected_bg = { r = 44, g = 44, b = 58 }
run_panel_bg = { r = 22, g = 22, b = 30 }
gutter_text = { r = 108, g = 108, b = 128 }
status_text = "white"
status_modified = "yellow"
hint_text = { r = 130, g = 226, b = 255 }
cursor_bg = "white"
cursor_text = "black"
extra_cursor_bg = { r = 255, g = 140, b = 154 }
selection_bg = { r = 54, g = 51, b = 84 }
search_match_bg = { r = 44, g = 44, b = 58 }
search_current_bg = { r = 162, g = 119, b = 255 }
border = { r = 61, g = 61, b = 77 }
scrollbar = { r = 61, g = 61, b = 77 }
scrollbar_thumb = { r = 162, g = 119, b = 255 }
sidebar_dir = { r = 120, g = 220, b = 232 }
sidebar_file = { r = 200, g = 200, b = 208 }
sidebar_current = { r = 255, g = 202, b = 133 }
sidebar_selected = "white"
run_header_fg = { r = 70, g = 130, b = 180 }
run_header_unfocused_fg = { r = 45, g = 85, b = 120 }
run_output_fg = { r = 180, g = 200, b = 210 }
"#
        .to_string()
    }

    pub fn create_theme(name: &str) -> Result<(), String> {
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(
                "Theme name can only contain letters, numbers, underscores, and hyphens"
                    .to_string(),
            );
        }
        let themes_dir = Self::themes_dir().ok_or("Failed to get themes directory")?;
        fs::create_dir_all(&themes_dir).map_err(|e| e.to_string())?;
        let theme_path = themes_dir.join(format!("{}.ntc_theme", name));
        if theme_path.exists() {
            return Err(format!("Theme '{}' already exists", name));
        }
        let mut content = Self::get_example_theme_template();
        content = content.replace("name = \"mytheme\"", &format!("name = \"{}\"", name));
        fs::write(&theme_path, content).map_err(|e| e.to_string())?;
        let _ = crate::editor::edit_file(&theme_path);
        Self::reload_themes();
        Ok(())
    }

    pub fn rename_theme(old_name: &str, new_name: &str) -> Result<(), String> {
        if !Self::theme_exists(old_name) {
            return Err(format!("Theme '{}' not found", old_name));
        }
        if Self::theme_exists(new_name) {
            return Err(format!("Theme '{}' already exists", new_name));
        }
        let themes_dir = Self::themes_dir().ok_or("Failed to get themes directory")?;
        let old_path = themes_dir.join(format!("{}.ntc_theme", old_name));
        let new_path = themes_dir.join(format!("{}.ntc_theme", new_name));
        fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;
        let current = Self::current();
        if current.name == old_name {
            drop(current);
            Self::set_theme(new_name);
        }
        Self::reload_themes();
        Ok(())
    }

    pub fn remove_theme(name: &str) -> Result<(), String> {
        if name == "default" || name == "light" {
            return Err("Cannot remove built-in themes (default, light)".to_string());
        }
        if !Self::theme_exists(name) {
            return Err(format!("Theme '{}' not found", name));
        }
        let themes_dir = Self::themes_dir().ok_or("Failed to get themes directory")?;
        let theme_path = themes_dir.join(format!("{}.ntc_theme", name));
        fs::remove_file(theme_path).map_err(|e| e.to_string())?;
        let current = Self::current();
        if current.name == name {
            drop(current);
            Self::set_theme("default");
        }
        Self::reload_themes();
        Ok(())
    }

    pub fn get_theme_info(name: &str) -> Option<Theme> {
        let themes = Self::global().available_themes.read().ok()?;
        themes.get(name).cloned()
    }

    pub fn theme_exists(name: &str) -> bool {
        let themes = Self::global().available_themes.read().ok();
        themes.is_some_and(|t| t.contains_key(name))
    }

    pub fn export_theme(name: &str, output_path: &std::path::Path) -> Result<(), String> {
        let theme = Self::get_theme_info(name).ok_or(format!("Theme '{}' not found", name))?;
        let content = toml::to_string_pretty(&theme).map_err(|e| e.to_string())?;
        fs::write(output_path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn import_theme(path: &std::path::Path) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let theme: Theme = toml::from_str(&content).map_err(|e| e.to_string())?;
        let themes_dir = Self::themes_dir().ok_or("Failed to get themes directory")?;
        fs::create_dir_all(&themes_dir).map_err(|e| e.to_string())?;
        let dest = themes_dir.join(format!("{}.ntc_theme", theme.name));
        fs::write(&dest, content).map_err(|e| e.to_string())?;
        Self::reload_themes();
        Ok(())
    }

    pub fn reload_themes() {
        let mut themes = Self::global()
            .available_themes
            .write()
            .unwrap_or_else(|e| e.into_inner());
        *themes = Self::load_themes();
    }

    pub fn install_theme(path: &std::path::Path) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let theme: Theme = toml::from_str(&content).map_err(|e| e.to_string())?;
        let themes_dir = Self::themes_dir().ok_or("Failed to get themes directory")?;
        fs::create_dir_all(&themes_dir).map_err(|e| e.to_string())?;
        let dest = themes_dir.join(format!("{}.ntc_theme", theme.name));
        fs::write(&dest, content).map_err(|e| e.to_string())?;
        Self::reload_themes();
        Ok(())
    }

    pub fn take_theme_changed() -> bool {
        THEME_CHANGED.swap(false, Ordering::Acquire)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ThemeConfig {
    current_theme: String,
}

// ============================================================================
// Convenience macro for shell coloring
// ============================================================================

/// Apply shell theme color to a string using the colored crate.
/// Example: `ntc_shell_color!("hello", shell.success)`
#[macro_export]
macro_rules! ntc_shell_color {
    ($str:expr, $field:ident) => {{
        #[cfg(not(target_os = "android"))]
        {
            use colored::Colorize;
            let _theme = $crate::utils::theme::ThemeManager::current();
            $str.color(_theme.shell.$field.to_colored())
        }
        #[cfg(target_os = "android")]
        {
            let _theme = $crate::utils::theme::ThemeManager::current();
            format!("{}{}\x1b[0m", _theme.shell.$field.to_ansi_fg(), $str)
        }
    }};
}

// Legacy themed! macro, kept for compatibility
#[macro_export]
macro_rules! themed {
    ($str:expr, $color:expr) => {{
        let _theme = $crate::utils::theme::ThemeManager::current();
        let _color = $color(&_theme.shell);
        #[cfg(not(target_os = "android"))]
        {
            use colored::Colorize;
            $str.color(_color.to_colored())
        }
        #[cfg(target_os = "android")]
        {
            format!("{}{}\x1b[0m", _color.to_ansi_fg(), $str)
        }
    }};
}

// ============================================================================
// Tests
// ============================================================================
