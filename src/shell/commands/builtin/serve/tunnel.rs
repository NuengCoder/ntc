//! `cloudflared` tunnel bootstrap shared by the `serve` services.

use crate::output::{print_error, print_info};
use crate::t;
use std::path::PathBuf;

pub(crate) fn spawn_cloudflared(port: u16) {
    let url = format!("http://localhost:{}", port);
    let cf = find_cloudflared().unwrap_or_else(|| "cloudflared".to_string());
    match std::process::Command::new(&cf)
        .args(["tunnel", "--url", &url])
        .spawn()
    {
        Ok(mut c) => {
            print_info(&t!("cmd.serve.cloudflared_starting", c.id()));
            std::thread::spawn(move || {
                let _ = c.wait();
            });
        }
        Err(e) => {
            let hint = if crate::shell::commands::builtin::ntpkg::ntpkg::manager::is_termux() {
                "pkg install cloudflared"
            } else {
                "ntpkg install cloudflare"
            };
            print_error(&t!("cmd.serve.cloudflared_missing", e, hint));
        }
    }
}

fn find_cloudflared() -> Option<String> {
    if which::which("cloudflared").is_ok() {
        return Some("cloudflared".to_string());
    }
    let p = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("bin")
        .join("cloudflared");
    if p.exists() {
        return Some(p.to_string_lossy().to_string());
    }
    #[cfg(windows)]
    {
        let pe = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ntc")
            .join("bin")
            .join("cloudflared.exe");
        if pe.exists() {
            return Some(pe.to_string_lossy().to_string());
        }
    }
    None
}
