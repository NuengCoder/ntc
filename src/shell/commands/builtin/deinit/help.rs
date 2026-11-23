//! Help section owned by the `deinit` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_deinit_help_section() {
    print_section(
        "help.deinit.title",
        &["help.deinit.usage"],
        &[],
        &["help.deinit.example1"],
    );
}
