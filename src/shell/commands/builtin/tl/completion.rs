//! Static subcommand/flag completions for the `tl` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["file", "ext", "dir"],
    flat: false,
    nodes: &[CompletionNode {
        path: &[&["ext", "dir"]],
        case_sensitive: true,
        tokens: &["-s", "-n", "-l", "-r"],
        deeper: Some(&[]),
        beyond: None,
        value_slots: &[],
        suppress_after: &[],
        scope_flags: false,
    }],
};
