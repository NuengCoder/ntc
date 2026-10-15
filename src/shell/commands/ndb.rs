//! Shell command wrapper for the `ndb` database shell.
//!
//! `ndb` itself is implemented in `crate::ndb` (see `src/ndb/mod.rs`); this
//! module only bridges the ntc shell dispatcher to that nested shell.

use crate::navigator::Navigator;
use crate::t;

use anyhow::Result;

/// `ndb` — open the NTC Database shell for the current directory.
pub fn cmd_ndb(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();

    if args == "--help" || args == "-h" {
        crate::shell::help::print_ndb_help();
        return Ok(false);
    }

    if !args.is_empty() {
        crate::output::print_error(&t!("ndb.no_args"));
        return Ok(false);
    }

    crate::ndb::run_ndb_shell(nav)?;
    Ok(false)
}
