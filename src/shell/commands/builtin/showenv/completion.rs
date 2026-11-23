//! Static subcommand/flag completions for the `showenv` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--all", "-a", "-d", "--depth"],
    flat: false,
    nodes: &[],
};
