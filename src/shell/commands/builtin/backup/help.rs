//! Help sections owned by the `backup` command module: `print_backup_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_backup_help_section() {
    section_header("help.backup.title");
    usage("help.backup.create");
    usage("help.backup.list");
    usage("help.backup.restore");
    usage("help.backup.restore_choose");
    usage("help.backup.restore_deleted");
    usage("help.backup.undo");
    usage("help.backup.undo_cls");
    usage("help.backup.undo_force");
    usage("help.backup.clear");
    usage("help.backup.clear_force");
    usage("help.backup.recreate");
    usage("help.backup.diff");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.backup.example1");
    example_row("help.backup.example2");
    example_row("help.backup.example3");
    example_row("help.backup.example4");
}
