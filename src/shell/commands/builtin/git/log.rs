//! `git log` rendering.

use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::t;
use crate::utils::git as gitmod;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

pub(super) fn cmd_log(nav: &Navigator) -> Result<bool> {
    if !gitmod::is_repo(nav.current_path()) {
        print_error(&t!("git.not_repo"));
        return Ok(false);
    }
    let out = match gitmod::git_output(nav.current_path(), &["log", "--pretty=format:%h %s", "-20"])
    {
        Some(o) => o,
        None => {
            print_error(&t!("git.log_failed"));
            return Ok(false);
        }
    };
    if out.trim().is_empty() {
        print_warning(&t!("git.log_empty"));
        return Ok(false);
    }
    let theme = crate::utils::theme::ThemeManager::current();
    for line in out.lines() {
        if let Some(rest) = line.split_once(' ') {
            println!(
                "  {} {}",
                theme.shell.git_branch.paint_ansi_bold(rest.0),
                rest.1.dimmed()
            );
        } else {
            println!("  {}", line.dimmed());
        }
    }
    Ok(false)
}
