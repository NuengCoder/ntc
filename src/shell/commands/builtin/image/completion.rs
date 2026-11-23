//! Static subcommand/flag completions for the `image` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["display", "show", "--sixel"],
    flat: false,
    nodes: &[CompletionNode::at(&[&["display", "show"]], &["--sixel"])],
};
