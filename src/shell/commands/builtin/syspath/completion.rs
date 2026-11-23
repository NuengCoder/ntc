//! Static subcommand/flag completions for the `syspath` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["show", "ls", "add", "rm", "remove", "move", "mv"],
    flat: false,
    nodes: &[CompletionNode::at_cs(&[&["add"]], &["--cwd", "--here"])],
};
