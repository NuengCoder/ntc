//! Static subcommand/flag completions for the `theme` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "add", "use", "update", "rm", "remove", "edit", "info", "export", "import", "install",
        "make", "mk", "where", "wh", "reload", "n", "number", "rnm", "rename", "list", "ls",
        "check",
    ],
    flat: true,
    nodes: &[],
};
