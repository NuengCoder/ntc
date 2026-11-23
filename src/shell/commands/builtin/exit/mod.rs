//! `exit` — leave the shell, except inside the editor's embedded terminal.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_exit(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    // The embedded shell (editor terminal) must never close itself: the
    // editor owns that PTY, so exiting here would strand the panel.
    if std::env::var("NTC_IN_EDITOR").is_ok() {
        print_error(&t!("misc.exit.in_editor"));
        return Ok(false);
    }
    Ok(true)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "exit",
    aliases: &["quit", "esc"],
    help_topic: Some("exit"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_exit),
};
