//! `rnmf` — rename a file.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_rnmf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("misc.rnmf.missing"));
        println!("{}", t!("misc.rnmf.usage"));
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error(&t!("misc.rnmf.missing"));
        println!("{}", t!("misc.rnmf.usage"));
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&t!("misc.rnmf.not_exist", old));
    } else if old_path.is_dir() {
        print_error(&t!("misc.rnmf.is_dir", old));
    } else if new_path.exists() {
        print_error(&t!("misc.rnmf.exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&t!("misc.rnmf.renamed", old, new)),
            Err(e) => print_error(&t!("misc.rnmf.error", e)),
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "rnmf",
    aliases: &[],
    help_topic: Some("rnmf"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_rnmf),
};
