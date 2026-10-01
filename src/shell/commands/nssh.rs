//! `nssh` command: native SSH profiles, remote exec/shell, SFTP copy,
//! port scan and key generation.
//!
//! ```text
//! nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa
//! nssh ssh prod --cmd "uptime"
//! nssh cp ./app.tar.gz prod:/tmp/          # upload
//! nssh cp prod:/var/log/app.log ./         # download
//! nssh scan example.com --top 20
//! ```
//!
//! Transport is pure-Rust `russh` (see [`crate::nssh::ssh`]); passwords come
//! from `pass_env` or an interactive prompt only — never from argv, never
//! stored on disk.

use crate::navigator::Navigator;
use crate::nssh::scan as portscan;
use crate::nssh::ssh as transport;
use crate::nssh::store;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::interactive::{prompt_yes_no, InteractiveMenu, MenuItem};
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Duration;

fn tokenize(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut single = false;
    let mut double = false;
    for ch in input.chars() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            ' ' | '\t' if !single && !double => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out.into_iter().map(|t| strip_wrapping_quotes(&t)).collect()
}

fn strip_wrapping_quotes(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

fn print_usage() {
    println!("{}", t!("cmd.nssh.usage_title").cyan().bold());
    println!("  {}", t!("cmd.nssh.help_add"));
    println!("  {}", t!("cmd.nssh.help_ssh"));
    println!("  {}", t!("cmd.nssh.help_cp"));
    println!("  {}", t!("cmd.nssh.help_scan"));
    println!("  {}", t!("cmd.nssh.help_manage"));
    println!("  {}", t!("cmd.nssh.help_keygen"));
    println!("  {}", t!("cmd.nssh.help_help"));
    println!();
    println!("{}", t!("cmd.nssh.options_title").cyan().bold());
    println!("  {}", t!("cmd.nssh.help_host"));
    println!("  {}", t!("cmd.nssh.help_user"));
    println!("  {}", t!("cmd.nssh.help_port"));
    println!("  {}", t!("cmd.nssh.help_key"));
    println!("  {}", t!("cmd.nssh.help_pass_env"));
    println!("  {}", t!("cmd.nssh.help_cmd"));
    println!("  {}", t!("cmd.nssh.help_trust"));
    println!("  {}", t!("cmd.nssh.help_ask_pass"));
    println!("  {}", t!("cmd.nssh.help_timeout"));
    println!("  {}", t!("cmd.nssh.help_top"));
    println!("  {}", t!("cmd.nssh.help_scan_ports"));
    println!("  {}", t!("cmd.nssh.help_yes"));
    println!();
    println!("{}", t!("cmd.nssh.examples_title").green());
    println!("    {}", t!("cmd.nssh.example_add"));
    println!("    {}", t!("cmd.nssh.example_ssh"));
    println!("    {}", t!("cmd.nssh.example_cp"));
    println!("    {}", t!("cmd.nssh.example_scan"));
}

fn rt() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| anyhow::anyhow!("cannot start async runtime: {}", e))
}

/// Common `--host/--user/--port/--key/--pass-env` profile fields.
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
                let v = tokens.get(i + 1).ok_or_else(|| t!("cmd.nssh.missing_host"))?;
                f.host = Some(v.clone());
                i += 2;
            }
            "--user" | "-u" => {
                let v = tokens.get(i + 1).ok_or_else(|| t!("cmd.nssh.missing_user"))?;
                f.user = Some(v.clone());
                i += 2;
            }
            "--port" | "-p" => {
                let v = tokens.get(i + 1).ok_or_else(|| t!("cmd.nssh.missing_port"))?;
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
                let v = tokens.get(i + 1).ok_or_else(|| t!("cmd.nssh.missing_key"))?;
                f.key = Some(v.clone());
                i += 2;
            }
            "--no-key" => {
                f.no_key = true;
                i += 1;
            }
            "--pass-env" => {
                let v = tokens.get(i + 1).ok_or_else(|| t!("cmd.nssh.missing_pass_env"))?;
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

fn cmd_add(tokens: &[String]) -> Result<bool> {
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
    if store::get_profile(name).map_err(|e| anyhow::anyhow!("{}", e))?.is_some() {
        print_error(&t!("cmd.nssh.profile_exists", name));
        return Ok(false);
    }
    let profile = store::Profile {
        host,
        user: flags.user.unwrap_or_else(|| {
            std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "root".to_string())
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
        if std::env::var(v).is_err() {
            print_warning(&t!("cmd.nssh.pass_env_missing", v));
        }
    }
    store::save_profile(name, &profile).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_success(&t!("cmd.nssh.profile_saved", name));
    Ok(false)
}

fn cmd_list() -> Result<bool> {
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

fn cmd_info(name: &str) -> Result<bool> {
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

fn cmd_rm(tokens: &[String]) -> Result<bool> {
    if tokens.len() < 2 {
        print_error(&t!("cmd.nssh.rm_usage"));
        return Ok(false);
    }
    let name = &tokens[1];
    let yes = tokens.iter().any(|t| t == "--yes" || t == "-y");
    if !yes {
        let confirm =
            prompt_yes_no(&t!("cmd.nssh.rm_confirm", name), false).unwrap_or(false);
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

fn cmd_edit(tokens: &[String]) -> Result<bool> {
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
fn resolve_or_pick(name: Option<&str>) -> Result<Option<(String, store::Profile)>> {
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
            MenuItem::with_detail(
                n.clone(),
                format!("{}@{}:{}", p.user, p.host, p.port),
            )
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

/// Build auth from profile + flags: `pass_env` var, else `--ask-pass` prompt.
fn build_auth(
    profile: &store::Profile,
    ask_pass: bool,
    pass_value: Option<&str>,
) -> Result<transport::SshAuth> {
    let key_path = profile.key.as_deref().map(store::expand_home);
    if let Some(v) = pass_value {
        return Ok(transport::SshAuth {
            key_path,
            password: Some(v.to_string()),
        });
    }
    if let Some(ref var) = profile.pass_env {
        match std::env::var(var) {
            Ok(v) if !v.is_empty() => {
                return Ok(transport::SshAuth {
                    key_path,
                    password: Some(v),
                })
            }
            _ => print_warning(&t!("cmd.nssh.pass_env_unset", var)),
        }
    }
    if ask_pass {
        let prompt = t!("cmd.nssh.ask_pass", profile.user, profile.host);
        let pwd = rpassword::prompt_password(prompt).map_err(|e| anyhow::anyhow!("{}", e))?;
        return Ok(transport::SshAuth {
            key_path,
            password: Some(pwd),
        });
    }
    Ok(transport::SshAuth {
        key_path,
        password: None,
    })
}

fn cmd_ssh(tokens: &[String]) -> Result<bool> {
    // nssh ssh [name] [--cmd "..."] [--trust] [--ask-pass] [--timeout <s>]
    let mut name: Option<String> = None;
    let mut cmd: Option<String> = None;
    let mut trust = false;
    let mut ask_pass = false;
    let mut timeout = Duration::from_secs(30);
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--cmd" => {
                let v = tokens.get(i + 1).ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_cmd")))?;
                cmd = Some(v.clone());
                i += 2;
            }
            "--trust" => {
                trust = true;
                i += 1;
            }
            "--ask-pass" => {
                ask_pass = true;
                i += 1;
            }
            "--timeout" => {
                let v = tokens.get(i + 1).ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_timeout")))?;
                let s: u64 = v.parse().map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_timeout", v)))?;
                timeout = Duration::from_secs(s.max(1));
                i += 2;
            }
            _ => {
                if tokens[i].starts_with('-') {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                if name.is_some() {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                name = Some(tokens[i].clone());
                i += 1;
            }
        }
    }
    let Some((pname, profile)) = resolve_or_pick(name.as_deref())? else {
        return Ok(false);
    };
    let _ = pname;
    let auth = build_auth(&profile, ask_pass, None)?;
    print_info(&t!(
        "cmd.nssh.ssh_connecting",
        profile.user,
        profile.host,
        profile.port.to_string()
    ));
    let runtime = rt()?;
    let conn = runtime.block_on(transport::connect(
        &profile.host,
        profile.port,
        &profile.user,
        &auth,
        trust,
        timeout,
    ))?;
    if let Some(c) = cmd {
        let code = runtime.block_on(transport::exec(&conn, &c))?;
        if code != 0 {
            print_warning(&t!("cmd.nssh.cmd_exit", code.to_string()));
        }
    } else {
        print_info(&t!("cmd.nssh.shell_hint"));
        match runtime.block_on(transport::shell(&conn))? {
            transport::ShellEnd::Forced => print_warning(&t!("cmd.nssh.shell_forced")),
            transport::ShellEnd::Closed => {}
        }
        print_info(&t!("cmd.nssh.shell_back"));
    }
    let _ = runtime.block_on(conn.handle.disconnect(
        russh::Disconnect::ByApplication,
        "nssh done",
        "",
    ));
    Ok(false)
}

/// Split `<profile>:<path>`. Only a *known* profile name counts as remote
/// (so `C:\x` on Windows stays local).
fn split_remote(spec: &str) -> Option<(String, String)> {
    let (maybe_name, path) = spec.split_once(':')?;
    if !store::valid_profile_name(maybe_name) {
        return None;
    }
    // Windows drive letter (`C:\...`, `C:/...`) is never remote.
    if maybe_name.len() == 1 && path.starts_with(['\\', '/']) {
        return None;
    }
    let profiles = store::list_profiles().ok()?;
    profiles
        .contains_key(maybe_name)
        .then(|| (maybe_name.to_string(), path.to_string()))
}

fn progress_bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::with_template("{wide_bar} {bytes}/{total_bytes} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar()),
    );
    bar
}

fn cmd_cp(tokens: &[String], nav: &mut Navigator) -> Result<bool> {
    // nssh cp <src> <dst> [--trust] [--ask-pass] [--timeout <s>]
    let mut positional: Vec<String> = Vec::new();
    let mut trust = false;
    let mut ask_pass = false;
    let mut timeout = Duration::from_secs(30);
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--trust" => {
                trust = true;
                i += 1;
            }
            "--ask-pass" => {
                ask_pass = true;
                i += 1;
            }
            "--timeout" => {
                let v = tokens.get(i + 1).ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_timeout")))?;
                let s: u64 = v.parse().map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_timeout", v)))?;
                timeout = Duration::from_secs(s.max(1));
                i += 2;
            }
            _ => {
                if tokens[i].starts_with('-') {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                positional.push(tokens[i].clone());
                i += 1;
            }
        }
    }
    if positional.len() != 2 {
        print_error(&t!("cmd.nssh.cp_usage"));
        return Ok(false);
    }
    let cwd = nav.current_path().to_path_buf();
    let resolve_local = |s: &str| -> PathBuf {
        let p = Path::new(s);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            cwd.join(p)
        }
    };
    let src_remote = split_remote(&positional[0]);
    let dst_remote = split_remote(&positional[1]);
    match (src_remote, dst_remote) {
        (None, None) => {
            print_error(&t!("cmd.nssh.cp_no_remote"));
            Ok(false)
        }
        (Some(_), Some(_)) => {
            print_error(&t!("cmd.nssh.cp_two_sides"));
            Ok(false)
        }
        (None, Some((name, rpath))) => {
            // upload local -> remote
            let local = resolve_local(&positional[0]);
            if !local.is_file() {
                print_error(&t!("cmd.nssh.cp_local_missing", local.display()));
                return Ok(false);
            }
            let profile = store::get_profile(&name)
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .expect("profile checked by split_remote");
            let remote = expand_remote_path(&rpath, local.file_name().and_then(|n| n.to_str()));
            let auth = build_auth(&profile, ask_pass, None)?;
            let runtime = rt()?;
            runtime.block_on(cp_upload(&profile, &auth, &local, &remote, trust, timeout))?;
            Ok(false)
        }
        (Some((name, rpath)), None) => {
            // download remote -> local
            let profile = store::get_profile(&name)
                .map_err(|e| anyhow::anyhow!("{}", e))?
                .expect("profile checked by split_remote");
            let mut local = resolve_local(&positional[1]);
            if local.is_dir() || positional[1].ends_with(['/', '\\']) {
                if let Some(base) = Path::new(&rpath).file_name() {
                    local = local.join(base);
                }
            }
            let auth = build_auth(&profile, ask_pass, None)?;
            let runtime = rt()?;
            runtime.block_on(cp_download(&profile, &auth, &rpath, &local, trust, timeout))?;
            Ok(false)
        }
    }
}

/// `remote/` + local basename when the target is a directory.
fn expand_remote_path(rpath: &str, basename: Option<&str>) -> String {
    if rpath.ends_with('/') {
        match basename {
            Some(b) => format!("{}{}", rpath, b),
            None => rpath.to_string(),
        }
    } else {
        rpath.to_string()
    }
}

const CP_SIZE_GUARD: u64 = 256 * 1024 * 1024;

async fn cp_upload(
    profile: &store::Profile,
    auth: &transport::SshAuth,
    local: &Path,
    remote: &str,
    trust: bool,
    timeout: Duration,
) -> Result<()> {
    let data = std::fs::read(local).map_err(|e| anyhow::anyhow!("{}", t!("cmd.nssh.cp_read_fail", e)))?;
    if data.len() as u64 > CP_SIZE_GUARD {
        return Err(anyhow::anyhow!("{}", t!("cmd.nssh.cp_too_large")));
    }
    print_info(&t!(
        "cmd.nssh.cp_uploading",
        local.display(),
        profile.host,
        remote
    ));
    let conn = transport::connect(&profile.host, profile.port, &profile.user, auth, trust, timeout).await?;
    let sftp = transport::sftp(&conn).await?;
    let bar = progress_bar(data.len() as u64);
    // Chunked write so the progress bar moves on big files.
    let mut f = sftp
        .create(remote.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("sftp create failed: {}", e))?;
    {
        use tokio::io::AsyncWriteExt as _;
        for chunk in data.chunks(256 * 1024) {
            f.write_all(chunk)
                .await
                .map_err(|e| anyhow::anyhow!("sftp write failed: {}", e))?;
            bar.inc(chunk.len() as u64);
        }
        f.shutdown()
            .await
            .map_err(|e| anyhow::anyhow!("sftp close failed: {}", e))?;
    }
    bar.finish_and_clear();
    print_success(&t!("cmd.nssh.cp_done", data.len()));
    Ok(())
}

async fn cp_download(
    profile: &store::Profile,
    auth: &transport::SshAuth,
    remote: &str,
    local: &Path,
    trust: bool,
    timeout: Duration,
) -> Result<()> {
    print_info(&t!(
        "cmd.nssh.cp_downloading",
        profile.host,
        remote,
        local.display()
    ));
    let conn = transport::connect(&profile.host, profile.port, &profile.user, auth, trust, timeout).await?;
    let sftp = transport::sftp(&conn).await?;
    let data = sftp
        .read(remote.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("sftp read failed: {}", e))?;
    if let Some(parent) = local.parent() {
        std::fs::create_dir_all(parent).map_err(|e| anyhow::anyhow!("{}", e))?;
    }
    std::fs::write(local, &data).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_success(&t!("cmd.nssh.cp_done", data.len()));
    Ok(())
}

fn cmd_scan(tokens: &[String]) -> Result<bool> {
    // nssh scan <host> [--ports <spec> | --top <n>] [--timeout <ms>]
    let mut host: Option<String> = None;
    let mut ports_spec: Option<String> = None;
    let mut top: Option<usize> = None;
    let mut timeout_ms = 800u64;
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--ports" => {
                let v = tokens.get(i + 1).ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_ports")))?;
                ports_spec = Some(v.clone());
                i += 2;
            }
            "--top" => {
                let v = tokens.get(i + 1).ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_top")))?;
                top = Some(v.parse().map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_top", v)))?);
                i += 2;
            }
            "--timeout" => {
                let v = tokens.get(i + 1).ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_timeout")))?;
                timeout_ms = v.parse().map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_timeout", v)))?;
                i += 2;
            }
            _ => {
                if tokens[i].starts_with('-') {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                if host.is_some() {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                host = Some(tokens[i].clone());
                i += 1;
            }
        }
    }
    let host = match host {
        Some(h) => h,
        None => {
            print_error(&t!("cmd.nssh.scan_usage"));
            return Ok(false);
        }
    };
    let ports: Vec<u16> = if let Some(spec) = ports_spec {
        portscan::parse_ports(&spec).map_err(|e| anyhow::anyhow!("{}", e))?
    } else {
        let n = top.unwrap_or(20).clamp(1, portscan::TOP_PORTS.len());
        portscan::TOP_PORTS[..n].to_vec()
    };
    let ips =
        portscan::resolve_host(&host, ports[0]).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_info(&t!(
        "cmd.nssh.scan_start",
        host,
        ports.len().to_string()
    ));
    let hits = portscan::scan_ports(&ips, &ports, Duration::from_millis(timeout_ms.max(50)));
    if hits.is_empty() {
        print_warning(&t!("cmd.nssh.scan_none", host));
    } else {
        println!("{}", t!("cmd.nssh.scan_title", host).cyan().bold());
        for h in &hits {
            println!("  {:<6} {}", h.port.to_string().green(), h.service.dimmed());
        }
        print_info(&t!(
            "cmd.nssh.scan_done",
            hits.len().to_string(),
            ports.len().to_string()
        ));
    }
    Ok(false)
}

fn cmd_keygen(args: &str) -> Result<bool> {
    let status = std::process::Command::new("ssh-keygen")
        .args(tokenize(args))
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.nssh.keygen_missing", e)))?
        .wait()
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    if !status.success() {
        print_warning(&t!("cmd.nssh.keygen_exit", status.code().unwrap_or(-1).to_string()));
    }
    Ok(false)
}

pub fn cmd_nssh(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        // Bare `nssh` → profile picker → ssh.
        let picked = resolve_or_pick(None)?;
        return match picked {
            Some((_, profile)) => {
                let auth = build_auth(&profile, true, None)?;
                print_info(&t!(
                    "cmd.nssh.ssh_connecting",
                    profile.user,
                    profile.host,
                    profile.port.to_string()
                ));
                let runtime = rt()?;
                let conn = runtime.block_on(transport::connect(
                    &profile.host,
                    profile.port,
                    &profile.user,
                    &auth,
                    false,
                    Duration::from_secs(30),
                ))?;
                print_info(&t!("cmd.nssh.shell_hint"));
                match runtime.block_on(transport::shell(&conn))? {
                    transport::ShellEnd::Forced => {
                        print_warning(&t!("cmd.nssh.shell_forced"))
                    }
                    transport::ShellEnd::Closed => {}
                }
                print_info(&t!("cmd.nssh.shell_back"));
                let _ = runtime.block_on(
                    conn.handle
                        .disconnect(russh::Disconnect::ByApplication, "nssh done", ""),
                );
                Ok(false)
            }
            None => Ok(false),
        };
    }
    match tokens[0].to_lowercase().as_str() {
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(false)
        }
        "add" => cmd_add(&tokens),
        "ls" | "list" => cmd_list(),
        "info" => {
            if tokens.len() < 2 {
                print_error(&t!("cmd.nssh.info_usage"));
                return Ok(false);
            }
            cmd_info(&tokens[1])
        }
        "rm" => cmd_rm(&tokens),
        "edit" => cmd_edit(&tokens),
        "ssh" => cmd_ssh(&tokens),
        "cp" => cmd_cp(&tokens, nav),
        "scan" => cmd_scan(&tokens),
        "keygen" => cmd_keygen(&tokens.get(1..).map(|s| s.join(" ")).unwrap_or_default()),
        _ => {
            // Bare `nssh <name>` = ssh into that profile.
            cmd_ssh(&{
                let mut v = vec!["ssh".to_string()];
                v.extend(tokens);
                v
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_quotes() {
        assert_eq!(
            tokenize(r#"ssh prod --cmd "uptime; df -h""#),
            vec!["ssh", "prod", "--cmd", "uptime; df -h"]
        );
    }

    #[test]
    fn split_remote_needs_known_profile() {
        // No profiles in a fresh temp HOME... store reads real config dir;
        // only assert Windows-drive safety which needs no store entry.
        assert!(split_remote("C:\\temp\\x").is_none());
        assert!(split_remote("C:/temp/x").is_none());
        assert!(split_remote("./local/file").is_none());
        assert!(split_remote("no-colon-here").is_none());
    }

    #[test]
    fn expand_remote_dir_appends_basename() {
        assert_eq!(
            expand_remote_path("/tmp/", Some("app.tar.gz")),
            "/tmp/app.tar.gz"
        );
        assert_eq!(expand_remote_path("/tmp/x.tgz", Some("a")), "/tmp/x.tgz");
    }

    #[test]
    fn bare_name_routes_to_ssh() {
        // Unknown profile prints an error and returns Ok(false).
        let mut nav = Navigator::new().unwrap();
        let r = cmd_nssh("definitely-not-a-profile-xyz", &mut nav).unwrap();
        assert!(!r);
    }
}
