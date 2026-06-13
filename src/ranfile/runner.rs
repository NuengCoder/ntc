use crate::output::{print_error, print_info, print_success, print_warning};
use crate::ranfile::parser::parse;
use crate::ranfile::types::Ranfile;
use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use std::path::Path;

/// Run a specific target by name (e.g. "build"), or `"standalone"` if none given.
pub(crate) fn run_target(name: &str, cwd: &Path, exec_fn: &mut dyn FnMut(&str, &Path) -> Result<bool>) -> Result<bool> {
    let ranfile = parse(cwd)?;

    // Verify target exists
    if !ranfile.targets.contains_key(name) {
        print_error(&format!("Target '{}' not found in NTCRANFILE.toml", name));
        println!("Run {} to see available targets.", "ran list".green());
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

    // Execute each target in order
    for target_name in &order {
        let target = &ranfile.targets[target_name];
        let cmd = expand_vars(&target.cmd, &ranfile.vars);

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
            return Err(format!("Circular dependency detected involving target '{}'", current));
        }
        if visited.contains(current) {
            return Ok(());
        }
        visited.insert(current.to_string());
        stack.insert(current.to_string());

        if let Some(target) = ranfile.targets.get(current) {
            for dep in &target.deps {
                if !ranfile.targets.contains_key(dep) {
                    return Err(format!("Target '{}' depends on '{}' but '{}' is not defined", current, dep, dep));
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

/// Expand $(VAR_NAME) references in a command string.
fn expand_vars(cmd: &str, vars: &std::collections::HashMap<String, String>) -> String {
    let mut result = String::with_capacity(cmd.len());
    let mut chars = cmd.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'(') {
            chars.next(); // consume '('
            let mut var_name = String::new();
            let mut found_close = false;
            for c in chars.by_ref() {
                if c == ')' {
                    found_close = true;
                    break;
                }
                var_name.push(c);
            }
            if found_close {
                let expanded = vars.get(&var_name).map(|s| s.as_str()).unwrap_or("");
                result.push_str(expanded);
            } else {
                // Unclosed $( — preserve the literal text
                result.push_str("$(");
                result.push_str(&var_name);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ranfile::types::RanTarget;
    use std::collections::HashMap;

    #[test]
    fn test_expand_vars() {
        let mut vars = HashMap::new();
        vars.insert("CC".to_string(), "gcc".to_string());
        vars.insert("FLAGS".to_string(), "-O2".to_string());

        assert_eq!(expand_vars("$(CC) main.c", &vars), "gcc main.c");
        assert_eq!(expand_vars("$(CC) $(FLAGS) main.c", &vars), "gcc -O2 main.c");
        assert_eq!(expand_vars("echo nothing", &vars), "echo nothing");
        assert_eq!(expand_vars("$(UNKNOWN)", &vars), "");
    }

    #[test]
    fn test_resolve_order_simple() {
        let mut targets = std::collections::HashMap::new();
        targets.insert("build".to_string(), RanTarget {
            deps: vec!["lint".to_string()],
            cmd: "echo build".to_string(),
        });
        targets.insert("lint".to_string(), RanTarget {
            deps: vec![],
            cmd: "echo lint".to_string(),
        });
        let rf = Ranfile { vars: HashMap::new(), targets };

        let order = resolve_order("build", &rf).unwrap();
        assert_eq!(order, vec!["lint", "build"]);
    }

    #[test]
    fn test_resolve_order_missing_dep() {
        let mut targets = std::collections::HashMap::new();
        targets.insert("build".to_string(), RanTarget {
            deps: vec!["nonexistent".to_string()],
            cmd: String::new(),
        });
        let rf = Ranfile { vars: HashMap::new(), targets };

        assert!(resolve_order("build", &rf).is_err());
    }
}
