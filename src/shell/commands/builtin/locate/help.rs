//! Help section owned by the `locate` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_locate_help_section() {
    print_section(
        "help.locate.title",
        &["help.locate.usage"],
        &[],
        &["help.locate.example1"],
    );
}
