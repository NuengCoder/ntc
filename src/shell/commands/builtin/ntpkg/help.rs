//! Help section owned by the `ntpkg` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ntpkg_help_section() {
    print_section(
        "help.ntpkg.title",
        &[
            "help.ntpkg.install",
            "help.ntpkg.uninstall",
            "help.ntpkg.list",
            "help.ntpkg.use",
            "help.ntpkg.which",
            "help.ntpkg.search",
            "help.ntpkg.info",
            "help.ntpkg.available",
            "help.ntpkg.help",
        ],
        &[],
        &[
            "help.ntpkg.example1",
            "help.ntpkg.example2",
            "help.ntpkg.example3",
            "help.ntpkg.example4",
            "help.ntpkg.example5",
            "help.ntpkg.example6",
            "help.ntpkg.example7",
            "help.ntpkg.example8",
        ],
    );
}
