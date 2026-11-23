//! Static subcommand/flag completions for the `nms` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["play", "scan", "help"],
    flat: false,
    nodes: &[
        CompletionNode::at(&[&["play"]], &["-sf", "--start-from"]),
        CompletionNode::at(
            &[&["scan"]],
            &["-d", "--depth", "-s", "--size", "-l", "--long", "--care"],
        ),
    ],
};
