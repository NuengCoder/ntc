//! Help section owned by the `tpb` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_tpb_help_section() {
    print_section(
        "help.tpb.title",
        &["help.tpb.usage", "help.tpb.history", "help.tpb.clear"],
        &[],
        &["help.tpb.example1"],
    );
}
