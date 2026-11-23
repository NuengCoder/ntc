//! Static subcommand/flag completions for the `txtc` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--line", "--lines", "-l", "--no-line", "--no-lines", "-nl"],
    flat: true,
    nodes: &[],
};
