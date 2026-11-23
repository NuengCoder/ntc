//! `viewd` — restricted `view` mode.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::report_view::ViewMode;
use super::view::cmd_view_impl;

pub(crate) fn cmd_viewd(args: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_view_impl(ViewMode::Dirs, args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "viewd",
    aliases: &[],
    help_topic: Some("viewd"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_viewd),
};
