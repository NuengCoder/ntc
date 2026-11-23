//! Help section owned by the `ntpx` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ntpx_help_section() {
    print_section(
        "help.ntpx.title",
        &["help.ntpx.usage"],
        &[],
        &["help.ntpx.example1"],
    );
}
