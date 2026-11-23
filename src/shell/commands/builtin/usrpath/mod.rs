//! `usrpath` — edit the per-user `PATH`.

pub(crate) mod completion;
pub(crate) mod help;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::path_env::run_usrpath;

pub(crate) fn cmd_usrpath(args: &str, nav: &mut Navigator) -> Result<bool> {
    run_usrpath(args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "usrpath",
    aliases: &[],
    help_topic: Some("usrpath"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_usrpath),
};
