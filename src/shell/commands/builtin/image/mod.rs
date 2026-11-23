//! `image` — render an image file in the terminal (feature-gated).

pub(crate) mod completion;
pub(crate) mod help;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use crate::navigator::Navigator;
use anyhow::Result;

#[cfg(feature = "image")]
mod render;

/// Without the `image` feature the command still exists, so completion and
/// `help` stay stable, but running it reports the disabled message.
#[cfg(not(feature = "image"))]
pub(crate) fn cmd_image(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    crate::output::print_error(&crate::t!("cmd.image_disabled"));
    Ok(false)
}

#[cfg(feature = "image")]
pub(crate) fn cmd_image(args: &str, nav: &mut Navigator) -> Result<bool> {
    render::cmd_image(args, nav)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "image",
    aliases: &["img"],
    help_topic: Some("image"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_image),
};
