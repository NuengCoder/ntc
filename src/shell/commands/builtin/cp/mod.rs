//! `cp` — copy files and directories.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_ops::{copy_path, copy_wildcard_batch, parse_mv_args, same_file};

/// `cp <src> <dst> [-f]` — copy files or directories.
/// Both operands accept teleport savepoints (`cp @tp1/report.pdf @docs`,
/// `cp @tp1 ./here`); `dst` inside an existing directory keeps the source
/// file name (`cp report.pdf @docs`).
/// A `*` wildcard source expands to the matching files (files only,
/// non-recursive) and requires `dst` to be a directory (created when
/// missing): `cp *.py out/`.
/// Quoted paths with spaces supported; `-f/--force` overwrites the target.
pub(crate) fn cmd_cp(args: &str, nav: &mut Navigator) -> Result<bool> {
    let (operands, force) = parse_mv_args(args);
    if operands.len() < 2 {
        print_error(&t!("misc.cp.missing"));
        println!("{}", t!("misc.cp.usage"));
        return Ok(false);
    }
    // `cp <pattern> <dir>`: expand the wildcard; dst must be a directory.
    if operands[0].contains('*') {
        let dst = match resolve_file_operand(&operands[1], nav) {
            Ok(p) => p,
            Err(_) => return Ok(false),
        };
        copy_wildcard_batch(&operands[0], &operands[1], &dst, force, nav);
        return Ok(false);
    }
    let src = match resolve_file_operand(&operands[0], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let mut dst = match resolve_file_operand(&operands[1], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    if !src.exists() {
        print_error(&t!("misc.cp.not_exist", operands[0]));
        return Ok(false);
    }
    // `cp file existing_dir/` copies inside the directory (never refuses).
    if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.push(name),
            None => {
                print_error(&t!("misc.cp.error", "invalid source name"));
                return Ok(false);
            }
        }
    }
    if same_file(&src, &dst) {
        print_error(&t!("misc.cp.same", operands[0]));
        return Ok(false);
    }
    if dst.exists() && !force {
        print_error(&t!("misc.cp.exists", operands[1]));
        return Ok(false);
    }
    if force && dst.exists() {
        let cleared = if dst.is_dir() && !src.is_dir() {
            // Never replace a directory with a file.
            print_error(&t!("misc.cp.error", "cannot overwrite directory with file"));
            return Ok(false);
        } else if dst.is_dir() {
            std::fs::remove_dir_all(&dst).is_ok()
        } else {
            std::fs::remove_file(&dst).is_ok()
        };
        if !cleared {
            print_error(&t!("misc.cp.error", "cannot clear existing target"));
            return Ok(false);
        }
    }

    match copy_path(&src, &dst) {
        Ok(()) => print_success(&t!("misc.cp.copied", operands[0], operands[1])),
        Err(e) => print_error(&t!("misc.cp.error", e)),
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "cp",
    aliases: &[],
    help_topic: Some("cp"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_cp),
};

#[cfg(test)]
mod tests {
    use super::super::super::support::file_ops::{tp_pair_config, tp_pair_fixture};
    use super::*;

    #[test]
    fn cp_pattern_copies_matches_into_dirs() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[
                ("a.py", "a"),
                ("b.py", "b"),
                ("c.txt", "c"),
                ("sub/d.py", "d"),
            ],
            &["out"],
        );
        let mut nav = Navigator::new().unwrap();
        // New dst directory is auto-created; only matching files move in.
        assert!(cmd_cp("*.py newdir", &mut nav).is_ok());
        assert_eq!(std::fs::read("newdir/a.py").unwrap(), b"a");
        assert_eq!(std::fs::read("newdir/b.py").unwrap(), b"b");
        assert!(!std::path::Path::new("newdir/c.txt").exists());
        // `<dir>/*.py` scopes into a subdirectory.
        assert!(cmd_cp("sub/*.py out", &mut nav).is_ok());
        assert_eq!(std::fs::read("out/d.py").unwrap(), b"d");
        // Sources stay in place for `cp`.
        assert!(std::path::Path::new("a.py").exists());
        assert!(std::path::Path::new("sub/d.py").exists());
    }

    #[test]
    fn cp_pattern_dst_must_be_a_dir() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("a.py", "a"), ("dst.txt", "x")],
            &[],
        );
        let mut nav = Navigator::new().unwrap();
        // A plain file destination is refused; nothing is copied.
        assert!(cmd_cp("*.py dst.txt", &mut nav).is_ok());
        assert_eq!(std::fs::read("dst.txt").unwrap(), b"x");
        assert!(std::path::Path::new("a.py").exists());
    }

    #[test]
    fn cp_pattern_no_match_creates_nothing() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("a.py", "a")], &[]);
        let mut nav = Navigator::new().unwrap();
        assert!(cmd_cp("*.zzz out", &mut nav).is_ok());
        // No match → no destination directory is created.
        assert!(!std::path::Path::new("out").exists());
    }

    #[test]
    fn cp_pattern_force_overwrites_targets() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("a.py", "new"), ("out/a.py", "old")],
            &["out"],
        );
        let mut nav = Navigator::new().unwrap();
        // Without -f the existing target is skipped.
        assert!(cmd_cp("*.py out", &mut nav).is_ok());
        assert_eq!(std::fs::read("out/a.py").unwrap(), b"old");
        // With -f it is overwritten.
        assert!(cmd_cp("*.py out -f", &mut nav).is_ok());
        assert_eq!(std::fs::read("out/a.py").unwrap(), b"new");
    }

    #[test]
    fn cp_tp_file_into_tp_dir() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            assert!(cmd_cp("@tp1/file.txt @tp2 -f", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("file.txt")).unwrap(),
                b"hello"
            );
            assert!(root.join("tp1").join("file.txt").exists());
        });
    }

    #[test]
    fn cp_tp_dir_onto_new() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            assert!(cmd_cp("@tp1 @tp2/new", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("new").join("file.txt")).unwrap(),
                b"hello"
            );
            assert_eq!(
                std::fs::read(root.join("tp2").join("new").join("sub").join("deep.txt")).unwrap(),
                b"deep"
            );
            assert!(root.join("tp1").exists());
        });
    }

    #[test]
    fn cp_tp_file_onto_file() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            std::fs::write(root.join("tp2").join("file.txt"), "old").unwrap();
            assert!(cmd_cp("@tp1/file.txt @tp2/file.txt -f", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("file.txt")).unwrap(),
                b"hello"
            );
        });
    }

    #[test]
    fn cp_tp_dir_onto_relative() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            assert!(cmd_cp("@tp1 test", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("test").join("file.txt")).unwrap(),
                b"hello"
            );
            assert!(cmd_cp("@tp1 ./test2", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("test2").join("sub").join("deep.txt")).unwrap(),
                b"deep"
            );
            assert!(root.join("tp1").exists());
        });
    }
}
