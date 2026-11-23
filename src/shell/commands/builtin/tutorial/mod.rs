//! `tutorial` — run the guided tutorial.

pub(crate) mod help;

#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod tutorial;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_tutorial(args: &str, nav: &mut Navigator) -> Result<bool> {
    crate::shell::commands::builtin::tutorial::tutorial::command(args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "tutorial",
    aliases: &["guide"],
    help_topic: Some("tutorial"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_tutorial),
};
