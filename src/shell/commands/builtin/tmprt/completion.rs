//! Static subcommand/flag completions for the `tmprt` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "com", "computer", "hw", "hardware", "weather", "w", "key", "city", "sensors", "sensor",
    ],
    flat: false,
    nodes: &[CompletionNode::at_cs(
        &[&["sensors", "sensor"]],
        &["status", "install", "uninstall", "debug"],
    )],
};
