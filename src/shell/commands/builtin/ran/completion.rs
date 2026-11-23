//! Static subcommand/flag completions for the `ran` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "init",
        "deinit",
        "help",
        "list",
        "ls",
        "check",
        "fmt",
        "--dry-run",
    ],
    flat: false,
    nodes: &[],
};
