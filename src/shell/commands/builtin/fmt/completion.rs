//! Static subcommand/flag completions for the `fmt` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec, ValueSlot};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[],
    flat: false,
    nodes: &[CompletionNode {
        path: &[&["*"]],
        case_sensitive: false,
        tokens: &["to", "-o", "--output"],
        deeper: Some(&[]),
        beyond: None,
        value_slots: &[ValueSlot {
            after: &["to"],
            values: &[
                "json", "yaml", "toml", "md", "html", "xml", "docx", "txt", "pdf", "csv", "tsv",
                "xlsx",
            ],
        }],
        suppress_after: &[],
        scope_flags: false,
    }],
};
