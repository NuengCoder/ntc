//! Static subcommand/flag completions for the `nssh` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "add", "ssh", "cp", "scan", "ls", "list", "info", "rm", "edit", "keygen",
    ],
    flat: false,
    nodes: &[
        CompletionNode::at(
            &[&["add", "edit"]],
            &[
                "--host",
                "--user",
                "-u",
                "--port",
                "-p",
                "--key",
                "--no-key",
                "--pass-env",
                "--no-pass-env",
            ],
        ),
        CompletionNode::at(
            &[&["ssh"]],
            &["--cmd", "--trust", "--ask-pass", "--timeout"],
        ),
        CompletionNode::at(&[&["cp"]], &["--trust", "--ask-pass", "--timeout"]),
        CompletionNode::at(&[&["scan"]], &["--ports", "--top", "--timeout"]),
        CompletionNode::at(&[&["rm"]], &["--yes", "-y"]),
    ],
};
