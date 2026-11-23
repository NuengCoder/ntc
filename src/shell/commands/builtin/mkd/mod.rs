//! `mkd` — create directories, including nested paths.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_ops::split_args;

pub(crate) fn cmd_mkd(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.mkd.missing"));
        return Ok(false);
    }
    for item in &items {
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if let Err(e) = std::fs::create_dir_all(&path) {
            print_error(&t!("misc.mkd.error", item, e));
        } else {
            print_success(&t!("misc.mkd.created", item));
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "mkd",
    aliases: &[],
    help_topic: Some("mkd"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_mkd),
};
