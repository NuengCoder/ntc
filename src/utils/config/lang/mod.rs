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
