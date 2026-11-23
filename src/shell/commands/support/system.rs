//! System-command execution: spawn, capture, Ctrl-C handling and shutdown
//! flags shared by the fallback dispatcher and `run`/`scr`/`ran`.

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::Result;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

/// Execute a system command
pub(crate) fn execute_system_command(cmd: &str, cwd: &Path) -> Result<bool> {
    let quiet = crate::shell::pipeline::is_quiet();
    if !quiet {
        print_info(&format!("Executing: {}", cmd));
        println!();
    }

    let status = run_system_command(cmd, cwd);
    if !quiet {
        println!();
    }

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                if !quiet {
                    print_success(&t!("cmd.completed"));
                }
            } else {
                // Non-zero exit counts as a failure for `&&` / `||`.
                crate::shell::pipeline::mark_failed();
                match exit_status.code() {
                    Some(code) => print_error(&format!("Command exited with code: {}", code)),
                    None => print_warning(&t!("cmd.terminated")),
                }
            }
            Ok(false)
        }
        Err(e) => {
            crate::shell::pipeline::mark_failed();
            print_error(&format!("Failed to execute command: {}", e));
            Ok(false)
        }
    }
}

static CHILD_PID: AtomicU32 = AtomicU32::new(0);

static SHUTDOWN_FLAGS: Mutex<Vec<Arc<AtomicBool>>> = Mutex::new(Vec::new());

pub fn register_shutdown_flag(flag: Arc<AtomicBool>) {
    if let Ok(mut v) = SHUTDOWN_FLAGS.lock() {
        v.push(flag);
    }
}

pub fn unregister_shutdown_flag(flag: &Arc<AtomicBool>) {
    if let Ok(mut v) = SHUTDOWN_FLAGS.lock() {
        v.retain(|f| !Arc::ptr_eq(f, flag));
    }
}

pub fn init_ctrlc_handler() {
    use std::sync::atomic::AtomicBool;
    static INIT: AtomicBool = AtomicBool::new(false);
    if INIT.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = ctrlc::set_handler(move || {
        // In `ntc ps`, Ctrl-C drops the PowerShell host (state is lost, the
        // next command starts a fresh one) so a runaway command can always
        // be aborted.
        if crate::shell::ext_complete::ps_mode_active() {
            crate::shell::ext_complete::kill_host();
        } else if crate::shell::ext_complete::bash_mode_active() {
            // Same contract for the integrated bash host: state is lost, the
            // next command starts a fresh shell.
            crate::shell::ext_complete::bash_kill_host();
        } else if crate::shell::ext_complete::zsh_mode_active() {
            // Same contract for the integrated zsh host on macOS.
            crate::shell::ext_complete::zsh_kill_host();
        }
        let pid = CHILD_PID.load(Ordering::SeqCst);
        if pid > 0 {
            #[cfg(windows)]
            {
                let _ = std::process::Command::new("taskkill")
                    .args(["/F", "/T", "/PID", &pid.to_string()])
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
        if let Ok(flags) = SHUTDOWN_FLAGS.lock() {
            for f in flags.iter() {
                f.store(true, Ordering::SeqCst);
            }
        }
    });
}

pub(crate) fn run_system_command(args: &str, cwd: &Path) -> Result<std::process::ExitStatus> {
    init_ctrlc_handler();

    // Input installed by a pipeline/`<` redirect (or piped process stdin).
    let input = crate::shell::stdin::take_input();
    let stdin_cfg = if input.is_some() {
        std::process::Stdio::piped()
    } else if crate::shell::stdin::script_mode() {
        // The shell is reading commands from this stdin; a child inheriting
        // it would consume the rest of the script.
        std::process::Stdio::null()
    } else {
        std::process::Stdio::inherit()
    };

    #[cfg(windows)]
    let mut child = std::process::Command::new("cmd")
        .args(["/C", args])
        .current_dir(cwd)
        .stdin(stdin_cfg)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    #[cfg(not(windows))]
    let mut child = std::process::Command::new("sh")
        .args(["-c", args])
        .current_dir(cwd)
        .stdin(stdin_cfg)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    if let Some(bytes) = input {
        if let Some(mut si) = child.stdin.take() {
            use std::io::Write;
            let _ = si.write_all(&bytes);
            // dropping `si` closes the pipe, signalling EOF to the child
        }
    }

    CHILD_PID.store(child.id(), Ordering::SeqCst);
    let status = child.wait()?;
    CHILD_PID.store(0, Ordering::SeqCst);
    Ok(status)
}

pub(crate) fn capture_system_command(args: &str, cwd: &Path) -> Result<std::process::Output> {
    #[cfg(windows)]
    let output = std::process::Command::new("cmd")
        .args(["/C", args])
        .current_dir(cwd)
        .output()?;

    #[cfg(not(windows))]
    let output = std::process::Command::new("sh")
        .args(["-c", args])
        .current_dir(cwd)
        .output()?;

    Ok(output)
}
