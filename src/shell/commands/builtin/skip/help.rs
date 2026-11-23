//! Help section owned by the `skip` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_skip_help_section() {
    print_section(
        "help.skip.title",
        &["help.skip.usage1", "help.skip.usage2"],
        &["help.skip.note"],
        &["help.skip.example1", "help.skip.example2"],
    );
}
