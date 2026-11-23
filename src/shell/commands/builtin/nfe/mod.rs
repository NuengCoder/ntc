//! `explorer` / `nfe` — open the graphical file explorer.

pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod nfe;

use crate::navigator::Navigator;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_explorer(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let path = if args.is_empty() {
        None
    } else {
        Some(std::path::Path::new(args))
    };
    crate::shell::commands::builtin::nfe::nfe::run_explorer(path)?;
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "explorer",
    aliases: &["nfe"],
    help_topic: Some("explorer"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_explorer),
};
