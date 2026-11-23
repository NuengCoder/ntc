//! Static subcommand/flag completions for the `fake` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["gen", "types", "locales", "group", "check", "help"],
    flat: false,
    nodes: &[
        CompletionNode::at(
            &[&["gen", "generate", "g"]],
            &[
                "-n", "--count", "-f", "--format", "-s", "--seed", "-o", "--output", "-g",
                "--group", "-l", "--locale", "-u", "--unique",
            ],
        ),
        CompletionNode::at(&[&["types", "type", "t"]], &["-s", "--search"]),
        CompletionNode::at(&[&["locales", "locale", "loc"]], &[]),
        CompletionNode {
            path: &[&["group", "grp", "groups"]],
            case_sensitive: false,
            tokens: &[
                "add", "edit", "rm", "remove", "rnm", "rename", "ls", "list", "info", "clear",
                "export", "import", "help",
            ],
            deeper: Some(&[]),
            beyond: None,
            value_slots: &[],
            suppress_after: &[],
            scope_flags: false,
        },
        CompletionNode::at(
            &[&["group", "grp", "groups"], &["add", "a", "edit", "ed"]],
            &[],
        ),
        CompletionNode::at(
            &[&["group", "grp", "groups"], &["rm", "remove", "delete"]],
            &["--force", "-f"],
        ),
        CompletionNode::at(
            &[&["group", "grp", "groups"], &["export", "exp"]],
            &["--all", "-a", "-o", "--output"],
        ),
        CompletionNode::at(
            &[&["group", "grp", "groups"], &["import", "imp"]],
            &["--force", "-f", "--prefix", "-p"],
        ),
    ],
};
