//! Help sections owned by the `todo` command module: `print_todo_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_todo_help_section() {
    section_header("help.todo.title");
    usage("help.todo.show");
    usage("help.todo.info");
    usage("help.todo.add");
    usage("help.todo.add_at");
    usage("help.todo.done");
    usage("help.todo.undo");
    usage("help.todo.edit");
    usage("help.todo.rm");
    usage("help.todo.clear");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.todo.example1");
    example_row("help.todo.example2");
    example_row("help.todo.example3");
}
