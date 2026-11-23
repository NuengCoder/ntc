//! Help section owned by the `igcared` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_igcared_help_section() {
    print_section(
        "help.igcared.title",
        &["help.igcared.usage"],
        &[],
        &["help.igcared.example1"],
    );
}
