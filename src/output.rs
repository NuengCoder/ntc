use crate::config::Config;
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cat_file_without_line_numbers() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3\n")?;

        let result = cat_file_with_line_numbers(&file_path, false)?;
        assert!(result.contains("test.txt:"));
        assert!(result.contains("line1"));
        assert!(!result.contains("LINE 1>>"));
        Ok(())
    }

    #[test]
    fn test_cat_file_with_line_numbers() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3\n")?;

        let result = cat_file_with_line_numbers(&file_path, true)?;
        assert!(result.contains("LINE 1>> line1"));
        assert!(result.contains("LINE 2>> line2"));
        assert!(result.contains("LINE 3>> line3"));
        Ok(())
    }

    #[test]
    fn test_cat_file_empty() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("empty.txt");
        fs::write(&file_path, "")?;

        let result = cat_file_with_line_numbers(&file_path, false)?;
        assert!(result.contains("empty.txt:"));
        Ok(())
    }

    #[test]
    fn test_cat_file_nonexistent_fails() {
        let result = cat_file(Path::new("nonexistent_file_xyz.txt"), false);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_creates_directories() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let nested_path = temp_dir.path().join("sub1").join("sub2").join("output.txt");

        write_file(&nested_path, "hello world")?;
        assert!(nested_path.exists());
        assert_eq!(fs::read_to_string(&nested_path)?, "hello world");
        Ok(())
    }

    #[test]
    fn test_write_file_overwrites() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");

        write_file(&file_path, "original")?;
        write_file(&file_path, "updated")?;

        assert_eq!(fs::read_to_string(&file_path)?, "updated");
        Ok(())
    }

    #[test]
    fn test_build_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let original = Config::global_get_output_path();
        Config::global_set_output_path(temp_dir.path());

        let path = build_output_path("report.txt");
        assert_eq!(path, temp_dir.path().join("report.txt"));

        Config::global_set_output_path(&original);
    }

    #[test]
    fn test_format_separator() {
        let sep = format_separator("TEST");
        assert!(sep.contains("TEST"));
        assert!(sep.contains('+'));
        assert!(sep.contains('-'));
    }

    #[test]
    fn test_print_separator_doesnt_panic() {
        print_separator("hello");
        print_separator("");
        print_separator("a very long title that exceeds the normal width significantly");
    }
}