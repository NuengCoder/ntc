//! `scan` command: file listing plus local network discovery.
//!
//! - `scan ext <ext...> [-d|--depth <n>] [-s|--size] [--care]` walks the current
//!   directory (respecting ignore/care, like `tl`) and prints matching files.
//!   Without `-s` only relative paths are shown; with `-s` each row also shows
//!   a human-readable size plus a total footer. With `--care` all ignore rules
//!   are skipped (like `view --care` / `size --care`).
//! - `scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]`
//!   TCP-connect port scan (default host 127.0.0.1, default top 20).
//! - `scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]`
//!   live-host sweep over a /24 (auto-detected) or explicit CIDR,
//!   unioned with the OS ARP table. Best effort, no privileges needed.

use crate::config::Config;
use crate::explorer::human_readable_size;
use crate::filetype::{
    format_ext_list, make_ignored_dir_filter, matches_any_extension, split_extensions,
};
use crate::navigator::Navigator;
use crate::nssh::scan as portscan;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::Result;
use colored::*;
use std::time::Duration;
use walkdir::WalkDir;

fn tokenize(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

struct ScanArgs {
    exts: Vec<String>,
    max_depth: Option<usize>,
    show_size: bool,
    care: bool,
}

/// Parse `scan ext` arguments: bare tokens are extensions, flags may appear
/// in any order (`scan ext rs c -d 3 -s --care`). Returns an error string key-ready
/// message on failure.
fn parse_scan_args(tokens: &[String]) -> Result<ScanArgs, String> {
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

struct ScanNameArgs {
    /// None = match everything (`scan files` bare, `*`, or `*.*` for files).
    pattern: Option<String>,
    max_depth: Option<usize>,
    show_size: bool,
    care: bool,
}

/// Parse `scan files` / `scan dir` arguments: at most one bare pattern
/// plus `-d|--depth`, `-s|--size`, `--care` in any order.
/// `usage_key` selects the usage message; `for_dir` rejects `*.*`.
fn parse_scan_name_args(
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
fn print_usage() {
    println!("{}", t!("cmd.scan.usage_title").cyan().bold());
    println!("  {}", t!("cmd.scan.help_ext"));
    println!("  {}", t!("cmd.scan.help_files"));
    println!("  {}", t!("cmd.scan.help_dir"));
    println!("  {}", t!("cmd.scan.help_port"));
    println!("  {}", t!("cmd.scan.help_ip"));
    println!("  {}", t!("cmd.scan.help_waep"));
    println!();
    println!("{}", t!("cmd.scan.examples_title").green());
    println!("    scan ext rs");
    println!("    scan ext rs c -d 2");
    println!("    scan ext rs c -s");
    println!("    scan ext rs --care");
    println!("    scan files");
    println!("    scan files *.rs -s");
    println!("    scan files k*.rs --care");
    println!("    scan dir");
    println!("    scan dir k* -d 2");
    println!("    scan port");
    println!("    scan port 192.168.1.1 --top 10");
    println!("    scan port 192.168.1.1 --all");
    println!("    scan ip");
    println!("    scan ip 192.168.1.0/24 --timeout 300");
    println!("    scan waep example.com");
    println!("    scan waep example.com/api -r -d 1");
    println!("    scan waep 192.168.1.5:8080 -x php,html --all");
}

pub fn cmd_scan(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_usage();
        return Ok(false);
    }
    let sub = tokens[0].to_lowercase();
    if sub == "help" || sub == "-h" || sub == "--help" {
        print_usage();
        return Ok(false);
    }
    match sub.as_str() {
        "ext" => cmd_scan_ext(&tokens[1..], nav),
        "files" => cmd_scan_files(&tokens[1..], nav),
        "dir" => cmd_scan_dir(&tokens[1..], nav),
        "port" => cmd_scan_port(&tokens[1..]),
        "ip" => cmd_scan_ip(&tokens[1..]),
        "waep" | "web" => cmd_scan_waep(&tokens[1..]),
        _ => {
            print_error(&t!("cmd.scan.unknown_sub", tokens[0]));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

fn cmd_scan_ext(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
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
fn walk_matching(
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
fn walk_all_files(root: &std::path::Path, max_depth: usize, care: bool) -> Vec<(String, u64)> {
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
fn cmd_scan_files(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
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
fn cmd_scan_dir(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
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

/// Parse the `--top N | --ports <spec> | --all` + `--timeout MS` flags
/// shared by `scan port` and `scan ip`. Returns the port list, the
/// timeout, and whether `--timeout` was passed explicitly (huge sweeps
/// without one get the fast default instead of the interactive one).
fn parse_net_flags(
    tokens: &[String],
    default_ports: &[u16],
    default_timeout_ms: u64,
    allow_all: bool,
) -> Result<(Vec<u16>, Duration, bool), String> {
    let mut ports_spec: Option<String> = None;
    let mut top: Option<usize> = None;
    let mut all = false;
    let mut timeout_ms = default_timeout_ms;
    let mut timeout_given = false;
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--ports" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.missing_ports"))?;
                ports_spec = Some(v.clone());
                i += 2;
            }
            "--top" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.missing_top"))?;
                top = Some(v.parse().map_err(|_| t!("cmd.scan.invalid_top", v))?);
                i += 2;
            }
            "--all" => {
                if !allow_all {
                    return Err(t!("cmd.scan.unknown_option", "--all"));
                }
                all = true;
                i += 1;
            }
            "--timeout" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.missing_timeout"))?;
                timeout_ms = v.parse().map_err(|_| t!("cmd.scan.invalid_timeout", v))?;
                timeout_given = true;
                i += 2;
            }
            other => {
                if other.starts_with('-') {
                    return Err(t!("cmd.scan.unknown_option", other));
                }
                return Err(t!("cmd.scan.unexpected_arg", other));
            }
        }
    }
    if timeout_ms < 50 {
        timeout_ms = 50;
    }
    let ports: Vec<u16> = if all {
        (1..=u16::MAX).collect()
    } else if let Some(spec) = ports_spec {
        portscan::parse_ports(&spec).map_err(|e| e.to_string())?
    } else {
        let n = top
            .unwrap_or(default_ports.len())
            .clamp(1, default_ports.len());
        default_ports[..n].to_vec()
    };
    Ok((ports, Duration::from_millis(timeout_ms), timeout_given))
}

/// `scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]`.
/// Default host is localhost; default range is the top 20.
fn cmd_scan_port(tokens: &[String]) -> Result<bool> {
    let mut host = "127.0.0.1".to_string();
    let mut rest: Vec<String> = Vec::new();
    for tok in tokens {
        if rest.is_empty() && !tok.starts_with('-') && host == "127.0.0.1" {
            // First bare token is the host — but `all`/`top` style words
            // are flags, handled below; a bare host never collides since
            // flag parsing rejects unknown bare tokens anyway. Keep the
            // simple rule: first bare token wins.
            host = tok.clone();
        } else {
            rest.push(tok.clone());
        }
    }
    let (ports, timeout, timeout_given) =
        match parse_net_flags(&rest, portscan::TOP_PORTS, 800, true) {
            Ok(v) => v,
            Err(e) => {
                print_error(&e);
                return Ok(false);
            }
        };
    // Huge sweeps without an explicit --timeout get the fast default
    // (a full 65K sweep at 800ms would take over an hour).
    let timeout = if !timeout_given && ports.len() > crate::netscan::FAST_TIMEOUT_ABOVE {
        Duration::from_millis(crate::netscan::FAST_TIMEOUT_MS)
    } else {
        timeout
    };
    let ips = match portscan::resolve_host(&host, ports[0]) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    print_info(&t!("cmd.scan.port_start", host, ports.len()));
    let total = ips.len() * ports.len();
    let progress = crate::netscan::SweepProgress::new(host.clone(), total);
    let swept = crate::netscan::sweep_hosts_async(
        &ips,
        &ports,
        timeout,
        crate::netscan::SCAN_CONCURRENCY,
        &progress,
        |_ip, port| {
            let service = portscan::service_name(port).unwrap_or("unknown");
            progress.print_hit(format!(
                "  {:<6} {}",
                port.to_string().green(),
                service.dimmed()
            ));
        },
    );
    let hits: Vec<_> = swept.into_iter().flat_map(|(_, v)| v).collect();
    if hits.is_empty() {
        print_warning(&t!("cmd.scan.port_none", host));
    } else {
        println!("{}", t!("cmd.scan.port_title", host).cyan().bold());
        for h in &hits {
            println!("  {:<6} {}", h.port.to_string().green(), h.service.dimmed());
        }
        print_success(&t!("cmd.scan.port_done", hits.len(), ports.len()));
    }
    Ok(false)
}

/// `scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]`.
/// Default range is the auto-detected local /24; per-host probes
/// default to the fast discovery set. Results union the TCP sweep
/// with the OS ARP table (`(arp)` rows had no open probe ports).
fn cmd_scan_ip(tokens: &[String]) -> Result<bool> {
    let mut cidr: Option<String> = None;
    let mut rest: Vec<String> = Vec::new();
    for tok in tokens {
        if cidr.is_none() && !tok.starts_with('-') {
            cidr = Some(tok.clone());
        } else {
            rest.push(tok.clone());
        }
    }
    let (ports, timeout, timeout_given) =
        match parse_net_flags(&rest, crate::netscan::DISCOVERY_PORTS, 400, false) {
            Ok(v) => v,
            Err(e) => {
                print_error(&e);
                return Ok(false);
            }
        };
    let (ips, label) = match cidr {
        Some(c) => {
            let list = match crate::netscan::parse_cidr(&c) {
                Ok(v) => v,
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            };
            let n = list.len();
            (list, format!("{} ({} hosts)", c, n))
        }
        None => match crate::netscan::local_subnet() {
            Ok((base, prefix)) => {
                let oct = base.octets();
                let mut list = Vec::new();
                for h in 1..=254u32 {
                    list.push(std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                        oct[0], oct[1], oct[2], h as u8,
                    )));
                }
                (
                    list,
                    format!("{}.{}.{}.0/{} (auto)", oct[0], oct[1], oct[2], prefix),
                )
            }
            Err(e) => {
                print_error(&e);
                return Ok(false);
            }
        },
    };
    print_info(&t!("cmd.scan.ip_start", label, ports.len()));
    // Big host × port matrices get the fast default unless the user
    // set --timeout explicitly (a /24 over 10 ports is 2540 probes).
    let timeout = if !timeout_given
        && ips.len().saturating_mul(ports.len()) > crate::netscan::FAST_TIMEOUT_ABOVE
    {
        Duration::from_millis(crate::netscan::FAST_TIMEOUT_MS)
    } else {
        timeout
    };
    let total = ips.len() * ports.len();
    let progress = crate::netscan::SweepProgress::new(label.clone(), total);
    let swept = crate::netscan::sweep_hosts_async(
        &ips,
        &ports,
        timeout,
        crate::netscan::SCAN_CONCURRENCY,
        &progress,
        |ip, port| {
            let service = portscan::service_name(port).unwrap_or("unknown");
            if service == "unknown" {
                progress.print_hit(format!("  {}  {}", ip, port));
            } else {
                progress.print_hit(format!(
                    "  {}  {} {}",
                    ip,
                    port.to_string().green(),
                    service.dimmed()
                ));
            }
        },
    );
    let swept_ips: std::collections::HashSet<std::net::IpAddr> =
        swept.iter().map(|(ip, _)| *ip).collect();
    let mut arp_only: Vec<std::net::IpAddr> = crate::netscan::arp_table(&ips)
        .into_iter()
        .filter(|ip| !swept_ips.contains(ip))
        .collect();
    arp_only.sort();
    if swept.is_empty() && arp_only.is_empty() {
        print_warning(&t!("cmd.scan.ip_none", label));
        return Ok(false);
    }
    println!("{}", t!("cmd.scan.ip_title", label).cyan().bold());
    for (ip, open) in &swept {
        if open.is_empty() {
            println!("  {}", ip.to_string().green());
        } else {
            let ports_str = open
                .iter()
                .map(|h| {
                    if h.service == "unknown" {
                        h.port.to_string()
                    } else {
                        format!("{} {}", h.port, h.service)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            println!("  {}  {}", ip.to_string().green(), ports_str.dimmed());
        }
    }
    for ip in &arp_only {
        println!(
            "  {}  {}",
            ip.to_string().green(),
            t!("cmd.scan.ip_arp_only").dimmed()
        );
    }
    print_success(&t!(
        "cmd.scan.ip_done",
        swept.len() + arp_only.len(),
        ips.len()
    ));
    Ok(false)
}

/// Scan a directory on disk (used by tests without a Navigator).
#[cfg(test)]
fn scan_dir(
    root: &std::path::Path,
    exts: &[String],
    max_depth: usize,
    care: bool,
) -> Vec<(String, u64)> {
    let mut entries: Vec<(String, u64)> = Vec::new();
    if care {
        for entry in WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches_any_extension(&name, exts) {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            entries.push((rel, size));
        }
    } else {
        let walker = WalkDir::new(root)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| {
                // Tests run with the real global config; mirror cmd_scan filtering.
                (make_ignored_dir_filter(&Config::get_ignored_dirs_combined()))(e)
            });
        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if !matches_any_extension(&name, exts) {
                continue;
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let rel = match entry.path().strip_prefix(root) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => entry.path().display().to_string(),
            };
            entries.push((rel, size));
        }
    }
    entries.sort_by_key(|(rel, _)| rel.to_lowercase());
    entries
}

/// One enumeration pass: print hits as they land, collect the rows for
/// `-o`, and return the summary.
#[allow(clippy::too_many_arguments)]
fn waep_run(
    target: &crate::netscan::webpath::Target,
    words: &[String],
    opts: &crate::netscan::webpath::Options,
    accept: &[u16],
    show_all: bool,
    quiet: bool,
    // Recursion prints the full path (`/api/files`) so a hit on a deeper
    // level is not confused with a hit at the scan base (`/files`).
    show_full: bool,
    seen_urls: &mut std::collections::HashSet<String>,
    rows: &mut Vec<String>,
    hit_count: &mut usize,
) -> (crate::netscan::webpath::Summary, Vec<String>) {
    use crate::netscan::webpath::Kind;
    let accept_fn = move |status: u16| show_all || accept.contains(&status);
    let mut found: Vec<String> = Vec::new();
    let summary = crate::netscan::webpath::enumerate(target, words, opts, &accept_fn, &mut |hit| {
        // Recursive levels can rediscover a URL already reported; print it
        // once.
        if !seen_urls.insert(hit.url.clone()) {
            return;
        }
        *hit_count += 1;
        found.push(hit.path.clone());
        let kind = hit.probe.kind();
        let status = hit.probe.status.to_string();
        let status = match kind {
            Kind::Ok => status.green(),
            Kind::Redirect => status.cyan(),
            Kind::Forbidden => status.yellow(),
            Kind::Method => status.magenta(),
            Kind::Error => status.red(),
            Kind::Missing => status.dimmed(),
        };
        let ctype = hit
            .probe
            .content_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        let extra = if hit.probe.location.is_empty() {
            ctype
        } else {
            format!("-> {}", hit.probe.location)
        };
        let shown = if show_full {
            crate::netscan::webpath::join_path(&target.base_path, &hit.path)
        } else {
            hit.path.clone()
        };
        let line = format!(
            "  {:<28} {:>4} {:>5}ms  {}",
            shown,
            status,
            hit.probe.ms,
            extra.dimmed()
        );
        if !quiet {
            println!("{}", line);
        }
        rows.push(format!(
            "{} {} {} {}",
            shown, hit.probe.status, hit.probe.ms, extra
        ));
    });
    (summary, found)
}

/// Full help for `scan waep`: usage, examples, every flag and the option
/// semantics (base-relative results, accepted statuses, recursion).
fn print_waep_help() {
    println!("{}", t!("cmd.scan.waep_usage").cyan().bold());
    println!();
    println!("{}", t!("cmd.scan.examples_title").green());
    for ex in [
        "scan waep example.com",
        "scan waep 192.168.1.5:8080 -x php,html --all",
        "scan waep http://127.0.0.1:55555/api",
        "scan waep example.com/api -r -d 2",
        "scan waep example.com -w paths.txt -s 200,301,403 -o hits.txt",
    ] {
        println!("    {}", ex);
    }
    println!();
    println!("{}", t!("cmd.scan.waep_flags_title").green());
    print_indented(t!("cmd.scan.waep_flags"));
    println!();
    println!("{}", t!("cmd.scan.waep_options_title").green());
    print_indented(t!("cmd.scan.waep_options"));
}

/// Print a multi-line block with a uniform two-space indent regardless of how
/// each translation indents its own continuation lines.
fn print_indented(text: String) {
    for line in text.lines() {
        println!("  {}", line.trim_start());
    }
}

/// `scan waep <host|host:port|url[/base]> [flags]` — web path enumeration.
fn cmd_scan_waep(tokens: &[String]) -> Result<bool> {
    if tokens.is_empty() || matches!(tokens[0].to_lowercase().as_str(), "help" | "-h" | "--help") {
        print_waep_help();
        return Ok(false);
    }
    let spec = tokens[0].clone();
    let target = match crate::netscan::webpath::parse_target(&spec) {
        Ok(t) => t,
        Err(e) => {
            print_error(&t!("cmd.scan.waep_bad_target", e));
            return Ok(false);
        }
    };

    let mut opts = crate::netscan::webpath::Options::default();
    let mut words: Vec<String> = crate::netscan::webpath::default_words()
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut exts: Vec<String> = Vec::new();
    let mut accept: Vec<u16> = crate::netscan::webpath::DEFAULT_STATUS.to_vec();
    let mut show_all = false;
    let mut recursive = false;
    let mut depth: usize = 1;
    let mut output: Option<String> = None;
    let mut quiet = false;

    let mut i = 1;
    while i < tokens.len() {
        let flag = tokens[i].as_str();
        let value = |i: &mut usize, flag: &str| -> Result<String, String> {
            *i += 1;
            tokens
                .get(*i)
                .cloned()
                .ok_or_else(|| t!("netdiag.missing_value", flag))
        };
        match flag {
            "-w" | "--wordlist" => match value(&mut i, flag) {
                Ok(path) => {
                    let expanded = crate::utils::paths::expand_path_str(&path);
                    match std::fs::read_to_string(&expanded) {
                        Ok(text) => {
                            let parsed = crate::netscan::webpath::parse_words(&text);
                            if parsed.is_empty() {
                                print_warning(&t!("cmd.scan.waep_empty_wordlist", path));
                            } else {
                                words = parsed;
                            }
                        }
                        Err(e) => {
                            print_error(&t!("cmd.scan.waep_wordlist_failed", path, e));
                            return Ok(false);
                        }
                    }
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-x" | "--ext" => match value(&mut i, flag) {
                Ok(list) => {
                    exts = list
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-m" | "--method" => match value(&mut i, flag) {
                Ok(m) => {
                    let m = m.to_uppercase();
                    if m != "GET" && m != "HEAD" {
                        print_error(&t!("cmd.scan.waep_bad_method", m));
                        return Ok(false);
                    }
                    opts.method = m;
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-s" | "--status" => match value(&mut i, flag) {
                Ok(list) => {
                    let parsed: Vec<u16> = list
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u16>().ok())
                        .collect();
                    if parsed.is_empty() {
                        print_error(&t!("cmd.scan.waep_bad_status", list));
                        return Ok(false);
                    }
                    accept = parsed;
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "--all" => show_all = true,
            "-r" | "--recursive" => recursive = true,
            "-d" | "--depth" => match value(&mut i, flag) {
                Ok(v) => match v.parse::<usize>() {
                    Ok(n) if n >= 1 => depth = n.min(4),
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_depth", v));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-c" | "--concurrency" => match value(&mut i, flag) {
                Ok(v) => match v.parse::<usize>() {
                    Ok(n) if n >= 1 => opts.concurrency = n.min(64),
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_concurrency", v));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "--timeout" => match value(&mut i, flag) {
                Ok(v) => match v.parse::<u64>() {
                    Ok(n) if n >= 50 => opts.timeout_ms = n,
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_timeout", v));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-H" | "--header" => match value(&mut i, flag) {
                Ok(h) => match h.split_once(':') {
                    Some((k, v)) if !k.trim().is_empty() => opts
                        .headers
                        .push((k.trim().to_string(), v.trim().to_string())),
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_header", h));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-A" | "--user-agent" => match value(&mut i, flag) {
                Ok(v) => opts.user_agent = v,
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-o" | "--output" => match value(&mut i, flag) {
                Ok(path) => output = Some(crate::utils::paths::expand_path_str(&path)),
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-q" | "--quiet" => quiet = true,
            other => {
                print_error(&t!("cmd.scan.unknown_option", other));
                return Ok(false);
            }
        }
        i += 1;
    }

    // Re-base the word list on the scanned path (`.../api` probes
    // `/api/files`, not `/api/api/files`).
    let words = crate::netscan::webpath::adapt_words_to_base(&words, &target.base_path);
    let words = crate::netscan::webpath::expand_suffixes(&words, &exts);
    if words.is_empty() {
        print_error(&t!("cmd.scan.waep_empty_wordlist", target.base_url()));
        return Ok(false);
    }
    print_info(&t!("cmd.scan.waep_start", target.base_url(), words.len()));

    let mut rows: Vec<String> = Vec::new();
    let mut hit_count = 0usize;
    let mut probed = 0usize;
    let mut errors = 0usize;
    let start = std::time::Instant::now();

    // Depth 1 is the base scan; `-r -d N` adds N-1 descent levels under
    // directory-looking hits. Each level's bases are built by appending the
    // discovered (base-relative) path, so URLs never double a segment.
    let max_levels = if recursive { depth.max(1) } else { 1 };
    let mut frontier: Vec<crate::netscan::webpath::Target> = vec![target.clone()];
    let mut seen_bases: std::collections::HashSet<String> = std::collections::HashSet::new();
    seen_bases.insert(target.base_path.clone());
    let mut seen_urls: std::collections::HashSet<String> = std::collections::HashSet::new();
    for level in 1..=max_levels {
        let mut next: Vec<crate::netscan::webpath::Target> = Vec::new();
        for sub in &frontier {
            // A deeper level gets the same word list re-based on its own path
            // (root `-r` into `/api` must probe `/api/files`, never
            // `/api/api/files`).
            let sub_words = crate::netscan::webpath::adapt_words_to_base(&words, &sub.base_path);
            let (summary, found) = waep_run(
                sub,
                &sub_words,
                &opts,
                &accept,
                show_all,
                quiet,
                level > 1,
                &mut seen_urls,
                &mut rows,
                &mut hit_count,
            );
            probed += summary.probed;
            errors += summary.errors;
            if level >= max_levels {
                continue;
            }
            // Only directory-looking hits (no dot in the last segment) can
            // have children; a handful per level keeps a scan bounded.
            for path in found.into_iter().take(8) {
                let last = path.rsplit('/').next().unwrap_or("");
                if last.is_empty() || last.contains('.') {
                    continue;
                }
                let mut deeper = sub.clone();
                deeper.base_path = crate::netscan::webpath::join_path(&sub.base_path, &path);
                if deeper.base_path.len() > 200 || !seen_bases.insert(deeper.base_path.clone()) {
                    continue;
                }
                next.push(deeper);
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }

    let elapsed = start.elapsed().as_millis() as u64;
    if hit_count == 0 {
        // Every probe failing is an unreachable target, not an empty site.
        if probed > 0 && errors >= probed {
            print_error(&t!("cmd.scan.waep_unreachable", target.base_url()));
        } else {
            print_warning(&t!("cmd.scan.waep_none", target.base_url()));
        }
    } else {
        println!("{}", t!("cmd.scan.waep_summary", hit_count, probed).cyan());
    }
    if errors > 0 && errors < probed {
        print_warning(&t!("cmd.scan.waep_some_failed", errors, probed));
    }
    if elapsed > 0 {
        println!("{}", t!("cmd.scan.waep_elapsed", elapsed).dimmed());
    }
    if let Some(path) = output {
        let body = rows.join("\n") + "\n";
        match crate::output::write_file(std::path::Path::new(&path), &body) {
            Ok(()) => print_success(&t!("cmd.scan.waep_saved", path)),
            Err(e) => print_error(&t!("cmd.scan.waep_save_failed", path, e)),
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_exts_and_flags_in_any_order() {
        let p = parse_scan_args(&tokenize("rs c -d 3 -s")).unwrap();
        assert_eq!(p.exts, vec!["rs", "c"]);
        assert_eq!(p.max_depth, Some(3));
        assert!(p.show_size);
        assert!(!p.care);

        let p = parse_scan_args(&tokenize("-s --depth 2 .RS, c")).unwrap();
        assert_eq!(p.exts, vec!["rs", "c"]);
        assert_eq!(p.max_depth, Some(2));
        assert!(p.show_size);
        assert!(!p.care);

        let p = parse_scan_args(&tokenize("rs --care -s")).unwrap();
        assert_eq!(p.exts, vec!["rs"]);
        assert!(p.show_size);
        assert!(p.care);

        let p = parse_scan_args(&tokenize("--care rs c -d 2")).unwrap();
        assert_eq!(p.exts, vec!["rs", "c"]);
        assert_eq!(p.max_depth, Some(2));
        assert!(p.care);
    }

    #[test]
    fn parse_rejects_missing_exts_and_bad_depth() {
        assert!(parse_scan_args(&tokenize("-s")).is_err());
        assert!(parse_scan_args(&tokenize("rs -d abc")).is_err());
        assert!(parse_scan_args(&tokenize("rs --bogus")).is_err());
    }

    #[test]
    fn parse_net_flags_top_ports_all_timeout() {
        use std::time::Duration;
        let (ports, to, given) =
            parse_net_flags(&tokenize("--top 3"), portscan::TOP_PORTS, 800, true).unwrap();
        assert_eq!(ports, portscan::TOP_PORTS[..3].to_vec());
        assert_eq!(to, Duration::from_millis(800));
        assert!(!given);

        let (ports, _, _) = parse_net_flags(
            &tokenize("--ports 22,80-81"),
            portscan::TOP_PORTS,
            800,
            true,
        )
        .unwrap();
        assert_eq!(ports, vec![22, 80, 81]);

        let (ports, _, _) =
            parse_net_flags(&tokenize("--all"), portscan::TOP_PORTS, 800, true).unwrap();
        assert_eq!(ports.len(), 65535);
        assert_eq!((ports[0], ports[65534]), (1, 65535));

        // --all rejected where not allowed (scan ip).
        assert!(parse_net_flags(&tokenize("--all"), portscan::TOP_PORTS, 400, false).is_err());

        let (_, to, given) =
            parse_net_flags(&tokenize("--timeout 10"), portscan::TOP_PORTS, 800, true).unwrap();
        assert_eq!(to, Duration::from_millis(50)); // floored
        assert!(given);

        assert!(parse_net_flags(&tokenize("--top abc"), portscan::TOP_PORTS, 800, true).is_err());
        assert!(parse_net_flags(&tokenize("--timeout"), portscan::TOP_PORTS, 800, true).is_err());
        assert!(parse_net_flags(&tokenize("--bogus"), portscan::TOP_PORTS, 800, true).is_err());
        assert!(parse_net_flags(&tokenize("oops"), portscan::TOP_PORTS, 800, true).is_err());
    }

    #[test]
    fn walk_filters_by_extension_case_insensitively() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        std::fs::write(dir.path().join("notes.TXT"), "hi").unwrap();
        std::fs::write(dir.path().join("photo.png"), "fake").unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("lib.RS"), "x").unwrap();

        let exts = vec!["rs".to_string()];
        let found = scan_dir(dir.path(), &exts, 10, false);
        let names: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
        assert_eq!(names, vec!["main.rs", "sub/lib.RS"]);

        // Depth limit excludes the nested file (depth 2).
        let shallow = scan_dir(dir.path(), &exts, 1, false);
        assert_eq!(shallow.len(), 1);

        // Multi-ext + no match.
        let multi = scan_dir(
            dir.path(),
            &["rs".to_string(), "txt".to_string()],
            10,
            false,
        );
        assert_eq!(multi.len(), 3);
        let none = scan_dir(dir.path(), &["zzz".to_string()], 10, false);
        assert!(none.is_empty());

        // Care mode returns the same set when nothing is ignored.
        let cared = scan_dir(dir.path(), &exts, 10, true);
        assert_eq!(cared.len(), 2);
    }

    #[test]
    fn wildcard_matches_segments_case_insensitively() {
        assert!(matches_wildcard("main.rs", "*.rs"));
        assert!(matches_wildcard("LIB.RS", "*.rs"));
        assert!(matches_wildcard("keep.ext", "k*.ext"));
        assert!(matches_wildcard("theend.ext", "*end.ext"));
        assert!(matches_wildcard("anything", "*"));
        assert!(matches_wildcard("main.rs", "main.rs"));
        assert!(matches_wildcard("MAIN.RS", "main.rs"));
        assert!(!matches_wildcard("main.rs", "main.py"));
        assert!(!matches_wildcard("main.rs", "k*"));
        assert!(!matches_wildcard("lib.rs", "*end.rs"));
        // Middle segments must appear in order.
        assert!(matches_wildcard("aXbYc", "a*b*c"));
        assert!(!matches_wildcard("aXc", "a*b*c"));
        // `*` matches empty runs.
        assert!(matches_wildcard("k", "k*"));
        assert!(matches_wildcard("end", "*end"));
        // Non-`*` chars stay literal.
        assert!(!matches_wildcard("kbc", "k?c"));
        assert!(!matches_wildcard("keep", "keep2"));
    }

    #[test]
    fn parse_name_args_pattern_and_flags() {
        let p =
            parse_scan_name_args(&tokenize("*.rs -d 3 -s"), "cmd.scan.files_usage", false).unwrap();
        assert_eq!(p.pattern.as_deref(), Some("*.rs"));
        assert_eq!(p.max_depth, Some(3));
        assert!(p.show_size);
        assert!(!p.care);

        let p = parse_scan_name_args(
            &tokenize("--care k* --depth 2"),
            "cmd.scan.files_usage",
            false,
        )
        .unwrap();
        assert_eq!(p.pattern.as_deref(), Some("k*"));
        assert_eq!(p.max_depth, Some(2));
        assert!(p.care);

        // Bare / `*` / `*.*` for files all mean match-all.
        for args in ["", "*", "*.*"] {
            let p = parse_scan_name_args(&tokenize(args), "cmd.scan.files_usage", false).unwrap();
            assert!(p.pattern.is_none(), "args: {:?}", args);
        }
        // `*` for dir means match-all, but `*.*` is rejected.
        let p = parse_scan_name_args(&tokenize("*"), "cmd.scan.dir_usage", true).unwrap();
        assert!(p.pattern.is_none());
        assert!(parse_scan_name_args(&tokenize("*.*"), "cmd.scan.dir_usage", true).is_err());

        // Second bare token, bad depth, unknown flag all fail.
        assert!(parse_scan_name_args(&tokenize("a* b*"), "cmd.scan.files_usage", false).is_err());
        assert!(
            parse_scan_name_args(&tokenize("*.rs -d abc"), "cmd.scan.files_usage", false).is_err()
        );
        assert!(parse_scan_name_args(&tokenize("--bogus"), "cmd.scan.files_usage", false).is_err());
    }

    #[test]
    fn walk_matching_files_dirs_care_and_sizes() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[
                ("keep.rs", "12345678"),
                ("skip.txt", "hi"),
                ("sub/deep.rs", "xyz"),
                ("target/secret.rs", "q"),
            ],
            &["sub", "target", "kbox", "backend"],
        );
        let mut cfg = Config::default();
        cfg.ignored_directory_names.insert("target".to_string());
        Config::with_test_config(cfg, || {
            let root = std::env::current_dir().unwrap();
            // Files: `*.rs` without care skips the ignored `target/`.
            let found = walk_matching(
                &root,
                10,
                false,
                false,
                &(|n: &str| matches_wildcard(n, "*.rs")),
            );
            let mut rels: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
            rels.sort();
            assert_eq!(rels, vec!["keep.rs", "sub/deep.rs"]);
            // Care mode includes everything.
            let found = walk_matching(&root, 10, true, false, &(|_: &str| true));
            assert_eq!(found.len(), 4);
            // Dirs: `k*` matches only `kbox`; the root itself is never listed.
            let found = walk_matching(
                &root,
                10,
                false,
                true,
                &(|n: &str| matches_wildcard(n, "k*")),
            );
            let rels: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
            assert_eq!(rels, vec!["kbox"]);
            // Dirs: match-all skips ignored `target/` without care ...
            let found = walk_matching(&root, 10, false, true, &(|_: &str| true));
            let mut rels: Vec<&str> = found.iter().map(|(r, _)| r.as_str()).collect();
            rels.sort();
            assert_eq!(rels, vec!["backend", "kbox", "sub"]);
            // ... and includes it with care.
            let found = walk_matching(&root, 10, true, true, &(|_: &str| true));
            assert_eq!(found.len(), 4);
            // Sizes aggregate descendant file bytes (`sub` holds 3 bytes).
            let found = walk_matching(&root, 10, false, true, &(|_: &str| true));
            let size_of = |d: &str| {
                found
                    .iter()
                    .find(|(r, _)| r == d)
                    .map(|(_, s)| *s)
                    .unwrap_or(u64::MAX)
            };
            assert_eq!(size_of("sub"), 3);
            assert_eq!(size_of("backend"), 0);
            // Depth limit excludes nested entries (depth 2).
            let shallow = walk_matching(&root, 1, false, false, &(|_: &str| true));
            assert_eq!(shallow.len(), 2);
            let shallow_dirs = walk_matching(&root, 1, false, true, &(|_: &str| true));
            assert_eq!(shallow_dirs.len(), 3);
        });
    }
}
