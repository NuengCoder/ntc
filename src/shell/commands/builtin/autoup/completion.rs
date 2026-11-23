//! Static subcommand/flag completions for the `autoup` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "list", "ls", "add", "rm", "remove", "disable", "enable", "help",
    ],
    flat: false,
    nodes: &[],
};
