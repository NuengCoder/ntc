//! Static subcommand/flag completions for the `smtp` command.

use crate::shell::commands::core::completion::{CompletionNode, CompletionSpec};

pub(crate) static COMPLETION: CompletionSpec = CompletionSpec {
    root: &["-i", "--server", "--id-ls"],
    flat: false,
    nodes: &[CompletionNode::at(
        &[],
        &[
            "-i",
            "--server",
            "-p",
            "--port",
            "-u",
            "--user",
            "-pwd",
            "--password",
            "--id",
            "--id-ls",
            "--id-info",
            "--deadlock",
            "--id-rm",
            "-t",
            "--to",
            "-c",
            "--cc",
            "--bcc",
            "-s",
            "--subject",
            "-b",
            "--body",
            "--body-file",
            "-bf",
            "--body-format",
            "-f",
            "--attach",
            "-h",
            "--help",
        ],
    )],
};
