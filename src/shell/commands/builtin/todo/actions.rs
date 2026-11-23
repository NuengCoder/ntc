//! `todo` subcommand parsing, rendering, and task mutations.

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{anyhow, bail, Result};
use std::io::{self, Write};
use std::path::Path;

use super::model::{load_tasks, save_tasks, strip_priority_suffix, Task};

pub(super) fn priority_tag(priority: u8) -> String {
    match priority {
        2 => " !!".red().bold().to_string(),
        1 => " !".yellow().bold().to_string(),
        _ => String::new(),
    }
}

pub(super) fn display_tasks(tasks: &[Task]) {
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

pub(super) fn display_single(tasks: &[Task], n: usize) {
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

pub(super) fn parse_add_args(args: &str) -> Result<(Task, Option<usize>)> {
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

pub(super) fn parse_index(args: &str, usage_key: &str) -> Result<usize> {
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

pub(super) fn get_task_mut(tasks: &mut [Task], n: usize) -> Result<&mut Task> {
    if n == 0 || n > tasks.len() {
        bail!("{}", t!("cmd.todo.task_not_found", n, tasks.len()));
    }
    Ok(&mut tasks[n - 1])
}

/// Parse `<select-n> <target-n>` for `todo mv` / `todo rp`. Both are 1-based;
/// a missing, non-numeric or zero index reports `usage_key`.
pub(super) fn parse_pair(args: &str, usage_key: &str) -> Result<(usize, usize)> {
    let args = args.trim();
    if args.is_empty() {
        bail!("{}", t!(usage_key));
    }
    let mut nums = args.split_whitespace();
    let mut next = |usage: &str| -> Result<usize> {
        let raw = nums.next().ok_or_else(|| anyhow!("{}", t!(usage)))?;
        let n: usize = raw.parse().map_err(|_| anyhow!("{}", t!(usage_key)))?;
        if n == 0 {
            bail!("{}", t!("cmd.todo.one_based"));
        }
        Ok(n)
    };
    let from = next(usage_key)?;
    let to = next(usage_key)?;
    if nums.next().is_some() {
        // The body validates arity so `usage_key` (a real message) is shown,
        // never a raw key.
        bail!("{}", t!(usage_key));
    }
    Ok((from, to))
}

/// Move the task at `from` to position `to` (1-based), shifting the rest:
/// `[a,b,c,d,e]` + `mv 5 1` becomes `[e,a,b,c,d]`.
pub(super) fn move_task(tasks: &mut Vec<Task>, from: usize, to: usize) -> Result<()> {
    if from == 0 || from > tasks.len() || to == 0 || to > tasks.len() {
        bail!(
            "{}",
            t!("cmd.todo.task_not_found", from.max(to), tasks.len())
        );
    }
    if from == to {
        return Ok(());
    }
    let task = tasks.remove(from - 1);
    tasks.insert(to - 1, task);
    Ok(())
}

/// Swap the tasks at `a` and `b` (1-based): `[a,b,c,d,e]` + `rp 5 1` becomes
/// `[e,b,c,d,a]`.
pub(super) fn swap_task(tasks: &mut [Task], a: usize, b: usize) -> Result<()> {
    if a == 0 || a > tasks.len() || b == 0 || b > tasks.len() {
        bail!("{}", t!("cmd.todo.task_not_found", a.max(b), tasks.len()));
    }
    if a != b {
        tasks.swap(a - 1, b - 1);
    }
    Ok(())
}

pub(super) fn cmd_info(args: &str, cwd: &Path) -> Result<bool> {
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

pub(super) fn cmd_add(args: &str, cwd: &Path) -> Result<bool> {
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

pub(super) fn cmd_done(args: &str, cwd: &Path) -> Result<bool> {
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

pub(super) fn cmd_undo(args: &str, cwd: &Path) -> Result<bool> {
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

pub(super) fn cmd_edit(args: &str, cwd: &Path) -> Result<bool> {
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

pub(super) fn cmd_rm(args: &str, cwd: &Path) -> Result<bool> {
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

pub(super) fn cmd_move(args: &str, cwd: &Path) -> Result<bool> {
    let (from, to) = match parse_pair(args, "cmd.todo.move_usage") {
        Ok(v) => v,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    let mut tasks = load_tasks(cwd)?;
    if from == to {
        print_info(&t!("cmd.todo.same_position", from));
        return Ok(false);
    }
    if let Err(e) = move_task(&mut tasks, from, to) {
        print_error(&e.to_string());
        return Ok(false);
    }
    let label = tasks[to - 1].text.clone();
    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_moved", from, to, label));
    Ok(false)
}

pub(super) fn cmd_replace(args: &str, cwd: &Path) -> Result<bool> {
    let (a, b) = match parse_pair(args, "cmd.todo.replace_usage") {
        Ok(v) => v,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };
    let mut tasks = load_tasks(cwd)?;
    if a == b {
        print_info(&t!("cmd.todo.same_position", a));
        return Ok(false);
    }
    if let Err(e) = swap_task(&mut tasks, a, b) {
        print_error(&e.to_string());
        return Ok(false);
    }
    let first = tasks[a - 1].text.clone();
    let second = tasks[b - 1].text.clone();
    save_tasks(cwd, &tasks)?;
    print_success(&t!("cmd.todo.task_replaced", a, b, first, second));
    Ok(false)
}

pub(super) fn cmd_clear(_args: &str, cwd: &Path) -> Result<bool> {
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
