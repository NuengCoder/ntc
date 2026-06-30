//! Multi-theme support for ntc shell and editor.
//! Defines the color types (`ThemeColor`), theme structs (`SyntaxTheme`,
//! `ShellTheme`, `EditorTheme`, `Theme`), and re-exports `ThemeManager`
//! for runtime theme CRUD operations.

use serde::de::{self, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::AtomicBool;

static THEME_CHANGED: AtomicBool = AtomicBool::new(false);

// ============================================================================
// Color types (bridges colored::Color and crossterm::style::Color)
// ============================================================================

/// Platform-agnostic color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb { r: u8, g: u8, b: u8 },
    Ansi { code: u8 },
}

const COLOR_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright_black", "bright_red", "bright_green", "bright_yellow",
    "bright_blue", "bright_magenta", "bright_cyan", "bright_white",
];

impl Serialize for ThemeColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ThemeColor::Black => serializer.serialize_str("black"),
            ThemeColor::Red => serializer.serialize_str("red"),
            ThemeColor::Green => serializer.serialize_str("green"),
            ThemeColor::Yellow => serializer.serialize_str("yellow"),
            ThemeColor::Blue => serializer.serialize_str("blue"),
            ThemeColor::Magenta => serializer.serialize_str("magenta"),
            ThemeColor::Cyan => serializer.serialize_str("cyan"),
            ThemeColor::White => serializer.serialize_str("white"),
            ThemeColor::BrightBlack => serializer.serialize_str("bright_black"),
            ThemeColor::BrightRed => serializer.serialize_str("bright_red"),
            ThemeColor::BrightGreen => serializer.serialize_str("bright_green"),
            ThemeColor::BrightYellow => serializer.serialize_str("bright_yellow"),
            ThemeColor::BrightBlue => serializer.serialize_str("bright_blue"),
            ThemeColor::BrightMagenta => serializer.serialize_str("bright_magenta"),
            ThemeColor::BrightCyan => serializer.serialize_str("bright_cyan"),
            ThemeColor::BrightWhite => serializer.serialize_str("bright_white"),
            ThemeColor::Rgb { r, g, b } => {
                let mut m = serializer.serialize_map(Some(3))?;
                m.serialize_entry("r", r)?;
                m.serialize_entry("g", g)?;
                m.serialize_entry("b", b)?;
                m.end()
            }
            ThemeColor::Ansi { code } => {
                let mut m = serializer.serialize_map(Some(1))?;
                m.serialize_entry("code", code)?;
                m.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ThemeColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ThemeColorVisitor;

        impl<'de> Visitor<'de> for ThemeColorVisitor {
            type Value = ThemeColor;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a color name string or RGB/ANSI map")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<ThemeColor, E> {
                match v {
                    "black" => Ok(ThemeColor::Black),
                    "red" => Ok(ThemeColor::Red),
                    "green" => Ok(ThemeColor::Green),
                    "yellow" => Ok(ThemeColor::Yellow),
                    "blue" => Ok(ThemeColor::Blue),
                    "magenta" => Ok(ThemeColor::Magenta),
                    "cyan" => Ok(ThemeColor::Cyan),
                    "white" => Ok(ThemeColor::White),
                    "bright_black" => Ok(ThemeColor::BrightBlack),
                    "bright_red" => Ok(ThemeColor::BrightRed),
                    "bright_green" => Ok(ThemeColor::BrightGreen),
                    "bright_yellow" => Ok(ThemeColor::BrightYellow),
                    "bright_blue" => Ok(ThemeColor::BrightBlue),
                    "bright_magenta" => Ok(ThemeColor::BrightMagenta),
                    "bright_cyan" => Ok(ThemeColor::BrightCyan),
                    "bright_white" => Ok(ThemeColor::BrightWhite),
                    _ => Err(de::Error::unknown_variant(v, COLOR_NAMES)),
                }
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<ThemeColor, M::Error> {
                let mut r = None::<u8>;
                let mut g = None::<u8>;
                let mut b = None::<u8>;
                let mut code = None::<u8>;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "r" => r = Some(map.next_value()?),
                        "g" => g = Some(map.next_value()?),
                        "b" => b = Some(map.next_value()?),
                        "code" => code = Some(map.next_value()?),
                        _ => return Err(de::Error::unknown_field(&key, &["r", "g", "b", "code"])),
                    }
                }
                if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                    Ok(ThemeColor::Rgb { r, g, b })
                } else if let Some(code) = code {
                    Ok(ThemeColor::Ansi { code })
                } else {
                    Err(de::Error::custom("expected r, g, b for RGB or code for ANSI"))
                }
            }
        }

        deserializer.deserialize_any(ThemeColorVisitor)
    }
}

impl ThemeColor {
    /// Convert to `colored::Color` for shell output (non-Android)
    #[cfg(not(target_os = "android"))]
    pub fn to_colored(&self) -> colored::Color {
        match self {
            ThemeColor::Black => colored::Color::Black,
            ThemeColor::Red => colored::Color::Red,
            ThemeColor::Green => colored::Color::Green,
            ThemeColor::Yellow => colored::Color::Yellow,
            ThemeColor::Blue => colored::Color::Blue,
            ThemeColor::Magenta => colored::Color::Magenta,
            ThemeColor::Cyan => colored::Color::Cyan,
            ThemeColor::White => colored::Color::White,
            ThemeColor::BrightBlack => colored::Color::BrightBlack,
            ThemeColor::BrightRed => colored::Color::BrightRed,
            ThemeColor::BrightGreen => colored::Color::BrightGreen,
            ThemeColor::BrightYellow => colored::Color::BrightYellow,
            ThemeColor::BrightBlue => colored::Color::BrightBlue,
            ThemeColor::BrightMagenta => colored::Color::BrightMagenta,
            ThemeColor::BrightCyan => colored::Color::BrightCyan,
            ThemeColor::BrightWhite => colored::Color::BrightWhite,
            ThemeColor::Rgb { r, g, b } => colored::Color::TrueColor { r: *r, g: *g, b: *b },
            // colored 2.x has no ANSI 256-colour variant — fall back to White.
            // The editor (crossterm) path handles ANSI codes correctly via Color::AnsiValue.
            ThemeColor::Ansi { code: _ } => colored::Color::White,
        }
    }

    /// Convert to ANSI foreground color escape code (Android/Termux)
    #[cfg(target_os = "android")]
    pub fn to_ansi_fg(&self) -> String {
        match self {
            ThemeColor::Black => "\x1b[30m".to_string(),
            ThemeColor::Red => "\x1b[31m".to_string(),
            ThemeColor::Green => "\x1b[32m".to_string(),
            ThemeColor::Yellow => "\x1b[33m".to_string(),
            ThemeColor::Blue => "\x1b[34m".to_string(),
            ThemeColor::Magenta => "\x1b[35m".to_string(),
            ThemeColor::Cyan => "\x1b[36m".to_string(),
            ThemeColor::White => "\x1b[37m".to_string(),
            ThemeColor::BrightBlack => "\x1b[90m".to_string(),
            ThemeColor::BrightRed => "\x1b[91m".to_string(),
            ThemeColor::BrightGreen => "\x1b[92m".to_string(),
            ThemeColor::BrightYellow => "\x1b[93m".to_string(),
            ThemeColor::BrightBlue => "\x1b[94m".to_string(),
            ThemeColor::BrightMagenta => "\x1b[95m".to_string(),
            ThemeColor::BrightCyan => "\x1b[96m".to_string(),
            ThemeColor::BrightWhite => "\x1b[97m".to_string(),
            ThemeColor::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
            ThemeColor::Ansi { code } => format!("\x1b[38;5;{}m", code),
        }
    }

    /// Convert to `crossterm::style::Color` for editor
    pub fn to_crossterm(&self) -> crossterm::style::Color {
        match self {
            ThemeColor::Black => crossterm::style::Color::Black,
            ThemeColor::Red => crossterm::style::Color::Red,
            ThemeColor::Green => crossterm::style::Color::Green,
            ThemeColor::Yellow => crossterm::style::Color::Yellow,
            ThemeColor::Blue => crossterm::style::Color::Blue,
            ThemeColor::Magenta => crossterm::style::Color::Magenta,
            ThemeColor::Cyan => crossterm::style::Color::Cyan,
            ThemeColor::White => crossterm::style::Color::White,
            ThemeColor::BrightBlack => crossterm::style::Color::DarkGrey,
            ThemeColor::BrightRed => crossterm::style::Color::DarkRed,
            ThemeColor::BrightGreen => crossterm::style::Color::DarkGreen,
            ThemeColor::BrightYellow => crossterm::style::Color::DarkYellow,
            ThemeColor::BrightBlue => crossterm::style::Color::DarkBlue,
            ThemeColor::BrightMagenta => crossterm::style::Color::DarkMagenta,
            ThemeColor::BrightCyan => crossterm::style::Color::DarkCyan,
            ThemeColor::BrightWhite => crossterm::style::Color::Grey,
            ThemeColor::Rgb { r, g, b } => crossterm::style::Color::Rgb { r: *r, g: *g, b: *b },
            ThemeColor::Ansi { code } => crossterm::style::Color::AnsiValue(*code),
        }
    }
}

// ============================================================================
// Syntax highlighting theme (for editor)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SyntaxTheme {
    pub keyword: ThemeColor,
    pub string: ThemeColor,
    pub comment: ThemeColor,
    pub number: ThemeColor,
    pub r#type: ThemeColor,
    pub builtin: ThemeColor,
    pub function: ThemeColor,
    pub operator: ThemeColor,
    pub punctuation: ThemeColor,
    pub attribute: ThemeColor,
    pub macro_token: ThemeColor,
    pub regex: ThemeColor,
    pub tag: ThemeColor,
    pub constant: ThemeColor,
    pub normal: ThemeColor,
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        Self {
            keyword: ThemeColor::Rgb { r: 200, g: 160, b: 255 },
            string: ThemeColor::Rgb { r: 103, g: 228, b: 128 },
            comment: ThemeColor::Rgb { r: 108, g: 108, b: 128 },
            number: ThemeColor::Rgb { r: 255, g: 202, b: 133 },
            r#type: ThemeColor::Rgb { r: 120, g: 220, b: 232 },
            builtin: ThemeColor::Rgb { r: 255, g: 140, b: 154 },
            function: ThemeColor::Rgb { r: 130, g: 226, b: 255 },
            operator: ThemeColor::Rgb { r: 200, g: 200, b: 208 },
            punctuation: ThemeColor::Rgb { r: 132, g: 132, b: 154 },
            attribute: ThemeColor::Rgb { r: 162, g: 119, b: 255 },
            macro_token: ThemeColor::Rgb { r: 176, g: 131, b: 240 },
            regex: ThemeColor::Rgb { r: 252, g: 169, b: 110 },
            tag: ThemeColor::Rgb { r: 80, g: 228, b: 200 },
            constant: ThemeColor::Rgb { r: 229, g: 192, b: 123 },
            normal: ThemeColor::Rgb { r: 200, g: 200, b: 208 },
        }
    }
}

// ============================================================================
// Shell theme (colored::Color equivalents)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShellTheme {
    pub prompt_bracket: ThemeColor,
    pub prompt_path: ThemeColor,
    pub prompt_watcher: ThemeColor,
    pub prompt_arrow: ThemeColor,
    pub success: ThemeColor,
    pub error: ThemeColor,
    pub warning: ThemeColor,
    pub info: ThemeColor,
    pub tree_branch: ThemeColor,
    pub tree_dir: ThemeColor,
    pub tree_file: ThemeColor,
    pub tree_ignored: ThemeColor,
    pub tree_size: ThemeColor,
    pub command_output: ThemeColor,
    pub separator: ThemeColor,
    pub help_header: ThemeColor,
    pub help_section: ThemeColor,
    pub help_example: ThemeColor,
    pub teleport_name: ThemeColor,
    pub alias_name: ThemeColor,
    pub alias_command: ThemeColor,
}

impl Default for ShellTheme {
    fn default() -> Self {
        Self {
            prompt_bracket: ThemeColor::Cyan,
            prompt_path: ThemeColor::Blue,
            prompt_watcher: ThemeColor::Yellow,
            prompt_arrow: ThemeColor::Green,
            success: ThemeColor::Green,
            error: ThemeColor::Red,
            warning: ThemeColor::Yellow,
            info: ThemeColor::Cyan,
            tree_branch: ThemeColor::Green,
            tree_dir: ThemeColor::Blue,
            tree_file: ThemeColor::White,
            tree_ignored: ThemeColor::BrightBlack,
            tree_size: ThemeColor::Yellow,
            command_output: ThemeColor::White,
            separator: ThemeColor::Cyan,
            help_header: ThemeColor::Cyan,
            help_section: ThemeColor::Cyan,
            help_example: ThemeColor::Green,
            teleport_name: ThemeColor::BrightBlue,
            alias_name: ThemeColor::Blue,
            alias_command: ThemeColor::BrightBlack,
        }
    }
}

// ============================================================================
// Editor UI theme (backgrounds, borders, status bar, etc.)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EditorTheme {
    pub editor_bg: ThemeColor,
    pub gutter_bg: ThemeColor,
    pub status_bg: ThemeColor,
    pub hint_bg: ThemeColor,
    pub sidebar_bg: ThemeColor,
    pub sidebar_selected_bg: ThemeColor,
    pub run_panel_bg: ThemeColor,
    pub gutter_text: ThemeColor,
    pub status_text: ThemeColor,
    pub status_modified: ThemeColor,
    pub hint_text: ThemeColor,
    pub cursor_bg: ThemeColor,
    pub cursor_text: ThemeColor,
    pub extra_cursor_bg: ThemeColor,
    pub selection_bg: ThemeColor,
    pub search_match_bg: ThemeColor,
    pub search_current_bg: ThemeColor,
    pub border: ThemeColor,
    pub scrollbar: ThemeColor,
    pub scrollbar_thumb: ThemeColor,
    pub sidebar_dir: ThemeColor,
    pub sidebar_file: ThemeColor,
    pub sidebar_current: ThemeColor,
    pub sidebar_selected: ThemeColor,
    pub run_header_fg: ThemeColor,
    pub run_header_unfocused_fg: ThemeColor,
    pub run_output_fg: ThemeColor,
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self {
            editor_bg: ThemeColor::Rgb { r: 13, g: 13, b: 21 },
            gutter_bg: ThemeColor::Rgb { r: 13, g: 13, b: 21 },
            status_bg: ThemeColor::Rgb { r: 162, g: 119, b: 255 },
            hint_bg: ThemeColor::Rgb { r: 21, g: 21, b: 29 },
            sidebar_bg: ThemeColor::Rgb { r: 13, g: 13, b: 21 },
            sidebar_selected_bg: ThemeColor::Rgb { r: 44, g: 44, b: 58 },
            run_panel_bg: ThemeColor::Rgb { r: 22, g: 22, b: 30 },
            gutter_text: ThemeColor::Rgb { r: 108, g: 108, b: 128 },
            status_text: ThemeColor::White,
            status_modified: ThemeColor::Yellow,
            hint_text: ThemeColor::Rgb { r: 130, g: 226, b: 255 },
            cursor_bg: ThemeColor::White,
            cursor_text: ThemeColor::Black,
            extra_cursor_bg: ThemeColor::Rgb { r: 255, g: 140, b: 154 },
            selection_bg: ThemeColor::Rgb { r: 54, g: 51, b: 84 },
            search_match_bg: ThemeColor::Rgb { r: 44, g: 44, b: 58 },
            search_current_bg: ThemeColor::Rgb { r: 162, g: 119, b: 255 },
            border: ThemeColor::Rgb { r: 61, g: 61, b: 77 },
            scrollbar: ThemeColor::Rgb { r: 61, g: 61, b: 77 },
            scrollbar_thumb: ThemeColor::Rgb { r: 162, g: 119, b: 255 },
            sidebar_dir: ThemeColor::Rgb { r: 120, g: 220, b: 232 },
            sidebar_file: ThemeColor::Rgb { r: 200, g: 200, b: 208 },
            sidebar_current: ThemeColor::Rgb { r: 255, g: 202, b: 133 },
            sidebar_selected: ThemeColor::White,
            run_header_fg: ThemeColor::Rgb { r: 70, g: 130, b: 180 },
            run_header_unfocused_fg: ThemeColor::Rgb { r: 45, g: 85, b: 120 },
            run_output_fg: ThemeColor::Rgb { r: 180, g: 200, b: 210 },
        }
    }
}

// ============================================================================
// Complete Theme (all-in-one)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub syntax: SyntaxTheme,
    #[serde(default)]
    pub shell: ShellTheme,
    #[serde(default)]
    pub editor: EditorTheme,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            name: "default".to_string(),
            author: Some("NuengCoder".to_string()),
            description: Some("Default ntc theme".to_string()),
            syntax: SyntaxTheme::default(),
            shell: ShellTheme::default(),
            editor: EditorTheme::default(),
        }
    }

    #[deprecated(since = "2.3.0", note = "use default_theme() instead")]
    pub fn dark_theme() -> Self {
        let mut t = Self::default_theme();
        t.name = "dark".into();
        t
    }

    pub fn light_theme() -> Self {
        Self {
            name: "light".to_string(),
            author: Some("NuengCoder".to_string()),
            description: Some("Light theme for bright environments".to_string()),
            syntax: SyntaxTheme {
                keyword: ThemeColor::Rgb { r: 120, g: 60, b: 180 },
                string: ThemeColor::Rgb { r: 30, g: 140, b: 50 },
                comment: ThemeColor::Rgb { r: 120, g: 120, b: 120 },
                number: ThemeColor::Rgb { r: 200, g: 120, b: 40 },
                r#type: ThemeColor::Rgb { r: 40, g: 140, b: 160 },
                builtin: ThemeColor::Rgb { r: 180, g: 60, b: 80 },
                function: ThemeColor::Rgb { r: 50, g: 140, b: 180 },
                operator: ThemeColor::Rgb { r: 80, g: 80, b: 80 },
                punctuation: ThemeColor::Rgb { r: 100, g: 100, b: 100 },
                attribute: ThemeColor::Rgb { r: 130, g: 80, b: 200 },
                macro_token: ThemeColor::Rgb { r: 150, g: 100, b: 210 },
                regex: ThemeColor::Rgb { r: 200, g: 120, b: 60 },
                tag: ThemeColor::Rgb { r: 40, g: 160, b: 140 },
                constant: ThemeColor::Rgb { r: 180, g: 130, b: 60 },
                normal: ThemeColor::Rgb { r: 30, g: 30, b: 30 },
            },
            shell: ShellTheme {
                prompt_bracket: ThemeColor::BrightBlue,
                prompt_path: ThemeColor::Blue,
                prompt_watcher: ThemeColor::Yellow,
                prompt_arrow: ThemeColor::Green,
                success: ThemeColor::Green,
                error: ThemeColor::Red,
                warning: ThemeColor::Yellow,
                info: ThemeColor::Blue,
                tree_branch: ThemeColor::Green,
                tree_dir: ThemeColor::Blue,
                tree_file: ThemeColor::Black,
                tree_ignored: ThemeColor::BrightBlack,
                tree_size: ThemeColor::Yellow,
                command_output: ThemeColor::Black,
                separator: ThemeColor::BrightBlue,
                help_header: ThemeColor::BrightBlue,
                help_section: ThemeColor::Blue,
                help_example: ThemeColor::Green,
                teleport_name: ThemeColor::Blue,
                alias_name: ThemeColor::Blue,
                alias_command: ThemeColor::BrightBlack,
            },
            editor: EditorTheme {
                editor_bg: ThemeColor::Rgb { r: 245, g: 245, b: 245 },
                gutter_bg: ThemeColor::Rgb { r: 240, g: 240, b: 240 },
                status_bg: ThemeColor::Rgb { r: 100, g: 80, b: 200 },
                hint_bg: ThemeColor::Rgb { r: 235, g: 235, b: 235 },
                sidebar_bg: ThemeColor::Rgb { r: 240, g: 240, b: 240 },
                sidebar_selected_bg: ThemeColor::Rgb { r: 210, g: 210, b: 220 },
                run_panel_bg: ThemeColor::Rgb { r: 235, g: 235, b: 235 },
                gutter_text: ThemeColor::Rgb { r: 120, g: 120, b: 120 },
                status_text: ThemeColor::White,
                status_modified: ThemeColor::Yellow,
                hint_text: ThemeColor::Rgb { r: 80, g: 140, b: 180 },
                cursor_bg: ThemeColor::Black,
                cursor_text: ThemeColor::White,
                extra_cursor_bg: ThemeColor::Rgb { r: 220, g: 100, b: 120 },
                selection_bg: ThemeColor::Rgb { r: 200, g: 190, b: 220 },
                search_match_bg: ThemeColor::Rgb { r: 210, g: 210, b: 220 },
                search_current_bg: ThemeColor::Rgb { r: 160, g: 120, b: 220 },
                border: ThemeColor::Rgb { r: 180, g: 180, b: 200 },
                scrollbar: ThemeColor::Rgb { r: 180, g: 180, b: 200 },
                scrollbar_thumb: ThemeColor::Rgb { r: 140, g: 100, b: 200 },
                sidebar_dir: ThemeColor::Rgb { r: 60, g: 140, b: 160 },
                sidebar_file: ThemeColor::Rgb { r: 60, g: 60, b: 60 },
                sidebar_current: ThemeColor::Rgb { r: 200, g: 120, b: 40 },
                sidebar_selected: ThemeColor::Black,
                run_header_fg: ThemeColor::Rgb { r: 60, g: 100, b: 140 },
                run_header_unfocused_fg: ThemeColor::Rgb { r: 35, g: 65, b: 95 },
                run_output_fg: ThemeColor::Rgb { r: 40, g: 50, b: 60 },
            },
        }
    }
    
}

// ============================================================================
// Shell color helpers
// ============================================================================

/// Apply a shell theme color to a string.
/// On non-Android: uses the `colored` crate.
/// On Android/Termux: uses raw ANSI escape codes.
#[cfg(not(target_os = "android"))]
pub fn paint_with_shell_color(s: &str, color: &ThemeColor) -> colored::ColoredString {
    use colored::Colorize;
    s.color(color.to_colored())
}

/// Android version: wraps string with ANSI foreground color + reset.
#[cfg(target_os = "android")]
pub fn paint_with_shell_color(s: &str, color: &ThemeColor) -> String {
    format!("{}{}\x1b[0m", color.to_ansi_fg(), s)
}

/// Convenience: get shell color for a specific accessor, then paint a string.
/// Example: `shell_paint("hello", |s| s.success)` to get theme.shell.success color.
#[cfg(not(target_os = "android"))]
pub fn shell_paint(s: &str, accessor: fn(&ShellTheme) -> &ThemeColor) -> colored::ColoredString {
    use colored::Colorize;
    let theme = ThemeManager::current();
    let color = accessor(&theme.shell);
    s.color(color.to_colored())
}

/// Android version: get shell color via accessor, then paint with ANSI escapes.
#[cfg(target_os = "android")]
pub fn shell_paint(s: &str, accessor: fn(&ShellTheme) -> &ThemeColor) -> String {
    let theme = ThemeManager::current();
    let color = accessor(&theme.shell);
    format!("{}{}\x1b[0m", color.to_ansi_fg(), s)
}

pub mod manager;
pub use manager::ThemeManager;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_switching() {
        assert!(ThemeManager::set_theme("default"));
        assert_eq!(ThemeManager::current().name, "default");
        assert!(ThemeManager::set_theme("light"));
        assert_eq!(ThemeManager::current().name, "light");
    }
    
    #[test]
    fn test_list_themes() {
        let themes = ThemeManager::list_themes();
        assert!(themes.contains(&"default".to_string()));
        assert!(themes.contains(&"light".to_string()));
    }
    
    #[test]
    fn test_color_conversion() {
        let color = ThemeColor::Rgb { r: 255, g: 0, b: 0 };
        #[cfg(not(target_os = "android"))]
        {
            let _colored = color.to_colored();
        }
        let _crossterm = color.to_crossterm();
    }

    #[test]
    fn test_theme_color_roundtrip() {
        // Full theme TOML matching the failing nightwork.ntc_theme format
        let toml_str = r#"
name = "test"
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
success = "green"
[editor]
editor_bg = { r = 30, g = 13, b = 60 }
gutter_bg = { r = 13, g = 13, b = 21 }
status_text = "white"
hint_text = { r = 130, g = 226, b = 255 }
cursor_bg = "white"
selection_bg = { r = 54, g = 51, b = 84 }
"#;
        let theme: Theme = toml::from_str(toml_str).unwrap();
        assert_eq!(theme.name, "test");
        assert_eq!(theme.syntax.keyword, ThemeColor::Magenta);
        assert_eq!(theme.syntax.comment, ThemeColor::BrightBlack);
        assert_eq!(theme.syntax.function, ThemeColor::BrightBlue);
        assert_eq!(theme.editor.editor_bg, ThemeColor::Rgb { r: 30, g: 13, b: 60 });
        assert_eq!(theme.editor.gutter_bg, ThemeColor::Rgb { r: 13, g: 13, b: 21 });
        assert_eq!(theme.editor.status_text, ThemeColor::White);
        assert_eq!(theme.editor.hint_text, ThemeColor::Rgb { r: 130, g: 226, b: 255 });
        assert_eq!(theme.editor.selection_bg, ThemeColor::Rgb { r: 54, g: 51, b: 84 });

        // Roundtrip: serialize then deserialize again
        let serialized = toml::to_string_pretty(&theme).unwrap();
        let theme2: Theme = toml::from_str(&serialized).unwrap();
        assert_eq!(theme.name, theme2.name);
        assert_eq!(theme.syntax.keyword, theme2.syntax.keyword);
        assert_eq!(theme.editor.editor_bg, theme2.editor.editor_bg);
    }
}
