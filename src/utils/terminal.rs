use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag set by config::load() — theme.rs reads it to pick ASCII icons.
static ASCII_MODE: AtomicBool = AtomicBool::new(false);

/// Set the ASCII-fallback mode (called by config during startup).
pub fn set_ascii_mode(enabled: bool) {
    ASCII_MODE.store(enabled, Ordering::Relaxed);
}

/// Whether ASCII-only output should be used (icons, box-drawing, etc).
pub fn is_ascii_mode() -> bool {
    ASCII_MODE.load(Ordering::Relaxed)
}

/// Terminal capabilities detected at startup.
pub struct TerminalCapabilities {
    /// Whether ANSI escape codes render correctly (false on Win7/8 conhost)
    pub color: bool,
    /// Whether stdout is a real terminal (not piped/redirected)
    pub is_terminal: bool,
    /// Whether Termux environment is detected
    pub is_termux: bool,
}

/// Detect terminal capabilities at startup.
pub fn detect() -> TerminalCapabilities {
    TerminalCapabilities {
        color: ansi_supported(),
        is_terminal: std::io::stdout().is_terminal(),
        is_termux: termux_detected(),
    }
}

/// Whether ASCII-only output should be used for icons etc.
pub fn ascii_fallback_needed() -> bool {
    let caps = detect();
    !caps.is_terminal || !caps.color
}

/// Check ANSI color support (Win7/8 legacy conhost detection)
fn ansi_supported() -> bool {
    #[cfg(windows)]
    {
        match std::process::Command::new("cmd")
            .args(["/c", "ver"])
            .output()
        {
            Ok(output) => {
                let s = String::from_utf8_lossy(&output.stdout);
                if let Some(ver_part) = s.split('[').nth(1) {
                    let ver_str = ver_part.split(']').next().unwrap_or("");
                    let ver_str = ver_str.trim_start_matches("Version ");
                    if let Some(major_str) = ver_str.split('.').next() {
                        if let Ok(major) = major_str.trim().parse::<u32>() {
                            return major >= 10;
                        }
                    }
                }
                true
            }
            Err(_) => true,
        }
    }
    #[cfg(not(windows))]
    {
        true
    }
}

fn termux_detected() -> bool {
    std::env::var("PREFIX").is_ok_and(|p| p.contains("/data/data/com.termux"))
}

/// Replace unicode box-drawing characters with ASCII equivalents
/// when `is_ascii_mode()` is active.
/// - Corners/T-junctions → `+`
/// - Verticals → `|`
/// - Horizontals → `-`
pub fn asciify(s: &str) -> String {
    if !is_ascii_mode() {
        return s.to_string();
    }
    s.chars()
        .map(|c| match c {
            '╔' | '╗' | '╚' | '╝' | '╒' | '╕' | '╘' | '╛' | '╠' | '╣' | '╞' | '╡'
            | '╤' | '╧' | '╪' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴'
            | '┼' => '+',
            '║' | '│' => '|',
            '═' | '─' => '-',
            _ => c,
        })
        .collect()
}
