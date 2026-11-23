//! Static subcommand/flag completions for the `init` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec, ValueSlot};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["ran", "local", "opencode", "project", "--force", "-f"],
    flat: false,
    nodes: &[CompletionNode {
        path: &[&["project"]],
        case_sensitive: false,
        tokens: &["--lang", "-l", "--license", "--no"],
        deeper: None,
        beyond: None,
        value_slots: &[
            ValueSlot {
                after: &["--lang", "-l"],
                values: &[
                    "python", "rust", "go", "js", "ts", "java", "c", "c++", "other",
                ],
            },
            ValueSlot {
                after: &["--license"],
                values: &["MIT", "APACHE"],
            },
            ValueSlot {
                after: &["--no"],
                values: &["docker", "git", "ai", "doc", "ntc", "oc", "lc"],
            },
        ],
        suppress_after: &[],
        scope_flags: false,
    }],
};
