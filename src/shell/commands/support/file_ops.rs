//! Shared helpers for file-operation commands.

use crate::navigator::Navigator;
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use std::path::{Path, PathBuf};

/// Split a comma-separated operand list, trimming empty entries.
pub(crate) fn split_args(args: &str) -> Vec<String> {
    args.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// True when running as root (Unix) or Administrator (Windows).
pub(crate) fn is_root_or_admin() -> bool {
    #[cfg(unix)]
    {
        // SAFETY: `geteuid` takes no arguments, cannot fail and has no
        // preconditions, so calling it is always sound.
        return unsafe { libc::geteuid() } == 0;
    }
    #[cfg(windows)]
    {
        // `net session` succeeds only when elevated.
        return std::process::Command::new("net")
            .args(["session"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

/// `deadlock.vault` removal is restricted to root/administrator.
pub(crate) fn is_deadlock_vault(path: &Path) -> bool {
    let candidates = [
        crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path(),
        crate::shell::commands::builtin::deadlock::deadlock::vault::legacy_vault_path(),
    ];
    // Direct match (same type / path).
    if candidates.iter().any(|vault| path == vault) {
        return true;
    }
    // Canonicalize both sides so different CWDs, relative paths and symlinks
    // all resolve to the real file location.
    let canon = match path.canonicalize() {
        Ok(c) => c,
        Err(_) => return false,
    };
    candidates
        .iter()
        .filter_map(|vault| vault.canonicalize().ok())
        .any(|vault| canon == vault)
}

/// True when `path` is the dedicated directory that holds the deadlock vault.
pub(crate) fn is_deadlock_vault_dir(path: &Path) -> bool {
    let vault = crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path();
    let Some(vault_dir) = vault.parent() else {
        return false;
    };
    match (path.canonicalize(), vault_dir.canonicalize()) {
        (Ok(dir), Ok(vault_dir)) => dir == vault_dir,
        _ => false,
    }
}

/// Match the `*`-wildcard name pattern `pat` against the entries directly
/// inside `base`, keeping files (`want_dirs == false`) or directories.
/// The pattern is end-anchored so `*.log` means "ends with .log", not
/// "contains". Sorted. Pure for testability.
pub(crate) fn expand_glob_in_dir_filtered(base: &Path, pat: &str, want_dirs: bool) -> Vec<PathBuf> {
    let anchored = format!("{}$", pat);
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            let keep = match entry.file_type() {
                Ok(t) if want_dirs => t.is_dir(),
                Ok(t) => t.is_file(),
                Err(_) => false,
            };
            if !keep {
                continue;
            }
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if crate::search::matches_wildcard(&anchored, &name) {
                out.push(base.join(name));
            }
        }
    }
    out.sort();
    out
}

/// Match the `*`-wildcard file-name pattern `pat` against the files
/// directly inside `base`. Directories never match — patterns only ever
/// target files. Sorted. Pure for testability.
pub(crate) fn expand_glob_in_dir(base: &Path, pat: &str) -> Vec<PathBuf> {
    expand_glob_in_dir_filtered(base, pat, false)
}

/// Expand a `cp`/`mv` src operand containing a `*` wildcard. The parent
/// directory is resolved through `resolve_file_operand` so plain paths
/// (`dir/*.py`), CWD-relative patterns (`*.py`) and teleport subpaths
/// (`@tp/*.py`) all expand from the right base.
pub(crate) fn expand_operand_pattern(spec: &str, nav: &Navigator) -> Vec<PathBuf> {
    let raw = Path::new(spec);
    match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = match resolve_file_operand(&dir.to_string_lossy(), nav) {
                Ok(p) => p,
                Err(_) => return Vec::new(),
            };
            let pat = raw.file_name().and_then(|n| n.to_str()).unwrap_or(spec);
            expand_glob_in_dir(&base, pat)
        }
        None => expand_glob_in_dir(nav.current_path(), spec),
    }
}

/// True when both paths point at the same file (guards `cp` onto itself).
pub(crate) fn same_file(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

/// Copy `src` to `dst` then remove `src` (cross-device `mv` fallback).
pub(crate) fn move_by_copy(src: &Path, dst: &Path) -> std::io::Result<()> {
    copy_path(src, dst)?;
    if src.is_dir() {
        std::fs::remove_dir_all(src)?;
    } else {
        std::fs::remove_file(src)?;
    }
    Ok(())
}

/// Copy a file or directory tree to `dst` (no removal).
pub(crate) fn copy_path(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        copy_dir_all(src, dst)?;
    } else {
        if let Some(parent) = dst.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::copy(src, dst)?;
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in walkdir::WalkDir::new(src).min_depth(1) {
        let entry =
            entry.map_err(|e| std::io::Error::other(format!("walk {}: {}", src.display(), e)))?;
        let rel = entry
            .path()
            .strip_prefix(src)
            .map_err(std::io::Error::other)?;
        let target = dst.join(rel);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// Shared wildcard prelude for `cp`/`mv`: expand `src_pat` into files and
/// require `dst` to be a directory (created when missing). Returns the
/// destination directory and the matched sources, or `None` after printing
/// a localized error (`no_match_key` / `dst_dir_key`).
pub(crate) fn expand_wildcard_src(
    src_pat: &str,
    dst: &Path,
    nav: &Navigator,
    no_match_key: &str,
    dst_dir_key: &str,
) -> Option<(PathBuf, Vec<PathBuf>)> {
    let sources = expand_operand_pattern(src_pat, nav);
    if sources.is_empty() {
        crate::output::print_error(&t!(no_match_key, src_pat));
        return None;
    }
    if dst.is_dir() {
        return Some((dst.to_path_buf(), sources));
    }
    if dst.exists() {
        crate::output::print_error(&t!(dst_dir_key, dst.display()));
        return None;
    }
    if std::fs::create_dir_all(dst).is_err() {
        crate::output::print_error(&t!(dst_dir_key, dst.display()));
        return None;
    }
    Some((dst.to_path_buf(), sources))
}

/// `cp <pattern> <dir>`: copy every file matched by a `*` source into
/// `dst_spec` (which must be, or become, a directory). `-f` overwrites an
/// existing target file; a target directory is never replaced by a file.
pub(crate) fn copy_wildcard_batch(
    src_pat: &str,
    dst_spec: &str,
    dst: &Path,
    force: bool,
    nav: &Navigator,
) {
    let Some((dst_dir, sources)) =
        expand_wildcard_src(src_pat, dst, nav, "misc.cp.no_match", "misc.cp.dst_dir")
    else {
        return;
    };
    let mut done = 0usize;
    for src in sources {
        let name = match src.file_name() {
            Some(n) => n,
            None => continue,
        };
        let target = dst_dir.join(name);
        if same_file(&src, &target) {
            crate::output::print_error(&t!("misc.cp.same", src.display()));
            continue;
        }
        if target.exists() && !force {
            crate::output::print_error(&t!("misc.cp.exists", target.display()));
            continue;
        }
        if force && target.exists() {
            if target.is_dir() {
                // Never replace a directory with a file.
                crate::output::print_error(&t!(
                    "misc.cp.error",
                    "cannot overwrite directory with file"
                ));
                continue;
            }
            if std::fs::remove_file(&target).is_err() {
                crate::output::print_error(&t!("misc.cp.error", "cannot clear existing target"));
                continue;
            }
        }
        match copy_path(&src, &target) {
            Ok(()) => done += 1,
            Err(e) => crate::output::print_error(&t!("misc.cp.error", e)),
        }
    }
    if done > 0 {
        crate::output::print_success(&t!("misc.cp.wild_done", done, dst_spec));
    }
}

/// `mv <pattern> <dir>`: move every file matched by a `*` source into
/// `dst_spec` (which must be, or become, a directory). Mirrors
/// `copy_wildcard_batch`'s overwrite rules, with a copy+delete fallback.
pub(crate) fn move_wildcard_batch(
    src_pat: &str,
    dst_spec: &str,
    dst: &Path,
    force: bool,
    nav: &Navigator,
) {
    let Some((dst_dir, sources)) =
        expand_wildcard_src(src_pat, dst, nav, "misc.mv.no_match", "misc.mv.dst_dir")
    else {
        return;
    };
    let mut done = 0usize;
    for src in sources {
        let name = match src.file_name() {
            Some(n) => n,
            None => continue,
        };
        let target = dst_dir.join(name);
        if target.exists() && !force {
            crate::output::print_error(&t!("misc.mv.exists", target.display()));
            continue;
        }
        if force && target.exists() {
            if target.is_dir() {
                crate::output::print_error(&t!(
                    "misc.mv.error",
                    "cannot overwrite directory with file"
                ));
                continue;
            }
            if std::fs::remove_file(&target).is_err() {
                crate::output::print_error(&t!("misc.mv.error", "cannot clear existing target"));
                continue;
            }
        }
        match std::fs::rename(&src, &target) {
            Ok(()) => done += 1,
            Err(_) => match move_by_copy(&src, &target) {
                Ok(()) => done += 1,
                Err(e) => crate::output::print_error(&t!("misc.mv.error", e)),
            },
        }
    }
    if done > 0 {
        crate::output::print_success(&t!("misc.mv.wild_done", done, dst_spec));
    }
}

/// Split `mv` args into operands + force flag.
/// Respects double quotes: `mv "my file.txt" sub/ -f`.
pub(crate) fn parse_mv_args(args: &str) -> (Vec<String>, bool) {
    let mut operands: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut force = false;
    let push_token = |tok: String, operands: &mut Vec<String>, force: &mut bool| {
        if tok == "-f" || tok == "--force" {
            *force = true;
        } else if !tok.is_empty() {
            operands.push(tok);
        }
    };
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    push_token(std::mem::take(&mut current), &mut operands, &mut force);
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        push_token(current, &mut operands, &mut force);
    }
    (operands, force)
}

/// Fixture with two savepoints (`tp1` holding files, empty `tp2`) in a
/// temp cwd. Returns the fixture root; the caller seeds `cfg.teleports`.
#[cfg(test)]
pub(crate) fn tp_pair_fixture() -> (crate::utils::test_support::TempCwd, std::path::PathBuf) {
    let cwd = crate::utils::test_support::TempCwd::with_fixture(
        &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
        &["tp1", "tp1/sub", "tp2"],
    );
    let root = std::env::current_dir().unwrap();
    (cwd, root)
}

#[cfg(test)]
pub(crate) fn tp_pair_config(root: &std::path::Path) -> crate::config::Config {
    let mut cfg = crate::config::Config::default();
    cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
    cfg.teleports.insert("tp2".to_string(), root.join("tp2"));
    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mv_args_basic_and_force() {
        let (ops, force) = parse_mv_args("a.txt b.txt");
        assert_eq!(ops, vec!["a.txt", "b.txt"]);
        assert!(!force);
        let (ops, force) = parse_mv_args("a.txt sub/ -f");
        assert_eq!(ops, vec!["a.txt", "sub/"]);
        assert!(force);
        let (ops, force) = parse_mv_args("--force \"my file.txt\" \"new name.txt\"");
        assert_eq!(ops, vec!["my file.txt", "new name.txt"]);
        assert!(force);
    }

    #[test]
    fn parse_mv_args_needs_two_operands() {
        let (ops, _) = parse_mv_args("only-one");
        assert_eq!(ops.len(), 1);
        let (ops, _) = parse_mv_args("");
        assert!(ops.is_empty());
    }

    #[test]
    fn is_deadlock_vault_matches_canonical_path() {
        // Isolate first: this test creates files next to `vault_path()`, which
        // must never be the real user vault.
        let home = crate::utils::test_support::IsolatedConfigHome::new();
        let vault = crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path();
        assert!(
            vault.starts_with(home.path()),
            "test isolation failed: {} is outside {}",
            vault.display(),
            home.path().display()
        );
        // Direct path equality.
        assert!(is_deadlock_vault(&vault));
        std::fs::create_dir_all(vault.parent().unwrap()).unwrap();
        std::fs::write(&vault, "x").ok();
        assert!(is_deadlock_vault(&vault));
        // A different file is not the vault.
        let other = vault.with_file_name("not_the_vault.txt");
        std::fs::write(&other, "x").ok();
        assert!(!is_deadlock_vault(&other));
    }

    #[test]
    fn deadlock_vault_path_is_isolated_in_tests() {
        // Snapshot the real config root first: the isolation helper points
        // XDG_CONFIG_HOME at its tempdir, and on Linux `dirs::config_dir()`
        // follows it, which made the "real user data" check below compare the
        // vault against the isolated directory.
        let real_config = dirs::config_dir();
        let home = crate::utils::test_support::IsolatedConfigHome::new();
        let vault = crate::shell::commands::builtin::deadlock::deadlock::vault::vault_path();
        assert!(
            vault.starts_with(home.path()),
            "vault path escaped the temp config home: {}",
            vault.display()
        );
        // And it must not point into the real user config directory, even
        // though `dirs::config_dir()` ignores NTC_CONFIG_HOME on Windows.
        if let Some(real) = real_config {
            let real_vault_dir = real.join("ntc").join("deadlock");
            assert!(
                !vault.starts_with(&real_vault_dir),
                "vault path resolves to real user data: {}",
                vault.display()
            );
        }
    }
}
