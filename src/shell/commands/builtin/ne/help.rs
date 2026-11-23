//! Help section owned by the `ne` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ne_help_section() {
    print_section(
        "help.ne.title",
        &[
            "help.ne.open",
            "help.ne.line",
            "help.ne.welcome",
            "help.ne.init",
            "help.ne.super",
        ],
        &[],
        &[
            "help.ne.example1",
            "help.ne.example2",
            "help.ne.example3",
            "help.ne.example4",
        ],
    );
}
