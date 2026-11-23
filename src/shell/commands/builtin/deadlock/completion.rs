//! Static subcommand/flag completions for the `deadlock` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "ls",
        "add",
        "rmpw",
        "master-pass",
        "forget",
        "destroy",
        "help",
        "exit",
    ],
    flat: false,
    nodes: &[],
};
