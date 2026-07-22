use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_info};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const MAX_ENTRIES: usize = 6;

#[derive(Debug, Serialize, Deserialize)]
struct ClipEntry {
    content: String,
    ts: u64,
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
    entries.insert(0, ClipEntry { content: content.to_string(), ts });
    entries.truncate(MAX_ENTRIES);
    let _ = save_history(&entries);
}

pub fn cmd_clip(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let subcmd = parts.first().copied().unwrap_or("");

    match subcmd {
        "" | "list" | "ls" => cmd_list(),
        "select" | "sel" => {
            let n = parts.get(1).and_then(|s| s.parse::<usize>().ok());
            match n {
                Some(i) if i >= 1 => cmd_select(i),
                _ => {
                    print_error("Usage: clip select <N>");
                    Ok(false)
                }
            }
        }
        "clear" | "cl" => cmd_clear(),
        _ => {
            print_error(&format!("Unknown clip subcommand: {}", subcmd));
            println!("  clip               Show clipboard history");
            println!("  clip list          Show clipboard history");
            println!("  clip select <N>    Copy entry N back to clipboard");
            println!("  clip clear         Clear clipboard history");
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
        print_info("Clipboard history is empty.");
        return Ok(false);
    }

    println!("  Clipboard History (last {}):", MAX_ENTRIES);
    for (i, entry) in entries.iter().enumerate() {
        let date = chrono::DateTime::from_timestamp(entry.ts as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "?".to_string());
        let one_line = entry.content.lines().next().unwrap_or("");
        let preview = if one_line.len() > 60 {
            &one_line[..60]
        } else {
            one_line
        };
        println!("  {}. [{}] {}", i + 1, date, preview);
    }
    Ok(false)
}

fn cmd_select(n: usize) -> Result<bool> {
    let entries = load_history();
    let idx = n.saturating_sub(1);
    let entry = match entries.get(idx) {
        Some(e) => e,
        None => {
            print_error(&format!("Entry #{} not found (have {} entries).", n, entries.len()));
            return Ok(false);
        }
    };

    #[cfg(not(target_os = "android"))]
    {
        let mut cb = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
        cb.set_text(&entry.content)
            .map_err(|e| anyhow::anyhow!("Failed to set clipboard: {}", e))?;
    }
    #[cfg(target_os = "android")]
    {
        use std::io::Write as _;
        let mut child = std::process::Command::new("termux-clipboard-set")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("termux-clipboard-set not available: {}", e))?;
        if let Some(ref mut stdin) = child.stdin.take() {
            stdin.write_all(entry.content.as_bytes())?;
        }
        child.wait()?;
    }

    let date = chrono::DateTime::from_timestamp(entry.ts as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "?".to_string());
    print_success(&format!("Copied entry #{} ({}) to clipboard.", n, date));
    Ok(false)
}

fn cmd_clear() -> Result<bool> {
    let entries = load_history();
    if entries.is_empty() {
        print_info("Clipboard history is already empty.");
        return Ok(false);
    }
    print!("  Clear {} clipboard entr{}? (y/N): ", entries.len(), if entries.len() == 1 { "y" } else { "ies" });
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
        save_history(&[])?;
        print_success("Clipboard history cleared.");
    } else {
        print_info("Clear cancelled.");
    }
    Ok(false)
}
