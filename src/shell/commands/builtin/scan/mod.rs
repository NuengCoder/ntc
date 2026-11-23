//! `scan` command: file listing plus local network discovery.

pub(crate) mod completion;
pub(crate) mod help;
pub(crate) mod netscan;

mod files;
mod net;
mod pid;
mod waep;

#[cfg(test)]
mod tests;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use files::{cmd_scan_dir, cmd_scan_ext, cmd_scan_files};
use net::{cmd_scan_ip, cmd_scan_port};
use pid::cmd_scan_pid;
use waep::cmd_scan_waep;

pub(super) fn tokenize(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
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
    out
}

fn print_usage() {
    println!("{}", t!("cmd.scan.usage_title").cyan().bold());
    println!("  {}", t!("cmd.scan.help_ext"));
    println!("  {}", t!("cmd.scan.help_files"));
    println!("  {}", t!("cmd.scan.help_dir"));
    println!("  {}", t!("cmd.scan.help_port"));
    println!("  {}", t!("cmd.scan.help_ip"));
    println!("  {}", t!("cmd.scan.help_pid"));
    println!("  {}", t!("cmd.scan.help_waep"));
    println!();
    println!("{}", t!("cmd.scan.examples_title").green());
    println!("    scan ext rs");
    println!("    scan ext rs c -d 2");
    println!("    scan ext rs c -s");
    println!("    scan ext rs --care");
    println!("    scan files");
    println!("    scan files *.rs -s");
    println!("    scan files k*.rs --care");
    println!("    scan dir");
    println!("    scan dir k* -d 2");
    println!("    scan port");
    println!("    scan port 192.168.1.1 --top 10");
    println!("    scan port 192.168.1.1 --all");
    println!("    scan ip");
    println!("    scan ip 192.168.1.0/24 --timeout 300");
    println!("    scan pid");
    println!("    scan pid localhost");
    println!("    scan pid prod");
    println!("    scan pid --sort name --filter chrome");
    println!("    scan pid --sort memory --desc");
    println!("    scan pid --sort memory --limit 10");
    println!("    scan pid --min-memory 100MB --max-memory 1GB");
    println!("    scan pid --offset 20 --limit 10");
    println!("    scan pid deploy@example.com:2222 --key ~/.ssh/id_ed25519 --trust");
    println!("    scan waep example.com");
    println!("    scan waep example.com/api -r -d 1");
    println!("    scan waep 192.168.1.5:8080 -x php,html --all");
}

pub fn cmd_scan(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_usage();
        return Ok(false);
    }
    let sub = tokens[0].to_lowercase();
    if sub == "help" || sub == "-h" || sub == "--help" {
        print_usage();
        return Ok(false);
    }
    match sub.as_str() {
        "ext" => cmd_scan_ext(&tokens[1..], nav),
        "files" => cmd_scan_files(&tokens[1..], nav),
        "dir" => cmd_scan_dir(&tokens[1..], nav),
        "port" => cmd_scan_port(&tokens[1..]),
        "ip" => cmd_scan_ip(&tokens[1..]),
        "pid" => cmd_scan_pid(&tokens[1..]),
        "waep" | "web" => cmd_scan_waep(&tokens[1..]),
        _ => {
            print_error(&t!("cmd.scan.unknown_sub", tokens[0]));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "scan",
    aliases: &[],
    help_topic: Some("scan"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_scan),
};
