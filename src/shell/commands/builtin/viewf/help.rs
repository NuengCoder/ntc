//! Help section owned by the `viewf` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_viewf_help_section() {
    print_section(
        "help.viewf.title",
        &[
            "help.viewf.tree",
            "help.viewf.file_usage",
            "help.viewf.size",
            "help.viewf.sort",
            "help.viewf.ext",
            "help.viewf.care",
            "help.viewf.time",
            "help.viewf.copy",
        ],
        &[],
        &[
            "help.viewf.example1",
            "help.viewf.example2",
            "help.viewf.example3",
            "help.viewf.example4",
        ],
    );
}
