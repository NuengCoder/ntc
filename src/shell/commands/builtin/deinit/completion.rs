//! Static subcommand/flag completions for the `deinit` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["ran", "local", "opencode", "help"],
    flat: false,
    nodes: &[],
};
