//! Help section owned by the `decode` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_decode_help_section() {
    print_section(
        "help.decode.title",
        &[
            "help.decode.b2",
            "help.decode.b10",
            "help.decode.b64",
            "help.decode.hex",
            "help.decode.url",
            "help.decode.utf8",
            "help.decode.utf16",
            "help.decode.ascii",
        ],
        &[],
        &["help.decode.example1"],
    );
}
