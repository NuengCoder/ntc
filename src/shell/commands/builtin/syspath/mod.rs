//! `syspath` — edit the machine-wide `PATH`.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::path_env::run_syspath;

pub(crate) fn cmd_syspath(args: &str, nav: &mut Navigator) -> Result<bool> {
    run_syspath(args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "syspath",
    aliases: &[],
    help_topic: Some("syspath"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_syspath),
};
