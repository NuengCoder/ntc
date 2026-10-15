//! `nssh` profile store: named SSH targets in `~/.config/ntc/nssh.toml`.
//!
//! ```toml
//! [profiles.prod]
//! host = "203.0.113.10"
//! user = "root"
//! port = 22
//! key = "~/.ssh/id_rsa"      # optional
//! pass_env = "PROD_PASS"     # optional: env-var name only, never plaintext
//! ```
//!
//! Passwords are never persisted. File is created with 0600 permissions on
//! Unix so hostnames/usernames don't leak to other users either.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// One saved SSH target.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    #[serde(default)]
    pub host: String,
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass_env: Option<String>,
}

fn default_user() -> String {
    whoami_fallback()
}

fn default_port() -> u16 {
    22
}

/// Current login name for the `user` default.
fn whoami_fallback() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "root".to_string())
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct StoreFile {
    #[serde(default)]
    profiles: BTreeMap<String, Profile>,
}

/// Filesystem home of the store (`~/.config/ntc/nssh.toml`).
pub fn store_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("nssh.toml")
}

/// Profile names must be simple tokens (also keeps shell completion sane).
pub fn valid_profile_name(name: &str) -> bool {
    !name.trim().is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
}

fn load_file() -> Result<StoreFile, String> {
    let path = store_path();
    if !path.exists() {
        return Ok(StoreFile::default());
    }
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read store: {}", e))?;
    toml::from_str(&text).map_err(|e| format!("store is not valid TOML: {}", e))
}

fn save_file(store: &StoreFile) -> Result<(), String> {
    let path = store_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create dir: {}", e))?;
    }
    let text = toml::to_string_pretty(store).map_err(|e| format!("cannot encode store: {}", e))?;
    std::fs::write(&path, text).map_err(|e| format!("cannot write store: {}", e))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

/// Expand a leading `~` against the home directory.
pub fn expand_home(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/").or_else(|| path.strip_prefix('~')) {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest.trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}

/// Default private-key candidates probed when a profile sets no `key`.
pub fn default_key_candidates() -> Vec<PathBuf> {
    ["~/.ssh/id_ed25519", "~/.ssh/id_rsa", "~/.ssh/id_ecdsa"]
        .into_iter()
        .map(expand_home)
        .filter(|p| p.is_file())
        .collect()
}

pub fn list_profiles() -> Result<BTreeMap<String, Profile>, String> {
    Ok(load_file()?.profiles)
}

pub fn get_profile(name: &str) -> Result<Option<Profile>, String> {
    Ok(load_file()?.profiles.get(name).cloned())
}

pub fn save_profile(name: &str, profile: &Profile) -> Result<(), String> {
    if !valid_profile_name(name) {
        return Err(format!(
            "invalid profile name '{}' (letters, digits, _ - . only)",
            name
        ));
    }
    let mut store = load_file()?;
    store.profiles.insert(name.to_string(), profile.clone());
    save_file(&store)
}

pub fn delete_profile(name: &str) -> Result<bool, String> {
    let mut store = load_file()?;
    let removed = store.profiles.remove(name).is_some();
    if removed {
        save_file(&store)?;
    }
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_names() {
        assert!(valid_profile_name("prod"));
        assert!(valid_profile_name("web-01.example"));
        assert!(valid_profile_name("a_b"));
        assert!(!valid_profile_name(""));
        assert!(!valid_profile_name("has space"));
        assert!(!valid_profile_name("semi;colon"));
        assert!(!valid_profile_name("a/b"));
    }

    #[test]
    fn expand_home_tilde() {
        let p = expand_home("~/.ssh/id_rsa");
        assert!(
            p.to_string_lossy().ends_with(".ssh/id_rsa")
                || p.to_string_lossy().ends_with(".ssh\\id_rsa")
        );
        assert_eq!(expand_home("/abs/path"), PathBuf::from("/abs/path"));
    }
}
