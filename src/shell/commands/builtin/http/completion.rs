//! Static subcommand/flag completions for the `http` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "QUERY"],
    flat: false,
    nodes: &[CompletionNode::at(
        &[],
        &[
            "-H",
            "--header",
            "-d",
            "--data",
            "--json",
            "-q",
            "--query",
            "-a",
            "--auth",
            "--bearer",
            "-t",
            "--timeout",
            "-s",
            "--silent",
            "-o",
            "--output",
            "-v",
            "--verbose",
            "--no-pretty",
            "-h",
            "--help",
        ],
    )],
};
