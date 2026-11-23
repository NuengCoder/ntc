//! Help sections owned by the `fake` command module: `print_fake_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_fake_help_section() {
    section_header("help.fake.title");
    usage("help.fake.gen");
    usage("help.fake.gen_options");
    usage("help.fake.gen_group");
    usage("help.fake.gen_unique");
    usage("help.fake.gen_locale");
    usage("help.fake.types");
    usage("help.fake.types_search");
    usage("help.fake.locales");
    usage("help.fake.group_add");
    usage("help.fake.group_edit");
    usage("help.fake.group_rm");
    usage("help.fake.group_rnm");
    usage("help.fake.group_ls");
    usage("help.fake.group_info");
    usage("help.fake.group_clear");
    usage("help.fake.group_export");
    usage("help.fake.group_import");
    usage("help.fake.group_ref");
    usage("help.fake.help");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.fake.example1");
    example_row("help.fake.example2");
    example_row("help.fake.example3");
    example_row("help.fake.example4");
}
