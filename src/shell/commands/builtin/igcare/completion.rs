//! Static subcommand/flag completions for the `igcare` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["export", "import", "make", "mk", "check"],
    flat: false,
    nodes: &[CompletionNode::at(
        &[&["export"]],
        &["--all", "-a", "--select", "-s"],
    )],
};
