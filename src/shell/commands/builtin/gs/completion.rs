//! Static subcommand/flag completions for the `gs` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "-p", "--path", "-s", "--sort", "-d", "-l", "--limit", "-A", "--after", "-B", "--before",
        "-I", "-x", "-h", "-g", "-w", "-z",
    ],
    flat: true,
    nodes: &[],
};
