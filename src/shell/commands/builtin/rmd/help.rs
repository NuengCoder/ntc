//! Help section owned by the `rmd` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_rmd_help_section() {
    print_section(
        "help.rmd.title",
        &["help.rmd.usage"],
        &[],
        &["help.rmd.example1"],
    );
}
