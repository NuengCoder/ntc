//! Help section owned by the `trace` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_trace_help_section() {
    print_section(
        "help.trace.title",
        &["help.trace.usage"],
        &[],
        &["help.trace.example1"],
    );
}
