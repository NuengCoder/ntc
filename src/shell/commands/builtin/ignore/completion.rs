//! Static subcommand/flag completions for the `ignore` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["dir", "ext", "file"],
    flat: false,
    nodes: &[CompletionNode::scoped_cs(&[&["dir", "ext", "file"]])],
};
