use crate::config::Config;
use crate::filetype::{is_supported_format_with_config, make_ignored_dir_filter, FormatConfig};
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use anyhow::Result;
use colored::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn cmd_tl(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.trim().splitn(2, ' ').collect();
    let sub = parts[0];

    match sub {
        "" => {
            let _ = print_usage();
            Ok(false)
        }
        "file" => cmd_tl_file(parts.get(1).unwrap_or(&""), nav),
        "ext" => cmd_tl_ext(parts.get(1).unwrap_or(&""), nav),
        "dir" => cmd_tl_dir(parts.get(1).unwrap_or(&""), nav),
        _ => {
            print_error(&t!("cmd.tl.unknown_sub", sub));
            let _ = print_usage();
            Ok(false)
        }
    }
}

fn print_usage() -> Result<bool> {
    println!("{}", t!("cmd.tl.usage_title"));
    println!("  {}", t!("cmd.tl.help_file"));
    println!("  {}", t!("cmd.tl.help_ext"));
    println!("  {}", t!("cmd.tl.help_dir"));
    println!("  {}", t!("cmd.tl.help_sort"));
    Ok(false)
}

fn resolve_path(path_str: &str, nav: &Navigator) -> PathBuf {
    let p = Path::new(path_str);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        nav.current_path().join(p)
    }
}

fn count_lines(path: &Path) -> u64 {
    match fs::read_to_string(path) {
        Ok(s) => s.lines().count() as u64,
        Err(_) => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TlSort {
    Normal,
    LineAsc,
    LineDesc,
}

/// Parse `tl ext|dir <target> -s <-n|-l|-r>`.
/// Returns `Ok((target, None))` when no sort flags are given, otherwise the
/// chosen sort mode. Errors are localized and ready to print.
fn parse_tl_args(rest: &str) -> Result<(String, Option<TlSort>)> {
    let tokens: Vec<&str> = rest.split_whitespace().collect();

    let mut has_s = false;
    let mut mode: Option<TlSort> = None;
    let mut end = tokens.len();
    while end > 0 {
        let tok = tokens[end - 1];
        match tok {
            "-s" => {
                has_s = true;
                end -= 1;
            }
            "-n" => {
                mode = Some(TlSort::Normal);
                end -= 1;
            }
            "-l" => {
                mode = Some(TlSort::LineAsc);
                end -= 1;
            }
            "-r" => {
                mode = Some(TlSort::LineDesc);
                end -= 1;
            }
            _ => break,
        }
    }

    let target = tokens[..end].join(" ");

    if has_s && mode.is_none() {
        return Err(anyhow::anyhow!("{}", t!("cmd.tl.sort_usage")));
    }
    if !has_s && mode.is_some() {
        return Err(anyhow::anyhow!("{}", t!("cmd.tl.sort_usage")));
    }

    if target.is_empty() {
        return Ok((target, None));
    }

    Ok((target, mode))
}

/// Print a row (with optional reverse highlight) in the localized line format.
fn print_highlight_line(line: String, highlight: bool) {
    if highlight {
        println!("{}", line.reversed().bold());
    } else {
        println!("{}", line);
    }
}

fn cmd_tl_file(path_str: &str, nav: &Navigator) -> Result<bool> {
    if path_str.is_empty() {
        print_error(&t!("cmd.tl.file_usage"));
        return Ok(false);
    }
    let path = resolve_path(path_str, nav);

    if !path.exists() {
        print_error(&t!("cmd.tl.file_not_found", path_str));
        return Ok(false);
    }
    if !path.is_file() {
        print_error(&t!("cmd.tl.not_a_file", path_str));
        return Ok(false);
    }

    let lines = count_lines(&path);
    println!("{}  {}", path_str, lines);
    Ok(false)
}

fn cmd_tl_ext(rest: &str, nav: &Navigator) -> Result<bool> {
    let (ext_arg, sort) = match parse_tl_args(rest) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e.to_string());
            let _ = print_usage();
            return Ok(false);
        }
    };

    if ext_arg.is_empty() {
        print_error(&t!("cmd.tl.ext_usage"));
        return Ok(false);
    }

    let raw_ext = ext_arg.trim_start_matches('.');
    let root = nav.current_path().to_path_buf();
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();

    let mut total_lines = 0u64;
    let mut count = 0u64;
    let mut files_found = false;
    let mut entries: Vec<(String, u64)> = Vec::new();

    let walker = WalkDir::new(&root)
        .into_iter()
        .filter_entry(|e| (make_ignored_dir_filter(&ignored_dirs))(e));

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(e) = path.extension().and_then(|e| e.to_str()) {
                if e.eq_ignore_ascii_case(raw_ext) {
                    let lines = count_lines(path);
                    total_lines += lines;
                    count += 1;
                    files_found = true;
                    let rel = match path.strip_prefix(&root) {
                        Ok(p) => p.to_string_lossy().to_string(),
                        Err(_) => path.display().to_string(),
                    };
                    entries.push((rel, lines));
                }
            }
        }
    }

    let max = entries.iter().map(|(_, l)| *l).max().unwrap_or(0);

    match sort {
        Some(TlSort::LineAsc) => entries.sort_by_key(|(_, l)| *l),
        Some(TlSort::LineDesc) => entries.sort_by_key(|(_, l)| std::cmp::Reverse(*l)),
        _ => {}
    }

    for (rel, lines) in &entries {
        print_highlight_line(format!("{}  {}", rel, lines), *lines == max && max > 0);
    }

    if files_found {
        println!("{}", "─".repeat(40));
        println!("{}", t!("cmd.tl.total_files_lines", count, total_lines));
        if max > 0 {
            let highest = t!(
                "cmd.tl.highest",
                entries
                    .iter()
                    .find(|(_, l)| *l == max)
                    .map(|(r, _)| r)
                    .unwrap_or(&String::new()),
                max
            );
            print_highlight_line(highest, true);
        }
    } else {
        let test_name = format!("dummy.{}", raw_ext);
        let test_path = Path::new(&test_name);
        if !is_supported_format_with_config(test_path, &fmt_cfg) {
            println!("{}", t!("cmd.tl.ext_not_supported", raw_ext));
        } else {
            println!("{}", t!("cmd.tl.no_ext_files", raw_ext));
        }
    }

    Ok(false)
}

fn cmd_tl_dir(rest: &str, nav: &Navigator) -> Result<bool> {
    let (dir_str, sort) = match parse_tl_args(rest) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e.to_string());
            let _ = print_usage();
            return Ok(false);
        }
    };

    if dir_str.is_empty() {
        print_error(&t!("cmd.tl.dir_usage"));
        return Ok(false);
    }

    let dir = resolve_path(&dir_str, nav);

    if !dir.exists() || !dir.is_dir() {
        print_error(&t!("cmd.tl.dir_not_found", dir_str));
        return Ok(false);
    }

    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();

    let mut ext_map: BTreeMap<String, (u64, u64)> = BTreeMap::new();

    let walker = WalkDir::new(&dir)
        .into_iter()
        .filter_entry(|e| (make_ignored_dir_filter(&ignored_dirs))(e));

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if is_supported_format_with_config(path, &fmt_cfg) {
                let lines = count_lines(path);
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("(no ext)")
                    .to_lowercase();
                let entry = ext_map.entry(ext).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += lines;
            }
        }
    }

    let mut rows: Vec<(String, u64, u64)> = ext_map
        .into_iter()
        .map(|(ext, (count, lines))| (ext, count, lines))
        .collect();

    let max = rows.iter().map(|(_, _, l)| *l).max().unwrap_or(0);

    match sort {
        Some(TlSort::LineAsc) => rows.sort_by_key(|(_, _, l)| *l),
        Some(TlSort::LineDesc) => rows.sort_by_key(|(_, _, l)| std::cmp::Reverse(*l)),
        _ => {}
    }

    let mut total_files = 0u64;
    let mut total_lines = 0u64;
    for (ext, count, lines) in &rows {
        print_highlight_line(
            format!(".{:<10} {:>5} files, {:>7} lines", ext, count, lines),
            *lines == max && max > 0,
        );
        total_files += count;
        total_lines += lines;
    }

    if total_files > 0 {
        println!("{}", "─".repeat(40));
        println!(
            "{}",
            t!("cmd.tl.total_files_lines", total_files, total_lines)
        );
        if max > 0 {
            let ext = rows
                .iter()
                .find(|(_, _, l)| *l == max)
                .map(|(e, _, _)| e)
                .cloned()
                .unwrap_or_default();
            let highest = t!("cmd.tl.highest", format!(".{}", ext), max);
            print_highlight_line(highest, true);
        }
    } else {
        println!("{}", t!("cmd.tl.no_supported_files", dir_str));
    }

    Ok(false)
}
