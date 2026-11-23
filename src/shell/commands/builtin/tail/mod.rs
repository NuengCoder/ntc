//! `tail` — line-window filter. The parser and reader live in
//! `support::text`; this module owns the command and its registry entry.

pub(crate) mod help;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::text::{run, Kind};

pub(crate) fn cmd_tail(args: &str, _nav: &mut Navigator) -> Result<bool> {
    run(Kind::Tail, args)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "tail",
    aliases: &[],
    help_topic: Some("tail"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_tail),
};
