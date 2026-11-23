//! `help` — compact help, full reference, or a topic.

// The help command owns a help.rs like every other command module.
pub(crate) mod completion;
#[allow(clippy::module_inception)]
pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::help::{print_compact_help, print_interactive_help};
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_help(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let topic = args.trim();
    if topic.is_empty() {
        // Compact by default: subcommand + what it is. `help --all` keeps the
        // long reference (every subcommand, note and example).
        print_compact_help();
    } else if matches!(topic.to_lowercase().as_str(), "-a" | "--all" | "all") {
        print_interactive_help();
    } else if !crate::shell::help::print_topic_help(topic) {
        print_error(&t!("help.unknown_topic", topic));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "help",
    aliases: &[],
    help_topic: Some("help"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_help),
};
