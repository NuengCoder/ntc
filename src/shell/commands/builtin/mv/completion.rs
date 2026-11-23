//! Static subcommand/flag completions for the `mv` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["-f", "--force"],
    flat: false,
    nodes: &[],
};
