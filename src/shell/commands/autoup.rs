use crate::navigator::Navigator;
use crate::output::print_error;
#[cfg(windows)]
use crate::output::{print_info, print_success, print_warning};
use crate::t;
#[cfg(windows)]
use anyhow::anyhow;
use anyhow::Result;
#[cfg(windows)]
use colored::*;
#[cfg(windows)]
use std::collections::HashSet;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
const RUN_KEY_CU: &str = "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run";
#[cfg(windows)]
const RUN_KEY_LM: &str = "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run";
#[cfg(windows)]
const APPROVED_KEY_CU: &str =
    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run";
#[cfg(windows)]
const APPROVED_KEY_LM: &str =
    "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run";

pub fn cmd_autoup(args: &str, _nav: &mut Navigator) -> Result<bool> {
    #[cfg(windows)]
    {
        let args = args.trim();
        if args.is_empty() {
            print_usage();
            return Ok(false);
        }
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let sub = parts[0].to_lowercase();
        let rest = parts.get(1).unwrap_or(&"").trim();
        match sub.as_str() {
            "list" | "ls" => cmd_list(),
            "add" => cmd_add(rest),
            "rm" | "remove" => cmd_rm(rest),
            "disable" => cmd_disable(rest),
            "enable" => cmd_enable(rest),
            "help" => {
                print_usage();
                Ok(false)
            }
            _ => {
                print_usage();
                Ok(false)
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = (args, _nav);
        print_error(&t!("autoup.windows_only"));
        Ok(false)
    }
}

#[cfg(windows)]
fn print_usage() {
    println!("{}", t!("autoup.usage_title").cyan().bold());
    println!("{}", t!("autoup.help_list"));
    println!("{}", t!("autoup.help_add"));
    println!("{}", t!("autoup.help_rm"));
    println!("{}", t!("autoup.help_disable"));
    println!("{}", t!("autoup.help_enable"));
    println!("{}", t!("autoup.help_help"));
    println!();
    println!("{}", t!("autoup.source_tags").cyan().bold());
    println!("{}", t!("autoup.source_cu"));
    println!("{}", t!("autoup.source_lm"));
    println!("{}", t!("autoup.source_sf"));
    println!();
    println!("{}", t!("autoup.usage_examples").green());
    println!("  autoup list");
    println!("  autoup add MyApp \"C:\\Program Files\\MyApp\\app.exe\"");
    println!("  autoup add Chrome \"C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe https://example.com\"");
    println!("  autoup rm MyApp");
    println!("  autoup disable OneDrive");
    println!("  autoup enable OneDrive");
}

// ── Data model ─────────────────────────────────────────────────────────────

#[cfg(windows)]
struct StartupEntry {
    name: String,
    command: String,
    source: &'static str,
    disabled: bool,
}

// ── List command ────────────────────────────────────────────────────────────

#[cfg(windows)]
fn cmd_list() -> Result<bool> {
    let cu_disabled = read_disabled_names(APPROVED_KEY_CU)?;
    let lm_disabled = read_disabled_names(APPROVED_KEY_LM)?;
    let mut entries: Vec<StartupEntry> = Vec::new();

    if let Ok(cu) = read_run_key_entries(RUN_KEY_CU) {
        for (name, command) in cu {
            entries.push(StartupEntry {
                disabled: cu_disabled.contains(&name),
                source: "CU",
                name,
                command,
            });
        }
    }

    if let Ok(lm) = read_run_key_entries(RUN_KEY_LM) {
        for (name, command) in lm {
            entries.push(StartupEntry {
                disabled: lm_disabled.contains(&name),
                source: "LM",
                name,
                command,
            });
        }
    }

    if let Ok(sf) = read_startup_folder_entries() {
        for (name, command) in sf {
            entries.push(StartupEntry {
                disabled: false,
                source: "SF",
                name,
                command,
            });
        }
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║{:^60}", t!("autoup.list_title").cyan().bold());
    println!("╠══════════════════════════════════════════════════════════════════╣");

    if entries.is_empty() {
        println!("║  {}", t!("autoup.list_empty").dimmed());
    } else {
        for (i, entry) in entries.iter().enumerate() {
            let state_tag = if entry.disabled {
                "D".yellow().to_string()
            } else {
                "A".green().to_string()
            };
            println!(
                "║  {:<3} [{}][{}] {}",
                format!("{}.", i + 1).yellow(),
                state_tag,
                entry.source,
                entry.name,
            );
            println!("║      {}", entry.command.dimmed());
        }
    }

    let a_count = entries.iter().filter(|e| !e.disabled).count();
    let d_count = entries.iter().filter(|e| e.disabled).count();
    let cu_count = entries.iter().filter(|e| e.source == "CU").count();
    let lm_count = entries.iter().filter(|e| e.source == "LM").count();
    let sf_count = entries.iter().filter(|e| e.source == "SF").count();

    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!(
        "║  {}",
        t!(
            "autoup.list_total",
            entries.len().to_string().cyan(),
            a_count.to_string().green(),
            d_count.to_string().yellow(),
            cu_count,
            lm_count,
            sf_count
        )
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    Ok(false)
}

// ── Add command ─────────────────────────────────────────────────────────────

#[cfg(windows)]
fn cmd_add(args: &str) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("autoup.usage_add"));
        return Ok(false);
    }

    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let name = parts[0].trim();
    let value = parts.get(1).unwrap_or(&"").trim();

    if name.is_empty() || value.is_empty() {
        print_error(&t!("autoup.usage_add"));
        return Ok(false);
    }

    let value = value.strip_prefix('"').unwrap_or(value);
    let value = value.strip_suffix('"').unwrap_or(value);

    if value.is_empty() {
        print_error(&t!("autoup.cmd_empty"));
        return Ok(false);
    }

    if read_entry_value(RUN_KEY_CU, name)?.is_some() {
        print_warning(&t!("autoup.already_exists", name));
        print_info(&t!("autoup.use_rm_first"));
        return Ok(false);
    }

    write_reg_sz(RUN_KEY_CU, name, value)?;
    print_success(&t!("autoup.added", name, value));
    println!();
    print_info(&t!("autoup.will_run"));
    Ok(false)
}

// ── Remove command ──────────────────────────────────────────────────────────

#[cfg(windows)]
fn cmd_rm(args: &str) -> Result<bool> {
    let name = args.trim();
    if name.is_empty() {
        print_error(&t!("autoup.usage_rm"));
        return Ok(false);
    }

    if read_entry_value(RUN_KEY_CU, name)?.is_some() {
        let _ = delete_reg(APPROVED_KEY_CU, name);
        delete_reg(RUN_KEY_CU, name)?;
        print_success(&t!("autoup.removed", name));
    } else if read_entry_value(RUN_KEY_LM, name)?.is_some() {
        if is_admin() {
            let _ = delete_reg(APPROVED_KEY_LM, name);
            delete_reg(RUN_KEY_LM, name)?;
            print_success(&t!("autoup.removed_system", name));
        } else {
            print_error(&t!("autoup.hklm_hint", name));
        }
    } else if entry_in_startup_folder(name)? {
        print_error(&t!("autoup.sf_rm_hint", name));
    } else {
        print_error(&t!("autoup.not_found", name));
    }

    Ok(false)
}

// ── Disable command ─────────────────────────────────────────────────────────

#[cfg(windows)]
fn cmd_disable(args: &str) -> Result<bool> {
    let name = args.trim();
    if name.is_empty() {
        print_error(&t!("autoup.usage_disable"));
        return Ok(false);
    }

    let in_cu = read_entry_value(RUN_KEY_CU, name)?.is_some();
    let in_lm = read_entry_value(RUN_KEY_LM, name)?.is_some();

    if in_cu {
        let disabled = read_disabled_names(APPROVED_KEY_CU)?;
        if disabled.contains(name) {
            print_warning(&t!("autoup.already_disabled", name));
            return Ok(false);
        }
        set_disabled_state(APPROVED_KEY_CU, name, true)?;
        print_success(&t!("autoup.disabled", name));
    } else if in_lm {
        if is_admin() {
            let disabled = read_disabled_names(APPROVED_KEY_LM)?;
            if disabled.contains(name) {
                print_warning(&t!("autoup.already_disabled", name));
                return Ok(false);
            }
            set_disabled_state(APPROVED_KEY_LM, name, true)?;
            print_success(&t!("autoup.disabled_system", name));
        } else {
            print_error(&t!("autoup.hklm_hint_short", name));
        }
    } else if entry_in_startup_folder(name)? {
        print_error(&t!("autoup.sf_disable_hint", name));
    } else {
        print_error(&t!("autoup.not_found", name));
    }

    Ok(false)
}

// ── Enable command ──────────────────────────────────────────────────────────

#[cfg(windows)]
fn cmd_enable(args: &str) -> Result<bool> {
    let name = args.trim();
    if name.is_empty() {
        print_error(&t!("autoup.usage_enable"));
        return Ok(false);
    }

    let in_cu = read_entry_value(RUN_KEY_CU, name)?.is_some();
    let in_lm = read_entry_value(RUN_KEY_LM, name)?.is_some();

    if in_cu {
        let disabled = read_disabled_names(APPROVED_KEY_CU)?;
        if !disabled.contains(name) {
            print_warning(&t!("autoup.already_enabled", name));
            return Ok(false);
        }
        set_disabled_state(APPROVED_KEY_CU, name, false)?;
        print_success(&t!("autoup.enabled", name));
    } else if in_lm {
        if is_admin() {
            let disabled = read_disabled_names(APPROVED_KEY_LM)?;
            if !disabled.contains(name) {
                print_warning(&t!("autoup.already_enabled", name));
                return Ok(false);
            }
            set_disabled_state(APPROVED_KEY_LM, name, false)?;
            print_success(&t!("autoup.enabled_system", name));
        } else {
            print_error(&t!("autoup.hklm_hint_short", name));
        }
    } else {
        print_error(&t!("autoup.not_found", name));
    }

    Ok(false)
}

// ── Admin check ─────────────────────────────────────────────────────────────

#[cfg(windows)]
fn is_admin() -> bool {
    std::process::Command::new("whoami")
        .args(["/groups"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("S-1-16-12288"))
        .unwrap_or(false)
}

// ── Registry readers ────────────────────────────────────────────────────────

#[cfg(windows)]
fn read_run_key_entries(key: &str) -> Result<Vec<(String, String)>> {
    let output = std::process::Command::new("reg")
        .args(["query", key])
        .output()
        .map_err(|e| anyhow!("{}", t!("autoup.reg_query_fail", e)))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();
    for line in stdout.lines() {
        if let Some((name, value)) = parse_sz_line(line) {
            entries.push((name, value));
        }
    }
    Ok(entries)
}

#[cfg(windows)]
fn read_entry_value(key: &str, name: &str) -> Result<Option<String>> {
    let output = std::process::Command::new("reg")
        .args(["query", key, "/v", name])
        .output()
        .map_err(|e| anyhow!("{}", t!("autoup.reg_query_fail", e)))?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some((_, value)) = parse_sz_line(line) {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

#[cfg(windows)]
fn read_disabled_names(key: &str) -> Result<HashSet<String>> {
    let mut disabled = HashSet::new();
    let output = std::process::Command::new("reg")
        .args(["query", key])
        .output()
        .map_err(|e| anyhow!("{}", t!("autoup.reg_query_fail", e)))?;

    if !output.status.success() {
        return Ok(disabled);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some((name, hex)) = parse_binary_line(line) {
            if hex.starts_with("03") {
                disabled.insert(name);
            }
        }
    }
    Ok(disabled)
}

#[cfg(windows)]
fn read_startup_folder_entries() -> Result<Vec<(String, String)>> {
    let appdata = match std::env::var("APPDATA") {
        Ok(v) => v,
        Err(_) => return Ok(Vec::new()),
    };
    let startup_dir = Path::new(&appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup");

    let mut entries = Vec::new();
    if !startup_dir.exists() {
        return Ok(entries);
    }

    let read_dir = match std::fs::read_dir(&startup_dir) {
        Ok(d) => d,
        Err(_) => return Ok(entries),
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                entries.push((name.to_string(), path.to_string_lossy().to_string()));
            }
        }
    }

    Ok(entries)
}

#[cfg(windows)]
fn entry_in_startup_folder(name: &str) -> Result<bool> {
    let appdata = match std::env::var("APPDATA") {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };
    let startup_dir = Path::new(&appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup");
    if !startup_dir.exists() {
        return Ok(false);
    }
    Ok(startup_dir.join(format!("{}.lnk", name)).exists())
}

// ── Line parsers ────────────────────────────────────────────────────────────

#[cfg(windows)]
fn parse_sz_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    let marker = "REG_SZ";
    let idx = trimmed.find(marker)?;
    let name = trimmed[..idx].trim().to_string();
    let value = trimmed[idx + marker.len()..].trim().to_string();
    if name.is_empty() {
        return None;
    }
    Some((name, value))
}

#[cfg(windows)]
fn parse_binary_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    let marker = "REG_BINARY";
    let idx = trimmed.find(marker)?;
    let name = trimmed[..idx].trim().to_string();
    let raw = trimmed[idx + marker.len()..].trim().to_string();
    let hex = raw.replace(' ', "");
    if name.is_empty() || hex.is_empty() {
        return None;
    }
    Some((name, hex))
}

// ── Registry writers ────────────────────────────────────────────────────────

#[cfg(windows)]
fn write_reg_sz(key: &str, name: &str, value: &str) -> Result<()> {
    let status = std::process::Command::new("reg")
        .args(["add", key, "/v", name, "/t", "REG_SZ", "/d", value, "/f"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| anyhow!("{}", t!("autoup.reg_exec_fail", e)))?;

    if !status.success() {
        return Err(anyhow!("{}", t!("autoup.reg_write_fail")));
    }
    Ok(())
}

#[cfg(windows)]
fn delete_reg(key: &str, name: &str) -> Result<()> {
    let status = std::process::Command::new("reg")
        .args(["delete", key, "/v", name, "/f"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| anyhow!("{}", t!("autoup.reg_exec_fail", e)))?;

    if !status.success() {
        return Err(anyhow!("{}", t!("autoup.reg_delete_fail")));
    }
    Ok(())
}

#[cfg(windows)]
fn set_disabled_state(key: &str, name: &str, disabled: bool) -> Result<()> {
    let hex = if disabled {
        "030000000000000000000000"
    } else {
        "020000000000000000000000"
    };

    let status = std::process::Command::new("reg")
        .args(["add", key, "/v", name, "/t", "REG_BINARY", "/d", hex, "/f"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| anyhow!("{}", t!("autoup.reg_exec_fail", e)))?;

    if !status.success() {
        return Err(anyhow!("{}", t!("autoup.reg_disable_fail")));
    }
    Ok(())
}
