use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::commands::builtin::ran::ranfile::parser::{expand_command, parse};
use crate::shell::commands::builtin::ran::ranfile::types::Ranfile;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Run a specific target by name (e.g. "build"), or `"standalone"` if none given.
///
/// With `dry_run` the resolved plan is printed and nothing is executed.
pub(crate) fn run_target(
    name: &str,
    cwd: &Path,
    dry_run: bool,
    exec_fn: &mut dyn FnMut(&str, &Path) -> Result<bool>,
) -> Result<bool> {
    let ranfile = parse(cwd)?;

    // Verify target exists
    if !ranfile.targets.contains_key(name) {
        print_error(&format!("Target '{}' not found in NTCRANFILE.toml", name));
        println!("{}", t!("ranfile.runner.hint", "ran list".green()));
        return Ok(false);
    }

    // Resolve topological order
    let order = match resolve_order(name, &ranfile) {
        Ok(o) => o,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };

    if dry_run {
        print_plan(name, cwd, &ranfile, &order);
        return Ok(true);
    }

    // `[env]` is scoped to the run: every target (and anything it calls)
    // sees these variables, and the caller's environment is restored after.
    let _env_scope = EnvScope::apply(&ranfile.env);

    // Execute each target in order
    for target_name in &order {
        let target = &ranfile.targets[target_name];
        let cmd = expand_command(&target.cmd, &ranfile);

        if !cmd.is_empty() {
            println!();
            print_info(&format!("[{}] Executing: {}", target_name, cmd));
            println!();

            let ok = exec_fn(&cmd, cwd)?;

            if !ok {
                print_error(&format!("[{}] Command failed — aborting", target_name));
                return Ok(false);
            }
        } else if target.deps.is_empty() {
            // Target with no cmd and no deps — nothing to do
            print_warning(&format!("[{}] No command or dependencies", target_name));
        } else {
            print_info(&format!("[{}] Dependencies satisfied", target_name));
        }
    }

    print_success(&format!("Target '{}' completed successfully.", name));
    Ok(true)
}

/// Print what `run_target` would do, in execution order.
fn print_plan(name: &str, cwd: &Path, ranfile: &Ranfile, order: &[String]) {
    println!();
    println!("{}", t!("ran.dry_run.title").yellow().bold());
    println!("{}", t!("ran.dry_run.header", name, cwd.display()).dimmed());
    println!();

    for (idx, target_name) in order.iter().enumerate() {
        let target = &ranfile.targets[target_name];
        println!(
            "[{}/{}] {}",
            (idx + 1).to_string().cyan(),
            order.len().to_string().cyan(),
            target_name.green().bold()
        );
        let cmd = expand_command(&target.cmd, &ranfile);
        if !cmd.is_empty() {
            println!("    {}", cmd.dimmed());
        }
    }

    println!();
    if !ranfile.env.is_empty() {
        let mut env: Vec<String> = ranfile
            .env
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        env.sort();
        println!("{}", t!("ran.dry_run.env", env.join(" ")).dimmed());
    }
    println!(
        "{}",
        t!("ran.dry_run.summary", order.len(), order.join(" -> "))
    );
    println!();
}

/// Temporarily sets `[env]` entries for the duration of a run.
///
/// Commands are executed in this process (system commands inherit its
/// environment), so the variables are set here and the previous values are
/// restored when the guard drops — including after an error or a panic.
struct EnvScope {
    previous: Vec<(String, Option<std::ffi::OsString>)>,
}

impl EnvScope {
    fn apply(env: &HashMap<String, String>) -> Self {
        let mut previous = Vec::with_capacity(env.len());
        for (key, value) in env {
            previous.push((key.clone(), std::env::var_os(key)));
            std::env::set_var(key, value);
        }
        Self { previous }
    }
}

impl Drop for EnvScope {
    fn drop(&mut self) {
        for (key, value) in self.previous.drain(..) {
            match value {
                Some(previous) => std::env::set_var(key, previous),
                None => std::env::remove_var(key),
            }
        }
    }
}

/// Topologically sort the dependency graph for the given target.
fn resolve_order(name: &str, ranfile: &Ranfile) -> Result<Vec<String>, String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut order: Vec<String> = Vec::new();

    fn dfs(
        current: &str,
        ranfile: &Ranfile,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if stack.contains(current) {
            return Err(format!(
                "Circular dependency detected involving target '{}'",
                current
            ));
        }
        if visited.contains(current) {
            return Ok(());
        }
        visited.insert(current.to_string());
        stack.insert(current.to_string());

        if let Some(target) = ranfile.targets.get(current) {
            for dep in &target.deps {
                if !ranfile.targets.contains_key(dep) {
                    return Err(format!(
                        "Target '{}' depends on '{}' but '{}' is not defined",
                        current, dep, dep
                    ));
                }
                dfs(dep, ranfile, visited, stack, order)?;
            }
        }

        stack.remove(current);
        order.push(current.to_string());
        Ok(())
    }

    let mut stack = HashSet::new();
    dfs(name, ranfile, &mut visited, &mut stack, &mut order)?;

    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::commands::builtin::ran::ranfile::types::RanTarget;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_order_simple() {
        let mut targets = std::collections::HashMap::new();
        targets.insert(
            "build".to_string(),
            RanTarget {
                deps: vec!["lint".to_string()],
                cmd: "echo build".to_string(),
            },
        );
        targets.insert(
            "lint".to_string(),
            RanTarget {
                deps: vec![],
                cmd: "echo lint".to_string(),
            },
        );
        let rf = Ranfile {
            vars: HashMap::new(),
            env: HashMap::new(),
            targets,
        };

        let order = resolve_order("build", &rf).unwrap();
        assert_eq!(order, vec!["lint", "build"]);
    }

    const ENV_GRAPH: &str = r#"
[env]
NTC_RAN_SCOPE_TEST = "on"

[targets.standalone]
deps = []
cmd = "echo standalone"

[targets.build]
deps = ["standalone"]
cmd = "echo build"
"#;

    fn ranfile_fixture(body: &str) -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::write(dir.path().join("NTCRANFILE.toml"), body).expect("write ranfile");
        dir
    }

    #[test]
    fn dry_run_reports_the_plan_without_running_anything() {
        let dir = ranfile_fixture(ENV_GRAPH);
        let mut calls = 0;
        let ok = run_target("build", dir.path(), true, &mut |_, _| {
            calls += 1;
            Ok(true)
        })
        .expect("dry run");
        assert!(ok);
        assert_eq!(calls, 0, "dry run executed a command");
    }

    #[test]
    fn env_entries_are_scoped_to_the_run() {
        const KEY: &str = "NTC_RAN_SCOPE_TEST";
        std::env::remove_var(KEY);
        let dir = ranfile_fixture(ENV_GRAPH);

        let mut seen = Vec::new();
        run_target("build", dir.path(), false, &mut |_, _| {
            seen.push(std::env::var(KEY).ok());
            Ok(true)
        })
        .expect("run");

        assert_eq!(seen, vec![Some("on".to_string()), Some("on".to_string())]);
        assert!(
            std::env::var(KEY).is_err(),
            "[env] leaked into the caller's environment"
        );
    }

    #[test]
    fn test_resolve_order_missing_dep() {
        let mut targets = std::collections::HashMap::new();
        targets.insert(
            "build".to_string(),
            RanTarget {
                deps: vec!["nonexistent".to_string()],
                cmd: String::new(),
            },
        );
        let rf = Ranfile {
            vars: HashMap::new(),
            env: HashMap::new(),
            targets,
        };

        assert!(resolve_order("build", &rf).is_err());
    }
}
