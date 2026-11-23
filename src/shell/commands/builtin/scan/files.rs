//! `scan` filesystem scans: extensions, files, and directories.

use crate::config::Config;
use crate::explorer::human_readable_size;
use crate::filetype::{
    format_ext_list, make_ignored_dir_filter, matches_any_extension, split_extensions,
};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use walkdir::WalkDir;

pub(super) struct ScanArgs {
    pub(super) exts: Vec<String>,
    pub(super) max_depth: Option<usize>,
    pub(super) show_size: bool,
    pub(super) care: bool,
}

/// Parse `scan ext` arguments: bare tokens are extensions, flags may appear
/// in any order (`scan ext rs c -d 3 -s --care`). Returns an error string key-ready
/// message on failure.
pub(super) fn parse_scan_args(tokens: &[String]) -> Result<ScanArgs, String> {
    let mut exts: Vec<String> = Vec::new();
    let mut max_depth: Option<usize> = None;
    let mut show_size = false;
    let mut care = false;
    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens[i].as_str();
        match tok.to_lowercase().as_str() {
            "-d" | "--depth" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.depth_usage"))?;
                match val.parse::<usize>() {
                    Ok(n) => max_depth = Some(n),
                    Err(_) => return Err(t!("cmd.scan.invalid_depth", val)),
                }
                i += 2;
            }
            "-s" | "--size" => {
                show_size = true;
                i += 1;
            }
            "--care" => {
                care = true;
                i += 1;
            }
            _ => {
                if tok.starts_with('-') {
                    return Err(t!("cmd.scan.unknown_option", tok));
                }
                for e in split_extensions(tok) {
                    if !exts.contains(&e) {
                        exts.push(e);
                    }
                }
                i += 1;
            }
        }
    }
    if exts.is_empty() {
        return Err(t!("cmd.scan.ext_usage"));
    }
    Ok(ScanArgs {
        exts,
        max_depth,
        show_size,
        care,
    })
}

/// `*`-wildcard match on a file/dir name, case-insensitive.
/// Only `*` is special (matches any run, including empty); every other
/// character is literal (`?`, `[`… stay literal).
/// No `*` at all means an exact (case-insensitive) match.
pub(crate) fn matches_wildcard(name: &str, pattern: &str) -> bool {
    if !pattern.contains('*') {
        return name.eq_ignore_ascii_case(pattern);
    }
    let name_lower = name.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    let parts: Vec<&str> = pattern_lower.split('*').collect();
    // First segment anchors the start (empty = no anchor).
    if !parts.is_empty() && !parts[0].is_empty() && !name_lower.starts_with(parts[0]) {
        return false;
    }
    // Last segment anchors the end.
    if parts.len() > 1 {
        let last = parts[parts.len() - 1];
        if !last.is_empty() && !name_lower.ends_with(last) {
            return false;
        }
    }
    // Middle segments must appear in order.
    let mut rest = name_lower.as_str();
    // Skip past the anchored prefix for middle matching.
    if !parts.is_empty() && !parts[0].is_empty() {
        rest = &rest[parts[0].len()..];
    }
    // Cut off the anchored suffix.
    if parts.len() > 1 {
        let last = parts[parts.len() - 1];
        if !last.is_empty() {
            rest = &rest[..rest.len() - last.len()];
        }
    }
    for part in parts.iter().skip(1).take(parts.len().saturating_sub(2)) {
        if part.is_empty() {
            continue;
        }
        match rest.find(*part) {
            Some(pos) => rest = &rest[pos + part.len()..],
            None => return false,
        }
    }
    true
}

pub(super) struct ScanNameArgs {
    /// None = match everything (`scan files` bare, `*`, or `*.*` for files).
    pub(super) pattern: Option<String>,
    pub(super) max_depth: Option<usize>,
    pub(super) show_size: bool,
    pub(super) care: bool,
}

/// Parse `scan files` / `scan dir` arguments: at most one bare pattern
/// plus `-d|--depth`, `-s|--size`, `--care` in any order.
/// `usage_key` selects the usage message; `for_dir` rejects `*.*`.
pub(super) fn parse_scan_name_args(
    tokens: &[String],
    usage_key: &str,
    for_dir: bool,
) -> Result<ScanNameArgs, String> {
    let mut pattern: Option<String> = None;
    let mut max_depth: Option<usize> = None;
    let mut show_size = false;
    let mut care = false;
    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens[i].as_str();
        match tok.to_lowercase().as_str() {
            "-d" | "--depth" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.depth_usage"))?;
                match val.parse::<usize>() {
                    Ok(n) => max_depth = Some(n),
                    Err(_) => return Err(t!("cmd.scan.invalid_depth", val)),
                }
                i += 2;
            }
            "-s" | "--size" => {
                show_size = true;
                i += 1;
            }
            "--care" => {
                care = true;
                i += 1;
            }
            _ => {
                if tok.starts_with('-') {
                    return Err(t!("cmd.scan.unknown_option", tok));
                }
                if pattern.is_some() {
                    return Err(t!("cmd.scan.unexpected_arg", tok));
                }
                pattern = Some(tok.to_string());
                i += 1;
            }
        }
    }
    if let Some(ref p) = pattern {
        if p == "*.*" {
            if for_dir {
                return Err(t!(usage_key));
            }
            pattern = None;
        } else if p == "*" {
            pattern = None;
        }
    }
    Ok(ScanNameArgs {
        pattern,
        max_depth,
        show_size,
        care,
    })
}

pub(super) fn cmd_scan_ext(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_scan_args(tokens) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };

    let root = nav.current_path().to_path_buf();
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let max_depth = parsed.max_depth.unwrap_or(Config::global_get_max_depth());

    let mut entries: Vec<(String, u64)> = Vec::new();
    if parsed.care {
        // Care mode: include everything, skip ignore filter
        // (same as `view --care` / `size --care`).
        let walker = WalkDir::new(&root).max_depth(max_depth).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches_any_extension(&name, &parsed.exts) {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(&root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            entries.push((rel, size));
        }
    } else {
        let walker = WalkDir::new(&root)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| (make_ignored_dir_filter(&ignored_dirs))(e));
        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches_any_extension(&name, &parsed.exts) {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(&root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            entries.push((rel, size));
        }
    }
    entries.sort_by_key(|(rel, _)| rel.to_lowercase());

    if entries.is_empty() {
        print_info(&t!("cmd.scan.no_files", format_ext_list(&parsed.exts)));
        return Ok(false);
    }

    println!(
        "{}",
        t!("cmd.scan.list_title", format_ext_list(&parsed.exts))
            .cyan()
            .bold()
    );
    if parsed.show_size {
        for (rel, size) in &entries {
            println!("  {} ({})", rel, human_readable_size(*size));
        }
        let total: u64 = entries.iter().map(|(_, s)| s).sum();
        println!("{}", "─".repeat(40));
        print_success(&t!(
            "cmd.scan.total_files_size",
            entries.len(),
            human_readable_size(total)
        ));
    } else {
        for (rel, _) in &entries {
            println!("  {}", rel);
        }
        println!("{}", "─".repeat(40));
        print_success(&t!("cmd.scan.total_files", entries.len()));
    }
    Ok(false)
}

/// Walk `root` collecting entries whose file/dir name matches `matches`.
/// For directories, sizes aggregate descendant file bytes gathered in the
/// same walk (no extra I/O). Returns `(relative path, size)` sorted
/// case-insensitively. The root itself is never listed.
pub(super) fn walk_matching(
    root: &std::path::Path,
    max_depth: usize,
    care: bool,
    want_dir: bool,
    matches: &dyn Fn(&str) -> bool,
) -> Vec<(String, u64)> {
    let mut files: Vec<(String, u64)> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();
    if care {
        for entry in WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path() == root {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches(&name) {
                continue;
            }
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            if want_dir {
                if entry.file_type().is_dir() {
                    dirs.push(rel);
                }
            } else if entry.file_type().is_file() {
                let size = std::fs::metadata(entry.path())
                    .map(|m| m.len())
                    .unwrap_or(0);
                files.push((rel, size));
            }
        }
    } else {
        let ignored_dirs = Config::get_ignored_dirs_combined();
        let walker = WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| (make_ignored_dir_filter(&ignored_dirs))(e));
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.path() == root {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches(&name) {
                continue;
            }
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            if want_dir {
                if entry.file_type().is_dir() {
                    dirs.push(rel);
                }
            } else if entry.file_type().is_file() {
                let size = std::fs::metadata(entry.path())
                    .map(|m| m.len())
                    .unwrap_or(0);
                files.push((rel, size));
            }
        }
    }
    if !want_dir {
        files.sort_by_key(|(rel, _)| rel.to_lowercase());
        return files;
    }
    // Aggregate descendant file bytes per directory. Every walked file
    // contributes to each ancestor dir (single walk, no extra I/O).
    // Aggregate over ALL files (not just name-matched ones) so dir
    // sizes stay truthful regardless of the name pattern.
    let mut dir_sizes: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    let all_files = walk_all_files(root, max_depth, care);
    for (rel, size) in &all_files {
        let mut parent = rel.as_str();
        while let Some(pos) = parent.rfind('/') {
            parent = &parent[..pos];
            *dir_sizes.entry(parent.to_string()).or_insert(0) += size;
        }
    }
    let mut out: Vec<(String, u64)> = dirs
        .into_iter()
        .map(|rel| {
            let size = dir_sizes.get(&rel).copied().unwrap_or(0);
            (rel, size)
        })
        .collect();
    out.sort_by_key(|(rel, _)| rel.to_lowercase());
    out
}

/// All files under `root` (name-unfiltered) for dir-size aggregation.
pub(super) fn walk_all_files(
    root: &std::path::Path,
    max_depth: usize,
    care: bool,
) -> Vec<(String, u64)> {
    let mut files: Vec<(String, u64)> = Vec::new();
    if care {
        for entry in WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            files.push((rel, size));
        }
    } else {
        let ignored_dirs = Config::get_ignored_dirs_combined();
        let walker = WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| (make_ignored_dir_filter(&ignored_dirs))(e));
        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            files.push((rel, size));
        }
    }
    files
}

/// `scan files [pattern] [-d|--depth <n>] [-s|--size] [--care]`.
/// No pattern (or `*` / `*.*`) lists every file.
pub(super) fn cmd_scan_files(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_scan_name_args(tokens, "cmd.scan.files_usage", false) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let label = parsed.pattern.clone().unwrap_or_else(|| "*".to_string());
    let matches = |name: &str| match parsed.pattern.as_deref() {
        None => true,
        Some(p) => matches_wildcard(name, p),
    };

    let root = nav.current_path().to_path_buf();
    let max_depth = parsed.max_depth.unwrap_or(Config::global_get_max_depth());
    let entries = walk_matching(&root, max_depth, parsed.care, false, &matches);

    if entries.is_empty() {
        print_info(&t!("cmd.scan.no_files", label));
        return Ok(false);
    }

    println!("{}", t!("cmd.scan.list_title", label).cyan().bold());
    if parsed.show_size {
        for (rel, size) in &entries {
            println!("  {} ({})", rel, human_readable_size(*size));
        }
        let total: u64 = entries.iter().map(|(_, s)| s).sum();
        println!("{}", "─".repeat(40));
        print_success(&t!(
            "cmd.scan.total_files_size",
            entries.len(),
            human_readable_size(total)
        ));
    } else {
        for (rel, _) in &entries {
            println!("  {}", rel);
        }
        println!("{}", "─".repeat(40));
        print_success(&t!("cmd.scan.total_files", entries.len()));
    }
    Ok(false)
}

/// `scan dir [pattern] [-d|--depth <n>] [-s|--size] [--care]`.
/// No pattern (or `*`) lists every directory; `*.*` is rejected.
pub(super) fn cmd_scan_dir(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_scan_name_args(tokens, "cmd.scan.dir_usage", true) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let label = parsed.pattern.clone().unwrap_or_else(|| "*".to_string());
    let matches = |name: &str| match parsed.pattern.as_deref() {
        None => true,
        Some(p) => matches_wildcard(name, p),
    };

    let root = nav.current_path().to_path_buf();
    let max_depth = parsed.max_depth.unwrap_or(Config::global_get_max_depth());
    let entries = walk_matching(&root, max_depth, parsed.care, true, &matches);

    if entries.is_empty() {
        print_info(&t!("cmd.scan.no_dirs", label));
        return Ok(false);
    }

    println!("{}", t!("cmd.scan.list_title", label).cyan().bold());
    if parsed.show_size {
        for (rel, size) in &entries {
            println!("  {} ({})", rel, human_readable_size(*size));
        }
        let total: u64 = entries.iter().map(|(_, s)| s).sum();
        println!("{}", "─".repeat(40));
        print_success(&t!(
            "cmd.scan.total_dirs_size",
            entries.len(),
            human_readable_size(total)
        ));
    } else {
        for (rel, _) in &entries {
            println!("  {}", rel);
        }
        println!("{}", "─".repeat(40));
        print_success(&t!("cmd.scan.total_dirs", entries.len()));
    }
    Ok(false)
}
