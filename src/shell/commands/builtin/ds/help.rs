//! Help section owned by the `ds` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ds_help_section() {
    print_section(
        "help.ds.title",
        &["help.ds.usage"],
        &[],
        &["help.ds.example1"],
    );
}
