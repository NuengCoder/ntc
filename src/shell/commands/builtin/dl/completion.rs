//! Static subcommand/flag completions for the `dl` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "--resume",
        "--stop",
        "--history",
        "--cancel",
        "-o",
        "--output",
    ],
    flat: true,
    nodes: &[],
};
