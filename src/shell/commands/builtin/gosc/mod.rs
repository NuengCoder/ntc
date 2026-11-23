//! `gosc` — navigate with the continuous directory chooser.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::shell::helpers::{gosc_loop, show_nav_view};
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_gosc(_args: &str, nav: &mut Navigator) -> Result<bool> {
    gosc_loop(nav)?;
    show_nav_view(nav);
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "gosc",
    aliases: &[],
    help_topic: Some("gosc"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_gosc),
};
