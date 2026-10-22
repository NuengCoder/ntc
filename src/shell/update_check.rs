//! Daily new-release notice shown once per 24h at interactive shell startup.
//!
//! Gates (in order): `NTC_NO_UPDATE_CHECK=1` / `config update_check OFF` →
//! silent; 24h throttle file → silent; no internet → silent; GitHub latest
//! not newer than `CARGO_PKG_VERSION` → silent. Otherwise prints one line:
//! `New NTC vX.Y.Z available (you have vA.B.C) — run 'ntc --upgrade' in
//! CMD / bash (outside NTC shell) or see <releases URL>`.
//!
//! The network work runs on a background thread so shell startup never
//! blocks; a cached newer version prints immediately on the next startup.

use crate::t;
use colored::*;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

const API_URL: &str = "https://api.github.com/repos/NuengCoder/ntc/releases/latest";
const RELEASES_URL: &str = "https://github.com/NuengCoder/ntc/releases";
const CHECK_INTERVAL_SECS: u64 = 24 * 3600;
const PROBE_TIMEOUT: Duration = Duration::from_millis(900);

fn state_path() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("ntc").join("update_check.json"))
}

fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn shell_hint() -> &'static str {
    if cfg!(windows) {
        "CMD"
    } else {
        "bash (outside NTC shell)"
    }
}

fn has_internet() -> bool {
    for target in ["8.8.8.8:53", "1.1.1.1:53"] {
        if let Ok(addr) = target.parse::<SocketAddr>() {
            if TcpStream::connect_timeout(&addr, PROBE_TIMEOUT).is_ok() {
                return true;
            }
        }
    }
    false
}

fn is_newer(latest: &str, current: &str) -> bool {
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
    for (l, c) in latest_parts.iter().zip(current_parts.iter()) {
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    latest_parts.len() > current_parts.len()
}

fn fetch_latest() -> Option<String> {
    let response = ureq::get(API_URL).header("User-Agent", "ntc").call().ok()?;
    let mut body = response.into_body();
    let mut raw = String::new();
    use std::io::Read;
    body.as_reader().read_to_string(&mut raw).ok()?;
    let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
    json["tag_name"]
        .as_str()
        .map(|s| s.trim_start_matches('v').to_string())
}

fn print_notice(latest: &str) {
    let current = env!("CARGO_PKG_VERSION");
    let msg = t!(
        "msg.update.available",
        format!("v{}", latest),
        format!("v{}", current),
        shell_hint(),
        RELEASES_URL.cyan().to_string()
    );
    println!("{}", msg.dimmed());
}

fn read_state() -> (u64, String, u64) {
    let Some(p) = state_path() else {
        return (0, String::new(), 0);
    };
    let Ok(raw) = std::fs::read_to_string(&p) else {
        return (0, String::new(), 0);
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return (0, String::new(), 0);
    };
    (
        v["last_check"].as_u64().unwrap_or(0),
        v["latest"].as_str().unwrap_or("").to_string(),
        v["last_notify"].as_u64().unwrap_or(0),
    )
}

fn write_state(last_check: u64, latest: &str, last_notify: u64) {
    let Some(p) = state_path() else { return };
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let v =
        serde_json::json!({"last_check": last_check, "latest": latest, "last_notify": last_notify});
    let _ = std::fs::write(&p, v.to_string());
}

/// Spawn the daily update check; never blocks the caller.
pub fn spawn_update_check() {
    if std::env::var("NTC_NO_UPDATE_CHECK").as_deref() == Ok("1") {
        return;
    }
    if !crate::config::Config::global_get_update_check_enabled() {
        return;
    }
    // Non-interactive use (`ntc <cmd>`, pipes, CI) never notifies.
    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        return;
    }
    let now = now_epoch();
    let (last_check, cached_latest, last_notify) = read_state();
    let current = env!("CARGO_PKG_VERSION");
    // Cached newer version: remind immediately (max 1x/24h), no network.
    if !cached_latest.is_empty()
        && is_newer(&cached_latest, current)
        && now.saturating_sub(last_notify) >= CHECK_INTERVAL_SECS
    {
        print_notice(&cached_latest);
        write_state(last_check, &cached_latest, now);
        return;
    }
    // Throttle network fetches to 1x/24h.
    if now.saturating_sub(last_check) < CHECK_INTERVAL_SECS {
        return;
    }
    std::thread::spawn(move || {
        // Condition 1: internet. Condition 2: GitHub tag newer than build.
        if !has_internet() {
            write_state(now, &cached_latest, last_notify);
            return;
        }
        let latest = match fetch_latest() {
            Some(v) => v,
            None => {
                write_state(now, &cached_latest, last_notify);
                return;
            }
        };
        write_state(now, &latest, last_notify);
        if !is_newer(&latest, current) {
            return;
        }
        // Fresh notify gate: another startup may have printed for this same
        // version while we were fetching — then stay silent (max 1x/24h).
        let (_, cached_now, notify_now) = read_state();
        if cached_now == latest
            && notify_now != 0
            && now_epoch().saturating_sub(notify_now) < CHECK_INTERVAL_SECS
        {
            return;
        }
        print_notice(&latest);
        write_state(now, &latest, now_epoch());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newer_detection_matches_upgrade_semantics() {
        assert!(is_newer("2.5.0", "2.4.6"));
        assert!(is_newer("2.4.7", "2.4.6"));
        assert!(!is_newer("2.4.6", "2.4.6"));
        assert!(!is_newer("2.4.5", "2.4.6"));
        assert!(!is_newer("1.9.9", "2.0.0"));
        assert!(is_newer("2.4.6.1", "2.4.6"));
    }

    #[test]
    fn shell_hint_names_outer_shell() {
        let hint = shell_hint();
        if cfg!(windows) {
            assert_eq!(hint, "CMD");
        } else {
            assert!(hint.contains("bash"));
        }
    }

    #[test]
    fn notice_template_fills_all_slots() {
        let msg = crate::t!(
            "msg.update.available",
            "v9.9.9",
            "v2.4.6",
            "bash (outside NTC shell)",
            RELEASES_URL
        );
        assert!(msg.contains("v9.9.9"));
        assert!(msg.contains("v2.4.6"));
        assert!(msg.contains("ntc --upgrade"));
        assert!(msg.contains(RELEASES_URL));
        assert!(!msg.contains("{}"));
    }
}
