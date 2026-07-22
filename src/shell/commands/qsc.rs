//! QSC (Quick Short Command) command: `qsc listen`, `qsc stop`, `qsc status`.

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::utils::qsc;

use anyhow::Result;
use colored::Colorize;

pub(crate) fn cmd_qsc(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let _subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "listen" | "start" => cmd_qsc_listen(),
        "stop" => cmd_qsc_stop(),
        "status" | "stat" => cmd_qsc_status(),
        "startup" => cmd_qsc_startup(_subargs),
        "daemon" => {
            // Internal: run the actual daemon event loop.
            // Users should not run this directly.
            qsc::run()
        }
        _ => {
            println!("{}", "QSC — Quick Short Command (global hotkey)".cyan().bold());
            println!();
            println!("{}", "Usage:".yellow());
            println!("  qsc listen              Start QSC daemon (background process)");
            println!("  qsc stop                Stop the QSC daemon");
            println!("  qsc status              Check if QSC daemon is running");
            println!("  qsc startup on|off      Register/unregister as a startup app (Windows/Linux)");
            println!();
            println!("  First configure the hotkey:");
            println!("    config qsc \"#c 0\"        Set hotkey to Ctrl+0");
            println!("    config qsc \"#c #s a\"     Set hotkey to Ctrl+Shift+A");
            println!("    config qsc \"#c #s #a F1\"  Set hotkey to Ctrl+Shift+Alt+F1");
            println!("    config qsc off            Disable QSC hotkey");
            println!();
            println!("{}", "Current setting:".green());
            match Config::global_get_qsc() {
                Some(ref hk) => {
                    let mut parts = Vec::new();
                    if hk.ctrl  { parts.push("Ctrl"); }
                    if hk.shift { parts.push("Shift"); }
                    if hk.alt   { parts.push("Alt"); }
                    parts.push(&hk.key);
                    println!("  Hotkey: {}", parts.join("+").cyan());
                }
                None => println!("  {}", "Hotkey: (not configured)".dimmed()),
            }
            match qsc::daemon_status() {
                Ok(true) => println!("  {}", "Daemon: running".green()),
                Ok(false) => println!("  {}", "Daemon: stopped".dimmed()),
                Err(e) => println!("  {} ({})", "Daemon: unknown".dimmed(), e),
            }
            Ok(false)
        }
    }
}

fn cmd_qsc_listen() -> Result<bool> {
    let hotkey = Config::global_get_qsc();
    if hotkey.is_none() {
        print_error("No QSC hotkey configured. Use 'config qsc \"#c 0\"' first.");
        return Ok(false);
    }

    match qsc::daemon_status() {
        Ok(true) => {
            print_warning("QSC daemon is already running. Use 'qsc stop' first.");
            return Ok(false);
        }
        Ok(false) => {}
        Err(e) => {
            print_error(&format!("Failed to check daemon status: {}", e));
            return Ok(false);
        }
    }

    match qsc::spawn_daemon() {
        Ok(()) => {
            print_success("QSC daemon started.");
            // Show current hotkey
            if let Some(ref hk) = hotkey {
                let mut parts = Vec::new();
                if hk.ctrl  { parts.push("Ctrl"); }
                if hk.shift { parts.push("Shift"); }
                if hk.alt   { parts.push("Alt"); }
                parts.push(&hk.key);
                print_info(&format!("Hotkey: {} → spawns new ntc terminal", parts.join("+")));
            }
        }
        Err(e) => print_error(&format!("Failed to start QSC daemon: {}", e)),
    }
    Ok(false)
}

fn cmd_qsc_stop() -> Result<bool> {
    match qsc::daemon_status() {
        Ok(false) => {
            print_warning("QSC daemon is not running.");
            return Ok(false);
        }
        Err(e) => {
            print_error(&format!("Failed to check daemon status: {}", e));
            return Ok(false);
        }
        Ok(true) => {}
    }

    match qsc::stop_daemon() {
        Ok(()) => print_success("QSC daemon stopped."),
        Err(e) => print_error(&format!("Failed to stop QSC daemon: {}", e)),
    }
    Ok(false)
}

fn cmd_qsc_startup(args: &str) -> Result<bool> {
    match args.trim() {
        "on" => {
            match qsc::startup_action("on") {
                Ok(()) => {
                    print_success("QSC daemon registered as a startup app.");
                    // Also start the daemon immediately
                    cmd_qsc_listen()
                }
                Err(e) => {
                    print_error(&format!("Failed to enable startup: {}", e));
                    Ok(false)
                }
            }
        }
        "off" => {
            match qsc::startup_action("off") {
                Ok(()) => {
                    print_success("QSC daemon removed from startup apps.");
                    Ok(false)
                }
                Err(e) => {
                    print_error(&format!("Failed to disable startup: {}", e));
                    Ok(false)
                }
            }
        }
        _ => {
            print_error("Usage: qsc startup on|off");
            Ok(false)
        }
    }
}

fn cmd_qsc_status() -> Result<bool> {
    match qsc::daemon_status() {
        Ok(true) => {
            print_info("QSC daemon: running");
            if let Some(ref hk) = Config::global_get_qsc() {
                let mut parts = Vec::new();
                if hk.ctrl  { parts.push("Ctrl"); }
                if hk.shift { parts.push("Shift"); }
                if hk.alt   { parts.push("Alt"); }
                parts.push(&hk.key);
                println!("  Hotkey: {}", parts.join("+").cyan());
            }
        }
        Ok(false) => print_warning("QSC daemon: stopped"),
        Err(e) => print_error(&format!("QSC daemon: unknown ({})", e)),
    }
    Ok(false)
}
