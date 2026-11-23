//! `config env` / `config rmenv` — local, global, session, and OS variables.

use crate::config::Config;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use crate::utils::{os_env, paths};
use anyhow::Result;

// ============================================================================
// config env / config rmenv
// ============================================================================

#[derive(Debug, Default, Clone, Copy)]
struct EnvFlags {
    global: bool,
    local: bool,
    os: bool,
    osg: bool,
    delete: bool,
    all: bool,
}

impl EnvFlags {
    fn scope_conflict(self) -> bool {
        (self.global as u8 + self.local as u8) > 1
            || (self.os as u8 + self.osg as u8) > 1
            || (self.all && (self.global || self.local || self.os || self.osg))
    }
}

/// Quote-aware tokenizer for `config env` (values may contain spaces).
fn split_env_tokens(input: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    for c in input.chars() {
        match quote {
            Some(q) => {
                if c == q {
                    quote = None;
                } else {
                    cur.push(c);
                }
            }
            None => {
                if c == '"' || c == '\'' {
                    quote = Some(c);
                } else if c.is_whitespace() {
                    if !cur.is_empty() {
                        out.push(std::mem::take(&mut cur));
                    }
                } else {
                    cur.push(c);
                }
            }
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Split trailing `-g/-l/-os/-osg/-d/-all` flags from the value tokens.
fn parse_env_flags(input: &str) -> (EnvFlags, Vec<String>) {
    let mut flags = EnvFlags::default();
    let mut kept: Vec<String> = Vec::new();
    for tok in split_env_tokens(input) {
        match tok.to_lowercase().as_str() {
            "-g" | "--global" => flags.global = true,
            "-l" | "--local" => flags.local = true,
            "-os" | "--os" | "--session" => flags.os = true,
            "-osg" | "--os-global" | "--os-persist" => flags.osg = true,
            "-d" | "--delete" | "-rm" | "--rm" => flags.delete = true,
            "-a" | "--all" => flags.all = true,
            _ => kept.push(tok),
        }
    }
    (flags, kept)
}

fn env_store_label(store: &str) -> String {
    t!(format!("cmd.config.env.store_{}", store).as_str())
}

fn env_origin(name: &str) -> Option<(&'static str, String)> {
    if let Some(v) = paths::session_var(name) {
        return Some(("session", v));
    }
    let (global, local) = Config::get_env_by_source();
    if let Some(v) = local.get(name) {
        return Some(("local", v.clone()));
    }
    if let Some(v) = global.get(name) {
        return Some(("global", v.clone()));
    }
    if let Some(v) = os_env::os_var(name) {
        return Some(("os", v));
    }
    if let Ok(v) = std::env::var(name) {
        return Some(("process", v));
    }
    None
}

fn env_list() {
    let (global, local) = Config::get_env_by_source();
    let session = paths::session_env();

    println!("{}", t!("cmd.config.env.title").cyan().bold());
    let mut any = false;
    for (store, map) in [
        ("local", &local),
        ("global", &global),
        ("session", &session),
    ] {
        if map.is_empty() {
            continue;
        }
        any = true;
        let mut names: Vec<&String> = map.keys().collect();
        names.sort();
        println!("  {}", env_store_label(store).yellow().bold());
        for name in names {
            println!(
                "    {} = {}",
                name.cyan(),
                map.get(name)
                    .map(String::as_str)
                    .unwrap_or_default()
                    .dimmed()
            );
        }
    }
    if !any {
        println!("  {}", t!("cmd.config.env.empty").dimmed());
    }
    println!();
    println!("{}", t!("cmd.config.env.list_hint").green());
}

fn env_show(name: &str) {
    match env_origin(name) {
        Some((store, value)) => println!(
            "{}",
            t!("cmd.config.env.get", name, value, env_store_label(store))
        ),
        None => println!("{}", t!("cmd.config.env.not_set", name).dimmed()),
    }
}

fn env_validate(name: &str) -> bool {
    if os_env::is_protected(name) {
        print_error(&t!("cmd.config.env.protected", name));
        return false;
    }
    if !os_env::is_valid_name(name) {
        print_error(&t!("cmd.config.env.invalid_name", name));
        return false;
    }
    true
}

fn env_set(name: &str, value: &str, flags: EnvFlags) -> Result<bool> {
    if !env_validate(name) {
        return Ok(false);
    }

    if flags.osg {
        if let Err(e) = os_env::set_os_var(name, value) {
            print_error(&t!("cmd.config.env.os_fail", e));
            return Ok(false);
        }
        paths::set_session_var(name, value);
        print_success(&t!(
            "cmd.config.env.set_os",
            name,
            value,
            os_env::os_store_location()
        ));
        if let Some(hint) = os_env::shell_wiring_hint() {
            println!("{}", t!("cmd.config.env.os_wiring", hint).dimmed());
        }
        return Ok(false);
    }

    if flags.os {
        paths::set_session_var(name, value);
        print_success(&t!(
            "cmd.config.env.set",
            name,
            value,
            env_store_label("session")
        ));
        return Ok(false);
    }

    let store = if flags.global {
        Config::set_global_env_var(name, value);
        "global"
    } else if flags.local {
        if !Config::set_local_env_var(name, value) {
            print_error(&t!("cmd.config.env.no_local"));
            return Ok(false);
        }
        "local"
    } else if Config::get_local_config_path().is_some() {
        if Config::set_local_env_var(name, value) {
            "local"
        } else {
            Config::set_global_env_var(name, value);
            "global"
        }
    } else {
        Config::set_global_env_var(name, value);
        "global"
    };

    // A session `-os` override would shadow the value just written.
    paths::clear_session_var(name);
    print_success(&t!(
        "cmd.config.env.set",
        name,
        value,
        env_store_label(store)
    ));
    Ok(false)
}

fn env_remove(name: &str, flags: EnvFlags) -> Result<bool> {
    if !env_validate(name) {
        return Ok(false);
    }

    let mut removed: Vec<&'static str> = Vec::new();

    let remove_session = || {
        if paths::session_var(name).is_some() {
            paths::remove_session_var(name);
            true
        } else {
            false
        }
    };

    if flags.all {
        for store in ["local", "global"] {
            let did = if store == "local" {
                Config::remove_local_env_var(name)
            } else {
                Config::remove_global_env_var(name)
            };
            if did {
                removed.push(store);
            }
        }
        if remove_session() {
            removed.push("session");
        }
        if os_env::remove_os_var(name).unwrap_or(false) {
            removed.push("os");
        }
    } else if flags.osg {
        if os_env::remove_os_var(name).unwrap_or(false) {
            removed.push("os");
        }
        if remove_session() {
            removed.push("session");
        }
    } else if flags.os {
        if remove_session() {
            removed.push("session");
        }
    } else if flags.local {
        if Config::remove_local_env_var(name) {
            removed.push("local");
        }
        remove_session();
    } else if flags.global {
        if Config::remove_global_env_var(name) {
            removed.push("global");
        }
        remove_session();
    } else {
        let mut did = Config::remove_local_env_var(name);
        let mut store: &'static str = "local";
        if !did {
            did = Config::remove_global_env_var(name);
            store = "global";
        }
        if did {
            removed.push(store);
        }
        remove_session();
    }

    if removed.is_empty() {
        print_info(&t!("cmd.config.env.remove_missing", name));
    } else {
        for store in removed {
            print_success(&t!("cmd.config.env.removed", name, env_store_label(store)));
        }
    }
    Ok(false)
}

/// Split `NAME value` and `NAME=value` into the same name/value pair.
fn split_env_assignment(tokens: &[String]) -> (String, String) {
    let first = tokens.first().cloned().unwrap_or_default();
    let rest = if tokens.len() > 1 {
        tokens[1..].join(" ")
    } else {
        String::new()
    };

    match first.split_once('=') {
        Some((name, inline)) => {
            let mut value = inline.to_string();
            if !rest.is_empty() {
                if !value.is_empty() {
                    value.push(' ');
                }
                value.push_str(&rest);
            }
            (name.to_string(), value)
        }
        None => (first, rest),
    }
}

pub(super) fn config_env(args: &str) -> Result<bool> {
    let (flags, tokens) = parse_env_flags(args);
    if flags.scope_conflict() {
        print_error(&t!("cmd.config.env.flag_conflict"));
        return Ok(false);
    }
    if tokens.is_empty() {
        env_list();
        return Ok(false);
    }

    let (name, value) = split_env_assignment(&tokens);

    if flags.delete {
        return env_remove(&name, flags);
    }
    if value.is_empty() {
        env_show(&name);
        return Ok(false);
    }
    env_set(&name, &value, flags)
}

pub(super) fn config_rmenv(args: &str) -> Result<bool> {
    let (mut flags, tokens) = parse_env_flags(args);
    if flags.scope_conflict() {
        print_error(&t!("cmd.config.env.flag_conflict"));
        return Ok(false);
    }
    if tokens.is_empty() {
        print_error(&t!("cmd.config.env.rm_usage"));
        return Ok(false);
    }
    if tokens.len() > 1 {
        print_error(&t!("cmd.config.env.rm_usage"));
        return Ok(false);
    }
    flags.delete = true;
    env_remove(&tokens[0], flags)
}

#[cfg(test)]
mod tests {
    use super::split_env_assignment;

    fn split(tokens: &[&str]) -> (String, String) {
        let tokens: Vec<String> = tokens.iter().map(|token| token.to_string()).collect();
        split_env_assignment(&tokens)
    }

    #[test]
    fn space_and_equals_forms_match() {
        assert_eq!(split(&["NAME", "value"]), split(&["NAME=value"]));
        assert_eq!(split(&["NAME"]), ("NAME".into(), String::new()));
        assert_eq!(split(&["NAME="]), ("NAME".into(), String::new()));
    }

    #[test]
    fn equals_form_keeps_remaining_value_text() {
        assert_eq!(
            split(&["NAME=hello", "world"]),
            ("NAME".into(), "hello world".into())
        );
        assert_eq!(
            split(&["NAME=hello=world"]),
            ("NAME".into(), "hello=world".into())
        );
    }
}
