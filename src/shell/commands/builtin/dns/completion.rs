//! Static subcommand/flag completions for the `dns` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec, ValueSlot};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["-t", "--type"],
    flat: false,
    nodes: &[CompletionNode {
        path: &[],
        case_sensitive: false,
        tokens: &["-t", "--type"],
        deeper: None,
        beyond: None,
        value_slots: &[ValueSlot {
            after: &["-t", "--type"],
            values: &["A", "AAAA", "MX", "NS", "CNAME", "TXT", "PTR", "SOA", "ALL"],
        }],
        suppress_after: &[],
        scope_flags: false,
    }],
};
