//! `ntpx` — run a package-provided tool without installing it globally.

pub(crate) mod help;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_ntpx(args: &str, nav: &mut Navigator) -> Result<bool> {
    crate::shell::commands::builtin::ntpkg::ntpkg::cmd_ntpx(args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ntpx",
    aliases: &[],
    help_topic: Some("ntpx"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ntpx),
};
