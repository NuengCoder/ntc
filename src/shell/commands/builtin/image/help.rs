//! Help sections owned by the `image` command module: `print_image_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_image_help_section() {
    section_header("help.image.title");
    usage("help.image.interactive");
    usage("help.image.open");
    usage("help.image.display");
    usage("help.image.show");
    usage("help.image.sixel");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.image.example1");
    example_row("help.image.example2");
}
