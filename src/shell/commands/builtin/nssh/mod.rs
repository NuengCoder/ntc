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
//! Transport is pure-Rust `russh` (see [`crate::shell::commands::builtin::nssh::nssh::ssh`]); passwords come
//! from the `pass_env` name (a deadlock pass-env, else an environment
//! variable) or an interactive prompt only — never from argv, never stored in
//! the profile file.

pub(crate) mod completion;
pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod nssh;

pub(crate) mod auth;
mod copy;
mod keygen;
mod profiles;
mod scan;
mod ssh;

#[cfg(test)]
mod tests;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use copy::cmd_cp;
use keygen::cmd_keygen;
use profiles::{cmd_add, cmd_edit, cmd_info, cmd_list, cmd_rm, resolve_or_pick};
use scan::cmd_scan;
use ssh::cmd_ssh;

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

/// Dispatch an `nssh` command line.
pub fn cmd_nssh(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        // Bare `nssh` → profile picker → ssh.
        let picked = resolve_or_pick(None)?;
        return match picked {
            Some((name, _)) => cmd_ssh(&{
                let mut v = vec!["ssh".to_string()];
                v.push(name);
                v
            }),
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

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "nssh",
    aliases: &[],
    help_topic: Some("nssh"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_nssh),
};
