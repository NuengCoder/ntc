//! Theme-backed replacements for the `colored` crate's `Colorize` methods.
//!
//! Most shell code was written against `colored` (`.cyan()`, `.yellow()`,
//! `.dimmed()`, ...). Those calls bypass the selected theme. This module keeps
//! the same call shape but resolves every color through `ThemeManager` and
//! emits raw ANSI only when `cfg color on` / `setc ON` allows it.
//!
//! New code should prefer `ShellTheme::paint` / `paint_bold`, but the
//! compatibility trait makes it possible to migrate existing call sites
//! incrementally without changing their formatting.

use std::fmt;

use super::{ansi_enabled, ShellTheme, ThemeColor, ThemeManager};

/// Semantic shell color roles. Existing theme fields cover most roles; the
/// table/muted/line-number roles are new and have defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellRole {
    Error,
    Success,
    Warning,
    Info,
    Muted,
    Separator,
    HelpHeader,
    HelpSection,
    HelpExample,
    PromptBracket,
    PromptPath,
    NtcCommand,
    TxtfFile,
    RalName,
    RalCommand,
    RalLsAlias,
    NtcSubCommand,
    SubSubCommand,
    Flag1,
    Flag2,
    TpName,
    TpPath,
    TpLsTpName,
    CommandOutput,
    TreeBranch,
    TreeDir,
    TreeFile,
    TreeIgnored,
    TreeSize,
    TeleportName,
    AliasName,
    AliasCommand,
    GitBranch,
    GitAdded,
    GitModified,
    GitDeleted,
    GitUntracked,
    GitConflict,
    TableHeader,
    TableBorder,
    LineNumber,
}

impl ShellTheme {
    /// Resolve a semantic role to the theme color configured for it.
    pub fn role(&self, role: ShellRole) -> ThemeColor {
        match role {
            ShellRole::Error => self.error,
            ShellRole::Success => self.success,
            ShellRole::Warning => self.warning,
            ShellRole::Info => self.info,
            ShellRole::Muted => self.muted,
            ShellRole::Separator => self.separator,
            ShellRole::HelpHeader => self.help_header,
            ShellRole::HelpSection => self.help_section,
            ShellRole::HelpExample => self.help_example,
            ShellRole::PromptBracket => self.prompt_bracket,
            ShellRole::PromptPath => self.prompt_path,
            ShellRole::NtcCommand => self.ntc_command,
            ShellRole::TxtfFile => self.txtf_file,
            ShellRole::RalName => self.ral_name,
            ShellRole::RalCommand => self.ral_command,
            ShellRole::RalLsAlias => self.ral_ls_alias,
            ShellRole::NtcSubCommand => self.ntc_sub_command,
            ShellRole::SubSubCommand => self.sub_sub_command,
            ShellRole::Flag1 => self.flag1,
            ShellRole::Flag2 => self.flag2,
            ShellRole::TpName => self.tp_name,
            ShellRole::TpPath => self.tp_path,
            ShellRole::TpLsTpName => self.tp_ls_tp_name,
            ShellRole::CommandOutput => self.command_output,
            ShellRole::TreeBranch => self.tree_branch,
            ShellRole::TreeDir => self.tree_dir,
            ShellRole::TreeFile => self.tree_file,
            ShellRole::TreeIgnored => self.tree_ignored,
            ShellRole::TreeSize => self.tree_size,
            ShellRole::TeleportName => self.teleport_name,
            ShellRole::AliasName => self.alias_name,
            ShellRole::AliasCommand => self.alias_command,
            ShellRole::GitBranch => self.git_branch,
            ShellRole::GitAdded => self.git_added,
            ShellRole::GitModified => self.git_modified,
            ShellRole::GitDeleted => self.git_deleted,
            ShellRole::GitUntracked => self.git_untracked,
            ShellRole::GitConflict => self.git_conflict,
            ShellRole::TableHeader => self.table_header,
            ShellRole::TableBorder => self.table_border,
            ShellRole::LineNumber => self.line_number,
        }
    }

    /// Paint text with a semantic shell role, respecting the color toggle.
    pub fn paint(&self, role: ShellRole, text: &str) -> String {
        self.role(role).paint_ansi(text)
    }

    /// Paint bold text with a semantic shell role.
    pub fn paint_bold(&self, role: ShellRole, text: &str) -> String {
        self.role(role).paint_ansi_bold(text)
    }
}

/// A string plus theme styling. It renders lazily so a theme switch or
/// `cfg color off` made after construction still affects what is printed.
#[derive(Debug, Clone)]
pub struct ThemedText {
    text: String,
    role: Option<ShellRole>,
    explicit: Option<ThemeColor>,
    bold: bool,
    dim: bool,
    italic: bool,
    reversed: bool,
}

impl ThemedText {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            role: None,
            explicit: None,
            bold: false,
            dim: false,
            italic: false,
            reversed: false,
        }
    }

    fn with_role(text: &str, role: ShellRole) -> Self {
        let mut styled = Self::new(text);
        styled.role = Some(role);
        styled
    }

    fn with_explicit(text: &str, color: ThemeColor) -> Self {
        let mut styled = Self::new(text);
        styled.explicit = Some(color);
        styled
    }

    fn resolve_color(&self) -> Option<ThemeColor> {
        if let Some(color) = self.explicit {
            return Some(color);
        }
        self.role.map(|role| {
            let theme = ThemeManager::current();
            let color = theme.shell.role(role);
            drop(theme);
            color
        })
    }

    fn set_role(mut self, role: ShellRole) -> Self {
        self.role = Some(role);
        self.explicit = None;
        self
    }

    pub fn black(self) -> Self {
        self.set_role(ShellRole::Muted)
    }
    pub fn red(self) -> Self {
        self.set_role(ShellRole::Error)
    }
    pub fn green(self) -> Self {
        self.set_role(ShellRole::Success)
    }
    pub fn yellow(self) -> Self {
        self.set_role(ShellRole::Warning)
    }
    pub fn blue(self) -> Self {
        self.set_role(ShellRole::AliasName)
    }
    pub fn magenta(self) -> Self {
        self.set_role(ShellRole::TreeDir)
    }
    pub fn purple(self) -> Self {
        self.set_role(ShellRole::TreeDir)
    }
    pub fn cyan(self) -> Self {
        self.set_role(ShellRole::Separator)
    }
    pub fn white(self) -> Self {
        self.set_role(ShellRole::CommandOutput)
    }
    pub fn bright_black(self) -> Self {
        self.set_role(ShellRole::Muted)
    }
    pub fn bright_red(self) -> Self {
        self.set_role(ShellRole::GitConflict)
    }
    pub fn bright_green(self) -> Self {
        self.set_role(ShellRole::Success)
    }
    pub fn bright_yellow(self) -> Self {
        self.set_role(ShellRole::TreeSize)
    }
    pub fn bright_blue(self) -> Self {
        self.set_role(ShellRole::TeleportName)
    }
    pub fn bright_magenta(self) -> Self {
        self.set_role(ShellRole::TreeDir)
    }
    pub fn bright_purple(self) -> Self {
        self.set_role(ShellRole::TreeDir)
    }
    pub fn bright_cyan(self) -> Self {
        self.set_role(ShellRole::PromptBracket)
    }
    pub fn bright_white(self) -> Self {
        self.set_role(ShellRole::CommandOutput)
    }

    /// Make this run bold.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make this run secondary/muted. A bare `.dimmed()` uses the theme's
    /// muted role; chaining after a color keeps that color and adds the ANSI
    /// dim attribute.
    pub fn dimmed(mut self) -> Self {
        if self.role.is_none() && self.explicit.is_none() {
            self.role = Some(ShellRole::Muted);
        } else {
            self.dim = true;
        }
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn reversed(mut self) -> Self {
        self.reversed = true;
        self
    }

    pub fn color<C: Into<ThemeColor>>(mut self, color: C) -> Self {
        self.explicit = Some(color.into());
        self
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl fmt::Display for ThemedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Honor `{:^50}` / `{:>10}` using the visible text width, not the
        // byte count of the ANSI escapes.
        let pad = f
            .width()
            .map(|w| w.saturating_sub(self.text.chars().count()))
            .unwrap_or(0);
        let (left_pad, right_pad) = match f.align().unwrap_or(fmt::Alignment::Left) {
            fmt::Alignment::Left => (0, pad),
            fmt::Alignment::Right => (pad, 0),
            fmt::Alignment::Center => (pad / 2, pad - pad / 2),
        };
        let fill = f.fill().to_string();
        let left = fill.repeat(left_pad);
        let right = fill.repeat(right_pad);

        if !ansi_enabled() {
            return write!(f, "{}{}{}", left, self.text, right);
        }

        let mut prefix = String::new();
        if self.bold {
            prefix.push_str("\x1b[1m");
        }
        if self.dim {
            prefix.push_str("\x1b[2m");
        }
        if self.italic {
            prefix.push_str("\x1b[3m");
        }
        if self.reversed {
            prefix.push_str("\x1b[7m");
        }
        if let Some(color) = self.resolve_color() {
            prefix.push_str(&color.to_ansi_fg());
        }

        if prefix.is_empty() {
            write!(f, "{}{}{}", left, self.text, right)
        } else {
            write!(f, "{}{}{}{}\x1b[0m", prefix, left, self.text, right)
        }
    }
}

impl From<colored::Color> for ThemeColor {
    fn from(color: colored::Color) -> Self {
        use colored::Color as C;
        match color {
            C::Black => ThemeColor::Black,
            C::Red => ThemeColor::Red,
            C::Green => ThemeColor::Green,
            C::Yellow => ThemeColor::Yellow,
            C::Blue => ThemeColor::Blue,
            C::Magenta => ThemeColor::Magenta,
            C::Cyan => ThemeColor::Cyan,
            C::White => ThemeColor::White,
            C::BrightBlack => ThemeColor::BrightBlack,
            C::BrightRed => ThemeColor::BrightRed,
            C::BrightGreen => ThemeColor::BrightGreen,
            C::BrightYellow => ThemeColor::BrightYellow,
            C::BrightBlue => ThemeColor::BrightBlue,
            C::BrightMagenta => ThemeColor::BrightMagenta,
            C::BrightCyan => ThemeColor::BrightCyan,
            C::BrightWhite => ThemeColor::BrightWhite,
            C::TrueColor { r, g, b } => ThemeColor::Rgb { r, g, b },
            _ => ThemeColor::White,
        }
    }
}

/// Drop-in replacement for `colored::Colorize` that resolves through the
/// active ntc theme. Existing chains like `.cyan().bold()` keep working.
pub trait ThemeColorize {
    fn black(&self) -> ThemedText;
    fn red(&self) -> ThemedText;
    fn green(&self) -> ThemedText;
    fn yellow(&self) -> ThemedText;
    fn blue(&self) -> ThemedText;
    fn magenta(&self) -> ThemedText;
    fn purple(&self) -> ThemedText;
    fn cyan(&self) -> ThemedText;
    fn white(&self) -> ThemedText;
    fn bright_black(&self) -> ThemedText;
    fn bright_red(&self) -> ThemedText;
    fn bright_green(&self) -> ThemedText;
    fn bright_yellow(&self) -> ThemedText;
    fn bright_blue(&self) -> ThemedText;
    fn bright_magenta(&self) -> ThemedText;
    fn bright_purple(&self) -> ThemedText;
    fn bright_cyan(&self) -> ThemedText;
    fn bright_white(&self) -> ThemedText;
    fn bold(&self) -> ThemedText;
    fn dimmed(&self) -> ThemedText;
    fn italic(&self) -> ThemedText;
    fn reversed(&self) -> ThemedText;
    fn color<C: Into<ThemeColor>>(&self, color: C) -> ThemedText;
}

impl ThemeColorize for str {
    fn black(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Muted)
    }
    fn red(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Error)
    }
    fn green(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Success)
    }
    fn yellow(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Warning)
    }
    fn blue(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::AliasName)
    }
    fn magenta(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::TreeDir)
    }
    fn purple(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::TreeDir)
    }
    fn cyan(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Separator)
    }
    fn white(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::CommandOutput)
    }
    fn bright_black(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Muted)
    }
    fn bright_red(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::GitConflict)
    }
    fn bright_green(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::Success)
    }
    fn bright_yellow(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::TreeSize)
    }
    fn bright_blue(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::TeleportName)
    }
    fn bright_magenta(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::TreeDir)
    }
    fn bright_purple(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::TreeDir)
    }
    fn bright_cyan(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::PromptBracket)
    }
    fn bright_white(&self) -> ThemedText {
        ThemedText::with_role(self, ShellRole::CommandOutput)
    }
    fn bold(&self) -> ThemedText {
        ThemedText::new(self).bold()
    }
    fn dimmed(&self) -> ThemedText {
        ThemedText::new(self).dimmed()
    }
    fn italic(&self) -> ThemedText {
        ThemedText::new(self).italic()
    }
    fn reversed(&self) -> ThemedText {
        ThemedText::new(self).reversed()
    }
    fn color<C: Into<ThemeColor>>(&self, color: C) -> ThemedText {
        ThemedText::with_explicit(self, color.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::theme::Theme;

    #[test]
    fn roles_resolve_to_theme_fields() {
        let theme = Theme::default_theme();
        assert_eq!(theme.shell.role(ShellRole::Error), theme.shell.error);
        assert_eq!(theme.shell.role(ShellRole::Muted), theme.shell.muted);
        assert_eq!(
            theme.shell.role(ShellRole::TableHeader),
            theme.shell.table_header
        );
        assert_eq!(
            theme.shell.role(ShellRole::LineNumber),
            theme.shell.line_number
        );
        assert_eq!(
            theme.shell.role(ShellRole::NtcCommand),
            theme.shell.ntc_command
        );
        assert_eq!(theme.shell.role(ShellRole::TxtfFile), theme.shell.txtf_file);
        assert_eq!(theme.shell.role(ShellRole::RalName), theme.shell.ral_name);
        assert_eq!(
            theme.shell.role(ShellRole::RalCommand),
            theme.shell.ral_command
        );
        assert_eq!(
            theme.shell.role(ShellRole::NtcSubCommand),
            theme.shell.ntc_sub_command
        );
        assert_eq!(
            theme.shell.role(ShellRole::SubSubCommand),
            theme.shell.sub_sub_command
        );
        assert_eq!(theme.shell.role(ShellRole::Flag1), theme.shell.flag1);
        assert_eq!(theme.shell.role(ShellRole::Flag2), theme.shell.flag2);
        assert_eq!(theme.shell.role(ShellRole::TpName), theme.shell.tp_name);
        assert_eq!(theme.shell.role(ShellRole::TpPath), theme.shell.tp_path);
        assert_eq!(
            theme.shell.role(ShellRole::RalLsAlias),
            theme.shell.ral_ls_alias
        );
        assert_eq!(
            theme.shell.role(ShellRole::TpLsTpName),
            theme.shell.tp_ls_tp_name
        );
        assert_eq!(theme.shell.role(ShellRole::RalLsAlias), ThemeColor::Cyan);
        assert_eq!(theme.shell.role(ShellRole::TpLsTpName), ThemeColor::Cyan);
    }

    #[test]
    fn explicit_colors_roundtrip_from_colored() {
        assert_eq!(ThemeColor::from(colored::Color::Cyan), ThemeColor::Cyan);
        assert_eq!(
            ThemeColor::from(colored::Color::TrueColor { r: 1, g: 2, b: 3 }),
            ThemeColor::Rgb { r: 1, g: 2, b: 3 }
        );
    }
}
