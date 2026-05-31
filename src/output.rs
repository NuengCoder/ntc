use crate::config::Config;
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
#[cfg(not(target_os = "android"))]
use arboard::Clipboard;

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

/// Check if stdout is a terminal.
pub fn is_terminal() -> bool {
    std::io::stdout().is_terminal()
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
        "+{:-<left$}{}{:-<right$}+\n",
        "",
        title,
        "",
        left = left_padding,
        right = right_padding
    )
}

/// Print success message in green.
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Print error message in red.
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg.red());
}

/// Print info message in cyan.
pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".cyan(), msg);
}

/// Print warning message in yellow.
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg.yellow());
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
        print_info(&format!("You can view it with: cat {}", temp_file.display()));  // FIXED HERE
    } else {
        print_warning(&format!(
            "Clipboard not supported on Android. {} report content shown above.",
            format
        ));
        println!("\n{}\n", content);
    }
    
    Ok(())
}

