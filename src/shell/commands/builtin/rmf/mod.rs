//! `rmf` — remove files, including `*` patterns.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_ops::{
    expand_glob_in_dir, is_deadlock_vault, is_root_or_admin, split_args,
};

pub(crate) fn cmd_rmf(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.rmf.missing"));
        return Ok(false);
    }
    for item in &items {
        if item.contains('*') {
            remove_rmf_pattern(item, nav)?;
            continue;
        }
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if !path.exists() {
            print_error(&t!("misc.rmf.not_exist", item));
        } else if path.is_dir() {
            print_error(&t!("misc.rmf.is_dir", item));
        } else if is_deadlock_vault(&path) && !is_root_or_admin() {
            print_error(&t!("misc.rmf.deadlock_vault_admin_only", item));
        } else {
            if is_deadlock_vault(&path) {
                // Root/administrator: lift the OS-level deletion barrier.
                let _ =
                    crate::shell::commands::builtin::deadlock::deadlock::protect::unprotect(&path);
            }
            match std::fs::remove_file(&path) {
                Ok(()) => print_success(&t!("misc.rmf.removed", item)),
                Err(e) => print_error(&t!("misc.rmf.error", item, e)),
            }
        }
    }
    Ok(false)
}

/// Expand one `rmf` operand containing `*` wildcards (`*.log`,
/// `test*`, `dir/*.tmp`, `@tp1/*.log`) into the matching files.
/// Directories never match — `rmf` only removes files. The parent part goes
/// through `resolve_file_operand`, so savepoints expand from the right base.
/// Pure for testability.
fn expand_rmf_pattern(item: &str, nav: &Navigator) -> Vec<PathBuf> {
    let raw = Path::new(item);
    match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = match resolve_file_operand(&dir.to_string_lossy(), nav) {
                Ok(p) => p,
                Err(_) => return Vec::new(),
            };
            let pat = raw.file_name().and_then(|n| n.to_str()).unwrap_or(item);
            expand_glob_in_dir(&base, pat)
        }
        None => expand_glob_in_dir(nav.current_path(), item),
    }
}

/// Delete the files matched by one `rmf` wildcard operand: single match
/// goes directly, multiple matches list + confirm first.
fn remove_rmf_pattern(item: &str, nav: &Navigator) -> Result<()> {
    let matches = expand_rmf_pattern(item, nav);
    if matches.is_empty() {
        print_error(&t!("misc.rmf.no_match", item));
        return Ok(());
    }
    if matches.len() > 1 {
        println!();
        println!(
            "{}",
            t!("misc.rmf.pattern_matches", matches.len(), item)
                .yellow()
                .bold()
        );
        for (i, p) in matches.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().yellow(), p.display());
        }
        println!();
        let ok = crate::utils::interactive::prompt_yes_no(
            &t!("misc.rmf.confirm_delete", matches.len()),
            false,
        )
        .unwrap_or(false);
        if !ok {
            println!("{}", t!("misc.rmf.cancelled").dimmed());
            return Ok(());
        }
    }
    for path in &matches {
        if is_deadlock_vault(path) {
            if !is_root_or_admin() {
                print_error(&t!("misc.rmf.deadlock_vault_admin_only", path.display()));
                continue;
            }
            // Root/administrator may remove it, but the OS-level barrier
            // (deny-DELETE ACE / immutable flag) must be lifted first.
            let _ = crate::shell::commands::builtin::deadlock::deadlock::protect::unprotect(path);
        }
        match std::fs::remove_file(path) {
            Ok(()) => print_success(&t!("misc.rmf.removed", path.display())),
            Err(e) => print_error(&t!("misc.rmf.error", path.display(), e)),
        }
    }
    Ok(())
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "rmf",
    aliases: &[],
    help_topic: Some("rmf"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_rmf),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rmf_pattern_expands_to_matching_files_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.log"), "a").unwrap();
        std::fs::write(dir.path().join("b.log"), "b").unwrap();
        std::fs::write(dir.path().join("c.txt"), "c").unwrap();
        std::fs::create_dir(dir.path().join("d.log")).unwrap();

        let nav = Navigator::for_test(dir.path());

        // `*.log` matches files only — the `d.log` directory is skipped.
        let got = expand_rmf_pattern("*.log", &nav);
        assert_eq!(
            got,
            vec![dir.path().join("a.log"), dir.path().join("b.log"),]
        );
        // Prefix form and no-match form.
        assert_eq!(expand_rmf_pattern("a*", &nav).len(), 1);
        assert!(expand_rmf_pattern("*.zzz", &nav).is_empty());

        // `sub/*.log` scopes into a subdirectory.
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/e.log"), "e").unwrap();
        let got = expand_rmf_pattern("sub/*.log", &nav);
        assert_eq!(got, vec![dir.path().join("sub/e.log")]);
    }

    #[test]
    fn cmd_rmf_refuses_deadlock_vault_for_non_root() {
        let home = crate::utils::test_support::IsolatedConfigHome::new();
        // Only meaningful when NOT root/admin (root can remove anything).
        if is_root_or_admin() {
            eprintln!("skipped: running as root/admin");
            return;
        }
        let vault = crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path();
        // Hard safety net: never write test garbage into the real vault.
        assert!(
            vault.starts_with(home.path()),
            "refusing to run: {} is outside the isolated config home",
            vault.display()
        );
        std::fs::create_dir_all(vault.parent().unwrap()).ok();
        std::fs::write(&vault, "dummy vault content").ok();
        assert!(vault.exists());

        let mut nav = Navigator::new().unwrap();
        let result = cmd_rmf(&vault.to_string_lossy(), &mut nav);
        assert!(result.is_ok());
        // The vault must survive the attempt.
        assert!(vault.exists());

        // Also blocked via wildcard pattern.
        let glob = vault
            .parent()
            .unwrap()
            .join("deadlock.*")
            .to_string_lossy()
            .to_string();
        let result = cmd_rmf(&glob, &mut nav);
        assert!(result.is_ok());
        assert!(vault.exists());
    }
}
