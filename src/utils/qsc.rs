//! QSC (Quick Short Command) — global hotkey daemon.
//! Registers a platform-specific global hotkey. When pressed, spawns a new
//! terminal running `ntc`. Runs as a background process (`ntc qsc daemon`).

#[cfg(any(windows, target_os = "linux"))]
use crate::config::QscHotkey;
use std::path::PathBuf;

#[cfg(windows)]
extern "system" {
    fn FreeConsole() -> i32;
}

// ---------------------------------------------------------------------------
// PID file management
// ---------------------------------------------------------------------------

fn pid_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("qsc.pid")
}

// ---------------------------------------------------------------------------
// Daemon lifecycle (cross-platform)
// ---------------------------------------------------------------------------

/// Spawn a background daemon process (`ntc qsc daemon`).
/// Kills any existing daemon first.
pub(crate) fn spawn_daemon() -> std::io::Result<()> {
    stop_daemon().ok();

    let exe = std::env::current_exe()?;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let child = std::process::Command::new(&exe)
            .args(["qsc", "daemon"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .stdin(std::process::Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;
        if let Some(parent) = pid_path().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(pid_path(), child.id().to_string())?;
    }
    #[cfg(not(windows))]
    {
        let child = std::process::Command::new(&exe)
            .args(["qsc", "daemon"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .stdin(std::process::Stdio::null())
            .spawn()?;
        if let Some(parent) = pid_path().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&pid_path(), child.id().to_string())?;
    }
    Ok(())
}

/// Stop the running daemon by PID file.
pub(crate) fn stop_daemon() -> std::io::Result<()> {
    let path = pid_path();
    if !path.exists() {
        return Ok(());
    }
    if let Ok(pid_str) = std::fs::read_to_string(&path) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            #[cfg(windows)]
            {
                let _ = std::process::Command::new("taskkill")
                    .args(["/F", "/PID", &pid.to_string()])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
            }
            #[cfg(not(windows))]
            {
                let _ = std::process::Command::new("kill")
                    .args(["-TERM", &pid.to_string()])
                    .status();
            }
        }
    }
    let _ = std::fs::remove_file(&path);
    Ok(())
}

/// Is the process with this PID still alive?
fn pid_alive(pid: u32) -> std::io::Result<bool> {
    #[cfg(windows)]
    {
        let out = std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid), "/NH"])
            .stdout(std::process::Stdio::piped())
            .output()?;
        Ok(String::from_utf8_lossy(&out.stdout).contains(&pid.to_string()))
    }
    #[cfg(not(windows))]
    {
        let status = std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()?;
        Ok(status.success())
    }
}

/// Register this process as the running daemon in the PID file.
///
/// `spawn_daemon` writes the file from the parent, but a daemon started
/// directly — the `HKCU\...\Run` entry at logon, or `ntc qsc daemon` typed
/// by hand — has no parent to do it. The loop treats a missing PID file as
/// the stop signal, so without claiming the file such a daemon exits on its
/// very first iteration and QSC never survives a reboot.
///
/// Returns `false` when another live daemon already owns the file.
fn claim_pid_file() -> std::io::Result<bool> {
    let path = pid_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(existing) = std::fs::read_to_string(&path) {
        if let Ok(pid) = existing.trim().parse::<u32>() {
            if pid == std::process::id() {
                return Ok(true);
            }
            if pid_alive(pid)? {
                return Ok(false);
            }
        }
    }
    std::fs::write(&path, std::process::id().to_string())?;
    Ok(true)
}

/// Check if the daemon is running.
pub(crate) fn daemon_status() -> std::io::Result<bool> {
    let path = pid_path();
    if !path.exists() {
        return Ok(false);
    }
    if let Ok(pid_str) = std::fs::read_to_string(&path) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            if pid_alive(pid)? {
                return Ok(true);
            }
        }
    }
    // Stale PID file
    let _ = std::fs::remove_file(&path);
    Ok(false)
}

/// Register or unregister the QSC daemon as a system startup item.
/// `"on"` enables startup (and also starts the daemon), `"off"` disables it.
pub(crate) fn startup_action(action: &str) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("failed to get exe path: {e}"))?;
    #[allow(unused_variables)]
    let exe_str = exe.to_string_lossy().to_string();

    #[cfg(windows)]
    {
        // Run via PowerShell with hidden window so no console appears at boot.
        let cmd = format!(
            "powershell -WindowStyle Hidden -NoProfile -Command \"& '{exe_str}' qsc daemon\""
        );
        match action {
            "on" => {
                let status = std::process::Command::new("reg")
                    .args([
                        "add",
                        "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                        "/v",
                        "NTC QSC Daemon",
                        "/d",
                        &cmd,
                        "/f",
                    ])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map_err(|e| format!("failed to run reg.exe: {e}"))?;
                if status.success() {
                    Ok(())
                } else {
                    Err("reg.exe failed — try running as administrator?".into())
                }
            }
            "off" => {
                let status = std::process::Command::new("reg")
                    .args([
                        "delete",
                        "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                        "/v",
                        "NTC QSC Daemon",
                        "/f",
                    ])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map_err(|e| format!("failed to run reg.exe: {e}"))?;
                if status.success() {
                    Ok(())
                } else {
                    Err("reg.exe failed — entry may not exist?".into())
                }
            }
            _ => Err("usage: qsc startup on|off".into()),
        }
    }
    #[cfg(target_os = "linux")]
    {
        let autostart_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("autostart");
        let desktop_path = autostart_dir.join("ntc-qsc.desktop");

        match action {
            "on" => {
                std::fs::create_dir_all(&autostart_dir)
                    .map_err(|e| format!("failed to create autostart dir: {e}"))?;
                let contents = format!(
                    "[Desktop Entry]\n\
                     Type=Application\n\
                     Name=NTC QSC Daemon\n\
                     Exec={exe_str} qsc daemon\n\
                     Terminal=false\n\
                     X-GNOME-Autostart-enabled=true\n"
                );
                std::fs::write(&desktop_path, &contents)
                    .map_err(|e| format!("failed to write desktop file: {e}"))?;
                Ok(())
            }
            "off" => {
                if desktop_path.exists() {
                    std::fs::remove_file(&desktop_path)
                        .map_err(|e| format!("failed to remove desktop file: {e}"))?;
                }
                Ok(())
            }
            _ => Err("usage: qsc startup on|off".into()),
        }
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        let _ = (exe, action);
        Err("startup is only supported on Windows and Linux".into())
    }
}

/// Open a new terminal window running `ntc`.
#[allow(dead_code)]
fn open_ntc_terminal() {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "", "ntc"])
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        // Try $TERMINAL, then x-terminal-emulator, then common terminals
        let terminals = &[
            std::env::var("TERMINAL").ok().unwrap_or_default(),
            "x-terminal-emulator".to_string(),
            "gnome-terminal".to_string(),
            "konsole".to_string(),
            "xterm".to_string(),
        ];
        for term in terminals {
            if term.is_empty() {
                continue;
            }
            let result = match term.as_str() {
                "gnome-terminal" => std::process::Command::new(term)
                    .arg("--")
                    .arg("ntc")
                    .spawn(),
                _ => std::process::Command::new(term)
                    .arg("-e")
                    .arg("ntc")
                    .spawn(),
            };
            if result.is_ok() {
                return;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .args(["-a", "Terminal", "ntc"])
            .spawn();
    }
}

// ---------------------------------------------------------------------------
// Platform-specific hotkey event loops
// ---------------------------------------------------------------------------

#[cfg(windows)]
mod platform {
    use super::*;

    type Uint = u32;
    type Bool = i32;

    const MOD_ALT: Uint = 0x0001;
    const MOD_CONTROL: Uint = 0x0002;
    const MOD_SHIFT: Uint = 0x0004;
    const MOD_NOREPEAT: Uint = 0x4000;
    const WM_HOTKEY: Uint = 0x0312;
    const PM_REMOVE: Uint = 0x0001;

    #[repr(C)]
    struct Msg {
        hwnd: isize,
        message: Uint,
        w_param: usize,
        l_param: isize,
        time: Uint,
        pt_x: i32,
        pt_y: i32,
    }

    extern "system" {
        fn RegisterHotKey(h_wnd: isize, id: i32, fs_modifiers: Uint, vk: Uint) -> Bool;
        fn UnregisterHotKey(h_wnd: isize, id: i32) -> Bool;
        fn PeekMessageW(
            msg: *mut Msg,
            h_wnd: isize,
            w_msg_filter_min: Uint,
            w_msg_filter_max: Uint,
            w_remove_msg: Uint,
        ) -> Bool;
        fn TranslateMessage(msg: *const Msg) -> Bool;
        fn DispatchMessageW(msg: *const Msg) -> isize;
    }

    fn key_to_vk(key: &str) -> Option<Uint> {
        if key.len() == 1 {
            let c = key.chars().next()?;
            if c.is_ascii_alphanumeric() {
                return Some(c as Uint);
            }
            return None;
        }
        match key {
            "F1" => Some(0x70),
            "F2" => Some(0x71),
            "F3" => Some(0x72),
            "F4" => Some(0x73),
            "F5" => Some(0x74),
            "F6" => Some(0x75),
            "F7" => Some(0x76),
            "F8" => Some(0x77),
            "F9" => Some(0x78),
            "F10" => Some(0x79),
            "F11" => Some(0x7A),
            "F12" => Some(0x7B),
            _ => None,
        }
    }

    fn modifiers_mask(hk: &QscHotkey) -> Uint {
        let mut m = MOD_NOREPEAT;
        if hk.ctrl {
            m |= MOD_CONTROL;
        }
        if hk.shift {
            m |= MOD_SHIFT;
        }
        if hk.alt {
            m |= MOD_ALT;
        }
        m
    }

    /// Returns the config modification time (for change detection).
    fn config_mtime() -> Option<std::time::SystemTime> {
        let p = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ntc")
            .join("config.toml");
        std::fs::metadata(&p).ok().and_then(|m| m.modified().ok())
    }

    pub(crate) fn run_daemon_loop() -> ! {
        // Detach from the console so no window stays visible at boot
        unsafe {
            FreeConsole();
        }

        // Reload config to get latest QSC setting
        crate::config::Config::reload_global();

        let mut hk = crate::config::Config::global_get_qsc();
        let mut last_mtime = config_mtime();
        let mut registered = false;

        loop {
            // Check for config changes
            let new_mtime = config_mtime();
            if new_mtime != last_mtime {
                last_mtime = new_mtime;
                crate::config::Config::reload_global();
                let new_hk = crate::config::Config::global_get_qsc();
                if new_hk != hk {
                    // Unregister old, register new
                    if registered {
                        unsafe {
                            UnregisterHotKey(0, 1);
                        }
                        registered = false;
                    }
                    hk = new_hk;
                }
            }

            // Register hotkey if needed
            if !registered {
                if let Some(ref hotkey) = hk {
                    if let Some(vk) = key_to_vk(&hotkey.key) {
                        let mods = modifiers_mask(hotkey);
                        let rc = unsafe { RegisterHotKey(0, 1, mods, vk) };
                        registered = rc != 0;
                    }
                }
            }

            // Peek at messages (non-blocking)
            let mut msg: Msg = unsafe { std::mem::zeroed() };
            unsafe {
                while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
                    if msg.message == WM_HOTKEY {
                        open_ntc_terminal();
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                    // Reset msg for next peek
                    msg = std::mem::zeroed();
                }
            }

            // Check if PID file still exists (stop signal)
            if !pid_path().exists() {
                // Clean up and exit
                if registered {
                    unsafe {
                        UnregisterHotKey(0, 1);
                    }
                }
                std::process::exit(0);
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use x11rb::protocol::xproto::ModMask;

    fn keysym_from_key(key: &str) -> Option<u32> {
        if key.len() == 1 {
            let c = key.chars().next()?;
            // Convert ASCII char to X11 keysym
            if c.is_ascii_lowercase() {
                return Some((c as u8).wrapping_sub(0x20) as u32); // 'a' -> 'A'
            }
            return Some(c as u32);
        }
        // F1-F12 mapping
        match key {
            "F1" => Some(0xFFBE),
            "F2" => Some(0xFFBF),
            "F3" => Some(0xFFC0),
            "F4" => Some(0xFFC1),
            "F5" => Some(0xFFC2),
            "F6" => Some(0xFFC3),
            "F7" => Some(0xFFC4),
            "F8" => Some(0xFFC5),
            "F9" => Some(0xFFC6),
            "F10" => Some(0xFFC7),
            "F11" => Some(0xFFC8),
            "F12" => Some(0xFFC9),
            _ => None,
        }
    }

    fn x11_mod_mask(hk: &QscHotkey) -> ModMask {
        let mut m = ModMask::default();
        if hk.ctrl {
            m |= ModMask::CONTROL;
        }
        if hk.shift {
            m |= ModMask::SHIFT;
        }
        if hk.alt {
            m |= ModMask::M1;
        }
        m
    }

    fn config_mtime() -> Option<std::time::SystemTime> {
        let p = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ntc")
            .join("config.toml");
        std::fs::metadata(&p).ok().and_then(|m| m.modified().ok())
    }

    fn keysym_to_keycode(
        conn: &(impl x11rb::protocol::xproto::ConnectionExt + x11rb::connection::Connection),
        keysym: u32,
    ) -> Option<u8> {
        let setup = conn.setup();
        let min = setup.min_keycode;
        let max = setup.max_keycode;
        let count = max - min + 1;
        let reply = conn.get_keyboard_mapping(min, count).ok()?.reply().ok()?;
        let per = reply.keysyms_per_keycode as usize;
        for (i, chunk) in reply.keysyms.chunks(per).enumerate() {
            if chunk.contains(&keysym) {
                return Some(min + i as u8);
            }
        }
        None
    }

    pub(crate) fn run_daemon_loop() -> ! {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;

        crate::config::Config::reload_global();
        let mut hk = crate::config::Config::global_get_qsc();
        let mut last_mtime = config_mtime();

        let (conn, screen_num) = match x11rb::connect(None) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("QSC: X11 connection failed: {}", e);
                std::process::exit(1);
            }
        };
        let screen = &conn.setup().roots[screen_num];
        let root = screen.root;

        let mut registered = false;
        let mut current_grab: Option<(u8, ModMask)> = None;

        loop {
            // Check config changes
            let new_mtime = config_mtime();
            if new_mtime != last_mtime {
                last_mtime = new_mtime;
                crate::config::Config::reload_global();
                let new_hk = crate::config::Config::global_get_qsc();
                if new_hk != hk {
                    if let Some((kc, mods)) = current_grab.take() {
                        let _ = conn.ungrab_key(kc, root, mods);
                        conn.flush().ok();
                        registered = false;
                    }
                    hk = new_hk;
                }
            }

            // Register hotkey if needed
            if !registered {
                if let Some(ref hotkey) = hk {
                    if let Some(ks) = keysym_from_key(&hotkey.key) {
                        let kc = keysym_to_keycode(&conn, ks);
                        if let Some(kc) = kc {
                            let mods = x11_mod_mask(hotkey);
                            let _ = conn.grab_key(
                                false,
                                root,
                                mods,
                                kc,
                                GrabMode::ASYNC,
                                GrabMode::ASYNC,
                            );
                            conn.flush().ok();
                            current_grab = Some((kc, mods));
                            registered = true;
                        }
                    }
                }
            }

            // Poll for events
            while let Some(event) = conn.poll_for_event().ok().flatten() {
                if matches!(event, x11rb::protocol::Event::KeyPress(_)) {
                    open_ntc_terminal();
                }
            }

            // Check stop signal
            if !pid_path().exists() {
                if let Some((kc, mods)) = current_grab {
                    let _ = conn.ungrab_key(kc, root, mods);
                    conn.flush().ok();
                }
                std::process::exit(0);
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use crate::t;
    pub(crate) fn run_daemon_loop() -> ! {
        eprintln!("{}", t!("qsc.macos_not_implemented"));
        eprintln!("{}", t!("qsc.macos_listen_hint"));
        std::process::exit(1);
    }
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
mod platform {
    use crate::t;
    pub(crate) fn run_daemon_loop() -> ! {
        eprintln!("{}", t!("qsc.unsupported_platform"));
        std::process::exit(1);
    }
}

/// Entry point for the daemon process.
pub(crate) fn run() -> ! {
    #[cfg(windows)]
    unsafe {
        // Detach console as early as possible to prevent window flash at boot.
        // (Also done inside the platform loop, but this is an extra safety net.)
        let _ = FreeConsole();
    }
    // Take ownership of the PID file before the loop: the daemon may have
    // been started by the startup registration, where nobody else writes it.
    if let Ok(false) = claim_pid_file() {
        // Another live daemon already owns the hotkey.
        std::process::exit(0);
    }
    platform::run_daemon_loop()
}
