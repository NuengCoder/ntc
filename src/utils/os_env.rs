//! OS-environment persistence used by `config env NAME value -osg`.
//!
//! Windows: the per-user `HKCU\Environment` registry key (same mechanism as
//! `usrpath`). Unix: `~/.config/ntc/env.sh`; ntc never edits shell rc files —
//! it prints the `source` line the user can add to their own profile.

use anyhow::Result;

#[cfg(unix)]
use std::path::PathBuf;

#[cfg(unix)]
use std::collections::BTreeMap;

#[cfg(windows)]
const USER_ENV_KEY: &str = r"HKCU\Environment";

// ============================================================================
// Windows registry
// ============================================================================

#[cfg(windows)]
fn registry_read_value(name: &str) -> Option<String> {
    let output = std::process::Command::new("reg")
        .args(["query", USER_ENV_KEY, "/v", name])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if !trimmed.to_lowercase().starts_with(&name.to_lowercase()) {
            continue;
        }
        let rest = trimmed[name.len()..].trim_start();
        // `<type>    <value>` — the value may contain spaces.
        let value = match rest.find(char::is_whitespace) {
            Some(pos) => rest[pos..].trim_start(),
            None => "",
        };
        return Some(value.to_string());
    }
    None
}

#[cfg(windows)]
fn registry_write_value(name: &str, value: &str) -> Result<()> {
    let kind = if value.contains('%') {
        "REG_EXPAND_SZ"
    } else {
        "REG_SZ"
    };
    let output = std::process::Command::new("reg")
        .args([
            "add",
            USER_ENV_KEY,
            "/v",
            name,
            "/t",
            kind,
            "/d",
            value,
            "/f",
        ])
        .output()?;
    if !output.status.success() {
        anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr).trim());
    }
    Ok(())
}

#[cfg(windows)]
fn registry_delete_value(name: &str) -> Result<bool> {
    let output = std::process::Command::new("reg")
        .args(["delete", USER_ENV_KEY, "/v", name, "/f"])
        .output()?;
    Ok(output.status.success())
}

#[cfg(windows)]
pub fn set_os_var(name: &str, value: &str) -> Result<()> {
    registry_write_value(name, value)
}

#[cfg(windows)]
pub fn remove_os_var(name: &str) -> Result<bool> {
    registry_delete_value(name)
}

#[cfg(windows)]
pub fn os_var(name: &str) -> Option<String> {
    registry_read_value(name)
}

/// Human-readable location of the persistent OS store.
#[cfg(windows)]
pub fn os_store_location() -> String {
    format!(r"{USER_ENV_KEY} (user environment)")
}

#[cfg(windows)]
pub fn shell_wiring_hint() -> Option<String> {
    None
}

// ============================================================================
// Unix: ~/.config/ntc/env.sh
// ============================================================================

#[cfg(unix)]
fn env_sh_path() -> PathBuf {
    crate::utils::paths::config_home()
        .join("ntc")
        .join("env.sh")
}

#[cfg(unix)]
fn read_env_sh() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let Ok(content) = std::fs::read_to_string(env_sh_path()) else {
        return map;
    };
    for line in content.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("export ") else {
            continue;
        };
        let Some((name, raw)) = rest.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        let value = raw.trim();
        let value = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\''));
        let value = value.unwrap_or(raw).replace("'\\''", "'");
        map.insert(name.to_string(), value);
    }
    map
}

#[cfg(unix)]
fn quote_shell(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(unix)]
fn write_env_sh(vars: &BTreeMap<String, String>) -> Result<()> {
    let path = env_sh_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut out = String::from("# managed by ntc — `config env NAME value -osg`\n");
    for (name, value) in vars {
        out.push_str(&format!("export {}={}\n", name, quote_shell(value)));
    }
    std::fs::write(path, out)?;
    Ok(())
}

#[cfg(unix)]
pub fn set_os_var(name: &str, value: &str) -> Result<()> {
    let mut vars = read_env_sh();
    vars.insert(name.to_string(), value.to_string());
    write_env_sh(&vars)
}

#[cfg(unix)]
pub fn remove_os_var(name: &str) -> Result<bool> {
    let mut vars = read_env_sh();
    let removed = vars.remove(name).is_some();
    if removed {
        write_env_sh(&vars)?;
    }
    Ok(removed)
}

#[cfg(unix)]
pub fn os_var(name: &str) -> Option<String> {
    read_env_sh().get(name).cloned()
}

/// Human-readable location of the persistent OS store.
#[cfg(unix)]
pub fn os_store_location() -> String {
    env_sh_path().to_string_lossy().into_owned()
}

/// The line the user can add to their shell rc to load the store in new
/// shells. ntc never edits rc files itself.
#[cfg(unix)]
pub fn shell_wiring_hint() -> Option<String> {
    Some(format!(
        ". \"{}\"",
        env_sh_path().to_string_lossy().replace('\\', "/")
    ))
}

// ============================================================================
// Shared helpers
// ============================================================================

/// Variables that must not be overwritten/removed through `config env`
/// (PATH has `usrpath`/`syspath`; the rest would break the shell).
pub const PROTECTED_NAMES: &[&str] = &[
    "PATH",
    "PATHEXT",
    "SYSTEMROOT",
    "WINDIR",
    "HOME",
    "USERPROFILE",
    "USERNAME",
    "COMSPEC",
];

pub fn is_protected(name: &str) -> bool {
    PROTECTED_NAMES.iter().any(|p| p.eq_ignore_ascii_case(name))
}

/// Valid NTC variable name: `[A-Za-z_][A-Za-z0-9_]*`.
pub fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_names() {
        assert!(is_valid_name("HOME"));
        assert!(is_valid_name("_x1"));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name("1X"));
        assert!(!is_valid_name("A-B"));
        assert!(!is_valid_name("A B"));
    }

    #[test]
    fn protects_core_variables() {
        assert!(is_protected("PATH"));
        assert!(is_protected("path"));
        assert!(is_protected("HOME"));
        assert!(!is_protected("MY_VAR"));
    }
}
