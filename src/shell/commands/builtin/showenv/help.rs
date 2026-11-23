//! Help section owned by the `showenv` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_showenv_help_section() {
    print_section(
        "help.showenv.title",
        &[
            "help.showenv.usage",
            "help.showenv.all",
            "help.showenv.depth",
        ],
        &[],
        &["help.showenv.example1"],
    );
}
