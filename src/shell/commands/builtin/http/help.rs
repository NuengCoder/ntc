//! Help section owned by the `http` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_http_help_section() {
    print_section(
        "help.http.title",
        &["help.http.usage", "help.http.usage2"],
        &["help.http.note"],
        &[
            "help.http.example1",
            "help.http.example2",
            "help.http.example3",
            "help.http.example4",
            "help.http.example5",
        ],
    );
}
