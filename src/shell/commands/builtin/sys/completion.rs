//! Static flags for the `sys` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["-l", "--log", "-d", "--delay"],
    flat: false,
    nodes: &[],
};
