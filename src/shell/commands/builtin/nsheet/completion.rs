//! Static subcommand/flag completions for the `nsheet` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "open",
        "view",
        "-d",
        "--delimiter",
        "--header",
        "--no-header",
        "-l",
        "--limit",
        "--all",
        "--sort",
        "--desc",
        "--filter",
        "--export",
        "-o",
        "--output",
        "--help",
        "-h",
    ],
    flat: true,
    nodes: &[],
};
