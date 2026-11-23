//! Help section owned by the `txtc` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_txtc_help_section() {
    print_section(
        "help.txtc.title",
        &["help.txtc.usage"],
        &[],
        &["help.txtc.example1"],
    );
}
