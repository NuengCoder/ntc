//! Help section owned by the `resetcg` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_resetcg_help_section() {
    print_section(
        "help.resetcg.title",
        &["help.resetcg.usage"],
        &[],
        &["help.resetcg.example1"],
    );
}
