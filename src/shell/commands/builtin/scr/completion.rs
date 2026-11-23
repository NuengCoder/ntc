//! Static subcommand/flag completions for the `scr` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["init", "ls", "list", "check", "help"],
    flat: false,
    nodes: &[],
};
