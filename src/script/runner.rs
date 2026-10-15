use crate::output::{print_error, print_info, print_success, print_warning};
use crate::script::interp;
use crate::script::parser::{self, Stmt};
use crate::t;

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Execute a `.ntc` script.
///
/// `exec_fn` runs a single (already interpolated) command line. It receives the
/// expanded command and whether it is a forced system command (`@` prefix).
/// Return `Ok(true)` on success, `Ok(false)` for a failed system command, or
/// `Err` for a hard failure.
pub(crate) fn run_script(
    path: &Path,
    args: &[String],
    continue_on_err: bool,
    exec_fn: &mut dyn FnMut(&str, bool) -> Result<bool>,
) -> Result<()> {
    let display = path.display().to_string();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            print_error(&t!("scr.not_found", display, e));
            return Ok(());
        }
    };
    let content = content.trim_start_matches('\u{feff}');
    let stmts = parser::parse(content);

    if stmts.is_empty() {
        print_warning(&t!("scr.no_commands", display));
        return Ok(());
    }

    let mut vars: HashMap<String, String> = HashMap::new();
    let mut executed = 0usize;
    let mut errors = 0usize;

    for (idx, stmt) in stmts.iter().enumerate() {
        let line_no = idx + 1;
        match stmt {
            Stmt::Set { name, value } => {
                let value = interp::interpolate(value, &vars, args);
                vars.insert(name.clone(), value);
            }
            Stmt::Cmd(cmd) | Stmt::ForceCmd(cmd) => {
                let expanded = interp::interpolate(cmd, &vars, args);
                let trimmed = expanded.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let force = matches!(stmt, Stmt::ForceCmd(_));

                println!();
                print_info(&t!("scr.executing", line_no, trimmed));
                executed += 1;

                match exec_fn(trimmed, force) {
                    Ok(true) => {}
                    Ok(false) => {
                        errors += 1;
                        if continue_on_err {
                            print_warning(&t!("scr.continue_warn", line_no));
                        } else {
                            print_error(&t!(
                                "scr.error_at",
                                display,
                                line_no,
                                t!("scr.cmd_failed")
                            ));
                            print_error(&t!("scr.failed_abort", display, line_no, errors));
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        errors += 1;
                        if continue_on_err {
                            print_warning(&t!("scr.continue_warn", line_no));
                        } else {
                            print_error(&t!("scr.error_at", display, line_no, e));
                            print_error(&t!("scr.failed_abort", display, line_no, errors));
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    if errors > 0 {
        print_warning(&t!("scr.done_errors", display, errors));
    } else {
        print_success(&t!("scr.completed", display, executed));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(
        content: &str,
        args: &[String],
        continue_on_err: bool,
    ) -> (Vec<(String, bool)>, Vec<usize>) {
        let mut seen: Vec<(String, bool)> = Vec::new();
        let mut failed: Vec<usize> = Vec::new();
        {
            let mut exec = |cmd: &str, force: bool| -> Result<bool> {
                seen.push((cmd.to_string(), force));
                if cmd.contains("FAIL") {
                    failed.push(seen.len());
                    Ok(false)
                } else {
                    Ok(true)
                }
            };
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("ntc_script_test.ntc");
            std::fs::write(&path, content).unwrap();
            let _ = run_script(&path, args, continue_on_err, &mut exec);
        }
        (seen, failed)
    }

    #[test]
    fn test_executes_in_order() {
        let (seen, _) = run("cd src\nsay hi\necho bye", &[], false);
        let cmds: Vec<&str> = seen.iter().map(|(c, _)| c.as_str()).collect();
        assert_eq!(cmds, vec!["cd src", "say hi", "echo bye"]);
    }

    #[test]
    fn test_variable_interpolation() {
        let (seen, _) = run("set dir = src\ncd $dir", &[], false);
        assert_eq!(seen[0].0, "cd src");
    }

    #[test]
    fn test_positional_args() {
        let (seen, _) = run("say $1 $@", &["a".into(), "b".into()], false);
        assert_eq!(seen[0].0, "say a a b");
    }

    #[test]
    fn test_force_flag() {
        let (seen, _) = run("@ echo forced", &[], false);
        assert!(seen[0].1);
    }

    #[test]
    fn test_non_force_flag() {
        let (seen, _) = run("echo normal", &[], false);
        assert!(!seen[0].1);
    }

    #[test]
    fn test_stops_on_error_by_default() {
        let (seen, _) = run("say one\nFAIL here\nsay three", &[], false);
        let cmds: Vec<&str> = seen.iter().map(|(c, _)| c.as_str()).collect();
        assert_eq!(cmds, vec!["say one", "FAIL here"]);
    }

    #[test]
    fn test_continues_with_flag() {
        let (seen, _) = run("say one\nFAIL here\nsay three", &[], true);
        let cmds: Vec<&str> = seen.iter().map(|(c, _)| c.as_str()).collect();
        assert_eq!(cmds, vec!["say one", "FAIL here", "say three"]);
    }

    #[test]
    fn test_skip_empty_after_interpolation() {
        let (seen, _) = run("$1\nsay ok", &[], false);
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, "say ok");
    }
}
