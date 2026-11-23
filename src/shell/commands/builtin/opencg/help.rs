//! Help section owned by the `opencg` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_opencg_help_section() {
    print_section(
        "help.opencg.title",
        &["help.opencg.usage"],
        &[],
        &["help.opencg.example1"],
    );
}
