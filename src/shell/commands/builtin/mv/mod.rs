//! `mv` — move or rename files and directories.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::shell::helpers::resolve_file_operand;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::file_ops::{move_by_copy, move_wildcard_batch, parse_mv_args};

/// `mv <src> <dst> [-f]` — move/rename files or directories.
/// Both operands accept teleport savepoints (`mv @tp1/report.pdf @docs`,
/// `mv @tp1 ./here`); `dst` inside an existing directory keeps the source
/// file name (`mv report.pdf @docs`).
/// A `*` wildcard source expands to the matching files (files only,
/// non-recursive) and requires `dst` to be a directory (created when
/// missing): `mv *.py out/`.
/// Quoted paths with spaces supported; `-f/--force` overwrites the target.
/// Falls back to copy+delete when `rename` fails (e.g. cross-device).
pub(crate) fn cmd_mv(args: &str, nav: &mut Navigator) -> Result<bool> {
    let (operands, force) = parse_mv_args(args);
    if operands.len() < 2 {
        print_error(&t!("misc.mv.missing"));
        println!("{}", t!("misc.mv.usage"));
        return Ok(false);
    }
    // `mv <pattern> <dir>`: expand the wildcard; dst must be a directory.
    if operands[0].contains('*') {
        let dst = match resolve_file_operand(&operands[1], nav) {
            Ok(p) => p,
            Err(_) => return Ok(false),
        };
        move_wildcard_batch(&operands[0], &operands[1], &dst, force, nav);
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
        print_error(&t!("misc.mv.not_exist", operands[0]));
        return Ok(false);
    }
    // `mv file existing_dir/` moves inside the directory (never refuses).
    if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.push(name),
            None => {
                print_error(&t!("misc.mv.error", "invalid source name"));
                return Ok(false);
            }
        }
    }
    if dst.exists() && !force {
        print_error(&t!("misc.mv.exists", operands[1]));
        return Ok(false);
    }
    if force && dst.exists() {
        let cleared = if dst.is_dir() && !src.is_dir() {
            // Never replace a directory with a file.
            print_error(&t!("misc.mv.error", "cannot overwrite directory with file"));
            return Ok(false);
        } else if dst.is_dir() {
            std::fs::remove_dir_all(&dst).is_ok()
        } else {
            std::fs::remove_file(&dst).is_ok()
        };
        if !cleared {
            print_error(&t!("misc.mv.error", "cannot clear existing target"));
            return Ok(false);
        }
    }

    match std::fs::rename(&src, &dst) {
        Ok(()) => print_success(&t!("misc.mv.moved", operands[0], operands[1])),
        Err(_) => match move_by_copy(&src, &dst) {
            Ok(()) => print_success(&t!("misc.mv.moved", operands[0], operands[1])),
            Err(e) => print_error(&t!("misc.mv.error", e)),
        },
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "mv",
    aliases: &["move"],
    help_topic: Some("mv"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_mv),
};

#[cfg(test)]
mod tests {
    use super::super::super::support::file_ops::{tp_pair_config, tp_pair_fixture};
    use super::*;

    #[test]
    fn mv_pattern_moves_matches_into_dir() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("a.py", "a"), ("b.py", "b")],
            &["dest"],
        );
        let mut nav = Navigator::new().unwrap();
        assert!(cmd_mv("*.py dest", &mut nav).is_ok());
        assert!(!std::path::Path::new("a.py").exists());
        assert!(!std::path::Path::new("b.py").exists());
        assert_eq!(std::fs::read("dest/a.py").unwrap(), b"a");
        assert_eq!(std::fs::read("dest/b.py").unwrap(), b"b");
    }

    #[test]
    fn mv_tp_file_into_tp_dir() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            std::fs::write(root.join("tp1").join("move.txt"), "bye").unwrap();
            assert!(cmd_mv("@tp1/move.txt @tp2 -f", &mut nav).is_ok());
            assert!(!root.join("tp1").join("move.txt").exists());
            assert_eq!(
                std::fs::read(root.join("tp2").join("move.txt")).unwrap(),
                b"bye"
            );
        });
    }

    #[test]
    fn mv_tp_dir_onto_new() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            assert!(cmd_mv("@tp1 @tp2/new2", &mut nav).is_ok());
            assert!(!root.join("tp1").exists());
            assert!(root.join("tp2").join("new2").join("file.txt").exists());
        });
    }

    #[test]
    fn mv_tp_file_onto_file() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            std::fs::write(root.join("tp1").join("move.txt"), "bye").unwrap();
            std::fs::write(root.join("tp2").join("move.txt"), "old").unwrap();
            assert!(cmd_mv("@tp1/move.txt @tp2/move.txt -f", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("move.txt")).unwrap(),
                b"bye"
            );
            assert!(!root.join("tp1").join("move.txt").exists());
        });
    }
}
