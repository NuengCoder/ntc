//! Static subcommand/flag completions for the `mode` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["admin", "normal"],
    flat: false,
    nodes: &[],
};
