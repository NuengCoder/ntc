//! Help section owned by the `cvcode` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_cvcode_help_section() {
    print_section(
        "help.cvcode.title",
        &["help.cvcode.usage"],
        &[],
        &["help.cvcode.example1", "help.cvcode.example2"],
    );
}
