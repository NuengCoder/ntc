//! Static subcommand/flag completions for the `report` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["project", "clip"],
    flat: false,
    nodes: &[
        CompletionNode::at_cs(&[&["clip"]], &[]),
        CompletionNode {
            path: &[&["project"]],
            case_sensitive: true,
            tokens: &[
                "txt", "json", "md", "html", "pdf", "docx", "xlsx", "--name", "-n",
            ],
            deeper: Some(&["--name", "-n"]),
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: false,
        },
    ],
};
