use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::{anyhow, bail, Result};
use colored::*;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
struct Task {
    done: bool,
    /// 0 = normal, 1 = `!`, 2 = `!!`
    priority: u8,
    text: String,
}

fn strip_priority_suffix(text: &str) -> (String, u8) {
    let trimmed = text.trim_end().to_string();
    if trimmed.ends_with("!!") {
        let body = trimmed[..trimmed.len() - 2].trim_end().to_string();
        if body.is_empty() {
            return (trimmed, 0);
        }
        return (body, 2);
    }
    if trimmed.ends_with('!') {
        let body = trimmed[..trimmed.len() - 1].trim_end().to_string();
        if body.is_empty() {
            return (trimmed, 0);
        }
        return (body, 1);
    }
    (trimmed, 0)
}

fn parse_line(line: &str) -> Option<Task> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix("- [ ]") {
        let (text, priority) = strip_priority_suffix(rest.trim());
        if text.is_empty() {
            return None;
        }
        return Some(Task {
            done: false,
            priority,
            text,
        });
    }
    if let Some(rest) = trimmed
        .strip_prefix("- [x]")
        .or_else(|| trimmed.strip_prefix("- [X]"))
    {
        let (text, priority) = strip_priority_suffix(rest.trim());
        if text.is_empty() {
            return None;
        }
        return Some(Task {
            done: true,
            priority,
            text,
        });
    }
    // Legacy bare line: treat as an open normal-priority task.
    let (text, priority) = strip_priority_suffix(trimmed);
    if text.is_empty() {
        return None;
    }
    Some(Task {
        done: false,
        priority,
        text,
    })
}

fn format_line(task: &Task) -> String {
    let mark = if task.done { "x" } else { " " };
    let prio = match task.priority {
        2 => " !!",
        1 => " !",
        _ => "",
    };
    format!("- [{}] {}{}", mark, task.text, prio)
}

fn todo_file_path(cwd: &Path) -> std::path::PathBuf {
    cwd.join("future_todo.txt")
}

fn load_tasks(cwd: &Path) -> Result<Vec<Task>> {
    let path = todo_file_path(cwd);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(content.lines().filter_map(parse_line).collect())
}

fn save_tasks(cwd: &Path, tasks: &[Task]) -> Result<()> {
    let path = todo_file_path(cwd);
    if tasks.is_empty() {
        std::fs::write(&path, "")?;
        return Ok(());
    }
    let content = tasks.iter().map(format_line).collect::<Vec<_>>().join("\n") + "\n";
    std::fs::write(&path, content)?;
    Ok(())
}

fn priority_tag(priority: u8) -> String {
    match priority {
        2 => " !!".red().bold().to_string(),
        1 => " !".yellow().bold().to_string(),
        _ => String::new(),
    }
}

fn display_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("  {}", t!("cmd.todo.no_tasks").dimmed());
        return;
    }
    let width = tasks.len().to_string().len().max(1);
    for (i, task) in tasks.iter().enumerate() {
        let checkbox = if task.done { "[x]" } else { "[ ]" };
        let text = if task.done {
            task.text.dimmed().to_string()
        } else {
            task.text.clone()
        };
        println!(
            "  {num:>width$}. {cb} {task}{prio}",
            num = i + 1,
            width = width,
            cb = checkbox,
            task = text,
            prio = priority_tag(task.priority),
        );
    }
}

fn display_single(tasks: &[Task], n: usize) {
    let width = tasks.len().to_string().len().max(1);
    let task = &tasks[n - 1];
    let checkbox = if task.done { "[x]" } else { "[ ]" };
    println!(
        "{num:>width$}. {cb} {task}{prio}",
        num = n,
        width = width,
        cb = checkbox,
        task = task.text,
        prio = priority_tag(task.priority),
    );
}

fn parse_add_args(args: &str) -> Result<(Task, Option<usize>)> {
    let args = args.trim();
    if !args.starts_with('"') {
        bail!("{}", t!("cmd.todo.must_be_quoted"));
    }
    let end_quote = args[1..]
        .find('"')
        .map(|i| i + 1)
        .ok_or_else(|| anyhow!("{}", t!("cmd.todo.missing_quote")))?;
    let raw_name = args[1..end_quote].trim().to_string();
    if raw_name.is_empty() {
        bail!("{}", t!("cmd.todo.empty_name"));
    }
    let (text, priority) = strip_priority_suffix(&raw_name);
    if text.is_empty() {
        bail!("{}", t!("cmd.todo.empty_name"));
    }
    let task = Task {
        done: false,
        priority,
        text,
    };
    let rest = args[end_quote + 1..].trim();
    if rest.is_empty() {
        return Ok((task, None));
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
        return Ok((task, Some(n)));
    }
    bail!("{}", t!("cmd.todo.unexpected_text", rest));
}

fn parse_index(args: &str, usage_key: &str) -> Result<usize> {
    let args = args.trim();
    if args.is_empty() {
        bail!("{}", t!(usage_key));
    }
    let n: usize = args.parse().map_err(|_| anyhow!("{}", t!(usage_key)))?;
    if n == 0 {
        bail!("{}", t!("cmd.todo.one_based"));
    }
    Ok(n)
}

fn get_task_mut(tasks: &mut [Task], n: usize) -> Result<&mut Task> {
    if n == 0 || n > tasks.len() {
        bail!("{}", t!("cmd.todo.task_not_found", n, tasks.len()));
    }
    Ok(&mut tasks[n - 1])
}

pub fn cmd_todo(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();
    let cwd = nav.current_path().to_path_buf();

    match subcmd {
        "" | "info" | "ls" | "list" => cmd_info(subargs, &cwd),
        "add" | "a" => cmd_add(subargs, &cwd),
        "done" | "do" | "check" => cmd_done(subargs, &cwd),
        "undo" | "un" | "uncheck" | "reopen" => cmd_undo(subargs, &cwd),
        "edit" | "ed" | "rnm" | "rename" => cmd_edit(subargs, &cwd),
        "rm" | "remove" => cmd_rm(subargs, &cwd),
        "clear" | "cl" => cmd_clear(subargs, &cwd),
        _ => {
            print_error(&t!("cmd.todo.unknown_sub", subcmd));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

fn print_usage() {
    println!("{}", t!("cmd.todo.usage_title").cyan().bold());
    println!("  {}", t!("cmd.todo.help_show"));
    println!("  {}", t!("cmd.todo.help_info"));
    println!("  {}", t!("cmd.todo.help_add"));
    println!("  {}", t!("cmd.todo.help_done"));
    println!("  {}", t!("cmd.todo.help_undo"));
    println!("  {}", t!("cmd.todo.help_edit"));
    println!("  {}", t!("cmd.todo.help_rm"));
    println!("  {}", t!("cmd.todo.help_clear"));
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
        display_single(&tasks, n);
    }
    Ok(false)
}

fn cmd_add(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.todo.add_usage"));
        return Ok(false);
    }
    let (task, position) = parse_add_args(args)?;
    let label = task.text.clone();
    let mut tasks = load_tasks(cwd)?;

    match position {
        None => tasks.push(task),
        Some(pos) => {
            let idx = pos - 1;
            if idx >= tasks.len() {
                tasks.push(task);
            } else {
                tasks.insert(idx, task);
            }
        }
    }

    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_added", label));
    Ok(false)
}

fn cmd_done(args: &str, cwd: &Path) -> Result<bool> {
    let n = match parse_index(args, "cmd.todo.done_usage") {
        Ok(n) => n,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    let mut tasks = load_tasks(cwd)?;
    let task = match get_task_mut(&mut tasks, n) {
        Ok(t) => t,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    if task.done {
        print_info(&t!("cmd.todo.already_done", n));
        return Ok(false);
    }
    task.done = true;
    let label = task.text.clone();
    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_done", label));
    Ok(false)
}

fn cmd_undo(args: &str, cwd: &Path) -> Result<bool> {
    let n = match parse_index(args, "cmd.todo.undo_usage") {
        Ok(n) => n,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    let mut tasks = load_tasks(cwd)?;
    let task = match get_task_mut(&mut tasks, n) {
        Ok(t) => t,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    if !task.done {
        print_info(&t!("cmd.todo.already_open", n));
        return Ok(false);
    }
    task.done = false;
    let label = task.text.clone();
    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_undone", label));
    Ok(false)
}

fn cmd_edit(args: &str, cwd: &Path) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("cmd.todo.edit_usage"));
        return Ok(false);
    }
    let (num_part, rest) = match args.find(' ') {
        Some(i) => (args[..i].trim(), args[i + 1..].trim()),
        None => {
            print_error(&t!("cmd.todo.edit_usage"));
            return Ok(false);
        }
    };
    let n: usize = num_part
        .parse()
        .map_err(|_| anyhow!("{}", t!("cmd.todo.edit_usage")))?;
    if n == 0 {
        print_error(&t!("cmd.todo.one_based"));
        return Ok(false);
    }
    if !rest.starts_with('"') {
        print_error(&t!("cmd.todo.must_be_quoted"));
        return Ok(false);
    }
    let end_quote = match rest[1..].find('"').map(|i| i + 1) {
        Some(i) => i,
        None => {
            print_error(&t!("cmd.todo.missing_quote"));
            return Ok(false);
        }
    };
    let raw_name = rest[1..end_quote].trim();
    if raw_name.is_empty() {
        print_error(&t!("cmd.todo.empty_name"));
        return Ok(false);
    }
    let trailing = rest[end_quote + 1..].trim();
    if !trailing.is_empty() {
        print_error(&t!("cmd.todo.unexpected_text", trailing));
        return Ok(false);
    }
    let (text, priority) = strip_priority_suffix(raw_name);
    if text.is_empty() {
        print_error(&t!("cmd.todo.empty_name"));
        return Ok(false);
    }
    let mut tasks = load_tasks(cwd)?;
    let task = match get_task_mut(&mut tasks, n) {
        Ok(t) => t,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    task.text = text;
    task.priority = priority;
    let label = task.text.clone();
    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_edited", label));
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
    print_success(&t!("cmd.todo.task_removed", removed.text));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_legacy_bare_line() {
        let t = parse_line("Fix bug").unwrap();
        assert!(!t.done);
        assert_eq!(t.priority, 0);
        assert_eq!(t.text, "Fix bug");
    }

    #[test]
    fn parse_checkbox_and_priority() {
        assert_eq!(
            parse_line("- [ ] Write docs !!").unwrap(),
            Task {
                done: false,
                priority: 2,
                text: "Write docs".into()
            }
        );
        assert_eq!(
            parse_line("- [x] Old !").unwrap(),
            Task {
                done: true,
                priority: 1,
                text: "Old".into()
            }
        );
        assert_eq!(parse_line("- [X] Caps").unwrap().done, true);
    }

    #[test]
    fn format_roundtrip() {
        let task = Task {
            done: false,
            priority: 2,
            text: "Ship".into(),
        };
        assert_eq!(format_line(&task), "- [ ] Ship !!");
        assert_eq!(parse_line(&format_line(&task)).unwrap(), task);
    }

    #[test]
    fn add_parses_priority_and_position() {
        let (task, pos) = parse_add_args("\"Fix bug !!\" -> 2").unwrap();
        assert_eq!(task.text, "Fix bug");
        assert_eq!(task.priority, 2);
        assert_eq!(pos, Some(2));
        let (task, pos) = parse_add_args("\"Plain\"").unwrap();
        assert_eq!(task.priority, 0);
        assert_eq!(pos, None);
    }

    #[test]
    fn done_undo_toggle() {
        let dir = tempfile::tempdir().unwrap();
        save_tasks(
            dir.path(),
            &[Task {
                done: false,
                priority: 0,
                text: "A".into(),
            }],
        )
        .unwrap();
        cmd_done("1", dir.path()).unwrap();
        let tasks = load_tasks(dir.path()).unwrap();
        assert!(tasks[0].done);
        cmd_undo("1", dir.path()).unwrap();
        let tasks = load_tasks(dir.path()).unwrap();
        assert!(!tasks[0].done);
    }

    #[test]
    fn edit_renames_and_reprioritizes() {
        let dir = tempfile::tempdir().unwrap();
        save_tasks(
            dir.path(),
            &[Task {
                done: false,
                priority: 0,
                text: "Old".into(),
            }],
        )
        .unwrap();
        cmd_edit("1 \"New !!\"", dir.path()).unwrap();
        let tasks = load_tasks(dir.path()).unwrap();
        assert_eq!(tasks[0].text, "New");
        assert_eq!(tasks[0].priority, 2);
    }
}
