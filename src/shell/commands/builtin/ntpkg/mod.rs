//! `ntpkg` — the package manager front end.

pub(crate) mod completion;
pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod ntpkg;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_ntpkg(args: &str, nav: &mut Navigator) -> Result<bool> {
    crate::shell::commands::builtin::ntpkg::ntpkg::cmd_ntpkg(args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ntpkg",
    aliases: &[],
    help_topic: Some("ntpkg"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ntpkg),
};
