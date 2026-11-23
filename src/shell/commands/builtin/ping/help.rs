//! Help section owned by the `ping` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ping_help_section() {
    print_section(
        "help.ping.title",
        &["help.ping.usage", "help.ping.flags"],
        &[],
        &["help.ping.example1"],
    );
}
