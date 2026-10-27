#[cfg(not(target_os = "android"))]
use anyhow::Context;
use anyhow::Result;
#[cfg(target_os = "android")]
use std::io::Write;

#[cfg(not(target_os = "android"))]
use arboard::Clipboard;

/// Check if running in Termux environment
#[cfg(target_os = "android")]
fn is_termux() -> bool {
    std::env::var("TERMUX_VERSION").is_ok()
        || std::path::Path::new("/data/data/com.termux/files/usr/bin/termux-clipboard-set").exists()
}

/// Copy to clipboard using Termux API (Android only)
#[cfg(target_os = "android")]
pub fn copy_to_clipboard_termux(content: &str) -> Result<bool> {
    use std::process::Command;

    if !is_termux() {
        return Ok(false);
    }

    // Try termux-clipboard-set first
    let status = Command::new("termux-clipboard-set").arg(content).status();

    if let Ok(status) = status {
        if status.success() {
            return Ok(true);
        }
    }

    // Fallback: write to temp file and use termux-clipboard-set with stdin
    let temp_file = std::env::temp_dir().join(format!("ntc_clipboard_{}.txt", std::process::id()));
    if let Ok(mut file) = std::fs::File::create(&temp_file) {
        let _ = file.write_all(content.as_bytes());
        let _ = file.sync_all();

        let status = Command::new("termux-clipboard-set")
            .arg(temp_file.to_str().unwrap_or(""))
            .status();

        let _ = std::fs::remove_file(temp_file);

        if let Ok(status) = status {
            if status.success() {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

/// Normalize CRLF / lone CR line endings to LF for history and storage.
pub fn normalize_newlines(content: &str) -> String {
    content.replace("\r\n", "\n").replace('\r', "\n")
}

/// Windows apps expect CRLF; every other platform takes LF.
#[cfg(all(not(target_os = "android"), windows))]
fn clipboard_payload(content: &str) -> String {
    content.replace("\r\n", "\n").replace('\n', "\r\n")
}

#[cfg(all(not(target_os = "android"), not(windows)))]
fn clipboard_payload(content: &str) -> String {
    content.to_string()
}

/// Put text on the system clipboard (non-Android).
#[cfg(not(target_os = "android"))]
pub fn set_clipboard_text(content: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().with_context(|| "Failed to access clipboard")?;
    clipboard
        .set_text(clipboard_payload(content))
        .with_context(|| "Failed to set clipboard")?;
    Ok(())
}

/// Copy to clipboard for all platforms
#[cfg(not(target_os = "android"))]
pub fn copy_to_clipboard(content: &str, format: &str) -> Result<()> {
    let content = normalize_newlines(content);
    crate::shell::commands::clipboard::push_clip(&content);
    set_clipboard_text(&content)
        .with_context(|| format!("Failed to copy {} report to clipboard", format))?;

    Ok(())
}

#[cfg(target_os = "android")]
pub fn copy_to_clipboard(content: &str, format: &str) -> Result<()> {
    // Try Termux clipboard first
    if copy_to_clipboard_termux(content)? {
        super::theme::print_success(&format!(
            "{} report copied to clipboard via Termux!",
            format
        ));
        return Ok(());
    }

    // Fallback: save to temp file and show path
    let temp_file = std::env::temp_dir().join(format!(
        "ntc_{}_{}.{}",
        format.to_lowercase(),
        chrono::Local::now().format("%Y%m%d_%H%M%S"),
        format.to_lowercase()
    ));

    if let Ok(mut file) = std::fs::File::create(&temp_file) {
        let _ = file.write_all(content.as_bytes());
        super::theme::print_warning(&format!(
            "Clipboard not available. {} report saved to: {}",
            format,
            temp_file.display()
        ));
        super::theme::print_info(&format!(
            "You can view it with: cat {}",
            temp_file.display()
        ));
    } else {
        super::theme::print_warning(&format!(
            "Clipboard not supported on Android. {} report content shown above.",
            format
        ));
        println!("\n{}\n", content);
    }

    Ok(())
}
