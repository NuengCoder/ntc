//! Help section owned by the `mv` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_mv_help_section() {
    print_section(
        "help.mv.title",
        &["help.mv.usage"],
        &[],
        &["help.mv.example1"],
    );
}
