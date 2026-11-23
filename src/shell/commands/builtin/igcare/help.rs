//! Help section owned by the `igcare` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_igcare_help_section() {
    print_section(
        "help.igcare.title",
        &[
            "help.igcare.export_all",
            "help.igcare.export_select",
            "help.igcare.import",
            "help.igcare.make",
        ],
        &[],
        &["help.igcare.example1"],
    );
}
