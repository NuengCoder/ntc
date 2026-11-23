//! Static subcommand/flag completions for the `qur` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "-s",
        "--select",
        "-rp",
        "--replace",
        "-dp",
        "--display",
        "-d",
        "--depth",
        "-l",
        "--limit",
    ],
    flat: false,
    nodes: &[CompletionNode {
        path: &[],
        case_sensitive: false,
        tokens: &[
            "-s",
            "--select",
            "-rp",
            "--replace",
            "-dp",
            "--display",
            "-d",
            "--depth",
            "-l",
            "--limit",
        ],
        deeper: None,
        beyond: None,
        value_slots: &[],
        suppress_after: &[
            "-s",
            "--select",
            "-rp",
            "--replace",
            "-d",
            "--depth",
            "-l",
            "--limit",
        ],
        scope_flags: false,
    }],
};
