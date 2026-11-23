//! `rnmd` — rename a directory.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_rnmd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("misc.rnmd.missing"));
        println!("{}", t!("misc.rnmd.usage"));
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error(&t!("misc.rnmd.missing"));
        println!("{}", t!("misc.rnmd.usage"));
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&t!("misc.rnmd.not_exist", old));
    } else if !old_path.is_dir() {
        print_error(&t!("misc.rnmd.not_dir", old));
    } else if new_path.exists() {
        print_error(&t!("misc.rnmd.exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&t!("misc.rnmd.renamed", old, new)),
            Err(e) => print_error(&t!("misc.rnmd.error", e)),
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "rnmd",
    aliases: &[],
    help_topic: Some("rnmd"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_rnmd),
};
