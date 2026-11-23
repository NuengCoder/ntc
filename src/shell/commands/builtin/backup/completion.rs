//! Static subcommand/flag completions for the `backup` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "create", "cr", "restore", "rs", "undo", "ud", "clear", "cl", "recreate", "re", "diff",
        "list", "ls",
    ],
    flat: false,
    nodes: &[
        CompletionNode::at_cs(
            &[&["restore", "rs"]],
            &["--choose", "--deleted", "--force", "-f"],
        ),
        CompletionNode::at_cs(&[&["undo", "ud"]], &["--cls", "--clear", "--force", "-f"]),
        CompletionNode::at_cs(&[&["clear", "cl"]], &["--force", "-f"]),
        CompletionNode::at_cs(&[&["recreate", "re"]], &["--force", "-f"]),
    ],
};
