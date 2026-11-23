//! Help sections owned by the `ndb` command module: `print_ndb_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

/// The `ndb` section of the interactive help.
pub(crate) fn print_ndb_help_section() {
    section_header("help.ndb.title");
    usage("help.ndb.launch");
    usage("help.ndb.root_help");
    usage("help.ndb.root_view");
    usage("help.ndb.root_interface");
    usage("help.ndb.root_sql");
    usage("help.ndb.root_exit");
    usage("help.ndb.db_viewf");
    usage("help.ndb.db_tables");
    usage("help.ndb.db_tablef");
    usage("help.ndb.db_schema");
    usage("help.ndb.db_write");
    usage("help.ndb.db_edit");
    usage("help.ndb.db_backup");
    usage("help.ndb.more");
    usage("help.ndb.indexes");
    usage("help.ndb.info");
    usage("help.ndb.dbal");
    usage("help.ndb.db_exit");
    usage("help.ndb.readonly");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.ndb.example1");
    example_row("help.ndb.example2");
}
