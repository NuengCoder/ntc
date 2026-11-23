//! Static subcommand/flag completions for the `local` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["init", "deinit", "help"],
    flat: true,
    nodes: &[],
};
