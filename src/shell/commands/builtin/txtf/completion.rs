//! Static subcommand/flag completions for the `txtf` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["--head", "--tail", "--betw", "--line", "--no-line"],
    flat: false,
    nodes: &[CompletionNode {
        path: &[],
        case_sensitive: false,
        tokens: &["--head", "--tail", "--betw", "--line", "--no-line"],
        deeper: None,
        beyond: None,
        value_slots: &[],
        suppress_after: &["--head", "--tail", "--betw"],
        scope_flags: false,
    }],
};
