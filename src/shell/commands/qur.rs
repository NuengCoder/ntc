//! `qur` — query files by glob, select content, display like `gs`, replace.
//!
//! Syntax:
//!   qur <file-glob> [-s "<pattern>"] [-rp "<replacement>"] [-dp] [-d <depth>] [-l <limit>]
//!
//! Modes:
//!   qur <glob>                  → list matching files (single existing file → cat it)
//!   qur <glob> -s "<pat>"       → summary per file (match counts)
//!   qur <glob> -s "<pat>" -dp   → detailed gs-style lines
//!   qur <glob> -s "<old>" -rp "<new>" → preview → confirm → in-place replace

use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::search;
use crate::shell::helpers::run_with_spinner;
use crate::t;

use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Binary / app extensions always skipped by `qur` (per user request:
/// exe, iso, zip, gz, tar + other non-text app/archive/media types).
/// The ignore/care filters (`FormatConfig`) still apply on top of this.
const QUR_SKIP_EXTS: &[&str] = &[
    "exe", "dll", "so", "bin", "iso", "img", "msi", "deb", "rpm", "apk", "jar", "war", "zip", "gz",
    "tar", "xz", "7z", "rar", "pdf", "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "mp3",
    "wav", "ogg", "flac", "mp4", "avi", "mkv", "mov", "o", "obj", "a", "lib", "pyc", "pyo",
    "class", "sqlite", "db", "db3", "vhd", "vhdx", "dmg",
];

#[derive(Debug, Clone)]
pub(crate) struct QurArgs {
    pub globs: Vec<String>,
    pub select: Option<String>,
    pub replace: Option<String>,
    pub display: bool,
    pub max_depth: usize,
    pub limit: Option<usize>,
    pub help: bool,
}

/// Tokenize respecting both double and single quotes.
fn tokenize_qur_args(args: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for ch in args.chars() {
        match quote {
            Some(q) => {
                if ch == q {
                    quote = None;
                } else {
                    current.push(ch);
                }
            }
            None => match ch {
                '"' | '\'' => quote = Some(ch),
                ' ' | '\t' => {
                    if !current.is_empty() {
                        parts.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(ch),
            },
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

fn parse_usize_value(v: Option<&String>) -> Option<usize> {
    v.and_then(|s| s.parse::<usize>().ok())
}

/// Parse `qur` args in any flag order. Returns Err(usage_key) on error.
pub(crate) fn parse_qur_args(args: &str) -> std::result::Result<QurArgs, String> {
    let default_depth = Config::global_get_max_depth();
    let parts = tokenize_qur_args(args);
    let mut globs: Vec<String> = Vec::new();
    let mut select: Option<String> = None;
    let mut replace: Option<String> = None;
    let mut display = false;
    let mut max_depth = default_depth;
    let mut limit: Option<usize> = None;
    let mut help = false;

    let mut i = 0;
    while i < parts.len() {
        let p = parts[i].as_str();
        // Support `-flag=value` form.
        let (flag, inline_val) = match p.split_once('=') {
            Some((f, v)) => (f, Some(v.to_string())),
            None => (p, None),
        };
        match flag {
            "-s" | "--select" => {
                let has_inline = inline_val.is_some();
                let val = inline_val.clone().or_else(|| {
                    parts.get(i + 1).cloned().filter(|nxt| {
                        // Next token is a value unless it is a known flag.
                        !matches!(
                            nxt.as_str(),
                            "-s" | "--select"
                                | "-rp"
                                | "--replace"
                                | "-dp"
                                | "--display"
                                | "-d"
                                | "--depth"
                                | "-l"
                                | "--limit"
                                | "-h"
                                | "--help"
                        ) && !nxt.starts_with("--limit=")
                    })
                });
                match val {
                    Some(v) => {
                        select = Some(v);
                        if !has_inline {
                            i += 1;
                        }
                    }
                    None => return Err("empty_select".into()),
                }
            }
            "-rp" | "--replace" => {
                let has_inline = inline_val.is_some();
                let val = inline_val.clone().or_else(|| {
                    parts.get(i + 1).cloned().filter(|nxt| {
                        !matches!(
                            nxt.as_str(),
                            "-s" | "--select"
                                | "-rp"
                                | "--replace"
                                | "-dp"
                                | "--display"
                                | "-d"
                                | "--depth"
                                | "-l"
                                | "--limit"
                                | "-h"
                                | "--help"
                        ) && !nxt.starts_with("--limit=")
                    })
                });
                match val {
                    Some(v) => {
                        replace = Some(v);
                        if !has_inline {
                            i += 1;
                        }
                    }
                    None => return Err("empty_replace".into()),
                }
            }
            "-dp" | "--display" => display = true,
            "-d" | "--depth" => {
                let has_inline = inline_val.is_some();
                let raw = inline_val.clone().or_else(|| parts.get(i + 1).cloned());
                match parse_usize_value(raw.as_ref()) {
                    Some(v) => {
                        max_depth = v;
                        if !has_inline {
                            i += 1;
                        }
                    }
                    None => {
                        // Treat as positional glob (e.g. a file literally named "-d").
                        globs.push(parts[i].clone());
                    }
                }
            }
            "-l" | "--limit" => {
                let has_inline = inline_val.is_some();
                let raw = inline_val.clone().or_else(|| parts.get(i + 1).cloned());
                match parse_usize_value(raw.as_ref()).filter(|v| *v > 0) {
                    Some(v) => {
                        limit = Some(v);
                        if !has_inline {
                            i += 1;
                        }
                    }
                    None => globs.push(parts[i].clone()),
                }
            }
            s if s.starts_with("--limit=") => {
                if let Some(v) = s["--limit=".len()..]
                    .parse::<usize>()
                    .ok()
                    .filter(|v| *v > 0)
                {
                    limit = Some(v);
                } else {
                    globs.push(parts[i].clone());
                }
            }
            "-h" | "--help" => help = true,
            other => globs.push(other.to_string()),
        }
        i += 1;
    }

    if !help && globs.is_empty() {
        return Err("missing_glob".into());
    }
    if replace.is_some() && select.is_none() {
        return Err("replace_needs_select".into());
    }
    if display && select.is_none() {
        return Err("display_needs_select".into());
    }

    Ok(QurArgs {
        globs,
        select,
        replace,
        display,
        max_depth,
        limit,
        help,
    })
}

/// True if the file extension is in the binary/app skip list.
fn is_qur_skipped_ext(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| QUR_SKIP_EXTS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Pure glob matcher used by `resolve_qur_files` (unit-testable).
/// - `rel` is the `/`-separated path relative to the search root.
/// - If the glob contains `/` or `\`, match against the relative path;
///   otherwise match against the file name.
/// - With wildcards → [`search::matches_wildcard`]; without → case-insensitive substring.
pub(crate) fn file_matches_glob(file_name: &str, rel: &str, glob: &str) -> bool {
    let norm = glob.replace('\\', "/");
    if norm.contains('/') {
        if search::has_wildcards(&norm) {
            search::matches_wildcard(&norm, rel)
                || search::matches_wildcard(norm.rsplit('/').next().unwrap_or(&norm), file_name)
        } else {
            rel.to_lowercase().contains(&norm.to_lowercase())
                || file_name.to_lowercase().contains(&glob.to_lowercase())
        }
    } else if search::has_wildcards(glob) {
        search::matches_wildcard(glob, file_name)
    } else {
        file_name.to_lowercase().contains(&glob.to_lowercase())
    }
}

/// Resolve globs to concrete files under `root` (recursive, ignore-aware).
/// Handles: direct file path, existing dir (search inside it), and
/// filename / relative-path glob patterns. Binary exts are skipped here;
/// size / UTF-8 checks happen at read time.
fn resolve_qur_files(root: &Path, globs: &[String], max_depth: usize) -> Vec<PathBuf> {
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();
    let mut out: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();

    for glob in globs {
        // Direct file hit (e.g. `qur file.txt`).
        let direct = root.join(glob);
        if direct.is_file() {
            if !is_qur_skipped_ext(&direct) && seen.insert(direct.clone()) {
                out.push(direct);
            }
            continue;
        }
        // Existing dir → search inside it with a catch-all.
        let (walk_root, eff_glob) = if direct.is_dir() {
            (direct, "*".to_string())
        } else {
            (root.to_path_buf(), glob.clone())
        };
        for (_name, path) in search::walk_files(&walk_root, max_depth, &ignored_dirs, &fmt_cfg) {
            if is_qur_skipped_ext(&path) {
                continue;
            }
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            let rel = path
                .strip_prefix(&walk_root)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| file_name.clone());
            if file_matches_glob(&file_name, &rel, &eff_glob) && seen.insert(path.clone()) {
                out.push(path);
            }
        }
    }

    out.sort();
    out
}

/// Count literal (case-insensitive) occurrences of `pat` in `line`.
pub(crate) fn count_occurrences(line: &str, pat_lower: &str, pat_len: usize) -> usize {
    if pat_lower.is_empty() {
        return 0;
    }
    let lower = line.to_lowercase();
    let mut count = 0;
    let mut start = 0;
    while start + pat_len <= lower.len() {
        match lower[start..].find(pat_lower) {
            Some(off) => {
                count += 1;
                start += off + pat_len.max(1);
            }
            None => break,
        }
    }
    count
}

/// Replace all literal (case-insensitive) occurrences of `pat` with `rep`.
/// Returns `(new_line, replacements)`.
pub(crate) fn replace_line_case_insensitive(line: &str, pat: &str, rep: &str) -> (String, usize) {
    if pat.is_empty() {
        return (line.to_string(), 0);
    }
    let pat_lower = pat.to_lowercase();
    let pat_len = pat.len();
    let lower = line.to_lowercase();
    let mut out = String::with_capacity(line.len() + 16);
    let mut count = 0;
    let mut i = 0;
    while i < line.len() {
        if i + pat_len <= lower.len() && lower[i..].starts_with(&pat_lower) {
            // Char-boundary safety for non-ASCII text.
            let mut end = (i + pat_len).min(line.len());
            while end < line.len() && !line.is_char_boundary(end) {
                end += 1;
            }
            let mut start = i;
            while start < end && !line.is_char_boundary(start) {
                start += 1;
            }
            out.push_str(rep);
            count += 1;
            i = end;
        } else {
            let ch = line[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    (out, count)
}

fn print_qur_usage() {
    print_error(&t!("misc.qur.usage"));
    println!("  {}", t!("misc.qur.example1"));
    println!("  {}", t!("misc.qur.example2"));
    println!("  {}", t!("misc.qur.example3"));
    println!("  {}", t!("misc.qur.example4"));
}

fn file_size_label(path: &Path) -> String {
    if let Ok(md) = std::fs::metadata(path) {
        let size = md.len();
        if size < 1024 {
            format!("({} B)", size)
        } else if size < 1024 * 1024 {
            format!("({:.1} KB)", size as f64 / 1024.0)
        } else {
            format!("({:.1} MB)", size as f64 / (1024.0 * 1024.0))
        }
    } else {
        String::new()
    }
}

pub fn cmd_qur(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.trim().is_empty() {
        print_qur_usage();
        return Ok(false);
    }
    let parsed = match parse_qur_args(args) {
        Ok(p) => p,
        Err(e) => {
            match e.as_str() {
                "replace_needs_select" => print_error(&t!("misc.qur.replace_needs_select")),
                "display_needs_select" => print_error(&t!("misc.qur.display_needs_select")),
                "empty_select" => print_error(&t!("misc.qur.empty_select")),
                "empty_replace" => print_error(&t!("misc.qur.empty_replace")),
                _ => print_qur_usage(),
            }
            return Ok(false);
        }
    };
    if parsed.help {
        print_qur_usage();
        return Ok(false);
    }

    let root = nav.current_path().to_path_buf();
    let files = run_with_spinner(&t!("misc.qur.searching"), || {
        resolve_qur_files(&root, &parsed.globs, parsed.max_depth)
    });

    if files.is_empty() {
        print_warning(&t!("misc.qur.no_files", parsed.globs.join(", ")));
        return Ok(false);
    }

    // ── bare mode: list files, or cat a single direct file ──
    if parsed.select.is_none() {
        if files.len() == 1
            && parsed.globs.len() == 1
            && !search::has_wildcards(&parsed.globs[0])
            && root.join(&parsed.globs[0]).is_file()
        {
            let show_lines = Config::global_get_show_line_numbers();
            crate::output::cat_file_with_syntax(&files[0], show_lines)?;
            return Ok(false);
        }
        println!();
        println!("{}", t!("misc.qur.files_header", files.len()).cyan().bold());
        for (i, f) in files.iter().enumerate() {
            let rel = search::display_content_path(f, &root);
            println!(
                "  {}. {} {}",
                (i + 1).to_string().yellow(),
                rel,
                file_size_label(f).dimmed()
            );
        }
        println!();
        return Ok(false);
    }

    // ── select mode ──
    let pattern = parsed.select.clone().unwrap();
    if pattern.trim().is_empty() {
        print_error(&t!("misc.qur.empty_select"));
        return Ok(false);
    }
    let matches = run_with_spinner(&t!("misc.qur.searching"), || {
        search::search_content_in_files(&files, &pattern)
    });
    if matches.is_empty() {
        print_warning(&t!("misc.qur.no_matches", pattern));
        return Ok(false);
    }
    let total_found = matches.len();

    // ── replace mode: preview → confirm → write ──
    if let Some(ref replacement) = parsed.replace {
        // Per-file occurrence counts (match lines may hold several hits).
        let pat_lower = pattern.to_lowercase();
        let pat_len = pattern.len();
        let mut per_file: Vec<(PathBuf, usize)> = Vec::new();
        let mut total_occ = 0usize;
        {
            use std::collections::BTreeMap;
            let mut map: BTreeMap<PathBuf, usize> = BTreeMap::new();
            for m in &matches {
                *map.entry(m.file_path.clone()).or_insert(0) +=
                    count_occurrences(&m.line, &pat_lower, pat_len).max(1);
            }
            for (p, c) in map {
                total_occ += c;
                per_file.push((p, c));
            }
        }
        println!();
        println!(
            "{}",
            t!(
                "misc.qur.replace_preview",
                pattern,
                replacement,
                total_occ,
                per_file.len()
            )
            .cyan()
            .bold()
        );
        let preview_n = matches.len().min(20);
        for (i, m) in matches.iter().take(preview_n).enumerate() {
            let rel = search::display_content_path(&m.file_path, &root);
            let highlighted = search::highlight_pattern(m.line.trim(), &pattern);
            println!("  {}. {}:{}: {}", i + 1, rel, m.line_number, highlighted);
        }
        if matches.len() > preview_n {
            println!("  ... ({} more)", matches.len() - preview_n);
        }
        println!();
        for (p, c) in &per_file {
            println!(
                "  {} — {} occurrence{}",
                search::display_content_path(p, &root),
                c,
                if *c == 1 { "" } else { "s" }
            );
        }
        println!();
        print!(
            "{} ",
            t!("misc.qur.confirm", total_occ, per_file.len()).yellow()
        );
        let _ = io::stdout().flush();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        let answer = answer.trim().to_lowercase();
        if answer != "y" && answer != "yes" {
            print_info(&t!("misc.qur.cancelled"));
            return Ok(false);
        }
        let mut files_written = 0usize;
        let mut occ_written = 0usize;
        let mut skipped = 0usize;
        for (path, _) in &per_file {
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => {
                    skipped += 1;
                    continue;
                }
            };
            let mut new_lines: Vec<String> = Vec::new();
            let mut file_occ = 0usize;
            for line in content.lines() {
                let (nl, c) = replace_line_case_insensitive(line, &pattern, replacement);
                file_occ += c;
                new_lines.push(nl);
            }
            let mut new_content = new_lines.join("\n");
            if content.ends_with('\n') {
                new_content.push('\n');
            }
            match std::fs::write(path, new_content) {
                Ok(_) => {
                    files_written += 1;
                    occ_written += file_occ;
                }
                Err(e) => {
                    print_warning(&format!("{}: {}", path.display(), e));
                    skipped += 1;
                }
            }
        }
        print_success(&t!("misc.qur.replaced", occ_written, files_written));
        if skipped > 0 {
            print_warning(&t!("misc.qur.skipped", skipped));
        }
        return Ok(false);
    }

    // ── display mode (-dp): gs-style detailed lines ──
    if parsed.display {
        let mut shown = matches;
        if shown.len() > 20_000 {
            print_warning(&t!("misc.qur.many_results", shown.len()));
            shown.truncate(20_000);
        }
        if let Some(lim) = parsed.limit {
            if shown.len() > lim {
                shown.truncate(lim);
            }
        }
        let output =
            search::format_content_results(&shown, &pattern, parsed.max_depth, &root, total_found);
        print!("{}", output);
        return Ok(false);
    }

    // ── select summary mode (-s without -dp/-rp) ──
    {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<PathBuf, usize> = BTreeMap::new();
        for m in &matches {
            *map.entry(m.file_path.clone()).or_insert(0) += 1;
        }
        let mut entries: Vec<(PathBuf, usize)> = map.into_iter().collect();
        entries.sort();
        if let Some(lim) = parsed.limit {
            if entries.len() > lim {
                entries.truncate(lim);
            }
        }
        println!();
        println!(
            "{}",
            t!(
                "misc.qur.summary_header",
                pattern,
                total_found,
                entries.len()
            )
            .cyan()
            .bold()
        );
        for (i, (p, c)) in entries.iter().enumerate() {
            println!(
                "  {}. {} ({} match{})",
                (i + 1).to_string().yellow(),
                search::display_content_path(p, &root),
                c,
                if *c == 1 { "" } else { "es" }
            );
        }
        if total_found > matches.len() {
            println!("  ... ({} total match lines)", total_found);
        }
        println!();
        println!("{}", t!("misc.qur.summary_hint").dimmed());
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bare_glob() {
        let a = parse_qur_args("*.*").unwrap();
        assert_eq!(a.globs, vec!["*.*".to_string()]);
        assert!(a.select.is_none());
        assert!(!a.display);
    }

    #[test]
    fn parse_select_display_replace() {
        let a = parse_qur_args("*.* -s \"foo bar\" -dp").unwrap();
        assert_eq!(a.select.as_deref(), Some("foo bar"));
        assert!(a.display);
        let b = parse_qur_args("*.rs -s old -rp new -d 3").unwrap();
        assert_eq!(b.select.as_deref(), Some("old"));
        assert_eq!(b.replace.as_deref(), Some("new"));
        assert_eq!(b.max_depth, 3);
    }

    #[test]
    fn parse_rejects_replace_without_select() {
        assert!(parse_qur_args("*.* -rp new").is_err());
        assert!(parse_qur_args("*.* -dp").is_err());
        assert!(parse_qur_args("").is_err());
    }

    #[test]
    fn glob_matches_wildcard_and_substring() {
        assert!(file_matches_glob("main.rs", "main.rs", "*.rs"));
        assert!(file_matches_glob("main.rs", "main.rs", "*.*"));
        assert!(!file_matches_glob("main.rs", "main.rs", "*.md"));
        assert!(file_matches_glob("main.rs", "src/main.rs", "src/*.rs"));
        assert!(file_matches_glob("README.md", "README.md", "read"));
    }

    #[test]
    fn replace_is_case_insensitive() {
        let (s, c) = replace_line_case_insensitive("Foo foo FOO", "foo", "bar");
        assert_eq!(s, "bar bar bar");
        assert_eq!(c, 3);
        let (s2, c2) = replace_line_case_insensitive("no hit", "zzz", "bar");
        assert_eq!(s2, "no hit");
        assert_eq!(c2, 0);
    }

    #[test]
    fn resolve_and_search_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.rs"), "hello foo\nsecond\n").unwrap();
        std::fs::write(dir.path().join("b.md"), "nothing here\n").unwrap();
        std::fs::write(dir.path().join("c.exe"), "foo binary\n").unwrap();
        let files = resolve_qur_files(dir.path(), &["*.*".to_string()], 0);
        let names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert!(names.contains(&"a.rs".to_string()));
        assert!(names.contains(&"b.md".to_string()));
        assert!(!names.contains(&"c.exe".to_string()));
        let hits = search::search_content_in_files(&files, "foo");
        assert_eq!(hits.len(), 1);
        assert!(hits[0].file_path.ends_with("a.rs"));
    }
}
