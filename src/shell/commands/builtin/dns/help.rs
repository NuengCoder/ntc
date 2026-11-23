//! Help section owned by the `dns` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_dns_help_section() {
    print_section(
        "help.dns.title",
        &["help.dns.usage"],
        &[],
        &["help.dns.example1"],
    );
}
