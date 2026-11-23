//! Help section owned by the `gs` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_gs_help_section() {
    print_section(
        "help.gs.title",
        &[
            "help.gs.usage",
            "help.gs.interactive",
            "help.gs.pipe",
            "help.gs.context",
            "help.gs.filters",
        ],
        &[],
        &[
            "help.gs.example1",
            "help.gs.example2",
            "help.gs.example3",
            "help.gs.example4",
            "help.gs.example5",
            "help.gs.example6",
            "help.gs.example7",
        ],
    );
}
