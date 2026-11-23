//! Static subcommand/flag completions for the `git` command.

use crate::shell::commands::core::completion::CompletionSpec;

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &[
        "status", "st", "branch", "b", "log", "l", "diff", "d", "add", "a", "commit", "push",
        "pull", "clone", "checkout", "co", "merge", "fetch", "remote", "init", "stash", "rebase",
        "tag", "help", "h",
    ],
    flat: false,
    nodes: &[],
};
