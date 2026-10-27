use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const MAX_ENTRIES: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipEntry {
    pub content: String,
    pub ts: u64,
}

fn clip_data_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("clip_history.json")
}

fn load_history() -> Vec<ClipEntry> {
    let path = clip_data_path();
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn load_history_public() -> Vec<ClipEntry> {
    load_history()
}

fn save_history(entries: &[ClipEntry]) -> Result<()> {
    let path = clip_data_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, serde_json::to_string_pretty(entries)?)?;
    Ok(())
}

/// Push a clipboard entry into history (dedup, max 6, most-recent-first).
pub fn push_clip(content: &str) {
    if content.is_empty() {
        return;
    }
    let mut entries = load_history();
    // Skip if same as most recent
    if entries.first().map(|e| e.content.as_str()) == Some(content) {
        return;
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    entries.insert(
        0,
        ClipEntry {
            content: content.to_string(),
            ts,
        },
    );
    entries.truncate(MAX_ENTRIES);
    let _ = save_history(&entries);
}

pub fn cmd_clip(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let subcmd = parts.first().copied().unwrap_or("");

    // `cmd | clip` (no subcommand): copy the piped input to the clipboard.
    if subcmd.is_empty() && crate::shell::stdin::has_any_input() {
        let content = crate::shell::stdin::read_to_string()
            .ok_or_else(|| anyhow::anyhow!("Failed to read stdin"))?;
        let content = crate::output::normalize_newlines(&content);
        if content.trim().is_empty() {
            print_info(&t!("cmd.clipboard.empty"));
            return Ok(false);
        }
        crate::output::copy_to_clipboard(&content, "TXT")?;
        print_success(&t!("cmd.clipboard.copied_piped"));
        return Ok(false);
    }

    match subcmd {
        "" | "list" | "ls" => cmd_list(),
        "select" | "sel" => {
            let n = parts.get(1).and_then(|s| s.parse::<usize>().ok());
            match n {
                Some(i) if i >= 1 => cmd_select(i),
                _ => {
                    print_error(&t!("cmd.clipboard.select_usage"));
                    Ok(false)
                }
            }
        }
        "clear" | "cl" => cmd_clear(),
        _ => {
            print_error(&t!("cmd.clipboard.unknown_sub", subcmd));
            println!("  {}", t!("cmd.clipboard.help_list"));
            println!("  {}", t!("cmd.clipboard.help_list"));
            println!("  {}", t!("cmd.clipboard.help_select"));
            println!("  {}", t!("cmd.clipboard.help_clear"));
            Ok(false)
        }
    }
}

fn cmd_list() -> Result<bool> {
    // Auto-capture the current system clipboard
    #[cfg(not(target_os = "android"))]
    {
        if let Ok(mut cb) = arboard::Clipboard::new() {
            if let Ok(text) = cb.get_text() {
                push_clip(&text);
            }
        }
    }

    let entries = load_history();
    if entries.is_empty() {
        print_info(&t!("cmd.clipboard.empty"));
        return Ok(false);
    }

    println!("  {}", t!("cmd.clipboard.history_header", MAX_ENTRIES));
    for (i, entry) in entries.iter().enumerate() {
        let date = chrono::DateTime::from_timestamp(entry.ts as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "?".to_string());
        let one_line = entry.content.lines().next().unwrap_or("");
        // Char-safe truncation (multibyte content must not panic).
        let preview: String = if one_line.chars().count() > 60 {
            let mut s: String = one_line.chars().take(60).collect();
            s.push_str("…");
            s
        } else {
            one_line.to_string()
        };
        let line_count = entry.content.lines().count();
        let suffix = if line_count > 1 {
            format!(" ({} lines)", line_count)
        } else {
            String::new()
        };
        println!("  {}. [{}] {}{}", i + 1, date, preview, suffix);
    }
    Ok(false)
}

fn cmd_select(n: usize) -> Result<bool> {
    let entries = load_history();
    let idx = n.saturating_sub(1);
    let entry = match entries.get(idx) {
        Some(e) => e,
        None => {
            print_error(&t!("cmd.clipboard.entry_not_found", n, entries.len()));
            return Ok(false);
        }
    };

    #[cfg(not(target_os = "android"))]
    {
        let mut cb = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("{}", t!("cmd.clipboard.access_fail", e)))?;
        cb.set_text(&entry.content)
            .map_err(|e| anyhow::anyhow!("{}", t!("cmd.clipboard.set_fail", e)))?;
    }
    #[cfg(target_os = "android")]
    {
        use std::io::Write;
        let mut child = std::process::Command::new("termux-clipboard-set")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("{}", t!("cmd.clipboard.termux_unavailable", e)))?;
        if let Some(ref mut stdin) = child.stdin.take() {
            stdin.write_all(entry.content.as_bytes())?;
        }
        child.wait()?;
    }

    let date = chrono::DateTime::from_timestamp(entry.ts as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "?".to_string());
    print_success(&t!("cmd.clipboard.copied_entry", n, date));
    Ok(false)
}

fn cmd_clear() -> Result<bool> {
    let entries = load_history();
    if entries.is_empty() {
        print_info(&t!("cmd.clipboard.already_empty"));
        return Ok(false);
    }
    if crate::utils::interactive::prompt_yes_no(
        &t!("cmd.clipboard.clear_prompt", entries.len()),
        true,
    )? {
        save_history(&[])?;
        // Clear the system clipboard too, so the next auto-capture does not
        // resurrect the just-cleared content.
        #[cfg(not(target_os = "android"))]
        if let Err(e) = crate::output::set_clipboard_text("") {
            print_warning(&t!("cmd.clipboard.clear_error", e));
        }
        // Android has no `arboard` clipboard: Termux's `termux-clipboard-set`
        // is the only way in, and an empty payload clears it.
        #[cfg(target_os = "android")]
        if let Err(e) = crate::output::copy_to_clipboard_termux("") {
            print_warning(&t!("cmd.clipboard.clear_error", e));
        }
        print_success(&t!("cmd.clipboard.cleared"));
    } else {
        print_info(&t!("cmd.clipboard.clear_cancelled"));
    }
    Ok(false)
}
