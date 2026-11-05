// src/utils/paths.rs
// Shared locations for files ntc owns under the platform config directory,
// plus `~` / `$VAR` expansion used by every path-taking command.

use crate::config::Config;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

/// Environment variable that overrides the platform config root.
///
/// It exists mainly so tests can point every on-disk store at a tempdir: on
/// Windows `dirs::config_dir()` calls the Known Folder API and ignores both
/// `APPDATA` and `XDG_CONFIG_HOME`, so env-based isolation silently failed.
/// It doubles as a portable-install escape hatch.
pub const CONFIG_HOME_ENV: &str = "NTC_CONFIG_HOME";

/// Platform config root, honoring [`CONFIG_HOME_ENV`].
pub fn config_home() -> PathBuf {
    std::env::var_os(CONFIG_HOME_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(dirs::config_dir)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// The user's home directory (`C:\Users\<name>` on Windows, `~` elsewhere).
pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir().filter(|p| !p.as_os_str().is_empty())
}

/// Session-only variables set by `config env NAME value -os`.
///
/// They shadow both ntc config stores for the lifetime of the shell process,
/// so `-os` is a true temporary override.
static SESSION_ENV: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn session_map() -> std::sync::MutexGuard<'static, HashMap<String, String>> {
    SESSION_ENV.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn set_session_var(name: &str, value: &str) {
    session_map().insert(name.to_string(), value.to_string());
    std::env::set_var(name, value);
}

pub fn remove_session_var(name: &str) -> bool {
    let removed = session_map().remove(name).is_some();
    std::env::remove_var(name);
    removed
}

pub fn session_var(name: &str) -> Option<String> {
    session_map().get(name).cloned()
}

pub fn session_env() -> HashMap<String, String> {
    session_map().clone()
}

/// Drop a session override (used when a config store is written for `name`).
pub fn clear_session_var(name: &str) -> bool {
    session_map().remove(name).is_some()
}

/// Look up an environment variable the way ntc resolves it:
/// session (`-os`) → local `ntconfig.toml` → global ntc config →
/// OS/process environment.
///
/// A case-insensitive (uppercase) retry follows the exact lookup so
/// `$home` still finds `HOME` on case-sensitive platforms.
pub fn resolve_env_var(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    if let Some(v) = session_var(name) {
        return Some(v);
    }
    let (global, local) = Config::get_env_by_source();
    if let Some(v) = local.get(name) {
        return Some(v.clone());
    }
    if let Some(v) = global.get(name) {
        return Some(v.clone());
    }
    if let Ok(v) = std::env::var(name) {
        return Some(v);
    }
    let upper = name.to_uppercase();
    if upper != name {
        if let Some(v) = session_var(&upper) {
            return Some(v);
        }
        if let Some(v) = local.get(&upper) {
            return Some(v.clone());
        }
        if let Some(v) = global.get(&upper) {
            return Some(v.clone());
        }
        if let Ok(v) = std::env::var(&upper) {
            return Some(v);
        }
    }
    None
}

/// Expand `~`, `$NAME`, `${NAME}` (and `%NAME%` on Windows) in a path-like
/// string. Surrounding single/double quotes are stripped. Unknown variables
/// are left literal so the caller's normal not-found error is shown.
pub fn expand_path_str(input: &str) -> String {
    let unquoted = strip_quotes(input.trim());
    let expanded = expand_env_refs(unquoted);
    expand_tilde(&expanded)
}

/// [`expand_path_str`] for an existing `Path`.
pub fn expand_path_buf(path: &Path) -> PathBuf {
    PathBuf::from(expand_path_str(&path.to_string_lossy()))
}

fn strip_quotes(s: &str) -> &str {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &s[1..s.len() - 1];
        }
    }
    s
}

fn expand_tilde(s: &str) -> String {
    if let Some(rest) = s.strip_prefix("~/").or_else(|| s.strip_prefix("~\\")) {
        if let Some(home) = home_dir() {
            return home.join(rest).to_string_lossy().into_owned();
        }
    } else if s == "~" {
        if let Some(home) = home_dir() {
            return home.to_string_lossy().into_owned();
        }
    }
    s.to_string()
}

fn is_name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Expand `$NAME`, `${NAME}` (and `%NAME%` on Windows) anywhere in `s`.
/// Unknown names are left literal, so text that is not a variable reference
/// survives untouched.
pub(crate) fn expand_env_refs(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '$' {
            if let Some(name) = dollar_name(&chars, i) {
                let close = name_close(&chars, i);
                if let Some(value) = resolve_env_var(&name) {
                    out.push_str(&value);
                } else {
                    out.extend(&chars[i..close]);
                }
                i = close;
                continue;
            }
            out.push(c);
            i += 1;
        } else if cfg!(windows) && c == '%' {
            if let Some(name) = percent_name(&chars, i) {
                let close = i + 2 + name.chars().count();
                if let Some(value) = resolve_env_var(&name) {
                    out.push_str(&value);
                } else {
                    out.extend(&chars[i..close]);
                }
                i = close;
                continue;
            }
            out.push(c);
            i += 1;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// Parse `$NAME` or `${NAME}` at `i`; returns the variable name only.
fn dollar_name(chars: &[char], i: usize) -> Option<String> {
    let mut j = i + 1;
    if j >= chars.len() {
        return None;
    }
    if chars[j] == '{' {
        j += 1;
        let start = j;
        while j < chars.len() && chars[j] != '}' {
            j += 1;
        }
        if j <= start || j >= chars.len() {
            return None;
        }
        let name: String = chars[start..j].iter().collect();
        if !name.chars().next().is_some_and(is_name_start)
            || !name
                .chars()
                .all(|c| is_name_char(c) || c == '.' || c == '-')
        {
            return None;
        }
        Some(name)
    } else {
        if !is_name_start(chars[j]) {
            return None;
        }
        let start = j;
        while j < chars.len() && is_name_char(chars[j]) {
            j += 1;
        }
        Some(chars[start..j].iter().collect())
    }
}

/// Index just past the reference starting at `i`.
fn name_close(chars: &[char], i: usize) -> usize {
    let mut j = i + 1;
    if j < chars.len() && chars[j] == '{' {
        while j < chars.len() && chars[j] != '}' {
            j += 1;
        }
        return (j + 1).min(chars.len());
    }
    while j < chars.len() && is_name_char(chars[j]) {
        j += 1;
    }
    j
}

/// Parse `%NAME%` at `i`; returns the variable name only.
fn percent_name(chars: &[char], i: usize) -> Option<String> {
    let start = i + 1;
    if start >= chars.len() || !is_name_start(chars[start]) {
        return None;
    }
    let mut j = start;
    while j < chars.len() && is_name_char(chars[j]) {
        j += 1;
    }
    if j < chars.len() && chars[j] == '%' {
        Some(chars[start..j].iter().collect())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_home_honors_the_override() {
        let _home = crate::utils::test_support::IsolatedConfigHome::new();
        assert_eq!(
            config_home(),
            std::env::var_os(CONFIG_HOME_ENV)
                .map(PathBuf::from)
                .unwrap()
        );
    }

    #[test]
    fn tilde_expands_to_home() {
        let Some(home) = home_dir() else {
            return;
        };
        assert_eq!(expand_path_str("~"), home.to_string_lossy());
        assert_eq!(expand_path_str("~/sub"), home.join("sub").to_string_lossy());
        assert_eq!(
            expand_path_str("~\\sub"),
            home.join("sub").to_string_lossy()
        );
        assert_eq!(
            expand_path_str("  \"~/quoted\"  "),
            home.join("quoted").to_string_lossy()
        );
    }

    #[test]
    fn dollar_and_braced_env_expand() {
        std::env::set_var("NTC_TEST_PATH_EXPAND", "C:/tmp/x");
        // Use a unique name and restore it so parallel tests are unaffected.
        assert_eq!(expand_path_str("$NTC_TEST_PATH_EXPAND"), "C:/tmp/x");
        assert_eq!(expand_path_str("${NTC_TEST_PATH_EXPAND}/y"), "C:/tmp/x/y");
        std::env::remove_var("NTC_TEST_PATH_EXPAND");
    }

    #[test]
    fn unknown_env_stays_literal() {
        std::env::remove_var("NTC_TEST_NO_SUCH_VAR_XYZ");
        assert_eq!(
            expand_path_str("$NTC_TEST_NO_SUCH_VAR_XYZ/a"),
            "$NTC_TEST_NO_SUCH_VAR_XYZ/a"
        );
        assert_eq!(
            expand_path_str("${NTC_TEST_NO_SUCH_VAR_XYZ}"),
            "${NTC_TEST_NO_SUCH_VAR_XYZ}"
        );
    }

    #[test]
    fn bare_dollar_is_untouched() {
        assert_eq!(expand_path_str("cost$5"), "cost$5");
        assert_eq!(expand_path_str("a$ b"), "a$ b");
    }

    #[test]
    fn ntc_config_env_wins_over_os() {
        std::env::set_var("NTC_TEST_PRECEDENCE", "os");
        let mut cfg = Config::new();
        cfg.env
            .insert("NTC_TEST_PRECEDENCE".to_string(), "ntc".to_string());
        Config::with_test_config(cfg, || {
            assert_eq!(
                resolve_env_var("NTC_TEST_PRECEDENCE").as_deref(),
                Some("ntc")
            );
        });
        std::env::remove_var("NTC_TEST_PRECEDENCE");
    }
}
