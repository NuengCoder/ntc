//! Help section owned by the `viewd` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_viewd_help_section() {
    print_section(
        "help.viewd.title",
        &[
            "help.viewd.usage",
            "help.viewd.depth",
            "help.viewd.size",
            "help.viewd.care",
            "help.viewd.sort",
            "help.viewd.time",
            "help.viewd.copy",
        ],
        &[],
        &[
            "help.viewd.example1",
            "help.viewd.example2",
            "help.viewd.example3",
            "help.viewd.example4",
        ],
    );
}
