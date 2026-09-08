//! Language/localization system for ntc.
//! Provides a global language pack loaded from built-in English defaults
//! with optional external `.ntc_lang` overrides from `~/.config/ntc/lang/`.

pub mod en;
pub mod th;
pub mod lo;
pub mod zh;

use std::collections::HashMap;
use std::sync::RwLock;

/// Macro to get a localized string by dot-notation key.
///
/// # Examples
/// ```
/// t!("help.nav.title")           // -> String
/// t!("config.lang.set", "thai")  // -> String with format arg
/// ```
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::utils::config::lang::get($key)
    };
    ($key:expr $(, $arg:expr)* $(,)?) => {
        $crate::utils::config::lang::format_get($key, &[$($arg.to_string()),*])
    };
}

/// Flatten nested TOML tables into dot-notation keys, merging into `map`.
fn flatten_toml(value: &toml::Value, prefix: &str, map: &mut HashMap<String, String>) {
    match value {
        toml::Value::Table(table) => {
            for (k, v) in table {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_toml(v, &new_prefix, map);
            }
        }
        toml::Value::String(s) => {
            map.insert(prefix.to_string(), s.clone());
        }
        _ => {}
    }
}

/// Load a language pack: start with built-in English defaults,
/// then merge external `.ntc_lang` if available.
pub fn load(code: &str) -> HashMap<String, String> {
    let mut map = match code {
        "chinese" => zh::build_map(),
        "thai" => th::build_map(),
        "lao" => lo::build_map(),
        _ => en::build_map(),
    };

    let lang_dir = dirs::config_dir()
        .map(|d| d.join("ntc").join("lang"));
    if let Some(dir) = lang_dir {
        let lang_file = dir.join(format!("{}.ntc_lang", code));
        if lang_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&lang_file) {
                if let Ok(toml_value) = content.parse::<toml::Value>() {
                    flatten_toml(&toml_value, "", &mut map);
                }
            }
        }
    }

    map
}

/// Global language pack singleton.
static LANG_PACK: std::sync::LazyLock<RwLock<HashMap<String, String>>> =
    std::sync::LazyLock::new(|| RwLock::new(en::build_map()));

/// Directly set the global language pack (no GLOBAL_CONFIG access — safe during boot).
pub fn set_map(map: HashMap<String, String>) {
    if let Ok(mut guard) = LANG_PACK.write() {
        *guard = map;
    }
}

/// Re-initialize the global language pack from the current config setting.
pub fn reload() {
    let code = crate::utils::config::Config::global_get_language();
    let map = load(&code);
    set_map(map);
}

/// Get a localized string by key. Falls back to the key itself if not found.
pub fn get(key: &str) -> String {
    if let Ok(guard) = LANG_PACK.read() {
        guard
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    } else {
        key.to_string()
    }
}

/// Get a localized string with runtime `{}` substitution.
/// Replaces `{}` in the template with the given args in order.
pub fn format_get(key: &str, args: &[String]) -> String {
    let template = get(key);
    if args.is_empty() {
        return template;
    }
    let mut result = String::with_capacity(template.len());
    let mut rest = template.as_str();
    for arg in args {
        if let Some(pos) = rest.find("{}") {
            result.push_str(&rest[..pos]);
            result.push_str(arg);
            rest = &rest[pos + 2..];
        } else {
            result.push_str(rest);
            rest = "";
            break;
        }
    }
    result.push_str(rest);
    result
}

/// List all available language codes: built-in + external `.ntc_lang` files.
pub fn available_languages() -> Vec<String> {
    let mut langs: Vec<String> = vec![
        "english".to_string(),
        "thai".to_string(),
        "lao".to_string(),
        "chinese".to_string(),
    ];

    if let Some(dir) = dirs::config_dir().map(|d| d.join("ntc").join("lang")) {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |e| e == "ntc_lang") {
                        if let Some(stem) = path.file_stem() {
                            let code = stem.to_string_lossy().to_string();
                            if !langs.contains(&code) {
                                langs.push(code);
                            }
                        }
                    }
                }
            }
        }
    }

    langs
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    /// Keys referenced by the Termux / npm-guide / cloudflared features.
    /// Every key must exist in ALL four bundled language packs so that
    /// non-English users never see a raw key string.
    const REQUIRED_KEYS: &[&str] = &[
        // Termux pkg recommendation
        "ntpkg.termux_use_pkg",
        "ntpkg.termux_cmd",
        "ntpkg.termux_no_pkg",
        // npm guide
        "ntpkg.npm_section_node_first",
        "ntpkg.npm_node_found_at",
        "ntpkg.npm_node_not_found",
        "ntpkg.npm_install_via_pkg",
        "ntpkg.npm_install_via_ntpkg",
        "ntpkg.npm_ntpkg_cmd",
        "ntpkg.npm_cmd",
        "ntpkg.npm_or_download_nodejs",
        "ntpkg.npm_or_fnm_nvm_volta",
        "ntpkg.npm_found_at",
        "ntpkg.npm_not_found",
        "ntpkg.npm_then_run",
        "ntpkg.npm_then_rerun",
        "ntpkg.npm_bin_not_in_path",
        "ntpkg.path_add_label",
        "ntpkg.npm_install_section",
        "ntpkg.npm_run_cmd_label",
        // cloudflared tunnel
        "cmd.serve.cloudflared_starting",
        "cmd.serve.cloudflared_missing",
    ];

    fn assert_all_present(map: HashMap<String, String>, lang: &str) {
        assert_keys_present(map, lang, REQUIRED_KEYS);
    }

    fn assert_keys_present(map: HashMap<String, String>, lang: &str, keys: &[&str]) {
        let missing: Vec<&str> = keys
            .iter()
            .copied()
            .filter(|k| !map.contains_key(*k))
            .collect();
        assert!(
            missing.is_empty(),
            "{lang} pack is missing keys: {:?}",
            missing
        );
    }

    #[test]
    fn termux_and_cloudflared_keys_exist_in_all_languages() {
        assert_all_present(super::en::build_map(), "english");
        assert_all_present(super::zh::build_map(), "chinese");
        assert_all_present(super::th::build_map(), "thai");
        assert_all_present(super::lo::build_map(), "lao");
    }

    /// Every `scr` script-runner key must exist in all four bundled language packs.
    const SCR_KEYS: &[&str] = &[
        "help.scr.continue",
        "help.scr.example1",
        "help.scr.example2",
        "help.scr.example3",
        "help.scr.help",
        "help.scr.init",
        "help.scr.ls",
        "help.scr.run",
        "help.scr.title",
        "scr.calling_not_allowed",
        "scr.cmd_failed",
        "scr.completed",
        "scr.continue_warn",
        "scr.desc_args",
        "scr.desc_force",
        "scr.desc_interp",
        "scr.desc_set",
        "scr.done_errors",
        "scr.error_at",
        "scr.executing",
        "scr.failed_abort",
        "scr.help_continue",
        "scr.help_help",
        "scr.help_init",
        "scr.help_ls",
        "scr.help_run",
        "scr.help_title",
        "scr.help_usage",
        "scr.init_created",
        "scr.init_exists",
        "scr.ls_empty",
        "scr.ls_title",
        "scr.no_commands",
        "scr.not_found",
        "scr.start",
        "scr.syscmd",
        "scr.syscmd_exit",
        "scr.syscmd_terminated",
    ];

    #[test]
    fn scr_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SCR_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SCR_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SCR_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SCR_KEYS);
    }

    /// Every `git` integration key must exist in all four bundled language packs.
    const GIT_KEYS: &[&str] = &[
        "git.ahead",
        "git.behind",
        "git.branch_failed",
        "git.branch_header",
        "git.checking_status",
        "git.clean",
        "git.completed",
        "git.detached_header",
        "git.executing",
        "git.exit_code",
        "git.failed",
        "git.help_branch",
        "git.help_diff",
        "git.help_example1",
        "git.help_example2",
        "git.help_log",
        "git.help_passthrough",
        "git.help_status",
        "git.help_title",
        "git.examples_header",
        "git.log_empty",
        "git.log_failed",
        "git.no_git_binary",
        "git.not_repo",
        "git.section_conflicts",
        "git.section_staged",
        "git.section_unstaged",
        "git.section_untracked",
        "git.status_failed",
        "git.terminated",
        "git.upstream",
        "editor.git_disabled",
        "editor.git_no_blame",
        "editor.git_no_diff",
        "editor.git_quit_hint",
        "editor.mode_git_diff",
        "help.config.git",
        "help.git.branch",
        "help.git.diff",
        "help.git.example1",
        "help.git.example2",
        "help.git.example3",
        "help.git.log",
        "help.git.passthrough",
        "help.git.status",
        "help.git.title",
        "msg.setGit.error",
        "msg.setGit.off",
        "msg.setGit.on",
        "msg.setGit.status",
    ];

    #[test]
    fn git_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", GIT_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", GIT_KEYS);
        assert_keys_present(super::th::build_map(), "thai", GIT_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", GIT_KEYS);
    }

    /// Every `dir_view` config key must exist in all four bundled language packs.
    const DIR_VIEW_KEYS: &[&str] = &[
        "help.config.dir_view",
        "msg.setDV.status",
        "msg.setDV.on",
        "msg.setDV.off",
        "msg.setDV.error",
        "msg.showcg.label_dir_view",
    ];

    #[test]
    fn dir_view_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", DIR_VIEW_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", DIR_VIEW_KEYS);
        assert_keys_present(super::th::build_map(), "thai", DIR_VIEW_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", DIR_VIEW_KEYS);
    }
}
