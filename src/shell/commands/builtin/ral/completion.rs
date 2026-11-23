//! Static subcommand/flag completions for the `ral` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "add", "edit", "rm", "remove", "list", "ls", "info", "rnm", "rename", "export", "import",
        "install", "make", "mk", "cls", "clear", "hintn", "check",
    ],
    flat: false,
    nodes: &[
        CompletionNode::at(&[&["export"]], &["--all", "-a", "--select", "-s"]),
        CompletionNode::at(&[&["info"]], &["--edit", "-e"]),
        CompletionNode::scoped(&[&[
            "add", "edit", "rm", "remove", "cls", "clear", "rnm", "rename", "import",
        ]]),
        CompletionNode {
            path: &[&["hintn"]],
            case_sensitive: false,
            tokens: &["add", "rm", "remove", "rnm", "rename", "info", "list", "ls"],
            deeper: Some(&[]),
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: false,
        },
        CompletionNode {
            path: &[&["hintn"], &["rm", "remove"]],
            case_sensitive: false,
            tokens: &["--force", "-f"],
            deeper: None,
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: true,
        },
        CompletionNode::scoped(&[&["hintn"], &["add", "rnm", "rename"]]),
    ],
};
