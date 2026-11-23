//! Help section owned by the `restorecg` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_restorecg_help_section() {
    print_section(
        "help.restorecg.title",
        &["help.restorecg.usage"],
        &[],
        &["help.restorecg.example1"],
    );
}
