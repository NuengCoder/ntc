//! `pathcp` — copy the current directory path to the clipboard.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_success;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_pathcp(_args: &str, nav: &mut Navigator) -> Result<bool> {
    let path = nav.current_path().to_string_lossy().to_string();
    crate::output::copy_to_clipboard(&path, "path")?;
    print_success(&t!("misc.pathcp.copied", path));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "pathcp",
    aliases: &[],
    help_topic: Some("pathcp"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_pathcp),
};
