//! Help section owned by the `ipcfg` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ipcfg_help_section() {
    print_section(
        "help.ipcfg.title",
        &["help.ipcfg.usage"],
        &[],
        &["help.ipcfg.example1"],
    );
}
