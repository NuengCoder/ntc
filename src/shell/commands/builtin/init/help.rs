//! Help section owned by the `init` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_init_help_section() {
    print_section(
        "help.init.title",
        &["help.init.usage"],
        &[],
        &["help.init.example1", "help.init.example2"],
    );
}
