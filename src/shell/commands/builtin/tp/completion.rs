//! Static subcommand/flag completions for the `tp` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "add", "jump", "to", "info", "list", "ls", "rm", "rnm", "rename", "cls", "help",
    ],
    flat: true,
    nodes: &[],
};
