//! `scan pid`: list processes with their PIDs.
//!
//! The optional target defaults to `localhost` (also accepted as
//! `host:localhost`). A bare token that matches a saved `nssh` profile uses
//! that profile's host/user/port/key; `[user@]host[:port]` is always treated
//! as a raw target. This machine is read through `sysinfo`; any other host
//! (or any profile) is reached over SSH and queried with POSIX/toybox `ps`
//! on Unix/Android or PowerShell on Windows.
//! Authentication reuses the `nssh` transport: keys, `--pass-env` (deadlock
//! vault or environment variable), then an interactive password prompt.
//!
//! ```text
//! scan pid
//! scan pid prod
//! scan pid --sort name --filter chrome --min-memory 100MB --limit 10
//! scan pid deploy@example.com:2222 --key ~/.ssh/id_ed25519 --trust
//! scan pid 192.0.2.10 --user root --pass-env PROD_PASS
//! ```

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::commands::builtin::nssh::auth as ssh_auth;
use crate::shell::commands::builtin::nssh::nssh::ssh as transport;
use crate::shell::commands::builtin::nssh::nssh::store;
use crate::t;
use crate::utils::explorer::{human_readable_size, parse_size_value};
use crate::utils::theme::style::ThemeColorize;
use anyhow::{anyhow, Result};
use std::io::IsTerminal;
use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;
use sysinfo::{Networks, System};

/// One process row: PID, name, memory in bytes.
type Row = (u32, String, u64);

/// Maximum total SSH authentication attempts (env password + prompted retries).
const MAX_AUTH_ATTEMPTS: u32 = 3;

/// POSIX process-table queries, tried in order (procps then BSD/macOS).
const REMOTE_PS_COMMANDS: &[&str] = &[
    "ps -A -o pid= -o rss= -o comm=",
    "ps -eo pid=,rss=,comm=",
    // Toybox (Android/Termux): RSS is printed in KiB, like procps.
    "ps -A -o PID,RSS,NAME",
];

/// PowerShell process-table queries for Windows OpenSSH hosts. The script is
/// UTF-16LE/base64 encoded because `-EncodedCommand` survives both cmd.exe and
/// PowerShell default shells without quoting differences.
pub(super) fn powershell_commands() -> Vec<String> {
    let script = "try { [Console]::OutputEncoding = [Text.Encoding]::UTF8 } catch {}; \
         Get-Process | ForEach-Object { '{0} {1} {2}' -f $_.Id, $_.WorkingSet64, $_.ProcessName }";
    let utf16: Vec<u8> = script.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, utf16);
    ["powershell", "pwsh"]
        .into_iter()
        .map(|exe| format!("{exe} -NoProfile -NonInteractive -EncodedCommand {encoded}"))
        .collect()
}

/// Commands tried when reading a remote process table, with the scale that
/// converts their RSS field to bytes (POSIX `ps` uses KiB, PowerShell bytes).
pub(super) fn remote_commands() -> Vec<(String, u64)> {
    let mut commands: Vec<(String, u64)> = REMOTE_PS_COMMANDS
        .iter()
        .map(|command| ((*command).to_string(), 1024))
        .collect();
    commands.extend(
        powershell_commands()
            .into_iter()
            .map(|command| (command, 1)),
    );
    commands
}

/// Ordering for `scan pid --sort`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PidSort {
    Memory,
    Pid,
    Name,
}

/// Parsed `scan pid` target and output options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PidTarget {
    pub host: String,
    pub user: Option<String>,
    pub port: u16,
    pub key: Option<String>,
    pub pass_env: Option<String>,
    pub trust: bool,
    pub timeout: Duration,
    /// Resolved `nssh` profile name, when the target was a bare token that
    /// matched one. Profile targets always use SSH, even for localhost.
    pub profile: Option<String>,
    pub sort: PidSort,
    pub desc: bool,
    pub filter: Option<String>,
    /// Minimum resident memory in bytes, applied with `filter`.
    pub min_memory: Option<u64>,
    /// Maximum resident memory in bytes (inclusive), applied with `filter`.
    pub max_memory: Option<u64>,
    /// Rows to skip after filtering/sorting (for paging).
    pub offset: usize,
    /// Cap on rows shown, applied after filtering and sorting.
    pub limit: Option<usize>,
}

impl Default for PidTarget {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            user: None,
            port: 22,
            key: None,
            pass_env: None,
            trust: false,
            timeout: Duration::from_secs(15),
            profile: None,
            sort: PidSort::Memory,
            desc: false,
            filter: None,
            min_memory: None,
            max_memory: None,
            offset: 0,
            limit: None,
        }
    }
}

fn parse_sort(value: &str) -> Result<PidSort, String> {
    match value.to_ascii_lowercase().as_str() {
        "memory" | "mem" | "size" | "rss" => Ok(PidSort::Memory),
        "pid" => Ok(PidSort::Pid),
        "name" | "cmd" | "command" => Ok(PidSort::Name),
        _ => Err(t!("cmd.scan.pid_bad_sort", value)),
    }
}

fn parse_port(value: &str) -> Result<u16, String> {
    value
        .parse::<u16>()
        .ok()
        .filter(|port| *port != 0)
        .ok_or_else(|| t!("cmd.scan.pid_bad_port", value))
}

/// Split `[user@]host[:port]`, accepting `[ipv6]` brackets when a port is
/// supplied. `None` user/port means "not specified".
fn parse_target(raw: &str) -> Result<(Option<String>, String, Option<u16>), String> {
    if raw.trim().is_empty() {
        return Err(t!("cmd.scan.pid_bad_host", raw));
    }
    let (user, host_part) = match raw.rsplit_once('@') {
        Some((user, host)) => {
            if user.is_empty() || host.is_empty() {
                return Err(t!("cmd.scan.pid_bad_host", raw));
            }
            (Some(user.to_string()), host)
        }
        None => (None, raw),
    };

    let (host, port) = if let Some(rest) = host_part.strip_prefix('[') {
        let (host, tail) = rest
            .split_once(']')
            .ok_or_else(|| t!("cmd.scan.pid_bad_host", raw))?;
        if host.is_empty() {
            return Err(t!("cmd.scan.pid_bad_host", raw));
        }
        let port = if tail.is_empty() {
            None
        } else if let Some(value) = tail.strip_prefix(':') {
            Some(parse_port(value)?)
        } else {
            return Err(t!("cmd.scan.pid_bad_host", raw));
        };
        (host.to_string(), port)
    } else if host_part.matches(':').count() == 1 {
        let (host, value) = host_part.split_once(':').expect("one colon");
        if host.is_empty() || value.is_empty() {
            return Err(t!("cmd.scan.pid_bad_host", raw));
        }
        (host.to_string(), Some(parse_port(value)?))
    } else {
        // No colon (hostname/IPv4) or several (bare IPv6 literal).
        (host_part.to_string(), None)
    };

    Ok((user, host, port))
}

/// Parse the tokens after `scan pid`: at most one `[user@]host[:port]` plus
/// SSH options. `host:<value>` is retained from the local-only syntax.
pub(super) fn parse_pid_target(tokens: &[String]) -> Result<PidTarget, String> {
    let mut target = PidTarget::default();
    let mut raw_host: Option<String> = None;
    let mut port_explicit = false;
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i].as_str();
        let next = |name: &str| -> Result<String, String> {
            tokens
                .get(i + 1)
                .cloned()
                .ok_or_else(|| t!("cmd.scan.pid_missing", name))
        };
        match token {
            "--trust" => {
                target.trust = true;
                i += 1;
            }
            "--user" | "-u" => {
                let value = next(token)?;
                if value.trim().is_empty() {
                    return Err(t!("cmd.scan.pid_bad_user"));
                }
                target.user = Some(value);
                i += 2;
            }
            "--port" | "-p" => {
                let value = next(token)?;
                target.port = parse_port(&value)?;
                port_explicit = true;
                i += 2;
            }
            "--key" => {
                target.key = Some(next(token)?);
                i += 2;
            }
            "--pass-env" => {
                target.pass_env = Some(next(token)?);
                i += 2;
            }
            "--timeout" => {
                let value = next(token)?;
                let secs = value
                    .parse::<u64>()
                    .ok()
                    .filter(|secs| *secs > 0)
                    .ok_or_else(|| t!("cmd.scan.pid_bad_timeout", value))?;
                target.timeout = Duration::from_secs(secs);
                i += 2;
            }
            "--sort" => {
                target.sort = parse_sort(&next(token)?)?;
                i += 2;
            }
            "--desc" => {
                target.desc = true;
                i += 1;
            }
            "--filter" => {
                let value = next(token)?;
                if value.trim().is_empty() {
                    return Err(t!("cmd.scan.pid_bad_filter"));
                }
                target.filter = Some(value);
                i += 2;
            }
            "--min-memory" | "--min-mem" => {
                let value = next(token)?;
                let bytes = parse_size_value(&value)
                    .ok_or_else(|| t!("cmd.scan.pid_bad_min_memory", value))?;
                target.min_memory = Some(bytes);
                i += 2;
            }
            "--max-memory" | "--max-mem" => {
                let value = next(token)?;
                let bytes = parse_size_value(&value)
                    .ok_or_else(|| t!("cmd.scan.pid_bad_max_memory", value))?;
                target.max_memory = Some(bytes);
                i += 2;
            }
            "--offset" => {
                let value = next(token)?;
                target.offset = value
                    .parse::<usize>()
                    .map_err(|_| t!("cmd.scan.pid_bad_offset", value))?;
                i += 2;
            }
            "--limit" | "--head" | "-l" => {
                let value = next(token)?;
                let limit = value
                    .parse::<usize>()
                    .ok()
                    .filter(|limit| *limit > 0)
                    .ok_or_else(|| t!("cmd.scan.pid_bad_limit", value))?;
                target.limit = Some(limit);
                i += 2;
            }
            _ if token.starts_with('-') => {
                return Err(t!("cmd.scan.unknown_option", token));
            }
            _ => {
                if raw_host.is_some() {
                    return Err(t!("cmd.scan.unexpected_arg", token));
                }
                raw_host = Some(token.to_string());
                i += 1;
            }
        }
    }

    if let (Some(min), Some(max)) = (target.min_memory, target.max_memory) {
        if min > max {
            return Err(t!("cmd.scan.pid_bad_range"));
        }
    }

    if let Some(raw) = raw_host {
        // `host:<value>` compatibility, but keep `host:2222` as a port on
        // the host named "host". A `host:` target is always raw (never a
        // profile name).
        let (raw, forced_raw) = match raw.strip_prefix("host:") {
            Some(rest) if !rest.chars().all(|c| c.is_ascii_digit()) => (rest, true),
            _ => (raw.as_str(), false),
        };
        let (raw_user, host, raw_port) = parse_target(raw)?;

        // Only a bare token can name a profile; `user@host` and `host:port`
        // are explicit raw targets. CLI flags override profile fields.
        if !forced_raw && raw_user.is_none() && raw_port.is_none() {
            let profile =
                store::get_profile(&host).map_err(|e| t!("cmd.scan.pid_profile_error", e))?;
            if let Some(profile) = profile {
                if profile.host.trim().is_empty() || profile.port == 0 {
                    return Err(t!("cmd.scan.pid_profile_bad", host));
                }
                if target.user.is_none() {
                    target.user = Some(profile.user);
                }
                if !port_explicit {
                    target.port = profile.port;
                }
                if target.key.is_none() {
                    target.key = profile.key;
                }
                if target.pass_env.is_none() {
                    target.pass_env = profile.pass_env;
                }
                target.host = profile.host;
                target.profile = Some(host);
                return Ok(target);
            }
        }

        target.host = host;
        if target.user.is_none() {
            target.user = raw_user;
        }
        if let Some(port) = raw_port {
            target.port = port;
        }
    }
    Ok(target)
}

fn local_ips() -> Vec<IpAddr> {
    let networks = Networks::new_with_refreshed_list();
    let mut ips = Vec::new();
    for (_name, data) in &networks {
        for net in data.ip_networks() {
            ips.push(net.addr);
        }
    }
    ips
}

/// True when `host` names this machine: localhost, its hostname, a loopback
/// address, or one of the local interface addresses.
pub(super) fn is_local_host(host: &str) -> bool {
    let name = host.trim().trim_matches(['[', ']']);
    if name.is_empty() {
        return true;
    }
    let lower = name.to_ascii_lowercase();
    if matches!(lower.as_str(), "localhost" | "127.0.0.1" | "::1") {
        return true;
    }
    if System::host_name().is_some_and(|h| h.eq_ignore_ascii_case(name)) {
        return true;
    }
    let Ok(addrs) = (name, 0u16).to_socket_addrs() else {
        return false;
    };
    let locals = local_ips();
    addrs.into_iter().any(|addr| {
        let ip = addr.ip();
        ip.is_loopback() || locals.iter().any(|local| *local == ip)
    })
}

fn truncate_name(name: &str, max: usize) -> String {
    if name.chars().count() <= max {
        return name.to_string();
    }
    let mut out: String = name.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn local_rows() -> Vec<Row> {
    let sys = System::new_all();
    sys.processes()
        .iter()
        .map(|(pid, proc_info)| {
            (
                pid.as_u32(),
                proc_info.name().to_string_lossy().into_owned(),
                proc_info.memory(),
            )
        })
        .collect()
}

/// Parse `pid rss name...` rows, scaling the RSS field to bytes. Header lines
/// and unparsable lines are skipped.
pub(super) fn parse_process_rows(stdout: &str, rss_scale: u64) -> Vec<Row> {
    stdout
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let pid = fields.next()?.parse::<u32>().ok()?;
            let rss = fields.next()?.parse::<u64>().ok()?;
            let name = fields.collect::<Vec<_>>().join(" ");
            if name.is_empty() {
                return None;
            }
            Some((pid, name, rss.saturating_mul(rss_scale)))
        })
        .collect()
}

fn default_ssh_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "root".to_string())
}

fn ssh_label(user: &str, host: &str, port: u16) -> String {
    let host = if host.contains(':') {
        format!("[{}]", host)
    } else {
        host.to_string()
    };
    if port == 22 {
        format!("{}@{}", user, host)
    } else {
        format!("{}@{}:{}", user, host, port)
    }
}

/// Apply the name filter (case-insensitive substring) and ordering. The
/// default direction is largest-memory-first, then ascending PID/name;
/// `desc` flips the primary key while PID stays the stable tie-breaker.
pub(super) fn filter_and_sort_rows(
    mut rows: Vec<Row>,
    filter: Option<&str>,
    min_memory: Option<u64>,
    max_memory: Option<u64>,
    sort: PidSort,
    desc: bool,
) -> Vec<Row> {
    if let Some(needle) = filter {
        let needle = needle.to_lowercase();
        rows.retain(|(_, name, _)| name.to_lowercase().contains(&needle));
    }
    if let Some(min) = min_memory {
        rows.retain(|(_, _, memory)| *memory >= min);
    }
    if let Some(max) = max_memory {
        rows.retain(|(_, _, memory)| *memory <= max);
    }
    rows.sort_by(|a, b| {
        let primary = match (sort, desc) {
            (PidSort::Memory, false) => b.2.cmp(&a.2),
            (PidSort::Memory, true) => a.2.cmp(&b.2),
            (PidSort::Pid, false) => a.0.cmp(&b.0),
            (PidSort::Pid, true) => b.0.cmp(&a.0),
            (PidSort::Name, false) => a.1.to_lowercase().cmp(&b.1.to_lowercase()),
            (PidSort::Name, true) => b.1.to_lowercase().cmp(&a.1.to_lowercase()),
        };
        primary.then_with(|| a.0.cmp(&b.0))
    });
    rows
}

/// Human-readable description of the `--min-memory`/`--max-memory` bounds.
fn memory_range_label(min: Option<u64>, max: Option<u64>) -> Option<String> {
    match (min, max) {
        (Some(min), Some(max)) => Some(t!(
            "cmd.scan.pid_range_between",
            human_readable_size(min),
            human_readable_size(max)
        )),
        (Some(min), None) => Some(t!("cmd.scan.pid_range_min", human_readable_size(min))),
        (None, Some(max)) => Some(t!("cmd.scan.pid_range_max", human_readable_size(max))),
        (None, None) => None,
    }
}

fn print_rows(label: &str, remote: bool, target: &PidTarget, rows: Vec<Row>) -> bool {
    let mut rows = filter_and_sort_rows(
        rows,
        target.filter.as_deref(),
        target.min_memory,
        target.max_memory,
        target.sort,
        target.desc,
    );
    let total = rows.len();
    if target.offset > 0 {
        rows = rows.into_iter().skip(target.offset).collect();
    }
    if let Some(limit) = target.limit {
        rows.truncate(limit);
    }
    if rows.is_empty() {
        if total > 0 && target.offset > 0 {
            print_warning(&t!("cmd.scan.pid_offset_empty", target.offset, total));
        } else {
            let range = memory_range_label(target.min_memory, target.max_memory);
            match (target.filter.as_deref(), range) {
                (Some(text), Some(range)) => print_warning(&t!(
                    "cmd.scan.pid_no_match_range_filter",
                    text,
                    range,
                    label
                )),
                (Some(text), None) => print_warning(&t!("cmd.scan.pid_no_match", text, label)),
                (None, Some(range)) => {
                    print_warning(&t!("cmd.scan.pid_no_match_range", range, label))
                }
                (None, None) => print_warning(&t!("cmd.scan.pid_none", label)),
            }
        }
        return false;
    }

    let title = if remote {
        t!("cmd.scan.pid_title_remote", label)
    } else {
        t!("cmd.scan.pid_title", label)
    };
    println!("{}", title.cyan().bold());
    println!("{}", t!("cmd.scan.pid_header").dimmed());
    for (pid, name, memory) in &rows {
        println!(
            "  {:>6}  {:<28}  {:>10}",
            pid,
            truncate_name(name, 28),
            human_readable_size(*memory)
        );
    }
    if rows.len() < total {
        print_success(&t!("cmd.scan.pid_done_limited", rows.len(), total));
    } else {
        print_success(&t!("cmd.scan.pid_done", rows.len()));
    }
    false
}

async fn remote_rows(conn: &transport::Connected, label: &str) -> Result<Vec<Row>> {
    let mut first_error = String::new();
    for (command, rss_scale) in remote_commands() {
        let captured = transport::exec_capture(conn, &command)
            .await
            .map_err(|e| anyhow!(t!("cmd.scan.pid_remote_failed", label, e)))?;
        if captured.status == 0 {
            let rows = parse_process_rows(&captured.stdout, rss_scale);
            if !rows.is_empty() {
                return Ok(rows);
            }
            if first_error.is_empty() {
                first_error = t!("cmd.scan.pid_ps_empty", command);
            }
        } else if first_error.is_empty() {
            let stderr = captured.stderr.trim();
            first_error = if stderr.is_empty() {
                t!("cmd.scan.pid_ps_status", command, captured.status)
            } else {
                t!(
                    "cmd.scan.pid_ps_status_msg",
                    command,
                    captured.status,
                    stderr
                )
            };
        }
    }
    Err(anyhow!(t!("cmd.scan.pid_ps_failed", label, first_error)))
}

fn cmd_remote_pid(target: &PidTarget) -> Result<bool> {
    let user = target.user.clone().unwrap_or_else(default_ssh_user);
    let label = ssh_label(&user, &target.host, target.port);
    print_info(&t!("cmd.scan.pid_connecting", label));

    let profile = store::Profile {
        host: target.host.clone(),
        user: user.clone(),
        port: target.port,
        key: target.key.clone(),
        pass_env: target.pass_env.clone(),
    };
    let auth = match ssh_auth::build_auth(&profile) {
        Ok(auth) => auth,
        Err(e) => {
            print_error(&format!("{}", e));
            return Ok(false);
        }
    };
    let runtime = match ssh_auth::rt() {
        Ok(runtime) => runtime,
        Err(e) => {
            print_error(&format!("{}", e));
            return Ok(false);
        }
    };

    let mut password = auth.password.clone();
    let mut connection = None;
    for attempt in 1..=MAX_AUTH_ATTEMPTS {
        let result = runtime.block_on(transport::connect(
            &target.host,
            target.port,
            &user,
            &transport::SshAuth {
                key_path: auth.key_path.clone(),
                password: password.clone(),
            },
            target.trust,
            target.timeout,
        ));
        match result {
            Ok(conn) => {
                connection = Some(conn);
                break;
            }
            Err(e) => {
                if !ssh_auth::is_auth_failure(&e) || attempt == MAX_AUTH_ATTEMPTS {
                    print_error(&format!("{}", e));
                    return Ok(false);
                }
                if !std::io::stdin().is_terminal() {
                    print_error(&t!("cmd.scan.pid_auth_no_tty", label));
                    return Ok(false);
                }
                match rpassword::prompt_password(t!("cmd.scan.pid_password_prompt", label)) {
                    Ok(entered) if !entered.is_empty() => password = Some(entered),
                    _ => return Ok(false),
                }
            }
        }
    }
    let Some(conn) = connection else {
        return Ok(false);
    };

    let rows = runtime.block_on(remote_rows(&conn, &label));
    let _ = runtime.block_on(conn.handle.disconnect(
        russh::Disconnect::ByApplication,
        "scan pid done",
        "",
    ));
    match rows {
        Ok(rows) => Ok(print_rows(&label, true, target, rows)),
        Err(e) => {
            print_error(&format!("{}", e));
            Ok(false)
        }
    }
}

/// `scan pid [[user@]host[:port]]` — list PID, process name and memory,
/// largest first. Local hosts are read in-process; remote hosts over SSH.
pub(super) fn cmd_scan_pid(tokens: &[String]) -> Result<bool> {
    if tokens
        .first()
        .is_some_and(|first| matches!(first.to_lowercase().as_str(), "help" | "-h" | "--help"))
    {
        println!("{}", t!("cmd.scan.pid_usage"));
        return Ok(false);
    }
    let target = match parse_pid_target(tokens) {
        Ok(target) => target,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    if let Some(profile) = target.profile.as_deref() {
        print_info(&t!("cmd.scan.pid_profile_using", profile));
    }
    if target.profile.is_none() && is_local_host(&target.host) {
        let rows = local_rows();
        Ok(print_rows(&target.host, false, &target, rows))
    } else {
        cmd_remote_pid(&target)
    }
}
