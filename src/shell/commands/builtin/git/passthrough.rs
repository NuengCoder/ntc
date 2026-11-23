//! `git` passthrough for real git subcommands.

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::git as gitmod;
use anyhow::Result;

pub(super) fn passthrough(nav: &Navigator, cmd: &str) -> Result<bool> {
    let cmd = cmd.trim_end();
    print_info(&t!("git.executing", cmd));
    println!();
    let status = super::super::super::run_system_command(cmd, nav.current_path());
    println!();
    match status {
        Ok(exit_status) => {
            gitmod::invalidate();
            if exit_status.success() {
                print_success(&t!("git.completed"));
            } else {
                // Non-zero git exit must reach the caller's exit status.
                crate::shell::pipeline::mark_failed();
                match exit_status.code() {
                    Some(code) => print_error(&t!("git.exit_code", code)),
                    None => print_warning(&t!("git.terminated")),
                }
            }
            Ok(false)
        }
        Err(e) => {
            crate::shell::pipeline::mark_failed();
            print_error(&t!("git.failed", e));
            Ok(false)
        }
    }
}
