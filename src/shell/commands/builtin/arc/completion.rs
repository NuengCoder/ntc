//! Static subcommand/flag completions for the `arc` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["create", "extract", "list", "help"],
    flat: false,
    nodes: &[],
};
