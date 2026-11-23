//! Static subcommand/flag completions for the `ui` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["classic"],
    flat: true,
    nodes: &[],
};
