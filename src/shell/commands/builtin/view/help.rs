//! Help section owned by the `view` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_view_help_section() {
    print_section(
        "help.view.title",
        &[
            "help.view.tree",
            "help.view.size",
            "help.view.filesize",
            "help.view.dirsize",
            "help.view.sort",
            "help.view.head",
            "help.view.tail",
            "help.view.file_usage",
            "help.view.file_only",
            "help.view.dir_only",
            "help.view.ext",
            "help.view.cp",
            "help.view.depth",
            "help.view.care",
            "help.view.time",
            "help.view.path",
        ],
        &[],
        &[
            "help.view.example1",
            "help.view.example2",
            "help.view.example3",
            "help.view.example4",
            "help.view.example5",
            "help.view.example6",
            "help.view.example7",
            "help.view.example8",
            "help.view.example9",
            "help.view.example10",
            "help.view.example11",
            "help.view.example12",
        ],
    );
}
