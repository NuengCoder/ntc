//! `todo` task model, format parsing, and file storage.

use crate::config::Config;
use crate::shell::commands::support::todo::todo_extension;
use crate::t;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) struct Task {
    #[serde(default)]
    pub(super) done: bool,
    /// 0 = normal, 1 = `!`, 2 = `!!`
    #[serde(default)]
    pub(super) priority: u8,
    pub(super) text: String,
}

/// Top-level wrapper used by JSON/TOML/YAML. TOML requires a table at the
/// document root, so all three share this shape.
#[derive(Debug, Default, Serialize, Deserialize)]
struct TaskFile {
    #[serde(default)]
    tasks: Vec<Task>,
}

pub(super) fn strip_priority_suffix(text: &str) -> (String, u8) {
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

pub(super) fn parse_line(line: &str) -> Option<Task> {
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

pub(super) fn format_line(task: &Task) -> String {
    let mark = if task.done { "x" } else { " " };
    let prio = match task.priority {
        2 => " !!",
        1 => " !",
        _ => "",
    };
    format!("- [{}] {}{}", mark, task.text, prio)
}

pub(super) fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub(super) fn html_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

pub(super) fn attr_value<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
    let idx = attrs.find(name)?;
    let after = attrs[idx + name.len()..].trim_start();
    let after = after.strip_prefix('=')?.trim_start();
    let quote = after.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let after = &after[1..];
    let end = after.find(quote)?;
    Some(&after[..end])
}

pub(super) fn format_html(tasks: &[Task]) -> String {
    let mut out = String::from(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n\
         <title>future_todo</title>\n</head>\n<body>\n<h1>future_todo</h1>\n<ul>\n",
    );
    for task in tasks {
        out.push_str(&format!(
            "  <li data-done=\"{}\" data-priority=\"{}\">{}</li>\n",
            task.done,
            task.priority,
            html_escape(&task.text)
        ));
    }
    out.push_str("</ul>\n</body>\n</html>\n");
    out
}

pub(super) fn parse_html(content: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    let mut rest = content;
    while let Some(start) = rest.find("<li") {
        let after = &rest[start..];
        let Some(open_end) = after.find('>') else {
            break;
        };
        let attrs = &after[..open_end];
        let body = &after[open_end + 1..];
        let Some(close) = body.find("</li>") else {
            break;
        };
        let text = html_unescape(body[..close].trim());
        if !text.is_empty() {
            let done = attr_value(attrs, "data-done")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            let priority = attr_value(attrs, "data-priority")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0u8);
            tasks.push(Task {
                done,
                priority,
                text,
            });
        }
        rest = &body[close + 5..];
    }
    tasks
}

pub(super) fn todo_file_path(cwd: &Path, format: &str) -> PathBuf {
    cwd.join(format!("future_todo.{}", todo_extension(format)))
}

pub(super) fn serialize_tasks(format: &str, tasks: &[Task]) -> Result<String> {
    match format {
        "txt" | "md" => {
            if tasks.is_empty() {
                Ok(String::new())
            } else {
                Ok(tasks.iter().map(format_line).collect::<Vec<_>>().join("\n") + "\n")
            }
        }
        "json" => Ok(serde_json::to_string_pretty(&TaskFile {
            tasks: tasks.to_vec(),
        })? + "\n"),
        "toml" => Ok(toml::to_string_pretty(&TaskFile {
            tasks: tasks.to_vec(),
        })?),
        "yaml" => Ok(serde_yaml::to_string(&TaskFile {
            tasks: tasks.to_vec(),
        })?),
        "html" => Ok(format_html(tasks)),
        _ => bail!("{}", t!("cmd.todo.unknown_format", format)),
    }
}

pub(super) fn parse_tasks(format: &str, content: &str) -> Result<Vec<Task>> {
    match format {
        "txt" | "md" => Ok(content.lines().filter_map(parse_line).collect()),
        "json" => {
            if let Ok(file) = serde_json::from_str::<TaskFile>(content) {
                Ok(file.tasks)
            } else if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(content) {
                Ok(tasks)
            } else {
                bail!("{}", t!("cmd.todo.parse_failed", "JSON"))
            }
        }
        "toml" => Ok(toml::from_str::<TaskFile>(content)?.tasks),
        "yaml" => {
            if let Ok(file) = serde_yaml::from_str::<TaskFile>(content) {
                Ok(file.tasks)
            } else if let Ok(tasks) = serde_yaml::from_str::<Vec<Task>>(content) {
                Ok(tasks)
            } else {
                bail!("{}", t!("cmd.todo.parse_failed", "YAML"))
            }
        }
        "html" => Ok(parse_html(content)),
        _ => bail!("{}", t!("cmd.todo.unknown_format", format)),
    }
}

pub(super) fn load_tasks_with(cwd: &Path, format: &str) -> Result<Vec<Task>> {
    let path = todo_file_path(cwd, format);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = std::fs::read_to_string(&path)?;
    parse_tasks(format, &content)
}

/// Load the configured file. When switching away from `txt`, a legacy
/// `future_todo.txt` is parsed and written in the new format once.
pub(super) fn load_tasks_migrating(cwd: &Path, format: &str) -> Result<Vec<Task>> {
    let path = todo_file_path(cwd, format);
    if path.exists() {
        return load_tasks_with(cwd, format);
    }
    if format != "txt" {
        let legacy = cwd.join("future_todo.txt");
        if legacy.exists() {
            let content = std::fs::read_to_string(&legacy)?;
            let tasks: Vec<Task> = content.lines().filter_map(parse_line).collect();
            let _ = save_tasks_with(cwd, format, &tasks);
            return Ok(tasks);
        }
    }
    Ok(Vec::new())
}

pub(super) fn load_tasks(cwd: &Path) -> Result<Vec<Task>> {
    load_tasks_migrating(cwd, &Config::global_get_todo_format())
}

pub(super) fn save_tasks_with(cwd: &Path, format: &str, tasks: &[Task]) -> Result<()> {
    let path = todo_file_path(cwd, format);
    let content = serialize_tasks(format, tasks)?;
    std::fs::write(&path, content)?;
    Ok(())
}

pub(super) fn save_tasks(cwd: &Path, tasks: &[Task]) -> Result<()> {
    save_tasks_with(cwd, &Config::global_get_todo_format(), tasks)
}
