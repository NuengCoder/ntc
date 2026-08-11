//! QSC (Quick Short Command) command: `qsc listen`, `qsc stop`, `qsc status`.

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
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
            println!(
                "{}",
                "QSC — Quick Short Command (global hotkey)".cyan().bold()
            );
            println!();
            println!("{}", t!("cmd.qsc.usage_title").yellow());
            println!("  {}", t!("cmd.qsc.help_listen"));
            println!("  {}", t!("cmd.qsc.help_stop"));
            println!("  {}", t!("cmd.qsc.help_status"));
            println!(
                "  qsc startup on|off      Register/unregister as a startup app (Windows/Linux)"
            );
            println!();
            println!("  {}", t!("cmd.qsc.help_configure"));
            println!("    {}", t!("cmd.qsc.help_conf_ex1"));
            println!("    {}", t!("cmd.qsc.help_conf_ex2"));
            println!("    {}", t!("cmd.qsc.help_conf_ex3"));
            println!("    {}", t!("cmd.qsc.help_conf_off"));
            println!();
            println!("{}", t!("cmd.qsc.current_setting").green());
            match Config::global_get_qsc() {
                Some(ref hk) => {
                    let mut parts = Vec::new();
                    if hk.ctrl {
                        parts.push("Ctrl");
                    }
                    if hk.shift {
                        parts.push("Shift");
                    }
                    if hk.alt {
                        parts.push("Alt");
                    }
                    parts.push(&hk.key);
                    println!("  {}", t!("cmd.qsc.hotkey_label", parts.join("+").cyan()));
                }
                None => println!("  {}", t!("cmd.qsc.hotkey_not_configured").dimmed()),
            }
            match qsc::daemon_status() {
                Ok(true) => println!("  {}", t!("cmd.qsc.daemon_running").green()),
                Ok(false) => println!("  {}", t!("cmd.qsc.daemon_stopped").dimmed()),
                Err(e) => println!("  {} ({})", t!("cmd.qsc.daemon_unknown").dimmed(), e),
            }
            Ok(false)
        }
    }
}

fn cmd_qsc_listen() -> Result<bool> {
    let hotkey = Config::global_get_qsc();
    if hotkey.is_none() {
        print_error(&t!("cmd.qsc.not_configured"));
        return Ok(false);
    }

    match qsc::daemon_status() {
        Ok(true) => {
            print_warning(&t!("cmd.qsc.already_running"));
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
            print_success(&t!("cmd.qsc.started"));
            // Show current hotkey
            if let Some(ref hk) = hotkey {
                let mut parts = Vec::new();
                if hk.ctrl {
                    parts.push("Ctrl");
                }
                if hk.shift {
                    parts.push("Shift");
                }
                if hk.alt {
                    parts.push("Alt");
                }
                parts.push(&hk.key);
                print_info(&format!(
                    "Hotkey: {} → spawns new ntc terminal",
                    parts.join("+")
                ));
            }
        }
        Err(e) => print_error(&format!("Failed to start QSC daemon: {}", e)),
    }
    Ok(false)
}

fn cmd_qsc_stop() -> Result<bool> {
    match qsc::daemon_status() {
        Ok(false) => {
            print_warning(&t!("cmd.qsc.not_running"));
            return Ok(false);
        }
        Err(e) => {
            print_error(&format!("Failed to check daemon status: {}", e));
            return Ok(false);
        }
        Ok(true) => {}
    }

    match qsc::stop_daemon() {
        Ok(()) => print_success(&t!("cmd.qsc.stopped")),
        Err(e) => print_error(&format!("Failed to stop QSC daemon: {}", e)),
    }
    Ok(false)
}

fn cmd_qsc_startup(args: &str) -> Result<bool> {
    match args.trim() {
        "on" => {
            match qsc::startup_action("on") {
                Ok(()) => {
                    print_success(&t!("cmd.qsc.startup_registered"));
                    // Also start the daemon immediately
                    cmd_qsc_listen()
                }
                Err(e) => {
                    print_error(&format!("Failed to enable startup: {}", e));
                    Ok(false)
                }
            }
        }
        "off" => match qsc::startup_action("off") {
            Ok(()) => {
                print_success(&t!("cmd.qsc.startup_removed"));
                Ok(false)
            }
            Err(e) => {
                print_error(&format!("Failed to disable startup: {}", e));
                Ok(false)
            }
        },
        _ => {
            print_error(&t!("cmd.qsc.startup_usage"));
            Ok(false)
        }
    }
}

fn cmd_qsc_status() -> Result<bool> {
    match qsc::daemon_status() {
        Ok(true) => {
            print_info(&t!("cmd.qsc.status_running"));
            if let Some(ref hk) = Config::global_get_qsc() {
                let mut parts = Vec::new();
                if hk.ctrl {
                    parts.push("Ctrl");
                }
                if hk.shift {
                    parts.push("Shift");
                }
                if hk.alt {
                    parts.push("Alt");
                }
                parts.push(&hk.key);
                println!("  {}", t!("cmd.qsc.hotkey_label", parts.join("+").cyan()));
            }
        }
        Ok(false) => print_warning(&t!("cmd.qsc.status_stopped")),
        Err(e) => print_error(&t!("cmd.qsc.daemon_unknown_err", e)),
    }
    Ok(false)
}
