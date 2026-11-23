//! Static subcommand/flag completions for the `size` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--care", "-p", "--path"],
    flat: false,
    nodes: &[],
};
