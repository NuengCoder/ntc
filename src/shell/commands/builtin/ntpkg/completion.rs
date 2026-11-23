//! Static subcommand/flag completions for the `ntpkg` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "install",
        "i",
        "uninstall",
        "rm",
        "list",
        "ls",
        "search",
        "info",
        "use",
        "which",
        "available",
        "avail",
        "help",
    ],
    flat: true,
    nodes: &[],
};
