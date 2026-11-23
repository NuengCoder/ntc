//! Static subcommand/flag completions for the `skill` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec, ValueSlot};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "download",
        "install",
        "i",
        "dl",
        "add",
        "remove",
        "delete",
        "uninstall",
        "u",
        "edit",
        "list",
        "ls",
        "avail",
        "available",
        "info",
        "sub",
        "help",
    ],
    flat: false,
    nodes: &[
        CompletionNode::at(
            &[&["download", "dl", "install", "i"]],
            &["--url", "--name", "--force", "-f"],
        ),
        CompletionNode::at(&[&["delete", "del", "uninstall", "u"]], &["--force", "-f"]),
        CompletionNode::at(&[&["list", "ls"]], &["--all", "-a"]),
        CompletionNode::at(
            &[&["info"]],
            &crate::shell::commands::builtin::skill::skill::harness::HARNESSES,
        ),
        CompletionNode::at(&[&["sub"]], &["ls", "list"]),
        CompletionNode {
            path: &[&["add"]],
            case_sensitive: false,
            tokens: &["to", "--global", "-g", "--force", "-f"],
            deeper: Some(&["--global", "-g", "--force", "-f"]),
            beyond: None,
            value_slots: &[ValueSlot {
                after: &["to", "from"],
                values: &crate::shell::commands::builtin::skill::skill::harness::HARNESSES,
            }],
            suppress_after: &[],
            scope_flags: false,
        },
        CompletionNode {
            path: &[&["remove", "rm"]],
            case_sensitive: false,
            tokens: &["from", "--global", "-g", "--force", "-f"],
            deeper: Some(&["--global", "-g", "--force", "-f"]),
            beyond: None,
            value_slots: &[ValueSlot {
                after: &["to", "from"],
                values: &crate::shell::commands::builtin::skill::skill::harness::HARNESSES,
            }],
            suppress_after: &[],
            scope_flags: false,
        },
    ],
};
