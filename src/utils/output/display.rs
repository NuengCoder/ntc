use crate::config::Config;
use crate::syntax::{color_for, SyntaxHighlighter};
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Read a file into a Rust String, detecting UTF-8 BOM, UTF-16LE BOM,
/// and UTF-16BE BOM. Without a BOM, tries UTF-8 first; on encoding
/// errors, falls back to Windows-1252.
fn read_file_as_string(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("Failed to read file '{}'", path.display()))?;
    if bytes.is_empty() {
        return Ok(String::new());
    }
    let content = if bytes.starts_with(b"\xef\xbb\xbf") {
        encoding_rs::UTF_8.decode_without_bom_handling(&bytes[3..]).0.into_owned()
    } else if bytes.starts_with(b"\xff\xfe") {
        encoding_rs::UTF_16LE.decode_without_bom_handling(&bytes[2..]).0.into_owned()
    } else if bytes.starts_with(b"\xfe\xff") {
        encoding_rs::UTF_16BE.decode_without_bom_handling(&bytes[2..]).0.into_owned()
    } else {
        let (decoded, had_errors) = encoding_rs::UTF_8.decode_without_bom_handling(&bytes);
        if had_errors {
            encoding_rs::WINDOWS_1252.decode_without_bom_handling(&bytes).0.into_owned()
        } else {
            decoded.into_owned()
        }
    };
    Ok(content)
}

/// Display file contents to stdout with optional line numbers.
pub fn cat_file(file_path: &Path, show_lines: bool) -> Result<()> {
    let content = read_file_as_string(file_path)?;

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
    let content = read_file_as_string(file_path)?;

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
    let content = read_file_as_string(file_path)?;

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
