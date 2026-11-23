//! `ping` — ICMP echo through the platform `ping` tool.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::shell::commands::support::net::diag::{
    parse_ping_args, ping_argv, ping_unsupported, run_tool, Os,
};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub fn cmd_ping(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() || matches!(tokens[0], "help" | "--help" | "-h") {
        print_ping_help();
        return Ok(false);
    }
    let (opts, host) = match parse_ping_args(args) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let os = Os::native();
    for flag in ping_unsupported(os, &opts) {
        print_warning(&t!("netdiag.flag_unsupported", flag));
    }
    let (prog, argv) = ping_argv(os, &opts, &host);
    match run_tool(&prog, &argv) {
        Ok(code) if code == 0 || code == 1 => {}
        Ok(code) => print_warning(&t!("dl.ping_exit_code", code)),
        Err(e) => print_error(&e.to_string()),
    }
    Ok(false)
}

fn print_ping_help() {
    println!("{}", t!("ping.help.title").bold());
    println!();
    println!("{}", t!("ping.help.usage").bold());
    println!("{}", t!("ping.help.usage_line"));
    println!();
    println!("{}", t!("ping.help.options").bold());
    println!("  {}", t!("ping.help.opt_count"));
    println!("  {}", t!("ping.help.opt_forever"));
    println!("  {}", t!("ping.help.opt_interval"));
    println!("  {}", t!("ping.help.opt_timeout"));
    println!("  {}", t!("ping.help.opt_size"));
    println!("  {}", t!("ping.help.opt_ttl"));
    println!("  {}", t!("ping.help.opt_family"));
    println!("  {}", t!("ping.help.opt_quiet"));
    println!("  {}", t!("ping.help.opt_help"));
    println!();
    println!("{}", t!("ping.help.examples").bold());
    println!("  ping example.com");
    println!("  ping 127.0.0.1 -c 10");
    println!("  ping example.com --size 64 -w 2000");
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ping",
    aliases: &[],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ping),
};
