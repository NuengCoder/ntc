//! `rmd` — remove directories, including `*` patterns.

pub(crate) mod help;

use super::super::support::file_ops::{
    expand_glob_in_dir_filtered, is_deadlock_vault_dir, is_root_or_admin, split_args,
};
use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_rmd(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.rmd.missing"));
        return Ok(false);
    }
    for item in &items {
        if item.contains('*') {
            remove_rmd_pattern(item, nav)?;
            continue;
        }
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if !path.exists() {
            print_error(&t!("misc.rmd.not_exist", item));
        } else if !path.is_dir() {
            print_error(&t!("misc.rmd.not_dir", item));
        } else if is_deadlock_vault_dir(&path) && !is_root_or_admin() {
            print_error(&t!("misc.rmd.deadlock_vault_admin_only", item));
        } else {
            if is_deadlock_vault_dir(&path) {
                // Root/administrator: lift the OS-level deletion barrier.
                let _ = crate::shell::commands::builtin::deadlock::deadlock::protect::unprotect(
                    &crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path(),
                );
            }
            if let Err(e) = std::fs::remove_dir_all(&path) {
                print_error(&t!("misc.rmd.error", item, e));
            } else {
                print_success(&t!("misc.rmd.removed", item));
            }
        }
    }
    Ok(false)
}

/// Expand one `rmd` operand containing `*` wildcards (`*`, `old*`,
/// `parent/*`, `@tp1/old*`) into the matching directories. Files never
/// match — `rmd` only removes directories. The parent part goes through
/// `resolve_file_operand`, so plain paths and teleport savepoints both
/// expand from the right base. Pure for testability.
fn expand_rmd_pattern(item: &str, nav: &Navigator) -> Vec<PathBuf> {
    let raw = Path::new(item);
    match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = match resolve_file_operand(&dir.to_string_lossy(), nav) {
                Ok(p) => p,
                Err(_) => return Vec::new(),
            };
            let pat = raw.file_name().and_then(|n| n.to_str()).unwrap_or(item);
            expand_glob_in_dir_filtered(&base, pat, true)
        }
        None => expand_glob_in_dir_filtered(nav.current_path(), item, true),
    }
}

/// Delete the directories matched by one `rmd` wildcard operand. A bulk
/// delete always lists the matches and asks for confirmation first, so
/// `rmd *` can never wipe the working directory in one keystroke.
fn remove_rmd_pattern(item: &str, nav: &Navigator) -> Result<()> {
    let matches = expand_rmd_pattern(item, nav);
    if matches.is_empty() {
        print_error(&t!("misc.rmd.no_match", item));
        return Ok(());
    }
    println!();
    println!(
        "{}",
        t!("misc.rmd.pattern_matches", matches.len(), item)
            .yellow()
            .bold()
    );
    for (i, p) in matches.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().yellow(), p.display());
    }
    println!();
    let ok = crate::utils::interactive::prompt_yes_no(
        &t!("misc.rmd.confirm_delete", matches.len()),
        false,
    )
    .unwrap_or(false);
    if !ok {
        println!("{}", t!("misc.rmd.cancelled").dimmed());
        return Ok(());
    }
    for path in &matches {
        if is_deadlock_vault_dir(path) {
            if !is_root_or_admin() {
                print_error(&t!("misc.rmd.deadlock_vault_admin_only", path.display()));
                continue;
            }
            // Root/administrator: lift the OS-level deletion barrier.
            let _ = crate::shell::commands::builtin::deadlock::deadlock::protect::unprotect(
                &crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path(),
            );
        }
        match std::fs::remove_dir_all(path) {
            Ok(()) => print_success(&t!("misc.rmd.removed", path.display())),
            Err(e) => print_error(&t!("misc.rmd.error", path.display(), e)),
        }
    }
    Ok(())
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "rmd",
    aliases: &[],
    help_topic: Some("rmd"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_rmd),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rmd_star_expands_to_subdirectories_only() {
        let tmp = tempfile::tempdir().unwrap();
        for d in ["alpha", "beta"] {
            std::fs::create_dir(tmp.path().join(d)).unwrap();
        }
        std::fs::write(tmp.path().join("notes.txt"), "x").unwrap();

        let nav = Navigator::for_test(tmp.path());
        let got = expand_rmd_pattern("*", &nav);
        assert_eq!(got.len(), 2, "got: {got:?}");
        assert!(got.iter().all(|p| p.is_dir()), "got: {got:?}");
        assert!(got.iter().any(|p| p.ends_with("alpha")));
        assert!(got.iter().any(|p| p.ends_with("beta")));
        assert!(!got.iter().any(|p| p.ends_with("notes.txt")));
    }

    #[test]
    fn rmd_pattern_accepts_a_parent_directory() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("keep/one")).unwrap();
        std::fs::create_dir_all(tmp.path().join("keep/two")).unwrap();
        std::fs::create_dir_all(tmp.path().join("other/three")).unwrap();

        let nav = Navigator::for_test(tmp.path());
        let pattern = format!("{}/*", tmp.path().join("keep").display());
        let got = expand_rmd_pattern(&pattern, &nav);
        assert_eq!(got.len(), 2, "got: {got:?}");
        let keep = tmp.path().join("keep");
        assert!(got.iter().all(|p| p.parent() == Some(keep.as_path())));
    }

    #[test]
    fn rmd_pattern_ignores_files_named_like_the_stub() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("old_dir")).unwrap();
        std::fs::write(tmp.path().join("old_file.txt"), "x").unwrap();

        let nav = Navigator::for_test(tmp.path());
        let got = expand_rmd_pattern("old*", &nav);
        assert_eq!(got.len(), 1, "got: {got:?}");
        assert!(got[0].ends_with("old_dir"));
    }
}
