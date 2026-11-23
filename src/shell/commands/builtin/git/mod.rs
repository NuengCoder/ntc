pub(crate) mod completion;
pub(crate) mod help;
// src/shell/commands/builtin/git/mod.rs
// Built-in `git` command: pretty-printed status/branch/log, passthrough for
// everything else (commit, add, push, pull, clone, ...).

mod branch;
mod log;
mod passthrough;
mod status;

#[cfg(test)]
mod tests;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::git as gitmod;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use branch::cmd_branch;
use log::cmd_log;
use passthrough::passthrough;
use status::cmd_status;

pub fn cmd_git(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("").trim();
    let subargs = parts.get(1).unwrap_or(&"").trim();

    if !gitmod::git_available() {
        print_error(&t!("git.no_git_binary"));
        return Ok(false);
    }

    match subcmd {
        "" | "status" | "st" => cmd_status(nav),
        // Bare `git branch` keeps the pretty list; anything with args
        // (`-M main`, `-a`, `-d x`, …) must reach real git — otherwise
        // the rename never executes and the prompt keeps showing `master`.
        "branch" | "b" if subargs.is_empty() => cmd_branch(nav),
        "branch" | "b" => passthrough(nav, &format!("git branch {subargs}")),
        "log" | "l" => cmd_log(nav),
        "diff" | "d" => passthrough(nav, &format!("git diff {subargs}")),
        "help" | "h" => print_git_help(),
        other => passthrough(nav, &format!("git {other} {subargs}")),
    }
}

fn print_git_help() -> Result<bool> {
    println!("{}", t!("git.help_title").cyan().bold());
    println!();
    println!("  {}", t!("git.help_status"));
    println!("  {}", t!("git.help_branch"));
    println!("  {}", t!("git.help_log"));
    println!("  {}", t!("git.help_diff"));
    println!("  {}", t!("git.help_passthrough"));
    println!();
    println!("{}", t!("git.examples_header").green());
    println!("  {}", t!("git.help_example1"));
    println!("  {}", t!("git.help_example2"));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "git",
    aliases: &[],
    help_topic: Some("git"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_git),
};
