//! Static subcommand/flag completions for the `ping` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "-c",
        "--count",
        "--forever",
        "-i",
        "--interval",
        "-w",
        "--timeout",
        "-s",
        "--size",
        "-t",
        "--ttl",
        "-4",
        "-6",
        "-q",
        "--quiet",
    ],
    flat: true,
    nodes: &[],
};
