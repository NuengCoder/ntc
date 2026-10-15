use crate::config::Config;
use crate::syntax::{color_for, SyntaxHighlighter};
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::t;

/// Read a file into a Rust String, detecting UTF-8 BOM, UTF-16LE BOM,
/// and UTF-16BE BOM. Without a BOM, tries UTF-8 first; on encoding
/// errors, falls back to Windows-1252.
fn read_file_as_string(path: &Path) -> Result<String> {
    let bytes =
        std::fs::read(path).with_context(|| t!("output.read_file_failed", path.display()))?;
    if bytes.is_empty() {
        return Ok(String::new());
    }
    let content = if bytes.starts_with(b"\xef\xbb\xbf") {
        encoding_rs::UTF_8
            .decode_without_bom_handling(&bytes[3..])
            .0
            .into_owned()
    } else if bytes.starts_with(b"\xff\xfe") {
        encoding_rs::UTF_16LE
            .decode_without_bom_handling(&bytes[2..])
            .0
            .into_owned()
    } else if bytes.starts_with(b"\xfe\xff") {
        encoding_rs::UTF_16BE
            .decode_without_bom_handling(&bytes[2..])
            .0
            .into_owned()
    } else {
        let (decoded, had_errors) = encoding_rs::UTF_8.decode_without_bom_handling(&bytes);
        if had_errors {
            encoding_rs::WINDOWS_1252
                .decode_without_bom_handling(&bytes)
                .0
                .into_owned()
        } else {
            decoded.into_owned()
        }
    };
    Ok(content)
}

/// Line range for partial file display (`txtf` / `view <file>`).
/// All bounds are 1-based and inclusive. Precedence when combined:
/// `Between` wins over `Head`, which wins over `Tail`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLineRange {
    Head(usize),
    Tail(usize),
    Between(usize, usize),
}

/// Parse `--betw n:m` (1-based inclusive, `n <= m`, both > 0).
/// Surrounding whitespace (including around the colon) is allowed.
pub fn parse_betw(raw: &str) -> Option<(usize, usize)> {
    let (a, b) = raw.split_once(':')?;
    let a: usize = a.trim().parse().ok()?;
    let b: usize = b.trim().parse().ok()?;
    if a == 0 || b == 0 || a > b {
        return None;
    }
    Some((a, b))
}

/// Combine optional `--head`/`--tail`/`--betw` values into one range.
/// `betw_raw` is the raw `--betw` argument when present; `Err(())`
/// means the `--betw` value was malformed.
pub fn parse_file_range(
    head: Option<usize>,
    tail: Option<usize>,
    betw_raw: Option<&str>,
) -> Result<Option<FileLineRange>, ()> {
    if let Some(raw) = betw_raw {
        match parse_betw(raw) {
            Some((a, b)) => return Ok(Some(FileLineRange::Between(a, b))),
            None => return Err(()),
        }
    }
    if let Some(n) = head {
        return Ok(Some(FileLineRange::Head(n)));
    }
    if let Some(n) = tail {
        return Ok(Some(FileLineRange::Tail(n)));
    }
    Ok(None)
}

/// Slice content lines by `range`, tagging each with its original
/// 1-based line number. Out-of-range bounds clamp to the file; a range
/// fully past EOF yields an empty vec.
pub fn slice_numbered_lines<'a>(
    lines: &[&'a str],
    range: Option<FileLineRange>,
) -> Vec<(usize, &'a str)> {
    let total = lines.len();
    let (from, to) = match range {
        None => (1, total),
        Some(FileLineRange::Head(n)) => (1, n.min(total)),
        Some(FileLineRange::Tail(n)) => (total.saturating_sub(n) + 1, total),
        Some(FileLineRange::Between(a, b)) => (a, b.min(total)),
    };
    if from > to || from > total {
        return Vec::new();
    }
    lines
        .iter()
        .enumerate()
        .skip(from - 1)
        .take(to - from + 1)
        .map(|(i, l)| (i + 1, *l))
        .collect()
}

/// Display file contents to stdout with optional line numbers.
pub fn cat_file(file_path: &Path, show_lines: bool) -> Result<()> {
    cat_file_range(file_path, show_lines, None)
}

/// Display a line range of a file (original numbers kept).
/// `None` prints the whole file, like [`cat_file`].
pub fn cat_file_range(
    file_path: &Path,
    show_lines: bool,
    range: Option<FileLineRange>,
) -> Result<()> {
    let content = read_file_as_string(file_path)?;

    println!(
        "{}",
        t!("output.filename_header", file_path.file_name().unwrap_or_default().to_string_lossy()).cyan().bold()
    );

    let lines: Vec<&str> = content.lines().collect();
    let selected = slice_numbered_lines(&lines, range);
    if range.is_some() && selected.is_empty() {
        println!("{}", t!("output.range_past_eof").dimmed());
        return Ok(());
    }

    if show_lines {
        for (no, line) in &selected {
            println!("{} {}", t!("output.line_prefix", no).yellow().bold(), line);
        }
    } else {
        for (_, line) in &selected {
            println!("{}", line);
        }
    }

    Ok(())
}

/// Whether raw ANSI codes may be emitted on this console.
///
/// Unlike `Config::global_get_color_enabled` (user preference) this also
/// respects VT availability: on legacy conhost where `SetConsoleMode(VT)`
/// fails, emitting `\x1b[...` prints `[91m` garbage, so we must stay plain
/// even if the user has `setc ON`.
pub fn raw_ansi_allowed() -> bool {
    Config::global_get_color_enabled()
        && colored::control::SHOULD_COLORIZE.should_colorize()
}

/// Convert a `crossterm::style::Color` to a raw ANSI foreground-color
/// escape sequence (e.g. `\x1b[38;2;R;G;Bm` for 24-bit RGB). This
/// intentionally bypasses the `colored` crate's `set_override` so that
/// syntax colors can be emitted even when the user has `setc OFF` —
/// the whole point of `cat_file_with_syntax` is to show colors.
/// Returns "" when the console can't render ANSI (legacy conhost).
fn crossterm_to_ansi(c: crossterm::style::Color) -> String {
    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        return String::new();
    }
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
    cat_file_with_syntax_range(file_path, show_lines, None)
}

/// Display a line range of a file with syntax highlighting, keeping
/// original line numbers. The highlighter still walks every line from
/// the top so multi-line constructs colorize correctly; only the
/// selected lines are printed.
pub fn cat_file_with_syntax_range(
    file_path: &Path,
    show_lines: bool,
    range: Option<FileLineRange>,
) -> Result<()> {
    let content = read_file_as_string(file_path)?;

    let color_enabled = Config::global_get_color_enabled();
    let _ = color_enabled;
    // On legacy conhost (VT unavailable) raw ANSI prints as `[91m` garbage.
    // Fall back to plain cat — readability beats broken colors.
    if !raw_ansi_allowed() {
        return cat_file_range(file_path, show_lines, range);
    }
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
        "{}{}{}",
        header_color,
        t!("output.filename_header", file_path.file_name().unwrap_or_default().to_string_lossy()),
        reset
    );
    println!();

    // The highlighter carries state (in_block_comment, in_xml_comment,
    // in_ps_comment) across lines, so we keep ONE instance alive for the
    // whole file. That way multi-line /* ... */, <!-- ... -->, and <# ... #>
    // regions colorize correctly from start to finish.
    let lines: Vec<&str> = content.lines().collect();
    let selected = slice_numbered_lines(&lines, range);
    if range.is_some() && selected.is_empty() {
        println!("{}", t!("output.range_past_eof").dimmed());
        return Ok(());
    }
    let wanted: std::collections::HashSet<usize> =
        selected.iter().map(|(no, _)| *no).collect();
    for (i, line) in lines.iter().enumerate() {
        let tokens = syntax.tokenize_line(i, line);
        // Original file line number, even for partial ranges.
        let no = i + 1;
        if !wanted.contains(&no) {
            continue;
        }

        if show_lines {
            print!(
                "{}{} {}",
                line_num_color,
                t!("output.line_prefix", no),
                reset,
            );
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

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn betw_parses_and_rejects() {
        assert_eq!(parse_betw("10:20"), Some((10, 20)));
        assert_eq!(parse_betw(" 3 : 5 "), Some((3, 5)));
        assert_eq!(parse_betw("5:5"), Some((5, 5)));
        assert_eq!(parse_betw("0:5"), None);
        assert_eq!(parse_betw("5:0"), None);
        assert_eq!(parse_betw("20:10"), None);
        assert_eq!(parse_betw("10"), None);
        assert_eq!(parse_betw("a:b"), None);
        assert_eq!(parse_betw(""), None);
        assert_eq!(parse_betw("10:20:30"), None);
    }

    #[test]
    fn file_range_precedence_is_betw_head_tail() {
        assert_eq!(
            parse_file_range(Some(5), Some(3), Some("2:4")).unwrap(),
            Some(FileLineRange::Between(2, 4))
        );
        assert_eq!(
            parse_file_range(Some(5), Some(3), None).unwrap(),
            Some(FileLineRange::Head(5))
        );
        assert_eq!(
            parse_file_range(None, Some(3), None).unwrap(),
            Some(FileLineRange::Tail(3))
        );
        assert_eq!(parse_file_range(None, None, None).unwrap(), None);
        assert!(parse_file_range(None, None, Some("9:2")).is_err());
    }

    #[test]
    fn slice_keeps_original_numbers_and_clamps() {
        let lines = vec!["a", "b", "c", "d", "e"];
        assert_eq!(
            slice_numbered_lines(&lines, None),
            vec![(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")]
        );
        assert_eq!(
            slice_numbered_lines(&lines, Some(FileLineRange::Head(2))),
            vec![(1, "a"), (2, "b")]
        );
        assert_eq!(
            slice_numbered_lines(&lines, Some(FileLineRange::Tail(2))),
            vec![(4, "d"), (5, "e")]
        );
        assert_eq!(
            slice_numbered_lines(&lines, Some(FileLineRange::Between(2, 4))),
            vec![(2, "b"), (3, "c"), (4, "d")]
        );
        // Clamp + past-EOF.
        assert_eq!(
            slice_numbered_lines(&lines, Some(FileLineRange::Between(4, 99))),
            vec![(4, "d"), (5, "e")]
        );
        assert!(slice_numbered_lines(&lines, Some(FileLineRange::Between(9, 99))).is_empty());
        assert!(slice_numbered_lines(&[], Some(FileLineRange::Head(5))).is_empty());
    }
}

/// Read file contents and return as string with optional line numbers.
pub fn cat_file_with_line_numbers(file_path: &Path, show_lines: bool) -> Result<String> {
    let content = read_file_as_string(file_path)?;

    let filename = file_path.file_name().unwrap_or_default().to_string_lossy();

    let mut output = String::new();
    output.push_str(&format!("{}\n", t!("output.filename_header", filename)));

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
            .with_context(|| t!("output.mkdir_failed", parent.display()))?;
    }

    let mut file = fs::File::create(file_path)
        .with_context(|| t!("output.mkfile_failed", file_path.display()))?;

    file.write_all(content.as_bytes())
        .with_context(|| t!("output.write_failed", file_path.display()))?;

    Ok(())
}

/// Build the full output path using the configured output directory.
pub fn build_output_path(filename: &str) -> PathBuf {
    let mut path = Config::global_get_output_path();
    path.push(filename);
    path
}
