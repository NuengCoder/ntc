//! Help section owned by the `nprintf` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_nprintf_help_section() {
    print_section(
        "help.nprintf.title",
        &["help.nprintf.usage"],
        &[
            "help.nprintf.conversions",
            "help.nprintf.specs",
            "help.nprintf.specs2",
            "help.nprintf.flags",
            "help.nprintf.flags_detail",
            "help.nprintf.escapes",
            "help.nprintf.escapes_detail",
            "help.nprintf.newline",
            "help.nprintf.newline_detail",
            "help.nprintf.quoting",
            "help.nprintf.quoting_detail",
            "help.nprintf.reuse",
        ],
        &[
            "help.nprintf.example1",
            "help.nprintf.example2",
            "help.nprintf.example3",
            "help.nprintf.example4",
            "help.nprintf.example5",
        ],
    );
}
