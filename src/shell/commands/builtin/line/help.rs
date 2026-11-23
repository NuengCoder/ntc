//! Help section owned by the `line` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_line_help_section() {
    print_section(
        "help.line.title",
        &["help.line.usage"],
        &[],
        &["help.line.example1"],
    );
}
