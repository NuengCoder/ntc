//! Help section owned by the `encode` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_encode_help_section() {
    print_section(
        "help.encode.title",
        &[
            "help.encode.b2",
            "help.encode.b10",
            "help.encode.b64",
            "help.encode.hex",
            "help.encode.url",
            "help.encode.utf8",
            "help.encode.utf16",
            "help.encode.ascii",
        ],
        &[],
        &["help.encode.example1", "help.encode.example2"],
    );
}
