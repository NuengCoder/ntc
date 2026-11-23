//! Static flag completions for the `count` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--line", "-l"],
    flat: false,
    nodes: &[],
};
