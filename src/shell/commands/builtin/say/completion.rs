//! Static subcommand/flag completions for the `say` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--loop", "-n", "-t"],
    flat: false,
    nodes: &[],
};
