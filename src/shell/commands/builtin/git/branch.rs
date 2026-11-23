//! `git branch` rendering.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::git as gitmod;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

pub(super) fn cmd_branch(nav: &Navigator) -> Result<bool> {
    if !gitmod::is_repo(nav.current_path()) {
        print_error(&t!("git.not_repo"));
        return Ok(false);
    }
    let out = match gitmod::git_output(nav.current_path(), &["branch"]) {
        Some(o) => o,
        None => {
            print_error(&t!("git.branch_failed"));
            return Ok(false);
        }
    };
    let theme = crate::utils::theme::ThemeManager::current();
    for line in out.lines() {
        let is_current = line.starts_with('*');
        let name = line.trim().trim_start_matches("* ").trim();
        if name.is_empty() {
            continue;
        }
        if is_current {
            println!(
                "  {} {}",
                theme.shell.git_branch.paint_ansi_bold("*"),
                theme.shell.success.paint_ansi_bold(name)
            );
        } else {
            println!("  {}", name.dimmed());
        }
    }
    Ok(false)
}
