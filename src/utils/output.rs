use crate::config::Config;
use crate::syntax::{color_for, SyntaxHighlighter};
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
#[cfg(not(target_os = "android"))]
use arboard::Clipboard;

// ============================================================================
// Output capture (used by the modern shell to keep command output inside the
// UI's body region instead of clobbering the input line)
// ============================================================================

/// Redirect stdout to a temporary file, capture output, then restore.
/// This captures ALL output — `println!`, `print!`, colored crate, everything.
#[cfg(windows)]
fn with_stdout_captured<F: FnOnce()>(f: F) -> String {
    use std::os::windows::io::AsRawHandle;
    use std::os::windows::io::RawHandle;

    // Create a temp file path
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join(format!("ntc_capture_{}.txt", std::process::id()));
    let tmp_file2 = tmp_file.clone();

    // Open a file for writing (will be the new stdout)
    let file = std::fs::File::create(&tmp_file)
        .expect("failed to create capture temp file");

    extern "system" {
        fn GetStdHandle(nStdHandle: u32) -> RawHandle;
        fn SetStdHandle(nStdHandle: u32, hHandle: RawHandle) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;

    // Save the original stdout handle
    let original_fd = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    // Redirect stdout to our file
    unsafe { SetStdHandle(STD_OUTPUT_HANDLE, file.as_raw_handle()) };

    f();

    // Flush stdout
    let _ = std::io::stdout().flush();
    drop(file);

    // Restore original stdout
    unsafe { SetStdHandle(STD_OUTPUT_HANDLE, original_fd) };

    // Read the captured output
    let result = std::fs::read_to_string(&tmp_file2).unwrap_or_default();
    let _ = std::fs::remove_file(&tmp_file2);
    result
}

/// Redirect stdout to a temporary file, capture output, then restore.
/// This captures ALL output — `println!`, `print!`, colored crate, everything.
#[cfg(not(windows))]
fn with_stdout_captured<F: FnOnce()>(f: F) -> String {
    use std::os::unix::io::AsRawFd;
    use libc;

    // Create a temp file path
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join(format!("ntc_capture_{}.txt", std::process::id()));
    let tmp_file2 = tmp_file.clone();

    // Open a file for writing
    let file = std::fs::File::create(&tmp_file)
        .expect("failed to create capture temp file");

    let fd = file.as_raw_fd();
    let saved_fd = unsafe { libc::dup(1) };

    // Redirect stdout (fd 1) to our file
    unsafe { libc::dup2(fd, 1) };

    f();

    // Flush stdout
    let _ = std::io::stdout().flush();
    drop(file);

    // Restore original stdout
    unsafe { libc::dup2(saved_fd, 1) };
    unsafe { libc::close(saved_fd) };

    // Read the captured output
    let result = std::fs::read_to_string(&tmp_file2).unwrap_or_default();
    let _ = std::fs::remove_file(&tmp_file2);
    result
}

/// Run `f` with command output captured to an in-memory buffer.
/// This captures ALL stdout output including `println!()`, the colored crate,
/// and the custom `print_info`/`print_error` helpers.
pub fn with_captured_output<F: FnOnce()>(f: F) -> String {
    with_stdout_captured(f)
}

/// Display file contents to stdout with optional line numbers.
pub fn cat_file(file_path: &Path, show_lines: bool) -> Result<()> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Print filename header in cyan
    println!(
        "{}:",
        file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .cyan()
            .bold()
    );

    if show_lines {
        for (i, line) in content.lines().enumerate() {
            println!(
                "{} {}",
                format!("LINE {}>>", i + 1).yellow().bold(),
                line
            );
        }
    } else {
        println!("{}", content);
    }

    Ok(())
}

/// Convert a `crossterm::style::Color` to a raw ANSI foreground-color
/// escape sequence (e.g. `\x1b[38;2;R;G;Bm` for 24-bit RGB). This
/// intentionally bypasses the `colored` crate's `set_override` so that
/// syntax colors can be emitted even when the user has `setc OFF` —
/// the whole point of `cat_file_with_syntax` is to show colors.
fn crossterm_to_ansi(c: crossterm::style::Color) -> String {
    use crossterm::style::Color as Cc;
    match c {
        Cc::Reset => "\x1b[0m".to_string(),
        Cc::Black => "\x1b[30m".to_string(),
        Cc::DarkGrey => "\x1b[90m".to_string(),
        Cc::Red => "\x1b[31m".to_string(),
        Cc::DarkRed => "\x1b[91m".to_string(),
        Cc::Green => "\x1b[32m".to_string(),
        Cc::DarkGreen => "\x1b[92m".to_string(),
        Cc::Yellow => "\x1b[33m".to_string(),
        Cc::DarkYellow => "\x1b[93m".to_string(),
        Cc::Blue => "\x1b[34m".to_string(),
        Cc::DarkBlue => "\x1b[94m".to_string(),
        Cc::Magenta => "\x1b[35m".to_string(),
        Cc::DarkMagenta => "\x1b[95m".to_string(),
        Cc::Cyan => "\x1b[36m".to_string(),
        Cc::DarkCyan => "\x1b[96m".to_string(),
        Cc::White => "\x1b[97m".to_string(),
        Cc::Grey => "\x1b[37m".to_string(),
        Cc::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
        Cc::AnsiValue(n) => format!("\x1b[38;5;{}m", n),
    }
}

/// Display file contents to stdout with optional line numbers, applying
/// syntax highlighting (the same palette used by the built-in editor)
/// when the file extension maps to a known language. Syntax colors
/// are emitted via raw ANSI codes so they work even when the user has
/// `setc OFF` — that's the whole point of this function. Falls back
/// to the plain `cat_file` behavior when the extension is unknown.
pub fn cat_file_with_syntax(file_path: &Path, show_lines: bool) -> Result<()> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let color_enabled = Config::global_get_color_enabled();
    let ext = file_path.extension().and_then(|e| e.to_str());
    let mut syntax = SyntaxHighlighter::new(ext);
    let lang_detected = syntax.language.is_some();

    // No language detected — fall back to the existing line-numbered /
    // plain rendering. (If the user wants colors in this case they can
    // use `setc ON` and the line numbers / header will be colored.)
    if !lang_detected {
        return cat_file(file_path, show_lines);
    }

    // Language is detected. Emit the header + line-number prefix using
    // raw ANSI codes as well, so the whole display is consistently
    // colored regardless of the `setc` toggle. (Otherwise the header
    // would be plain while the body is colored, which is jarring.)
    let header_color = "\x1b[36;1m"; // cyan bold
    let line_num_color = "\x1b[33;1m"; // yellow bold
    let reset = "\x1b[0m";

    print!(
        "{}{}:{}",
        header_color,
        file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
        reset
    );
    println!();

    // Suppress the unused-variable warning for `color_enabled` — kept
    // for documentation: this function intentionally ignores it because
    // the whole point is to show syntax colors.
    let _ = color_enabled;

    // The highlighter carries state (in_block_comment, in_xml_comment,
    // in_ps_comment) across lines, so we keep ONE instance alive for the
    // whole file. That way multi-line /* ... */, <!-- ... -->, and <# ... #>
    // regions colorize correctly from start to finish.
    for (i, line) in content.lines().enumerate() {
        let tokens = syntax.tokenize_line(i, line);

        if show_lines {
            print!("{color}LINE {num}>> {reset}{color}", color = line_num_color, num = i + 1, reset = reset);
            // Stay in the yellow-bold prefix style for the gap.
        }

        // Walk char-by-char (not byte-by-byte) to stay safe on UTF-8.
        // Group consecutive characters sharing the same token color into
        // a single run, so we emit one ANSI prefix per run.
        let mut current_color: Option<crossterm::style::Color> = None;
        let mut current_text = String::new();

        for (char_start, ch) in line.char_indices() {
            let tt = tokens
                .iter()
                .find(|t| char_start >= t.start && char_start < t.end)
                .map(|t| t.token_type);
            let color = tt.map(color_for);

            if color != current_color {
                if !current_text.is_empty() {
                    match current_color {
                        Some(c) => print!("{}{}{}", crossterm_to_ansi(c), current_text, reset),
                        None => print!("{}{}", reset, current_text),
                    }
                    current_text.clear();
                }
                current_color = color;
            }
            current_text.push(ch);
        }

        // Flush the trailing run.
        if !current_text.is_empty() {
            match current_color {
                Some(c) => print!("{}{}{}", crossterm_to_ansi(c), current_text, reset),
                None => print!("{}{}", reset, current_text),
            }
        }

        // End the line — println! would add an extra newline for empty lines.
        println!();
    }

    Ok(())
}

/// Read file contents and return as string with optional line numbers.
pub fn cat_file_with_line_numbers(file_path: &Path, show_lines: bool) -> Result<String> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    let filename = file_path.file_name().unwrap_or_default().to_string_lossy();

    let mut output = String::new();
    output.push_str(&format!("{}:\n", filename));

    if show_lines {
        for (i, line) in content.lines().enumerate() {
            output.push_str(&format!("LINE {}>> {}\n", i + 1, line));
        }
    } else {
        output.push_str(&content);
        if !content.ends_with('\n') {
            output.push('\n');
        }
    }

    Ok(output)
}

/// Write content to a file, creating parent directories if needed.
pub fn write_file(file_path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    let mut file = fs::File::create(file_path)
        .with_context(|| format!("Failed to create file: {}", file_path.display()))?;

    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write to file: {}", file_path.display()))?;

    Ok(())
}

/// Build the full output path using the configured output directory.
pub fn build_output_path(filename: &str) -> PathBuf {
    let mut path = Config::global_get_output_path();
    path.push(filename);
    path
}

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
        "+{:-<left$}{}{left:-<right$}+\n",
        "",
        title,
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

// src/output.rs - Replace the entire clipboard section

/// Check if running in Termux environment
#[cfg(target_os = "android")]
fn is_termux() -> bool {
    std::env::var("TERMUX_VERSION").is_ok() 
        || std::path::Path::new("/data/data/com.termux/files/usr/bin/termux-clipboard-set").exists()
}

/// Copy to clipboard using Termux API (Android only)
#[cfg(target_os = "android")]
pub fn copy_to_clipboard_termux(content: &str) -> Result<bool> {
    use std::process::Command;
    use std::io::Write;
    
    if !is_termux() {
        return Ok(false);
    }
    
    // Try termux-clipboard-set first
    let status = Command::new("termux-clipboard-set")
        .arg(content)
        .status();
    
    if let Ok(status) = status {
        if status.success() {
            return Ok(true);
        }
    }
    
    // Fallback: write to temp file and use termux-clipboard-set with stdin
    let temp_file = std::env::temp_dir().join(format!("ntc_clipboard_{}.txt", std::process::id()));
    if let Ok(mut file) = std::fs::File::create(&temp_file) {
        let _ = file.write_all(content.as_bytes());
        let _ = file.sync_all();
        
        let status = Command::new("termux-clipboard-set")
            .arg(temp_file.to_str().unwrap_or(""))
            .status();
        
        let _ = std::fs::remove_file(temp_file);
        
        if let Ok(status) = status {
            if status.success() {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

/// Copy to clipboard for all platforms
#[cfg(not(target_os = "android"))]
pub fn copy_to_clipboard(content: &str, format: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .with_context(|| "Failed to access clipboard")?;
    
    clipboard.set_text(content.to_string())
        .with_context(|| format!("Failed to copy {} report to clipboard", format))?;
    
    Ok(())
}

#[cfg(target_os = "android")]
pub fn copy_to_clipboard(content: &str, format: &str) -> Result<()> {
    // Try Termux clipboard first
    if copy_to_clipboard_termux(content)? {
        print_success(&format!("{} report copied to clipboard via Termux!", format));
        return Ok(());
    }
    
    // Fallback: save to temp file and show path
    let temp_file = std::env::temp_dir().join(format!("ntc_{}_{}.{}", 
        format.to_lowercase(),
        chrono::Local::now().format("%Y%m%d_%H%M%S"),
        format.to_lowercase()
    ));
    
    if let Ok(mut file) = std::fs::File::create(&temp_file) {
        use std::io::Write;
        let _ = file.write_all(content.as_bytes());
        print_warning(&format!(
            "Clipboard not available. {} report saved to: {}",
            format,
            temp_file.display()
        ));
        print_info(&format!("You can view it with: cat {}", temp_file.display()));
    } else {
        print_warning(&format!(
            "Clipboard not supported on Android. {} report content shown above.",
            format
        ));
        println!("\n{}\n", content);
    }
    
    Ok(())
}