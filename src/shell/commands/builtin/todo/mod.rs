//! `todo` — manage the project's `future_todo` task list.

pub(crate) mod completion;
pub(crate) mod help;

mod actions;
mod model;

#[cfg(test)]
mod tests;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use actions::{
    cmd_add, cmd_clear, cmd_done, cmd_edit, cmd_info, cmd_move, cmd_replace, cmd_rm, cmd_undo,
};

pub fn cmd_todo(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();
    let cwd = nav.current_path().to_path_buf();

    match subcmd {
        "" | "info" | "ls" | "list" => cmd_info(subargs, &cwd),
        "add" | "a" => cmd_add(subargs, &cwd),
        "done" | "do" | "check" => cmd_done(subargs, &cwd),
        "undo" | "un" | "uncheck" | "reopen" => cmd_undo(subargs, &cwd),
        "edit" | "ed" | "rnm" | "rename" => cmd_edit(subargs, &cwd),
        "rm" | "remove" => cmd_rm(subargs, &cwd),
        "move" | "mv" => cmd_move(subargs, &cwd),
        "replace" | "rp" | "swap" => cmd_replace(subargs, &cwd),
        "clear" | "cl" => cmd_clear(subargs, &cwd),
        _ => {
            print_error(&t!("cmd.todo.unknown_sub", subcmd));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

fn print_usage() {
    println!("{}", t!("cmd.todo.usage_title").cyan().bold());
    println!("  {}", t!("cmd.todo.help_show"));
    println!("  {}", t!("cmd.todo.help_info"));
    println!("  {}", t!("cmd.todo.help_add"));
    println!("  {}", t!("cmd.todo.help_done"));
    println!("  {}", t!("cmd.todo.help_undo"));
    println!("  {}", t!("cmd.todo.help_edit"));
    println!("  {}", t!("cmd.todo.help_rm"));
    println!("  {}", t!("cmd.todo.help_move"));
    println!("  {}", t!("cmd.todo.help_replace"));
    println!("  {}", t!("cmd.todo.help_clear"));
    println!("  {}", t!("cmd.todo.help_format"));
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "todo",
    aliases: &["td"],
    help_topic: Some("todo"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_todo),
};
