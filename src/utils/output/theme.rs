use colored::*;

/// Print a colored separator line to stdout.
pub fn print_separator(title: &str) {
    let width: usize = 71;
    let title_len = title.len();
    let left_padding = (width.saturating_sub(title_len)) / 2;
    let right_padding = width.saturating_sub(title_len).saturating_sub(left_padding);

    println!(
        "{}+{:-<left$}{}{:-<right$}+{}",
        "+".blue(),
        "",
        title.yellow().bold(),
        "",
        "+".blue(),
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

/// Print success message using theme color.
pub fn print_success(msg: &str) {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        let icon = "✓".color(theme.shell.success.to_colored()).bold();
        let text = msg.color(theme.shell.success.to_colored());
        println!("{} {}", icon, text);
    }
    #[cfg(target_os = "android")]
    {
        println!("✓ {}", msg);
    }
}

/// Print error message using theme color.
pub fn print_error(msg: &str) {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        let icon = "✗".color(theme.shell.error.to_colored()).bold();
        let text = msg.color(theme.shell.error.to_colored());
        eprintln!("{} {}", icon, text);
    }
    #[cfg(target_os = "android")]
    {
        eprintln!("✗ {}", msg);
    }
}

/// Print info message using theme color.
pub fn print_info(msg: &str) {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        let icon = "ℹ".color(theme.shell.info.to_colored());
        let text = msg.color(theme.shell.info.to_colored());
        println!("{} {}", icon, text);
    }
    #[cfg(target_os = "android")]
    {
        println!("ℹ {}", msg);
    }
}

/// Print warning message using theme color.
pub fn print_warning(msg: &str) {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        let icon = "⚠".color(theme.shell.warning.to_colored());
        let text = msg.color(theme.shell.warning.to_colored());
        println!("{} {}", icon, text);
    }
    #[cfg(target_os = "android")]
    {
        println!("⚠ {}", msg);
    }
}
