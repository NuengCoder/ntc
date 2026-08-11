use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::{anyhow, bail, Result};
use colored::*;
use std::io::{self, Write};
use std::path::Path;

fn todo_file_path(cwd: &Path) -> std::path::PathBuf {
    cwd.join("future_todo.txt")
}

fn load_tasks(cwd: &Path) -> Result<Vec<String>> {
    let path = todo_file_path(cwd);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(content
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect())
}

fn save_tasks(cwd: &Path, tasks: &[String]) -> Result<()> {
    let path = todo_file_path(cwd);
    let content = tasks.join("\n") + "\n";
    std::fs::write(&path, content)?;
    Ok(())
}

fn display_tasks(tasks: &[String]) {
    if tasks.is_empty() {
        println!("  {}", t!("cmd.todo.no_tasks").dimmed());
        return;
    }
    let width = tasks.len().to_string().len().max(1);
    for (i, task) in tasks.iter().enumerate() {
        println!(
            "  {num:>width$}. {task}",
            num = i + 1,
            width = width,
            task = task
        );
    }
}

fn parse_add_args(args: &str) -> Result<(String, Option<usize>)> {
    let args = args.trim();
    if !args.starts_with('"') {
        bail!("{}", t!("cmd.todo.must_be_quoted"));
    }
    let end_quote = args[1..]
        .find('"')
        .map(|i| i + 1)
        .ok_or_else(|| anyhow!("{}", t!("cmd.todo.missing_quote")))?;
    let name = args[1..end_quote].trim().to_string();
    if name.is_empty() {
        bail!("{}", t!("cmd.todo.empty_name"));
    }
    let rest = args[end_quote + 1..].trim();
    if rest.is_empty() {
        return Ok((name, None));
    }
    if let Some(pos) = rest.find("->") {
        let num_str = rest[pos + 2..].trim();
        if num_str.is_empty() {
            bail!("{}", t!("cmd.todo.missing_position"));
        }
        let n: usize = num_str
            .parse()
            .map_err(|_| anyhow!("{}", t!("cmd.todo.invalid_position", num_str)))?;
        if n == 0 {
            bail!("{}", t!("cmd.todo.position_one_based"));
        }
        return Ok((name, Some(n)));
    }
    bail!("{}", t!("cmd.todo.unexpected_text", rest));
}

pub fn cmd_todo(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();
    let cwd = nav.current_path().to_path_buf();

    match subcmd {
        "" | "info" | "ls" | "list" => cmd_info(subargs, &cwd),
        "add" | "a" => cmd_add(subargs, &cwd),
        "rm" | "remove" => cmd_rm(subargs, &cwd),
        "clear" | "cl" => cmd_clear(subargs, &cwd),
        _ => {
            print_error(&t!("cmd.todo.unknown_sub", subcmd));
            println!();
            println!("{}", t!("cmd.todo.usage_title").cyan().bold());
            println!("  {}", t!("cmd.todo.help_show"));
            println!("  {}", t!("cmd.todo.help_info"));
            println!("  {}", t!("cmd.todo.help_add"));
            println!("  {}", t!("cmd.todo.help_rm"));
            println!("  {}", t!("cmd.todo.help_clear"));
            Ok(false)
        }
    }
}

fn cmd_info(args: &str, cwd: &Path) -> Result<bool> {
    let tasks = load_tasks(cwd)?;
    let args = args.trim();
    if args.is_empty() {
        println!("{}", t!("cmd.todo.list_title").cyan().bold());
        println!();
        display_tasks(&tasks);
    } else {
        let n: usize = args
            .parse()
            .map_err(|_| anyhow!("{}", t!("cmd.todo.info_usage")))?;
        if n == 0 || n > tasks.len() {
            print_error(&t!("cmd.todo.task_not_found", n, tasks.len()));
            return Ok(false);
        }
        let width = tasks.len().to_string().len().max(1);
        println!(
            "{num:>width$}. {task}",
            num = n,
            width = width,
            task = &tasks[n - 1]
        );
    }
    Ok(false)
}

fn cmd_add(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.todo.add_usage"));
        return Ok(false);
    }
    let (name, position) = parse_add_args(args)?;
    let mut tasks = load_tasks(cwd)?;

    match position {
        None => tasks.push(name.clone()),
        Some(pos) => {
            let idx = pos - 1;
            if idx >= tasks.len() {
                tasks.push(name.clone());
            } else {
                tasks.insert(idx, name.clone());
            }
        }
    }

    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_added", name));
    Ok(false)
}

fn cmd_rm(args: &str, cwd: &Path) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("cmd.todo.rm_usage"));
        return Ok(false);
    }
    let n: usize = args
        .parse()
        .map_err(|_| anyhow!("{}", t!("cmd.todo.rm_usage_num")))?;
    if n == 0 {
        print_error(&t!("cmd.todo.one_based"));
        return Ok(false);
    }
    let mut tasks = load_tasks(cwd)?;
    if n > tasks.len() {
        print_error(&t!("cmd.todo.task_not_found", n, tasks.len()));
        return Ok(false);
    }
    let removed = tasks.remove(n - 1);
    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_removed", removed));
    Ok(false)
}

fn cmd_clear(_args: &str, cwd: &Path) -> Result<bool> {
    let tasks = load_tasks(cwd)?;
    if tasks.is_empty() {
        print_info(&t!("cmd.todo.none_to_clear"));
        return Ok(false);
    }

    print_warning(&t!("cmd.todo.clear_prompt", tasks.len()));
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        save_tasks(cwd, &[])?;
        print_success(&t!("cmd.todo.all_cleared"));
    } else {
        print_info(&t!("cmd.todo.cancelled"));
    }
    Ok(false)
}
