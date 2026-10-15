

fn icon(unicode: &'static str, ascii: &'static str) -> &'static str {
    if crate::utils::terminal::is_ascii_mode() {
        ascii
    } else {
        unicode
    }
}

/// Print a colored separator line to stdout.
/// Unified raw-ANSI path so Termux shows the exact desktop colors
/// when `setc ON` (blue border, yellow-bold title), plain when OFF.
pub fn print_separator(title: &str) {
    use crate::utils::theme::ThemeColor;
    let width: usize = 71;
    let title_len = title.len();
    let left_padding = (width.saturating_sub(title_len)) / 2;
    let right_padding = width.saturating_sub(title_len).saturating_sub(left_padding);

    // Legacy desktop hues: blue border + yellow-bold title (matches
    // previous `.blue()` / `.yellow().bold()` exactly on every OS).
    let border_plus = ThemeColor::Blue.paint_ansi("+");
    let title_painted = ThemeColor::Yellow.paint_ansi_bold(title);
    println!(
        "{}+{:-<left$}{}{:-<right$}+{}",
        border_plus,
        "",
        title_painted,
        "",
        border_plus,
        left = left_padding,
        right = right_padding
    );
}

/// Format a separator line as a string (for reports, no color).
pub fn format_separator(title: &str) -> String {
    let width: usize = 71;
    let title_len = title.len();
    let left_padding = (width.saturating_sub(title_len)) / 2;
    let right_padding = width.saturating_sub(title_len).saturating_sub(left_padding);

    format!(
        "+{:-<left$}{}{:-<right$}+\n",
        "",
        title,
        "",
        left = left_padding,
        right = right_padding
    )
}

/// Print success message using theme color (unified raw-ANSI on every OS).
pub fn print_success(msg: &str) {
    let theme = crate::utils::theme::ThemeManager::current();
    let c = theme.shell.success.clone();
    drop(theme);
    println!("{} {}", c.paint_ansi_bold(icon("✓", "+")), c.paint_ansi(msg));
}

/// Print error message using theme color (unified raw-ANSI on every OS).
pub fn print_error(msg: &str) {
    let theme = crate::utils::theme::ThemeManager::current();
    let c = theme.shell.error.clone();
    drop(theme);
    eprintln!("{} {}", c.paint_ansi_bold(icon("✗", "x")), c.paint_ansi(msg));
}

/// Print info message using theme color (unified raw-ANSI on every OS).
pub fn print_info(msg: &str) {
    let theme = crate::utils::theme::ThemeManager::current();
    let c = theme.shell.info.clone();
    drop(theme);
    println!("{} {}", c.paint_ansi(icon("ℹ", "i")), c.paint_ansi(msg));
}

/// Print warning message using theme color (unified raw-ANSI on every OS).
pub fn print_warning(msg: &str) {
    let theme = crate::utils::theme::ThemeManager::current();
    let c = theme.shell.warning.clone();
    drop(theme);
    println!("{} {}", c.paint_ansi(icon("⚠", "!")), c.paint_ansi(msg));
}
