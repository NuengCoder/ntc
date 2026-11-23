//! `run` — execute a command line after alias expansion.

pub(crate) mod help;

use super::super::execute_system_command;
use crate::navigator::Navigator;
use crate::shell::alias::expand_command_line_interactive;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub fn cmd_run(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("alias.run.usage"));
        println!();
        println!("{}", t!("alias.run.examples_header").green());
        println!("{}", t!("alias.run.example_real"));
        println!("{}", t!("alias.run.example_alias"));
        println!("{}", t!("alias.run.example_alias_args"));
        println!("{}", t!("alias.run.example_param"));
        return Ok(false);
    }

    // `run deploy.ntc prod` runs the script: a `.ntc` file is a command line
    // in its own right, and never an OS executable.
    if crate::shell::commands::builtin::scr::looks_like_script(args, nav.current_path()) {
        return crate::shell::commands::builtin::scr::run_direct(args.trim(), nav);
    }

    let expanded = match expand_command_line_interactive(args, nav)? {
        None => return Ok(false),
        Some(s) => s,
    };
    if !expanded.trim().is_empty() {
        execute_system_command(&expanded, nav.current_path())?;
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "run",
    aliases: &["r"],
    help_topic: Some("run"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_run),
};
