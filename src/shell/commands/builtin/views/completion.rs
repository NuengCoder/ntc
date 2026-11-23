//! Static subcommand/flag completions for the `views` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["-n", "-s", "--size", "--sort", "--trim"],
    flat: false,
    nodes: &[CompletionNode {
        path: &[],
        case_sensitive: false,
        tokens: &["-n", "-s", "--size", "--sort", "--trim"],
        deeper: None,
        beyond: None,
        value_slots: &[],
        suppress_after: &["-s", "--size", "--sort", "--trim"],
        scope_flags: false,
    }],
};
