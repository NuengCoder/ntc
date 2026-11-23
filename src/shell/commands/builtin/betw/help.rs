//! Help sections owned by the `betw` command module: the `betw` reference and
//! the `pipe` operator documentation topic.

use crate::shell::help::format::print_section;

/// `pipe` is operator documentation, not a registry command.
pub(crate) fn print_pipe_help_section() {
    print_section(
        "help.pipe.title",
        &[
            "help.pipe.pipe",
            "help.pipe.and",
            "help.pipe.or",
            "help.pipe.seq",
            "help.pipe.in",
            "help.pipe.out",
            "help.pipe.append",
            "help.pipe.heredoc",
            "help.pipe.herestring",
            "help.pipe.filters",
        ],
        &["help.pipe.note", "help.pipe.hint"],
        &[
            "help.pipe.example1",
            "help.pipe.example2",
            "help.pipe.example3",
            "help.pipe.example4",
            "help.pipe.example5",
            "help.pipe.example6",
        ],
    );
}

pub(crate) fn print_betw_help_section() {
    print_section(
        "help.betw.title",
        &["help.betw.usage"],
        &[],
        &["help.betw.example1"],
    );
}
