use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag set by config::load() — theme.rs reads it to pick ASCII icons.
static ASCII_MODE: AtomicBool = AtomicBool::new(false);

/// Cached result of the Windows VT-enablement attempt.
/// `None` (false + NOT_INITIALIZED) means `init()` was never called;
/// callers should treat that as "unknown, try init first".
/// Windows-only: non-Windows `vt_enabled()`/`init()` early-return `true`,
/// so these would be dead code (and warnings) on Linux/Android/macOS builds.
#[cfg(windows)]
static VT_ENABLED: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static VT_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Set the ASCII-fallback mode (called by config during startup).
pub fn set_ascii_mode(enabled: bool) {
    ASCII_MODE.store(enabled, Ordering::Relaxed);
}

/// Whether ASCII-only output should be used (icons, box-drawing, etc).
pub fn is_ascii_mode() -> bool {
    ASCII_MODE.load(Ordering::Relaxed)
}

/// Whether VT processing is available for color output.
///
/// On non-Windows this is always true (Unix terminals handle ANSI natively).
/// On Windows it reflects the result of `init()`: true when
/// `ENABLE_VIRTUAL_TERMINAL_PROCESSING` could be enabled (or stdout is not
/// a console, e.g. piped — in which case codes are harmless and usually
/// stripped downstream).
pub fn vt_enabled() -> bool {
    #[cfg(not(windows))]
    {
        true
    }
    #[cfg(windows)]
    {
        if VT_INITIALIZED.load(Ordering::Relaxed) {
            return VT_ENABLED.load(Ordering::Relaxed);
        }
        // Lazily attempt init so `Config::load()` ordering can't leave us
        // in a permanently-disabled state.
        init()
    }
}

/// Attempt to enable Virtual Terminal processing on Windows stdout/stderr.
///
/// Must be called at the very start of `main()`, before any colored output.
/// - Returns `true` when ANSI codes will render (VT on, Windows Terminal,
///   or stdout redirected / not a console).
/// - Returns `false` on legacy conhost (Win7/8 or hardened Admin conhost)
///   where the OS refuses `ENABLE_VIRTUAL_TERMINAL_PROCESSING` — callers
///   must then strip colors and use ASCII boxes.
///
/// Safe to call multiple times; the result is cached.
pub fn init() -> bool {
    #[cfg(not(windows))]
    {
        return true;
    }
    #[cfg(windows)]
    {
        if VT_INITIALIZED.load(Ordering::Relaxed) {
            return VT_ENABLED.load(Ordering::Relaxed);
        }
        let ok = enable_vt_processing();
        VT_ENABLED.store(ok, Ordering::Relaxed);
        VT_INITIALIZED.store(true, Ordering::Relaxed);
        ok
    }
}

#[cfg(windows)]
fn enable_vt_processing() -> bool {
    use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        STD_ERROR_HANDLE, STD_OUTPUT_HANDLE,
    };

    // If stdout isn't a console at all (piped/file), ANSI codes don't hurt
    // the console — downstream tools strip or keep them. Report success so
    // piped output keeps colors when the user wants them.
    if !std::io::stdout().is_terminal() {
        return true;
    }

    let mut all_ok = true;
    for std_handle in [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
        unsafe {
            let handle: HANDLE = GetStdHandle(std_handle);
            if handle.is_null() || handle == INVALID_HANDLE_VALUE {
                continue; // handle unavailable — not fatal
            }
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) == 0 {
                // Not a console (redirected handle) — fine, skip.
                continue;
            }
            // Only add the VT bit — never touch DISABLE_NEWLINE_AUTO_RETURN
            // (0x0008). Setting it makes `\n` move down without returning to
            // column 0, which staircases every println! on legacy conhost.
            let want = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            if want != mode && SetConsoleMode(handle, want) == 0 {
                // OS refused (pre-Win10 conhost, legacy mode) — legacy path.
                all_ok = false;
            }
        }
    }
    all_ok
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
    // Explicit opt-outs always win, on every platform.
    let no_color_env = std::env::var_os("NO_COLOR").is_some()
        || matches!(std::env::var("TERM"), Ok(t) if t == "dumb" || t.is_empty());
    let is_terminal = std::io::stdout().is_terminal();
    let color = if no_color_env {
        false
    } else {
        ansi_supported()
    };
    TerminalCapabilities {
        color,
        is_terminal,
        is_termux: termux_detected(),
    }
}

/// Whether ASCII-only output should be used for icons etc.
pub fn ascii_fallback_needed() -> bool {
    let caps = detect();
    !caps.is_terminal || !caps.color
}

/// Check ANSI color support.
///
/// On Windows this now means: VT processing could actually be enabled
/// (or we're inside Windows Terminal / VS Code / piped), NOT just
/// "OS version >= 10". Version-only checks were the bug: Win10/Admin
/// conhost reports 10.x but renders `[91m` garbage with VT off.
fn ansi_supported() -> bool {
    #[cfg(windows)]
    {
        // Modern hosts that interpret ANSI themselves regardless of conhost mode.
        if std::env::var_os("WT_SESSION").is_some() {
            return true;
        }
        if let Ok(prog) = std::env::var("TERM_PROGRAM") {
            if prog == "vscode" || prog == "WindowsTerminal" {
                return true;
            }
        }
        if termux_detected() {
            return true;
        }
        // Real check: can we enable VT on this console?
        // If the OS refuses (pre-Win10 conhost, legacy/Admin conhost),
        // legacy conhost cannot render even basic ANSI, so report false
        // and let callers strip colors + use ASCII boxes.
        vt_enabled()
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
            '╔' | '╗' | '╚' | '╝' | '╒' | '╕' | '╘' | '╛' | '╠' | '╣' | '╞' | '╡' | '╤' | '╧'
            | '╪' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' => '+',
            '║' | '│' => '|',
            '═' | '─' => '-',
            _ => c,
        })
        .collect()
}
