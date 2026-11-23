//! Help section owned by the `sys` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_sys_help_section() {
    print_section(
        "help.sys.title",
        &["help.sys.monitor", "help.sys.passthrough"],
        &["help.sys.native"],
        &[
            "help.sys.example1",
            "help.sys.example2",
            "help.sys.example3",
        ],
    );
}
