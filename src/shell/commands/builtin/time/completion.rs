//! Static flag completions for the `time` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "--count",
        "--bench",
        "--compare",
        "--with",
        "-w",
        "--help",
        "-h",
    ],
    flat: false,
    nodes: &[],
};
