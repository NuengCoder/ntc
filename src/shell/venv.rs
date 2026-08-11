// src/shell/venv.rs
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

/// Detect if currently inside a Python virtual environment
pub fn detect_active_venv() -> Option<String> {
    if let Ok(venv_path) = env::var("VIRTUAL_ENV") {
        let path = Path::new(&venv_path);
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            return Some(name.to_string());
        }
    }
    None
}

/// Check if directory contains a virtual environment
pub fn has_venv_in_dir(dir: &Path) -> bool {
    let venv_names = &["venv", ".venv", "env"];
    for name in venv_names {
        let venv_path = dir.join(name);
        #[cfg(windows)]
        if venv_path.join("Scripts").exists() {
            return true;
        }
        #[cfg(not(windows))]
        if venv_path.join("bin").exists() {
            return true;
        }
    }
    false
}

/// Get the activate script path
pub fn get_activate_script_path(venv_dir: &Path) -> Option<PathBuf> {
    #[cfg(windows)]
    {
        let activate = venv_dir.join("Scripts").join("activate.bat");
        if activate.exists() {
            return Some(activate);
        }
    }

    #[cfg(not(windows))]
    {
        let activate = venv_dir.join("bin").join("activate");
        if activate.exists() {
            return Some(activate);
        }
    }

    None
}

/// Parse Windows batch activate script and extract environment changes
#[cfg(windows)]
pub fn parse_batch_activate(script_path: &Path) -> Result<HashMap<String, String>, String> {
    use std::fs;
    let content = fs::read_to_string(script_path)
        .map_err(|e| format!("Failed to read activate script: {}", e))?;

    let mut env_changes = HashMap::new();
    let venv_dir = script_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| "Cannot determine venv directory from activate script path".to_string())?;
    let scripts_dir = venv_dir.join("Scripts");

    // Get the original PATH before modification
    let original_path = env::var("PATH").unwrap_or_default();

    // The new PATH should have venv/Scripts at the front
    // Save original PATH for deactivation
    // Set VIRTUAL_ENV
    let new_path = format!("{};{}", scripts_dir.display(), original_path);
    env_changes.insert("PATH".to_string(), new_path);
    env_changes.insert("_OLD_VIRTUAL_PATH".to_string(), original_path);
    env_changes.insert(
        "VIRTUAL_ENV".to_string(),
        venv_dir.to_string_lossy().to_string(),
    );

    // Parse other environment variables from the script
    for line in content.lines() {
        let line = line.trim();

        // Look for "set VAR=VALUE" lines
        if line.starts_with("set ") && !line.contains("%") {
            let parts: Vec<&str> = line[4..].splitn(2, '=').collect();
            if parts.len() == 2 {
                let var = parts[0].trim();
                let val = parts[1].trim();
                // Skip PATH as we already handled it
                if var != "PATH" && !var.is_empty() {
                    env_changes.insert(var.to_string(), val.to_string());
                }
            }
        }
    }

    Ok(env_changes)
}

/// Parse Unix bash activate script
#[cfg(not(windows))]
pub fn parse_bash_activate(script_path: &Path) -> Result<HashMap<String, String>, String> {
    use std::fs;
    let content = fs::read_to_string(script_path)
        .map_err(|e| format!("Failed to read activate script: {}", e))?;

    let mut env_changes = HashMap::new();
    let venv_dir = script_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| "Cannot determine venv directory from activate script path".to_string())?;
    let bin_dir = venv_dir.join("bin");

    // Get original PATH
    let original_path = env::var("PATH").unwrap_or_default();

    // New PATH with venv/bin at front
    let new_path = format!("{}:{}", bin_dir.display(), original_path);
    env_changes.insert("PATH".to_string(), new_path);
    env_changes.insert("_OLD_VIRTUAL_PATH".to_string(), original_path);
    env_changes.insert(
        "VIRTUAL_ENV".to_string(),
        venv_dir.to_string_lossy().to_string(),
    );

    // Parse export statements
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("export ") && !line.starts_with("export PATH") {
            let parts: Vec<&str> = line[7..].splitn(2, '=').collect();
            if parts.len() == 2 {
                let var = parts[0].trim();
                let val = parts[1].trim().trim_matches(&['"', '\''][..]);
                if !var.is_empty() {
                    env_changes.insert(var.to_string(), val.to_string());
                }
            }
        }
    }

    Ok(env_changes)
}

/// Apply environment changes to current process
pub fn apply_venv_environment(changes: HashMap<String, String>) {
    for (key, value) in changes {
        env::set_var(key, value);
    }
}

/// Clear venv-related environment variables (deactivate)
pub fn clear_venv_environment() {
    // Restore original PATH
    if let Ok(original_path) = env::var("_OLD_VIRTUAL_PATH") {
        env::set_var("PATH", original_path);
        env::remove_var("_OLD_VIRTUAL_PATH");
    }

    // Remove venv-specific variables
    env::remove_var("VIRTUAL_ENV");
    env::remove_var("PYTHONHOME");

    // On Windows, also clear these if they exist
    env::remove_var("_OLD_VIRTUAL_PYTHONHOME");
}
