//! Static subcommand/flag completions for the `sort` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "--reverse",
        "-r",
        "--numeric",
        "-n",
        "--unique",
        "-u",
        "--ignore-case",
        "-f",
    ],
    flat: false,
    nodes: &[],
};
