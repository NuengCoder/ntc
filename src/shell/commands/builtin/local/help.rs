//! Help section owned by the `local` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_local_help_section() {
    print_section(
        "help.local.title",
        &["help.local.init", "help.local.help", "help.local.deinit"],
        &[],
        &["help.local.example1"],
    );
}
