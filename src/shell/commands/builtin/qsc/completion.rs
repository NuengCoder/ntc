//! Static subcommand/flag completions for the `qsc` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["listen", "start", "stop", "status", "stat", "startup"],
    flat: false,
    nodes: &[],
};
