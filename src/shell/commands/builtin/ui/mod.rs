//! `ui` — report that the old UI command was removed.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_ui(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    println!("{}", t!("misc.ui.removed"));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ui",
    aliases: &[],
    help_topic: Some("ui"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ui),
};
