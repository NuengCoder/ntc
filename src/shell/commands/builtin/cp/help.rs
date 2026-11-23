//! Help section owned by the `cp` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_cp_help_section() {
    print_section(
        "help.cp.title",
        &["help.cp.usage"],
        &[],
        &["help.cp.example1"],
    );
}
