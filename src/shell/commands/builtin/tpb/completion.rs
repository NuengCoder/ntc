//! Static subcommand/flag completions for the `tpb` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["history", "hist", "h", "clear", "cls"],
    flat: true,
    nodes: &[],
};
