//! Static subcommand/flag completions for the `qr` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["encode", "en", "decode", "de"],
    flat: false,
    nodes: &[],
};
