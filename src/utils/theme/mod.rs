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
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright_black",
    "bright_red",
    "bright_green",
    "bright_yellow",
    "bright_blue",
    "bright_magenta",
    "bright_cyan",
    "bright_white",
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
                    Err(de::Error::custom(
                        "expected r, g, b for RGB or code for ANSI",
                    ))
                }
            }
        }

        deserializer.deserialize_any(ThemeColorVisitor)
    }
}

impl ThemeColor {
    /// Convert to `colored::Color` for shell output.
    /// Available on every OS so desktop and Termux share one code path.
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
            ThemeColor::Rgb { r, g, b } => colored::Color::TrueColor {
                r: *r,
                g: *g,
                b: *b,
            },
            // colored 2.x has no ANSI 256-colour variant — fall back to White.
            // The editor (crossterm) path handles ANSI codes correctly via Color::AnsiValue.
            ThemeColor::Ansi { code: _ } => colored::Color::White,
        }
    }

    /// Convert to ANSI foreground color escape code.
    /// Identical on every OS (Linux/Win/macOS/Termux) so `setc ON`
    /// shows the exact same hue everywhere. Termux renders these
    /// natively (same as `txtf` syntax highlighting does).
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

    /// Paint a string with this theme color.
    /// On non-Android: returns `colored::ColoredString`.
    /// On Android: returns `String` with ANSI escapes.
    /// Prefer `paint_ansi()` below for startup/tree output so every OS
    /// emits byte-identical escapes.
    #[cfg(not(target_os = "android"))]
    pub fn paint(&self, s: &str) -> colored::ColoredString {
        colored::Colorize::color(s, self.to_colored())
    }

    #[cfg(target_os = "android")]
    pub fn paint(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.to_ansi_fg(), s)
    }

    /// Raw-ANSI paint shared by every OS (welcome, Current Directory,
    /// show_tree). Returns plain text when colors are disabled so
    /// `setc OFF` / `NO_COLOR` / `TERM=dumb` stay plain everywhere.
    pub fn paint_ansi(&self, s: &str) -> String {
        if !ansi_enabled() {
            return s.to_string();
        }
        format!("{}{}\x1b[0m", self.to_ansi_fg(), s)
    }

    /// Raw-ANSI bold paint (matches `.color().bold()` on desktop).
    pub fn paint_ansi_bold(&self, s: &str) -> String {
        if !ansi_enabled() {
            return s.to_string();
        }
        format!("\x1b[1m{}{}\x1b[0m", self.to_ansi_fg(), s)
    }

    /// Default background for editable fields when a theme doesn't define
    /// `text_field_bg`. Keeps the field tied to the theme instead of the
    /// old hardcoded gray: dark backgrounds are lifted, light ones deepened.
    pub fn input_bg_from(editor_bg: &ThemeColor) -> ThemeColor {
        match editor_bg {
            ThemeColor::Rgb { r, g, b } => {
                let luma = 0.2126 * *r as f32 + 0.7152 * *g as f32 + 0.0722 * *b as f32;
                let delta: i16 = if luma < 128.0 { 36 } else { -24 };
                ThemeColor::Rgb {
                    r: (*r as i16 + delta).clamp(0, 255) as u8,
                    g: (*g as i16 + delta).clamp(0, 255) as u8,
                    b: (*b as i16 + delta).clamp(0, 255) as u8,
                }
            }
            ThemeColor::White | ThemeColor::BrightWhite => ThemeColor::Rgb {
                r: 220,
                g: 220,
                b: 220,
            },
            ThemeColor::Black | ThemeColor::BrightBlack => ThemeColor::Rgb {
                r: 56,
                g: 56,
                b: 56,
            },
            _ => ThemeColor::Rgb {
                r: 60,
                g: 60,
                b: 60,
            },
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
            ThemeColor::Rgb { r, g, b } => crossterm::style::Color::Rgb {
                r: *r,
                g: *g,
                b: *b,
            },
            ThemeColor::Ansi { code } => crossterm::style::Color::AnsiValue(*code),
        }
    }

    /// `#rrggbb` for a browser, using the standard xterm palette for named and
    /// indexed colours. Used by `serve mdview` so the page highlights with the
    /// same colours the terminal editor uses.
    pub fn to_hex(&self) -> String {
        let (r, g, b) = match self {
            ThemeColor::Rgb { r, g, b } => (*r, *g, *b),
            ThemeColor::Ansi { code } => ansi_rgb(*code),
            named => BASE_PALETTE[named_index(named).min(BASE_PALETTE.len() - 1)],
        };
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}

/// Index into [`BASE_PALETTE`] for the sixteen named colours.
fn named_index(color: &ThemeColor) -> usize {
    match color {
        ThemeColor::Black => 0,
        ThemeColor::Red => 1,
        ThemeColor::Green => 2,
        ThemeColor::Yellow => 3,
        ThemeColor::Blue => 4,
        ThemeColor::Magenta => 5,
        ThemeColor::Cyan => 6,
        ThemeColor::White => 7,
        ThemeColor::BrightBlack => 8,
        ThemeColor::BrightRed => 9,
        ThemeColor::BrightGreen => 10,
        ThemeColor::BrightYellow => 11,
        ThemeColor::BrightBlue => 12,
        ThemeColor::BrightMagenta => 13,
        ThemeColor::BrightCyan => 14,
        // BrightWhite, and anything unmapped, lands on the brightest entry.
        _ => 15,
    }
}

/// The classic xterm colours, which is what the terminal shows for these names.
const BASE_PALETTE: [(u8, u8, u8); 16] = [
    (0, 0, 0),
    (128, 0, 0),
    (0, 128, 0),
    (128, 128, 0),
    (0, 0, 128),
    (128, 0, 128),
    (0, 128, 128),
    (192, 192, 192),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

/// Expand an xterm-256 index to RGB: the 16 base colours, the 6×6×6 cube, then
/// the 24-step greyscale ramp.
fn ansi_rgb(code: u8) -> (u8, u8, u8) {
    match code {
        0..=15 => BASE_PALETTE[code as usize],
        16..=231 => {
            let n = code - 16;
            let step = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            (step(n / 36), step((n % 36) / 6), step(n % 6))
        }
        _ => {
            let grey = 8 + (code - 232) * 10;
            (grey, grey, grey)
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
    /// Secondary text, matching the shell's muted role.
    pub muted: ThemeColor,
    /// Header row in box/table output.
    pub table_header: ThemeColor,
    /// Frames and separators in box/table output.
    pub table_border: ThemeColor,
    /// Gutter line numbers for text views.
    pub line_number: ThemeColor,
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        Self {
            keyword: ThemeColor::BrightCyan,
            string: ThemeColor::BrightGreen,
            comment: ThemeColor::BrightBlack,
            number: ThemeColor::BrightGreen,
            r#type: ThemeColor::Cyan,
            builtin: ThemeColor::BrightRed,
            function: ThemeColor::BrightYellow,
            operator: ThemeColor::Yellow,
            punctuation: ThemeColor::Yellow,
            attribute: ThemeColor::BrightCyan,
            macro_token: ThemeColor::BrightMagenta,
            regex: ThemeColor::BrightRed,
            tag: ThemeColor::BrightCyan,
            constant: ThemeColor::Cyan,
            normal: ThemeColor::BrightWhite,
            muted: ThemeColor::BrightBlack,
            table_header: ThemeColor::Cyan,
            table_border: ThemeColor::Cyan,
            line_number: ThemeColor::Yellow,
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
    /// Root command in a prompt line (`cfg`, `git`).
    pub ntc_command: ThemeColor,
    /// File path typed after `txtf` / `txtc`.
    pub txtf_file: ThemeColor,
    /// Run-alias name (`ral add <name>`, `run <name>`).
    pub ral_name: ThemeColor,
    /// Command body passed to `ral add` / `ral edit`.
    pub ral_command: ThemeColor,
    /// Legacy first-subcommand color, kept so existing theme files still load.
    /// Prompt first arguments now use `ntc_command`.
    pub ntc_sub_command: ThemeColor,
    /// Second subcommand in a prompt line (`cfg color on`).
    pub sub_sub_command: ThemeColor,
    /// Short flag such as `-d`.
    pub flag1: ThemeColor,
    /// Long flag such as `--depth`.
    pub flag2: ThemeColor,
    /// Teleport name such as `@web`.
    pub tp_name: ThemeColor,
    /// Path portion of a teleport or `tp add` path.
    pub tp_path: ThemeColor,
    /// Alias name shown in `ral ls`.
    pub ral_ls_alias: ThemeColor,
    /// Teleport name shown in `tp ls`.
    pub tp_ls_tp_name: ThemeColor,
    pub success: ThemeColor,
    pub error: ThemeColor,
    pub warning: ThemeColor,
    pub info: ThemeColor,
    /// Secondary text that used to be rendered with `.dimmed()`.
    pub muted: ThemeColor,
    /// Header row in box/table output (`nsheet`, reports).
    pub table_header: ThemeColor,
    /// Frames and separators in box/table output.
    pub table_border: ThemeColor,
    /// Gutter line numbers for `txtf` / `view`.
    pub line_number: ThemeColor,
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
    pub git_branch: ThemeColor,
    pub git_added: ThemeColor,
    pub git_modified: ThemeColor,
    pub git_deleted: ThemeColor,
    pub git_untracked: ThemeColor,
    pub git_conflict: ThemeColor,
}

impl Default for ShellTheme {
    fn default() -> Self {
        Self {
            prompt_bracket: ThemeColor::BrightCyan,
            prompt_path: ThemeColor::Cyan,
            prompt_watcher: ThemeColor::Yellow,
            prompt_arrow: ThemeColor::Green,
            ntc_command: ThemeColor::Yellow,
            txtf_file: ThemeColor::BrightGreen,
            ral_name: ThemeColor::BrightMagenta,
            ral_command: ThemeColor::Yellow,
            ntc_sub_command: ThemeColor::BrightCyan,
            sub_sub_command: ThemeColor::BrightBlue,
            flag1: ThemeColor::Yellow,
            flag2: ThemeColor::BrightYellow,
            tp_name: ThemeColor::BrightBlue,
            tp_path: ThemeColor::Cyan,
            ral_ls_alias: ThemeColor::Cyan,
            tp_ls_tp_name: ThemeColor::Cyan,
            success: ThemeColor::Green,
            error: ThemeColor::Red,
            warning: ThemeColor::Yellow,
            info: ThemeColor::Cyan,
            muted: ThemeColor::BrightBlack,
            table_header: ThemeColor::Cyan,
            table_border: ThemeColor::Cyan,
            line_number: ThemeColor::Yellow,
            tree_branch: ThemeColor::Rgb {
                r: 30,
                g: 220,
                b: 220,
            },
            tree_dir: ThemeColor::BrightMagenta,
            tree_file: ThemeColor::Yellow,
            tree_ignored: ThemeColor::BrightBlack,
            tree_size: ThemeColor::Yellow,
            command_output: ThemeColor::Yellow,
            separator: ThemeColor::Cyan,
            help_header: ThemeColor::Cyan,
            help_section: ThemeColor::Cyan,
            help_example: ThemeColor::Green,
            teleport_name: ThemeColor::BrightBlue,
            alias_name: ThemeColor::Blue,
            alias_command: ThemeColor::BrightBlack,
            git_branch: ThemeColor::BrightCyan,
            git_added: ThemeColor::Green,
            git_modified: ThemeColor::Yellow,
            git_deleted: ThemeColor::Red,
            git_untracked: ThemeColor::BrightBlack,
            git_conflict: ThemeColor::BrightRed,
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
    /// Background of editor text inputs (VS Code Dark+ `input.background`).
    pub text_field_bg: ThemeColor,
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
    /// Vertical guides drawn at each tab stop inside a line's leading
    /// whitespace (VS Code-style indent guides).
    pub indent_guide: ThemeColor,
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
            editor_bg: ThemeColor::Rgb {
                r: 46,
                g: 44,
                b: 51,
            },
            gutter_bg: ThemeColor::Rgb {
                r: 38,
                g: 36,
                b: 44,
            },
            status_bg: ThemeColor::Rgb {
                r: 130,
                g: 100,
                b: 210,
            },
            hint_bg: ThemeColor::Rgb {
                r: 28,
                g: 26,
                b: 34,
            },
            text_field_bg: ThemeColor::Rgb {
                r: 60,
                g: 60,
                b: 60,
            },
            sidebar_bg: ThemeColor::Rgb {
                r: 21,
                g: 19,
                b: 26,
            },
            sidebar_selected_bg: ThemeColor::Rgb {
                r: 50,
                g: 45,
                b: 65,
            },
            run_panel_bg: ThemeColor::Rgb {
                r: 26,
                g: 24,
                b: 32,
            },
            gutter_text: ThemeColor::Rgb {
                r: 100,
                g: 95,
                b: 118,
            },
            status_text: ThemeColor::White,
            status_modified: ThemeColor::Yellow,
            hint_text: ThemeColor::Rgb {
                r: 140,
                g: 222,
                b: 255,
            },
            cursor_bg: ThemeColor::White,
            cursor_text: ThemeColor::Black,
            extra_cursor_bg: ThemeColor::Rgb {
                r: 255,
                g: 140,
                b: 154,
            },
            selection_bg: ThemeColor::Rgb {
                r: 58,
                g: 50,
                b: 85,
            },
            search_match_bg: ThemeColor::Rgb {
                r: 48,
                g: 43,
                b: 63,
            },
            search_current_bg: ThemeColor::Rgb {
                r: 130,
                g: 100,
                b: 210,
            },
            border: ThemeColor::Rgb {
                r: 50,
                g: 46,
                b: 62,
            },
            scrollbar: ThemeColor::Rgb {
                r: 38,
                g: 35,
                b: 48,
            },
            scrollbar_thumb: ThemeColor::Rgb {
                r: 130,
                g: 100,
                b: 210,
            },
            indent_guide: ThemeColor::Rgb {
                r: 45,
                g: 41,
                b: 56,
            },
            sidebar_dir: ThemeColor::Rgb {
                r: 134,
                g: 215,
                b: 235,
            },
            sidebar_file: ThemeColor::Rgb {
                r: 200,
                g: 200,
                b: 215,
            },
            sidebar_current: ThemeColor::Rgb {
                r: 255,
                g: 208,
                b: 145,
            },
            sidebar_selected: ThemeColor::White,
            run_header_fg: ThemeColor::Rgb {
                r: 95,
                g: 145,
                b: 205,
            },
            run_header_unfocused_fg: ThemeColor::Rgb {
                r: 45,
                g: 85,
                b: 120,
            },
            run_output_fg: ThemeColor::Rgb {
                r: 185,
                g: 198,
                b: 212,
            },
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
            description: Some("Default ntc theme (VS Code Dark+ inspired)".to_string()),
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
}

// ============================================================================
// Shell color helpers
// ============================================================================

/// Apply a shell theme color to a string.
/// On non-Android: uses the `colored` crate.
/// On Android/Termux: uses raw ANSI escape codes.
#[cfg(not(target_os = "android"))]
pub fn paint_with_shell_color(s: &str, color: &ThemeColor) -> colored::ColoredString {
    colored::Colorize::color(s, color.to_colored())
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
    let theme = ThemeManager::current();
    let color = accessor(&theme.shell);
    colored::Colorize::color(s, color.to_colored())
}

/// Android version: get shell color via accessor, then paint with ANSI escapes.
#[cfg(target_os = "android")]
pub fn shell_paint(s: &str, accessor: fn(&ShellTheme) -> &ThemeColor) -> String {
    let theme = ThemeManager::current();
    let color = accessor(&theme.shell);
    format!("{}{}\x1b[0m", color.to_ansi_fg(), s)
}

/// Whether raw-ANSI startup/tree colors may be emitted.
/// Single gate for every OS: user preference (`setc ON`) AND terminal
/// capability (`NO_COLOR` / `TERM=dumb` disable). Bypasses the `colored`
/// crate's auto-detect so Termux behaves exactly like desktop — Termux
/// renders raw `\x1b[...m` natively (proven by `txtf` highlighting).
pub fn ansi_enabled() -> bool {
    if !crate::config::Config::global_get_color_enabled() {
        return false;
    }
    // Explicit opt-outs always win, on every platform.
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if let Ok(t) = std::env::var("TERM") {
        if t == "dumb" || t.is_empty() {
            return false;
        }
    }
    // Legacy Windows conhost cannot render raw escapes even when the user
    // enabled color; `detect()` folds the VT capability check in.
    crate::utils::terminal::detect().color
}

/// Unified shell painter: identical escapes on Linux/Win/macOS/Termux.
/// Returns plain text when `ansi_enabled()` is false.
pub fn shell_paint_ansi(s: &str, accessor: fn(&ShellTheme) -> &ThemeColor) -> String {
    let theme = ThemeManager::current();
    let color = accessor(&theme.shell).clone();
    color.paint_ansi(s)
}

/// Unified bold painter for headers (welcome, separator titles).
pub fn shell_paint_ansi_bold(s: &str, accessor: fn(&ShellTheme) -> &ThemeColor) -> String {
    let theme = ThemeManager::current();
    let color = accessor(&theme.shell).clone();
    color.paint_ansi_bold(s)
}

pub mod manager;
pub mod style;
pub use manager::ThemeManager;
pub use style::{ShellRole, ThemeColorize, ThemedText};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_switching() {
        assert!(ThemeManager::set_theme("default"));
        assert_eq!(ThemeManager::current().name, "default");
        // Light themes are no longer bundled.
        assert!(!ThemeManager::set_theme("light"));
        assert_eq!(ThemeManager::current().name, "default");
    }

    #[test]
    fn test_list_themes() {
        let themes = ThemeManager::list_themes();
        assert!(themes.contains(&"default".to_string()));
        assert!(!themes.contains(&"light".to_string()));
    }

    #[test]
    fn colours_convert_to_css_hex() {
        assert_eq!(ThemeColor::Rgb { r: 30, g: 144, b: 255 }.to_hex(), "#1e90ff");
        assert_eq!(ThemeColor::Black.to_hex(), "#000000");
        assert_eq!(ThemeColor::White.to_hex(), "#c0c0c0");
        assert_eq!(ThemeColor::BrightBlack.to_hex(), "#808080");
        assert_eq!(ThemeColor::BrightRed.to_hex(), "#ff0000");
        assert_eq!(ThemeColor::BrightCyan.to_hex(), "#00ffff");
        assert_eq!(ThemeColor::BrightWhite.to_hex(), "#ffffff");
    }

    #[test]
    fn ansi_indices_expand_like_a_terminal() {
        // The 6x6x6 cube starts at 16 and steps 0, 95, 135, 175, 215, 255.
        assert_eq!(ansi_rgb(16), (0, 0, 0));
        assert_eq!(ansi_rgb(17), (0, 0, 95));
        assert_eq!(ansi_rgb(21), (0, 0, 255));
        assert_eq!(ansi_rgb(196), (255, 0, 0));
        assert_eq!(ansi_rgb(231), (255, 255, 255));
        // The greyscale ramp starts at 232.
        assert_eq!(ansi_rgb(232), (8, 8, 8));
        assert_eq!(ansi_rgb(255), (238, 238, 238));
        assert_eq!(ansi_rgb(3), (128, 128, 0));
    }

    #[test]
    fn ls_name_roles_deserialize_and_default_to_cyan() {
        let custom: Theme = toml::from_str(
            r#"
name = "custom"
[shell]
ral_ls_alias = { r = 1, g = 2, b = 3 }
tp_ls_tp_name = "red"
"#,
        )
        .unwrap();
        assert_eq!(
            custom.shell.ral_ls_alias,
            ThemeColor::Rgb { r: 1, g: 2, b: 3 }
        );
        assert_eq!(custom.shell.tp_ls_tp_name, ThemeColor::Red);

        let fallback: Theme = toml::from_str("name = \"fallback\"\n").unwrap();
        assert_eq!(fallback.shell.ral_ls_alias, ThemeColor::Cyan);
        assert_eq!(fallback.shell.tp_ls_tp_name, ThemeColor::Cyan);
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
        assert_eq!(
            theme.editor.editor_bg,
            ThemeColor::Rgb {
                r: 30,
                g: 13,
                b: 60
            }
        );
        assert_eq!(
            theme.editor.gutter_bg,
            ThemeColor::Rgb {
                r: 13,
                g: 13,
                b: 21
            }
        );
        assert_eq!(theme.editor.status_text, ThemeColor::White);
        assert_eq!(
            theme.editor.hint_text,
            ThemeColor::Rgb {
                r: 130,
                g: 226,
                b: 255
            }
        );
        assert_eq!(
            theme.editor.selection_bg,
            ThemeColor::Rgb {
                r: 54,
                g: 51,
                b: 84
            }
        );

        // Roundtrip: serialize then deserialize again
        let serialized = toml::to_string_pretty(&theme).unwrap();
        let theme2: Theme = toml::from_str(&serialized).unwrap();
        assert_eq!(theme.name, theme2.name);
        assert_eq!(theme.syntax.keyword, theme2.syntax.keyword);
        assert_eq!(theme.editor.editor_bg, theme2.editor.editor_bg);
    }
}
