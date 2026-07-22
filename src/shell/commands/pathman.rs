use crate::navigator::Navigator;
use crate::output::print_error;
#[cfg(windows)]
use crate::output::{print_info, print_success};
use anyhow::Result;
#[cfg(windows)]
use anyhow::anyhow;
#[cfg(windows)]
use colored::*;
#[cfg(windows)]
use std::path::Path;

const USER_KEY: &str = "HKCU\\Environment";
const SYS_KEY: &str = "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";

pub fn cmd_usrpath(args: &str, nav: &mut Navigator) -> Result<bool> {
    dispatch_path(args, nav, USER_KEY, "usrpath")
}

pub fn cmd_syspath(args: &str, nav: &mut Navigator) -> Result<bool> {
    dispatch_path(args, nav, SYS_KEY, "syspath")
}

fn dispatch_path(args: &str, nav: &mut Navigator, key: &str, name: &str) -> Result<bool> {
    #[cfg(windows)]
    {
        let args = args.trim();
        if args.is_empty() {
            print_usage(name);
            return Ok(false);
        }
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let sub = parts[0].to_lowercase();
        let rest = parts.get(1).unwrap_or(&"").trim();
        match sub.as_str() {
            "show" | "ls" => cmd_show(key, name),
            "add" => cmd_add(key, name, rest, nav),
            "rm" | "remove" => cmd_rm(key, rest, name),
            "move" | "mv" => cmd_move(key, rest, name),
            _ => {
                print_usage(name);
                Ok(false)
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = (args, nav, key, name);
        print_error("usrpath/syspath is a Windows-only feature.");
        Ok(false)
    }
}

#[cfg(windows)]
fn print_usage(name: &str) {
    println!("{}", "Usage:".cyan().bold());
    println!("  {name} show                     Show PATH entries with indices");
    println!("  {name} add <path>               Add a directory to PATH");
    println!("  {name} add --cwd | --here       Add current directory to PATH");
    println!("  {name} rm <path-or-number>      Remove from PATH (by path or index)");
    println!("  {name} move <n> <m>             Move entry #n to position #m");
    println!();
    println!("{}", "Examples:".green());
    println!("  {name} show");
    println!("  {name} add C:\\Tools");
    println!("  {name} add --cwd");
    println!("  {name} rm 3                    Remove entry #3 from the list");
    println!("  {name} rm C:\\OldTools");
    println!("  {name} move 3 1               Move entry #3 to the top");
}

#[cfg(windows)]
fn print_level_name(key: &str) -> &'static str {
    if key == USER_KEY { "User" } else { "System" }
}

#[cfg(windows)]
fn cmd_show(key: &str, name: &str) -> Result<bool> {
    let current = read_path(key).unwrap_or_default();
    let entries = split_path(&current);

    println!();
    let level = print_level_name(key);
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║{:^60}", format!("{level} PATH ({name})").cyan().bold());
    println!("╠══════════════════════════════════════════════════════════════════╣");

    if entries.is_empty() {
        println!("║  {}", "(PATH is empty or not set)".dimmed());
    } else {
        for (i, entry) in entries.iter().enumerate() {
            println!("║  {:<3} {}", format!("{}.", i + 1).yellow(), entry);
        }
    }

    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!("  Total: {} entries", entries.len().to_string().cyan());
    println!();
    Ok(false)
}

#[cfg(windows)]
fn cmd_add(key: &str, name: &str, args: &str, nav: &Navigator) -> Result<bool> {
    let dir = if args.eq_ignore_ascii_case("--cwd") || args.eq_ignore_ascii_case("--here") {
        nav.current_path().to_string_lossy().to_string()
    } else if args.is_empty() {
        print_usage(name);
        return Ok(false);
    } else {
        args.to_string()
    };

    let dir_path = Path::new(&dir);
    let dir_display = if dir_path.is_absolute() {
        dir.clone()
    } else {
        nav.current_path().join(&dir).to_string_lossy().to_string()
    };

    let current = read_path(key).unwrap_or_default();
    let mut entries = split_path(&current);

    let normalized_new = normalize_path(&dir_display);

    if entries.iter().any(|e| normalize_path(e) == normalized_new) {
        print_info(&format!("Already in PATH: {}", dir_display));
        return Ok(false);
    }

    entries.push(dir_display.clone());

    let new_path = join_path(&entries);
    write_path(key, &new_path)?;

    print_success(&format!("Added to {name}: {}", dir_display));
    print_restart_note();
    Ok(false)
}

#[cfg(windows)]
fn cmd_rm(key: &str, args: &str, name: &str) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: usrpath rm <path-or-number>");
        return Ok(false);
    }

    let current = read_path(key).unwrap_or_default();
    let mut entries = split_path(&current);

    let removed = if let Ok(idx) = args.parse::<usize>() {
        // Remove by 1-based index
        if idx >= 1 && idx <= entries.len() {
            let entry = entries.remove(idx - 1);
            print_success(&format!("Removed from {name}: {entry}"));
            true
        } else {
            print_error(&format!("Index out of range: {idx} (1-{})", entries.len()));
            return Ok(false);
        }
    } else {
        // Remove by path match
        let target_normalized = normalize_path(args);
        let before = entries.len();
        entries.retain(|e| normalize_path(e) != target_normalized);
        if entries.len() < before {
            print_success(&format!("Removed from {name}: {args}"));
            true
        } else {
            print_error(&format!("Not found in PATH: {args}"));
            return Ok(false);
        }
    };

    let new_path = join_path(&entries);
    write_path(key, &new_path)?;

    if removed {
        print_restart_note();
    }
    Ok(false)
}

#[cfg(windows)]
fn cmd_move(key: &str, args: &str, name: &str) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.len() < 2 {
        print_error(&format!("Usage: {name} move <n> <m>"));
        return Ok(false);
    }

    let select = parts[0].parse::<usize>();
    let target = parts[1].parse::<usize>();

    let (select, target) = match (select, target) {
        (Ok(s), Ok(t)) => (s, t),
        _ => {
            print_error("Indices must be numbers");
            return Ok(false);
        }
    };

    let current = read_path(key).unwrap_or_default();
    let mut entries = split_path(&current);

    if select < 1 || select > entries.len() {
        print_error(&format!("Index out of range: {select} (1-{})", entries.len()));
        return Ok(false);
    }
    if target < 1 || target > entries.len() {
        print_error(&format!("Index out of range: {target} (1-{})", entries.len()));
        return Ok(false);
    }

    if select == target {
        print_info("Source and target are the same — nothing to move");
        return Ok(false);
    }

    let select_idx = select - 1;
    let mut target_idx = target - 1;

    let entry = entries.remove(select_idx);
    if select_idx < target_idx {
        target_idx -= 1;
    }
    entries.insert(target_idx, entry.clone());

    let new_path = join_path(&entries);
    write_path(key, &new_path)?;

    print_success(&format!("Moved to position #{target} in {name}: {entry}"));
    print_restart_note();
    Ok(false)
}

#[cfg(windows)]
fn read_path(key: &str) -> Result<String> {
    let output = std::process::Command::new("reg")
        .args(["query", key, "/v", "PATH"])
        .output()
        .map_err(|e| anyhow!("Failed to query registry: {e}"))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        if err.contains("does not exist") || err.contains("not found") {
            return Ok(String::new());
        }
        return Err(anyhow!("Registry query failed: {}", err.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("PATH") || trimmed.starts_with("path") {
            if let Some(val) = parse_reg_value(trimmed) {
                return Ok(val);
            }
        }
    }

    Ok(String::new())
}

#[cfg(windows)]
fn parse_reg_value(line: &str) -> Option<String> {
    let after_name = line.find(char::is_whitespace)?;
    let rest = line[after_name..].trim_start();
    let after_type = rest.find(char::is_whitespace)?;
    let val = rest[after_type..].trim_start();
    if val.is_empty() { None } else { Some(val.to_string()) }
}

#[cfg(windows)]
fn write_path(key: &str, path_value: &str) -> Result<()> {
    let output = std::process::Command::new("reg")
        .args([
            "add", key, "/v", "PATH", "/t", "REG_EXPAND_SZ",
            "/d", path_value, "/f",
        ])
        .output()
        .map_err(|e| anyhow!("Failed to write registry: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err_lower = stderr.to_lowercase();
        if err_lower.contains("access is denied") || err_lower.contains("access denied") {
            println!();
            print_info("Run your terminal as Administrator (right-click → \"Run as administrator\"),");
            print_info("then run the command again. Or use:  mode admin");
            return Err(anyhow::anyhow!("Access denied — administrator privileges required."));
        }
        return Err(anyhow!(
            "Failed to update PATH: {}",
            stderr.trim()
        ));
    }

    Ok(())
}

#[cfg(windows)]
fn split_path(value: &str) -> Vec<String> {
    value.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(windows)]
fn join_path(entries: &[String]) -> String {
    entries.join(";")
}

#[cfg(windows)]
fn normalize_path(s: &str) -> String {
    let trimmed = s.trim_end_matches('\\').trim_end_matches('/');
    trimmed.to_lowercase()
}

#[cfg(windows)]
fn print_restart_note() {
    println!();
    print_info("Changes will apply to new terminal windows.");
}
