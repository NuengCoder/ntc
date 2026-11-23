//! Static subcommand/flag completions for the `trace` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "-m",
        "--max-hops",
        "-w",
        "--timeout",
        "-q",
        "--queries",
        "-n",
        "--numeric",
        "-4",
        "-6",
        "--no-fallback",
    ],
    flat: true,
    nodes: &[],
};
