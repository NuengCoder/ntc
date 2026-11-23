//! Static subcommand/flag completions for the `encode` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "b2", "b10", "b64", "base64", "hex", "url", "utf8", "utf16", "ascii",
    ],
    flat: false,
    nodes: &[CompletionNode::at(&[], &["-o", "--output"])],
};
