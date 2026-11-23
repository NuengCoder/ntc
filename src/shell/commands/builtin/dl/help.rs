//! Help section owned by the `dl` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_dl_help_section() {
    print_section(
        "help.dl.title",
        &["help.dl.usage"],
        &[],
        &["help.dl.example1"],
    );
}
