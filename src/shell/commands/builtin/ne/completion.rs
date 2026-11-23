//! Static subcommand/flag completions for the `ne` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--init", "--super"],
    flat: false,
    nodes: &[CompletionNode::at_cs(&[&["--init", "--super"]], &["-I"])],
};
