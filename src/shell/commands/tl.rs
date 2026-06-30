use crate::config::Config;
use crate::filetype::{FormatConfig, is_supported_format_with_config, make_ignored_dir_filter};
use crate::navigator::Navigator;
use crate::output::print_error;
use anyhow::Result;
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
            print_error(&format!("Unknown subcommand '{}'.", sub));
            let _ = print_usage();
            Ok(false)
        }
    }
}

fn print_usage() -> Result<bool> {
    println!("Usage:");
    println!("  tl file <path>          Count lines in a single file");
    println!("  tl ext <extension>      Show all files with extension and total lines");
    println!("  tl dir <path>           Show supported extensions and total lines in directory");
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

fn cmd_tl_file(path_str: &str, nav: &Navigator) -> Result<bool> {
    if path_str.is_empty() {
        print_error("Usage: tl file <path>");
        return Ok(false);
    }
    let path = resolve_path(path_str, nav);

    if !path.exists() {
        print_error(&format!("File not found: {}", path_str));
        return Ok(false);
    }
    if !path.is_file() {
        print_error(&format!("Not a file: {}", path_str));
        return Ok(false);
    }

    let lines = count_lines(&path);
    println!("{}  {}", path_str, lines);
    Ok(false)
}

fn cmd_tl_ext(ext: &str, nav: &Navigator) -> Result<bool> {
    if ext.is_empty() {
        print_error("Usage: tl ext <extension>");
        return Ok(false);
    }

    let raw_ext = ext.trim_start_matches('.');
    let root = nav.current_path().to_path_buf();
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();

    let mut total_lines = 0u64;
    let mut count = 0u64;
    let mut files_found = false;

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
                    let rel = path.strip_prefix(&root).unwrap_or(path);
                    println!("{}  {}", rel.display(), lines);
                }
            }
        }
    }

    if files_found {
        println!("{}", "─".repeat(40));
        println!("total: {} files, {} lines", count, total_lines);
    } else {
        let test_name = format!("dummy.{}", raw_ext);
        let test_path = Path::new(&test_name);
        if !is_supported_format_with_config(test_path, &fmt_cfg) {
            println!(".{} not supported — blocked", raw_ext);
        } else {
            println!("No .{} files found", raw_ext);
        }
    }

    Ok(false)
}

fn cmd_tl_dir(dir_str: &str, nav: &Navigator) -> Result<bool> {
    if dir_str.is_empty() {
        print_error("Usage: tl dir <path>");
        return Ok(false);
    }

    let dir = resolve_path(dir_str, nav);

    if !dir.exists() || !dir.is_dir() {
        print_error(&format!("Directory not found: {}", dir_str));
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

    let mut total_files = 0u64;
    let mut total_lines = 0u64;
    for (ext, (count, lines)) in &ext_map {
        println!(".{:<10} {:>5} files, {:>7} lines", ext, count, lines);
        total_files += count;
        total_lines += lines;
    }

    if total_files > 0 {
        println!("{}", "─".repeat(40));
        println!("total: {:>5} files, {:>7} lines", total_files, total_lines);
    } else {
        println!("No supported files found in {}", dir_str);
    }

    Ok(false)
}
