//! `smtp` email client command and dispatcher.

pub(crate) mod help;

mod body;
mod config;
mod profiles;
mod send;

#[cfg(test)]
mod tests;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::PathBuf;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use body::BodyFormat;
use config::cmd_smtp_init;
use profiles::{cmd_smtp_create, cmd_smtp_info, cmd_smtp_ls, cmd_smtp_rm};
use send::{cmd_smtp_send, cmd_smtp_send_file};

/// Split a command line into arguments and remove exactly one matching
/// quote pair from each argument.
///
/// Only a quote at the *start* of an argument opens quoting, and only its
/// matching partner closes it. Quotes of the other kind (and quotes after
/// the first character) stay literal, so `-s "'TITLE'"` becomes `'TITLE'`
/// while `-s 'TITLE'` becomes `TITLE` — not `TITLE` and `TITLE`.
pub(super) fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut started = false;
    for ch in input.chars() {
        match quote {
            Some(q) if ch == q => quote = None,
            Some(_) => current.push(ch),
            None => match ch {
                '\'' | '"' if !started => {
                    quote = Some(ch);
                    started = true;
                }
                ' ' | '\t' if !started => {}
                ' ' | '\t' => {
                    tokens.push(std::mem::take(&mut current));
                    started = false;
                }
                _ => {
                    current.push(ch);
                    started = true;
                }
            },
        }
    }
    if started {
        tokens.push(current);
    }
    tokens
}

pub fn cmd_smtp(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_smtp_help();
        return Ok(false);
    }
    if tokens[0] == "init" {
        let mut path = PathBuf::from("ntc_smtp.json");
        let mut path_set = false;
        let mut force = false;
        for arg in &tokens[1..] {
            match arg.as_str() {
                "-f" | "--force" => force = true,
                other if !other.starts_with('-') && !path_set => {
                    path = PathBuf::from(crate::utils::paths::expand_path_str(other));
                    path_set = true;
                }
                other => anyhow::bail!("{}", t!("cmd.smtp.unknown_option", other)),
            }
        }
        return cmd_smtp_init(&path, force);
    }

    let mut i = 0;
    let mut server = String::new();
    let mut port: u16 = 587;
    let mut user = String::new();
    let mut password = String::new();
    let mut profile_id = String::new();
    let mut list_profiles = false;
    let mut info_id = String::new();
    let mut reveal_password = false;
    let mut remove_id = String::new();
    let mut to_addrs = Vec::new();
    let mut cc_addrs = Vec::new();
    let mut bcc_addrs = Vec::new();
    let mut subject = String::new();
    let mut body = String::new();
    let mut body_file: Option<String> = None;
    let mut body_format = BodyFormat::Auto;
    let mut attach_paths = Vec::new();
    let mut is_creating = false;
    let mut read_file: Option<String> = None;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-i" | "--server" => {
                i += 1;
                server = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_server")))?
                    .clone();
                is_creating = true;
            }
            "-p" | "--port" => {
                i += 1;
                port = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_port")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.smtp.invalid_port")))?;
            }
            "-u" | "--user" => {
                i += 1;
                user = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_user")))?
                    .clone();
            }
            "-pwd" | "--password" => {
                i += 1;
                password = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_password")))?
                    .clone();
            }
            "--id-ls" => list_profiles = true,
            "--id-info" => {
                i += 1;
                info_id = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_id_info")))?
                    .clone();
            }
            // With --id-info: show the app password after unlocking deadlock.
            "--deadlock" => reveal_password = true,
            "--id-rm" => {
                i += 1;
                remove_id = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_id_rm")))?
                    .clone();
            }
            "--id" => {
                i += 1;
                profile_id = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_id")))?
                    .clone();
            }
            "--to" | "-t" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_recipient_to")))?
                    .clone();
                to_addrs = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "--cc" | "-c" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_recipient_cc")))?
                    .clone();
                cc_addrs = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "--bcc" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_recipient_bcc")))?
                    .clone();
                bcc_addrs = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "-r" | "--read" | "--config" => {
                i += 1;
                read_file = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_read_file")))?
                        .clone(),
                );
            }
            "-s" | "--subject" => {
                i += 1;
                subject = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_subject")))?
                    .clone();
            }
            "-b" | "--body" => {
                i += 1;
                body = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_body")))?
                    .clone();
            }
            "--body-file" => {
                i += 1;
                body_file = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_body_file")))?
                        .clone(),
                );
            }
            "-bf" | "--body-format" => {
                i += 1;
                let value = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_body_format")))?;
                body_format = BodyFormat::from_cli(value).ok_or_else(|| {
                    anyhow::anyhow!("{}", t!("cmd.smtp.invalid_body_format", value))
                })?;
            }
            "-f" | "--attach" => {
                i += 1;
                let val = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.smtp.missing_path")))?
                    .clone();
                let paths: Vec<String> = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                attach_paths.extend(paths);
            }
            "--" => break,
            _ => {
                anyhow::bail!("{}", t!("cmd.smtp.unknown_option", tokens[i]));
            }
        }
        i += 1;
    }

    // Dispatch subcommands
    if list_profiles {
        return cmd_smtp_ls();
    }
    if !info_id.is_empty() {
        return cmd_smtp_info(&info_id, reveal_password);
    }
    if !remove_id.is_empty() {
        return cmd_smtp_rm(&remove_id);
    }
    if is_creating {
        return cmd_smtp_create(&server, port, &user, &password);
    }
    if let Some(file) = read_file {
        return cmd_smtp_send_file(
            &file,
            &to_addrs,
            &cc_addrs,
            &bcc_addrs,
            &subject,
            &body,
            body_file.as_deref(),
            body_format,
            &attach_paths,
        );
    }
    if !profile_id.is_empty() {
        return cmd_smtp_send(
            &profile_id,
            &to_addrs,
            &cc_addrs,
            &bcc_addrs,
            &subject,
            &body,
            body_file.as_deref(),
            body_format,
            &attach_paths,
        );
    }

    print_smtp_help();
    Ok(false)
}

fn print_smtp_help() {
    println!("{}", t!("cmd.smtp.help_title").bold());
    println!();
    println!("{}", t!("cmd.smtp.help_create_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_create_syntax"));
    println!("  {}", t!("cmd.smtp.help_create_example"));
    println!();
    println!("{}", t!("cmd.smtp.help_list_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_list_syntax"));
    println!();
    println!("{}", t!("cmd.smtp.help_info_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_info_syntax"));
    println!("  {}", t!("cmd.smtp.help_info_deadlock"));
    println!();
    println!("{}", t!("cmd.smtp.help_rm_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_rm_syntax"));
    println!();
    println!("{}", t!("cmd.smtp.help_send_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_send_syntax"));
    println!();
    println!("{}", t!("cmd.smtp.help_file_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_file_init"));
    println!("  {}", t!("cmd.smtp.help_file_send"));
    println!();
    println!("{}", t!("cmd.smtp.help_body_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_body_inline"));
    println!("  {}", t!("cmd.smtp.help_body_file"));
    println!("  {}", t!("cmd.smtp.help_body_format"));
    println!("  {}", t!("cmd.smtp.help_body_stdin"));
    println!();
    println!("{}", t!("cmd.smtp.help_attachments_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_attachments_desc"));
    println!("  {}", t!("cmd.smtp.help_attachments_example"));
    println!();
    println!("{}", t!("cmd.smtp.help_ports_title").cyan().bold());
    println!("  {}", t!("cmd.smtp.help_port_587"));
    println!("  {}", t!("cmd.smtp.help_port_465"));
    println!("  {}", t!("cmd.smtp.help_port_25"));
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "smtp",
    aliases: &[],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_smtp),
};
