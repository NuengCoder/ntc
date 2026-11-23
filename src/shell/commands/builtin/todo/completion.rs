//! Static subcommand/flag completions for the `todo` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "info", "add", "done", "undo", "edit", "rm", "remove", "move", "mv", "replace", "rp",
        "clear", "cl", "list", "ls",
    ],
    flat: false,
    nodes: &[],
};
