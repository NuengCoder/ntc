//! `trace` — route discovery through the platform traceroute tool.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::shell::commands::support::net::diag::{
    parse_trace_args, run_tool, trace_argv, trace_candidates, trace_ttl_walk, Os,
};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub fn cmd_trace(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() || matches!(tokens[0], "help" | "--help" | "-h") {
        print_trace_help();
        return Ok(false);
    }
    let (opts, host) = match parse_trace_args(args) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let os = Os::native();
    // Prefer the platform's traceroute; `tracepath` is a fallback when the
    // full tool is not installed.
    let mut program = None;
    for candidate in trace_candidates(os) {
        if which::which(candidate).is_ok() {
            program = Some(candidate);
            break;
        }
    }
    if let Some(prog) = program {
        let (prog, argv) = trace_argv(os, prog, &opts, &host);
        match run_tool(&prog, &argv) {
            Ok(_) => return Ok(false),
            Err(e) => {
                print_error(&e.to_string());
                return Ok(false);
            }
        }
    }
    if opts.no_fallback {
        print_error(&t!("netdiag.trace_missing"));
        return Ok(false);
    }
    print_warning(&t!("netdiag.trace_missing"));
    print_info(&t!("netdiag.trace_fallback"));
    trace_ttl_walk(os, &opts, &host);
    Ok(false)
}

fn print_trace_help() {
    println!("{}", t!("trace.help.title").bold());
    println!();
    println!("{}", t!("trace.help.usage").bold());
    println!("{}", t!("trace.help.usage_line"));
    println!();
    println!("{}", t!("trace.help.options").bold());
    println!("  {}", t!("trace.help.opt_max_hops"));
    println!("  {}", t!("trace.help.opt_timeout"));
    println!("  {}", t!("trace.help.opt_queries"));
    println!("  {}", t!("trace.help.opt_numeric"));
    println!("  {}", t!("trace.help.opt_family"));
    println!("  {}", t!("trace.help.opt_no_fallback"));
    println!("  {}", t!("trace.help.opt_help"));
    println!();
    println!("{}", t!("trace.help.examples").bold());
    println!("  trace example.com");
    println!("  trace 8.8.8.8 -m 15");
    println!("  trace example.com --no-fallback");
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "trace",
    aliases: &["tracert", "traceroute"],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_trace),
};
