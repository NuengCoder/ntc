//! Static subcommand/flag completions for the `clip` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["list", "ls", "select", "sel", "clear", "cl"],
    flat: false,
    nodes: &[],
};
