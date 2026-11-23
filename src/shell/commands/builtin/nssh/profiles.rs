use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::commands::builtin::nssh::nssh::store;
use crate::t;
use crate::utils::interactive::{prompt_yes_no, InteractiveMenu, MenuItem};
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

#[derive(Debug, Default)]
struct ProfileFlags {
    host: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    key: Option<String>,
    no_key: bool,
    pass_env: Option<String>,
    no_pass_env: bool,
}

fn parse_profile_flags(tokens: &[String], start: usize) -> Result<(ProfileFlags, usize), String> {
    let mut f = ProfileFlags::default();
    let mut i = start;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--host" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nssh.missing_host"))?;
                f.host = Some(v.clone());
                i += 2;
            }
            "--user" | "-u" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nssh.missing_user"))?;
                f.user = Some(v.clone());
                i += 2;
            }
            "--port" | "-p" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nssh.missing_port"))?;
                f.port = Some(
                    v.parse::<u16>()
                        .map_err(|_| t!("cmd.nssh.invalid_port", v))?,
                );
                if f.port == Some(0) {
                    return Err(t!("cmd.nssh.invalid_port", v));
                }
                i += 2;
            }
            "--key" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nssh.missing_key"))?;
                f.key = Some(v.clone());
                i += 2;
            }
            "--no-key" => {
                f.no_key = true;
                i += 1;
            }
            "--pass-env" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nssh.missing_pass_env"))?;
                f.pass_env = Some(v.clone());
                i += 2;
            }
            "--no-pass-env" => {
                f.no_pass_env = true;
                i += 1;
            }
            _ => break,
        }
    }
    Ok((f, i))
}

pub(super) fn cmd_add(tokens: &[String]) -> Result<bool> {
    if tokens.len() < 2 {
        print_error(&t!("cmd.nssh.add_usage"));
        return Ok(false);
    }
    let name = &tokens[1];
    if !store::valid_profile_name(name) {
        print_error(&t!("cmd.nssh.invalid_name", name));
        return Ok(false);
    }
    let (flags, rest) = parse_profile_flags(tokens, 2).map_err(|e| anyhow::anyhow!("{}", e))?;
    if rest != tokens.len() {
        print_error(&t!("cmd.nssh.unknown_option", tokens[rest]));
        return Ok(false);
    }
    let host = match flags.host {
        Some(h) if !h.trim().is_empty() => h,
        _ => {
            print_error(&t!("cmd.nssh.add_usage"));
            return Ok(false);
        }
    };
    if store::get_profile(name)
        .map_err(|e| anyhow::anyhow!("{}", e))?
        .is_some()
    {
        print_error(&t!("cmd.nssh.profile_exists", name));
        return Ok(false);
    }
    let profile = store::Profile {
        host,
        user: flags.user.unwrap_or_else(|| {
            std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "root".to_string())
        }),
        port: flags.port.unwrap_or(22),
        key: flags.key.clone(),
        pass_env: flags.pass_env.clone(),
    };
    if let Some(ref k) = profile.key {
        if !store::expand_home(k).exists() {
            print_warning(&t!("cmd.nssh.key_not_found", k));
        }
    }
    if let Some(ref v) = profile.pass_env {
        // A vault-only pass-env is the normal case: do not warn about a
        // missing env var when the deadlock vault can supply it.
        if crate::shell::commands::builtin::deadlock::deadlock::find_secret(v).is_some() {
            print_info(&t!("cmd.nssh.pass_env_registered", v));
        } else if std::env::var(v)
            .map(|value| !value.is_empty())
            .unwrap_or(false)
        {
            // Environment variable is set; nothing to warn about.
        } else if crate::shell::commands::builtin::deadlock::deadlock::vault::exists(
            &crate::shell::commands::builtin::deadlock::deadlock::vault::ensure_vault_location(),
        ) {
            print_info(&t!("cmd.nssh.pass_env_pending", v));
        } else {
            print_warning(&t!("cmd.nssh.pass_env_missing", v));
        }
    }
    store::save_profile(name, &profile).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_success(&t!("cmd.nssh.profile_saved", name));
    Ok(false)
}

pub(super) fn cmd_list() -> Result<bool> {
    let profiles = store::list_profiles().map_err(|e| anyhow::anyhow!("{}", e))?;
    if profiles.is_empty() {
        print_info(&t!("cmd.nssh.ls_empty"));
        return Ok(false);
    }
    println!("{}", t!("cmd.nssh.ls_title").cyan().bold());
    for (name, p) in &profiles {
        let auth = match (&p.key, &p.pass_env) {
            (Some(_), Some(_)) => "key+env",
            (Some(_), None) => "key",
            (None, Some(_)) => "env",
            (None, None) => "prompt",
        };
        println!(
            "  {:<20} {}@{}:{}  {}",
            name.cyan(),
            p.user,
            p.host,
            p.port,
            format!("[{}]", auth).dimmed()
        );
    }
    Ok(false)
}

pub(super) fn cmd_info(name: &str) -> Result<bool> {
    match store::get_profile(name).map_err(|e| anyhow::anyhow!("{}", e))? {
        Some(p) => {
            println!("{}", t!("cmd.nssh.info_title", name).cyan().bold());
            println!("  {} {}", t!("cmd.nssh.info_host").dimmed(), p.host);
            println!("  {} {}", t!("cmd.nssh.info_user").dimmed(), p.user);
            println!("  {} {}", t!("cmd.nssh.info_port").dimmed(), p.port);
            println!(
                "  {} {}",
                t!("cmd.nssh.info_key").dimmed(),
                p.key.as_deref().unwrap_or("-")
            );
            println!(
                "  {} {}",
                t!("cmd.nssh.info_pass").dimmed(),
                p.pass_env.as_deref().unwrap_or("-")
            );
        }
        None => print_error(&t!("cmd.nssh.profile_not_found", name)),
    }
    Ok(false)
}

pub(super) fn cmd_rm(tokens: &[String]) -> Result<bool> {
    if tokens.len() < 2 {
        print_error(&t!("cmd.nssh.rm_usage"));
        return Ok(false);
    }
    let name = &tokens[1];
    let yes = tokens.iter().any(|t| t == "--yes" || t == "-y");
    if !yes {
        let confirm = prompt_yes_no(&t!("cmd.nssh.rm_confirm", name), false).unwrap_or(false);
        if !confirm {
            print_info(&t!("cmd.nssh.rm_cancelled"));
            return Ok(false);
        }
    }
    if store::delete_profile(name).map_err(|e| anyhow::anyhow!("{}", e))? {
        print_success(&t!("cmd.nssh.rm_deleted", name));
    } else {
        print_error(&t!("cmd.nssh.profile_not_found", name));
    }
    Ok(false)
}

pub(super) fn cmd_edit(tokens: &[String]) -> Result<bool> {
    if tokens.len() < 2 {
        print_error(&t!("cmd.nssh.edit_usage"));
        return Ok(false);
    }
    let name = &tokens[1];
    let mut profile = match store::get_profile(name).map_err(|e| anyhow::anyhow!("{}", e))? {
        Some(p) => p,
        None => {
            print_error(&t!("cmd.nssh.profile_not_found", name));
            return Ok(false);
        }
    };
    let (flags, rest) = parse_profile_flags(tokens, 2).map_err(|e| anyhow::anyhow!("{}", e))?;
    if rest != tokens.len() {
        print_error(&t!("cmd.nssh.unknown_option", tokens[rest]));
        return Ok(false);
    }
    if let Some(h) = flags.host {
        profile.host = h;
    }
    if let Some(u) = flags.user {
        profile.user = u;
    }
    if let Some(p) = flags.port {
        profile.port = p;
    }
    if let Some(k) = flags.key {
        profile.key = Some(k);
    } else if flags.no_key {
        profile.key = None;
    }
    if let Some(v) = flags.pass_env {
        profile.pass_env = Some(v);
    } else if flags.no_pass_env {
        profile.pass_env = None;
    }
    store::save_profile(name, &profile).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_success(&t!("cmd.nssh.edit_saved", name));
    Ok(false)
}

/// Resolve `<name>` to a profile, or interactively pick one when omitted.
pub(super) fn resolve_or_pick(name: Option<&str>) -> Result<Option<(String, store::Profile)>> {
    if let Some(n) = name {
        return match store::get_profile(n).map_err(|e| anyhow::anyhow!("{}", e))? {
            Some(p) => Ok(Some((n.to_string(), p))),
            None => {
                print_error(&t!("cmd.nssh.profile_not_found", n));
                Ok(None)
            }
        };
    }
    let profiles = store::list_profiles().map_err(|e| anyhow::anyhow!("{}", e))?;
    if profiles.is_empty() {
        print_info(&t!("cmd.nssh.ls_empty"));
        return Ok(None);
    }
    let names: Vec<String> = profiles.keys().cloned().collect();
    let items: Vec<MenuItem> = names
        .iter()
        .map(|n| {
            let p = &profiles[n];
            MenuItem::with_detail(n.clone(), format!("{}@{}:{}", p.user, p.host, p.port))
        })
        .collect();
    match InteractiveMenu::new(&t!("cmd.nssh.pick_profile"))
        .items(items)
        .run()
    {
        Ok(Some(idx)) => {
            let n = names[idx].clone();
            Ok(Some((n.clone(), profiles[&n].clone())))
        }
        _ => Ok(None),
    }
}
