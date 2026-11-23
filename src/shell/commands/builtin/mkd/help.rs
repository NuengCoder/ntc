//! Help section owned by the `mkd` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_mkd_help_section() {
    print_section(
        "help.mkd.title",
        &["help.mkd.usage"],
        &[],
        &["help.mkd.example1"],
    );
}
