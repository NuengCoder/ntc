//! Static subcommand/flag completions for the `math` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["check", "timer", "fun"],
    flat: false,
    nodes: &[],
};
