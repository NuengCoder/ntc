//! `mkf` — create empty files, including parent directories.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_ops::split_args;

pub(crate) fn cmd_mkf(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.mkf.missing"));
        return Ok(false);
    }
    for item in &items {
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if path.exists() {
            print_warning(&t!("misc.mkf.exists", item));
            continue;
        }
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        match std::fs::write(&path, "") {
            Ok(()) => print_success(&t!("misc.mkf.created", item)),
            Err(e) => print_error(&t!("misc.mkf.error", item, e)),
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "mkf",
    aliases: &[],
    help_topic: Some("mkf"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_mkf),
};
