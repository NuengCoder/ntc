//! Shell command wrapper for the `deadlock` password-vault shell.
//!
//! `deadlock` itself is implemented in `crate::deadlock` (see
//! `src/deadlock/mod.rs`); this module only bridges the ntc shell dispatcher
//! to that nested shell.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;

use anyhow::Result;

/// `deadlock` — open the encrypted password vault.
pub fn cmd_deadlock(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();

    if args == "--help" || args == "-h" || args == "help" {
        crate::shell::help::print_deadlock_help();
        return Ok(false);
    }

    if !args.is_empty() {
        print_error(&t!("deadlock.no_args"));
        return Ok(false);
    }

    crate::deadlock::run_deadlock_shell()?;
    Ok(false)
}
