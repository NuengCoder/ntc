//! Network diagnostics: `ping` and `trace`.
//!
//! Both commands delegate to the platform tools (`ping`, `tracert`,
//! `traceroute`, `tracepath`) so they need no privileges and keep the output
//! users already know. Everything that decides *which* arguments are passed is
//! a pure function (`ping_argv`, `trace_argv`, `fallback_ping_argv`) so the
//! per-OS mapping is unit-tested without spawning anything.

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::t;
use anyhow::Result;
use colored::*;
use std::time::Duration;

/// Which platform's flag spelling to use. Passed explicitly so tests can
/// cover both mappings on any host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // one variant is only reachable on the other platform
pub(crate) enum Os {
    Windows,
    Unix,
}

impl Os {
    fn native() -> Self {
        #[cfg(windows)]
        {
            Os::Windows
        }
        #[cfg(not(windows))]
        {
            Os::Unix
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Family {
    V4,
    V6,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct PingOptions {
    pub count: Option<u32>,
    pub interval_ms: Option<u32>,
    pub timeout_ms: Option<u32>,
    pub size: Option<usize>,
    pub ttl: Option<u8>,
    pub family: Option<Family>,
    pub forever: bool,
    pub quiet: bool,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TraceOptions {
    pub max_hops: Option<u8>,
    pub timeout_ms: Option<u32>,
    pub queries: Option<u8>,
    pub numeric: bool,
    pub family: Option<Family>,
    pub no_fallback: bool,
}

/// Program + argv for one ping run.
pub(crate) fn ping_argv(os: Os, opts: &PingOptions, host: &str) -> (String, Vec<String>) {
    let mut args: Vec<String> = Vec::new();
    match os {
        Os::Windows => {
            match opts.family {
                Some(Family::V4) => args.push("-4".into()),
                Some(Family::V6) => args.push("-6".into()),
                None => {}
            }
            if opts.forever {
                args.push("-t".into());
            } else {
                let count = opts.count.unwrap_or(4);
                args.push("-n".into());
                args.push(count.to_string());
            }
            if let Some(ms) = opts.timeout_ms {
                args.push("-w".into());
                args.push(ms.to_string());
            }
            if let Some(bytes) = opts.size {
                args.push("-l".into());
                args.push(bytes.to_string());
            }
            if let Some(ttl) = opts.ttl {
                args.push("-i".into());
                args.push(ttl.to_string());
            }
        }
        Os::Unix => {
            match opts.family {
                Some(Family::V4) => args.push("-4".into()),
                Some(Family::V6) => args.push("-6".into()),
                None => {}
            }
            match opts.count {
                Some(n) if !opts.forever => {
                    args.push("-c".into());
                    args.push(n.to_string());
                }
                _ if !opts.forever => {
                    args.push("-c".into());
                    args.push("4".into());
                }
                // `--forever`: no `-c`, ping runs until Ctrl+C.
                _ => {}
            }
            if let Some(ms) = opts.interval_ms {
                // `-i` takes whole seconds on both Linux and BSD.
                args.push("-i".into());
                args.push(((ms as f64) / 1000.0).max(0.2).to_string());
            }
            if let Some(ms) = opts.timeout_ms {
                // `-W` is seconds (fractional on Linux).
                args.push("-W".into());
                args.push(format!("{:.3}", (ms as f64) / 1000.0));
            }
            if let Some(bytes) = opts.size {
                args.push("-s".into());
                args.push(bytes.to_string());
            }
            if let Some(ttl) = opts.ttl {
                args.push("-m".into());
                args.push(ttl.to_string());
            }
            if opts.quiet {
                args.push("-q".into());
            }
        }
    }
    args.push(host.to_string());
    ("ping".to_string(), args)
}

/// Flags that this platform's `ping` cannot express, so the caller warns
/// instead of silently ignoring them.
pub(crate) fn ping_unsupported(os: Os, opts: &PingOptions) -> Vec<&'static str> {
    let mut out = Vec::new();
    if os == Os::Windows {
        if opts.interval_ms.is_some() {
            out.push("--interval");
        }
        if opts.quiet {
            out.push("--quiet");
        }
    }
    out
}

/// Program + argv for one trace run against an available tool.
pub(crate) fn trace_argv(
    os: Os,
    prog: &str,
    opts: &TraceOptions,
    host: &str,
) -> (String, Vec<String>) {
    let mut args: Vec<String> = Vec::new();
    let win = os == Os::Windows;
    if win {
        if opts.numeric {
            args.push("-d".into());
        }
        if let Some(n) = opts.max_hops {
            args.push("-h".into());
            args.push(n.to_string());
        }
        if let Some(ms) = opts.timeout_ms {
            args.push("-w".into());
            args.push(ms.to_string());
        }
        match opts.family {
            Some(Family::V4) => args.push("-4".into()),
            Some(Family::V6) => args.push("-6".into()),
            None => {}
        }
    } else {
        // `tracepath` only knows the basic switches; everything else is
        // ignored there by design.
        let tracepath = prog.ends_with("tracepath");
        if opts.numeric || tracepath {
            args.push("-n".into());
        }
        if !tracepath {
            if let Some(n) = opts.max_hops {
                args.push("-m".into());
                args.push(n.to_string());
            }
            if let Some(ms) = opts.timeout_ms {
                args.push("-w".into());
                args.push(format!("{:.1}", (ms as f64) / 1000.0));
            }
            if let Some(q) = opts.queries {
                args.push("-q".into());
                args.push(q.to_string());
            }
        }
    }
    args.push(host.to_string());
    (prog.to_string(), args)
}

/// One-shot ping used by the TTL-walk fallback: TTL = hop, one packet.
pub(crate) fn fallback_ping_argv(os: Os, hop: u8, host: &str) -> (String, Vec<String>) {
    let mut args: Vec<String> = Vec::new();
    match os {
        Os::Windows => {
            args.push("-n".into());
            args.push("1".into());
            args.push("-w".into());
            args.push("1000".into());
            args.push("-i".into());
            args.push(hop.to_string());
        }
        Os::Unix => {
            args.push("-c".into());
            args.push("1".into());
            args.push("-W".into());
            args.push("1".into());
            args.push("-m".into());
            args.push(hop.to_string());
        }
    }
    args.push(host.to_string());
    ("ping".to_string(), args)
}

// ── argument parsing ────────────────────────────────────────────────────────

fn parse_u<T: std::str::FromStr>(value: &str) -> Option<T> {
    value.trim().parse::<T>().ok()
}

fn parse_ping_args(args: &str) -> Result<(PingOptions, String), String> {
    let mut opts = PingOptions::default();
    let tokens: Vec<String> = args.split_whitespace().map(str::to_string).collect();
    let mut host = String::new();
    let mut i = 0;
    let need = |i: usize, flag: &str| -> Result<String, String> {
        tokens
            .get(i + 1)
            .cloned()
            .ok_or_else(|| t!("netdiag.missing_value", flag))
    };
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-c" | "--count" => {
                let v = need(i, "--count")?;
                opts.count = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "--forever" | "-t0" => opts.forever = true,
            "-i" | "--interval" => {
                let v = need(i, "--interval")?;
                opts.interval_ms = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-w" | "--timeout" => {
                let v = need(i, "--timeout")?;
                opts.timeout_ms = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-s" | "--size" => {
                let v = need(i, "--size")?;
                opts.size = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-t" | "--ttl" => {
                let v = need(i, "--ttl")?;
                opts.ttl = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-4" => opts.family = Some(Family::V4),
            "-6" => opts.family = Some(Family::V6),
            "-q" | "--quiet" => opts.quiet = true,
            "--" => break,
            other if other.starts_with('-') && other.len() > 1 => {
                return Err(t!("netdiag.unknown_option", other));
            }
            other => {
                if !host.is_empty() {
                    return Err(t!("netdiag.unexpected_arg", other));
                }
                host = other.to_string();
            }
        }
        i += 1;
    }
    if host.is_empty() {
        return Err(t!("netdiag.host_required"));
    }
    if opts.forever && opts.count.is_some() {
        return Err(t!("netdiag.forever_or_count"));
    }
    Ok((opts, host))
}

fn parse_trace_args(args: &str) -> Result<(TraceOptions, String), String> {
    let mut opts = TraceOptions::default();
    let tokens: Vec<String> = args.split_whitespace().map(str::to_string).collect();
    let mut host = String::new();
    let mut i = 0;
    let need = |i: usize, flag: &str| -> Result<String, String> {
        tokens
            .get(i + 1)
            .cloned()
            .ok_or_else(|| t!("netdiag.missing_value", flag))
    };
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-m" | "--max-hops" => {
                let v = need(i, "--max-hops")?;
                opts.max_hops = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-w" | "--timeout" => {
                let v = need(i, "--timeout")?;
                opts.timeout_ms = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-q" | "--queries" => {
                let v = need(i, "--queries")?;
                opts.queries = Some(parse_u(&v).ok_or_else(|| t!("netdiag.invalid_value", v))?);
                i += 1;
            }
            "-n" | "--numeric" => opts.numeric = true,
            "-4" => opts.family = Some(Family::V4),
            "-6" => opts.family = Some(Family::V6),
            "--no-fallback" => opts.no_fallback = true,
            "--" => break,
            other if other.starts_with('-') && other.len() > 1 => {
                return Err(t!("netdiag.unknown_option", other));
            }
            other => {
                if !host.is_empty() {
                    return Err(t!("netdiag.unexpected_arg", other));
                }
                host = other.to_string();
            }
        }
        i += 1;
    }
    if host.is_empty() {
        return Err(t!("netdiag.host_required"));
    }
    Ok((opts, host))
}

// ── execution ───────────────────────────────────────────────────────────────

/// Spawn with inherited stdio and return the exit code.
fn run_tool(prog: &str, args: &[String]) -> Result<i32> {
    let status = std::process::Command::new(prog)
        .args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("{}", t!("netdiag.spawn_failed", prog, e)))?;
    Ok(status.code().unwrap_or(0))
}

pub fn cmd_ping(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() || matches!(tokens[0], "help" | "--help" | "-h") {
        print_ping_help();
        return Ok(false);
    }
    let (opts, host) = match parse_ping_args(args) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let os = Os::native();
    for flag in ping_unsupported(os, &opts) {
        print_warning(&t!("netdiag.flag_unsupported", flag));
    }
    let (prog, argv) = ping_argv(os, &opts, &host);
    match run_tool(&prog, &argv) {
        Ok(code) if code == 0 || code == 1 => {}
        Ok(code) => print_warning(&t!("dl.ping_exit_code", code)),
        Err(e) => print_error(&e.to_string()),
    }
    Ok(false)
}

pub fn cmd_trace(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() || matches!(tokens[0], "help" | "--help" | "-h") {
        print_trace_help();
        return Ok(false);
    }
    let (opts, host) = match parse_trace_args(args) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let os = Os::native();
    // Prefer the platform's traceroute; `tracepath` is a fallback when the
    // full tool is not installed.
    let mut program = None;
    for candidate in trace_candidates(os) {
        if which::which(candidate).is_ok() {
            program = Some(candidate);
            break;
        }
    }
    if let Some(prog) = program {
        let (prog, argv) = trace_argv(os, prog, &opts, &host);
        match run_tool(&prog, &argv) {
            Ok(_) => return Ok(false),
            Err(e) => {
                print_error(&e.to_string());
                return Ok(false);
            }
        }
    }
    if opts.no_fallback {
        print_error(&t!("netdiag.trace_missing"));
        return Ok(false);
    }
    print_warning(&t!("netdiag.trace_missing"));
    print_info(&t!("netdiag.trace_fallback"));
    trace_ttl_walk(os, &opts, &host);
    Ok(false)
}

fn trace_candidates(os: Os) -> &'static [&'static str] {
    match os {
        Os::Windows => &["tracert"],
        Os::Unix => &["traceroute", "tracepath"],
    }
}

/// TTL walk fallback: one ping per hop, reporting the IP that answers.
/// Locale-independent by construction (only IP literals are read).
fn trace_ttl_walk(os: Os, opts: &TraceOptions, host: &str) {
    let max = opts.max_hops.unwrap_or(30);
    println!("{}", t!("netdiag.trace_title", host, max).cyan().bold());
    let mut last: Option<String> = None;
    for hop in 1..=max {
        let (prog, argv) = fallback_ping_argv(os, hop, host);
        let output = std::process::Command::new(&prog)
            .args(&argv)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output();
        let text = match output {
            Ok(o) => format!(
                "{}\n{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            ),
            Err(e) => {
                print_error(&t!("netdiag.spawn_failed", prog, e));
                return;
            }
        };
        let ip = ttl_reply_ip(&text);
        match ip {
            Some(addr) => {
                println!("  {:>2}  {}", hop, addr.green());
                // Same address twice in a row means the path converged.
                if last.as_deref() == Some(addr.as_str()) || addr == host {
                    break;
                }
                last = Some(addr);
            }
            None => println!("  {:>2}  {}", hop, "*".dimmed()),
        }
        // Give the network a breath between hops.
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// First IP literal on a TTL-expired line. Only lines that mention "ttl" are
/// considered, so the header (which contains the destination address on
/// Windows) can never be mistaken for a hop.
fn ttl_reply_ip(text: &str) -> Option<String> {
    for line in text.lines() {
        // Windows says "TTL expired in transit", Linux/macOS "Time to live
        // exceeded"; both keep the marker language-independent enough to find
        // the reply that carries the hop address.
        let low = line.to_lowercase();
        if !(low.contains("ttl") || low.contains("time to live")) {
            continue;
        }
        for word in line.split(|c: char| !(c.is_ascii_alphanumeric() || ".:[]".contains(c))) {
            let candidate = word.trim_matches(|c: char| "[](),".contains(c));
            // `10.0.0.1:` (a reply followed by a colon) keeps the colon in the
            // token because IPv6 addresses need it; strip one trailing one.
            let candidate = candidate.strip_suffix(':').unwrap_or(candidate);
            if candidate.parse::<std::net::IpAddr>().is_ok() {
                return Some(candidate.to_string());
            }
        }
    }
    None
}

fn print_ping_help() {
    println!("{}", t!("ping.help.title").bold());
    println!();
    println!("{}", t!("ping.help.usage").bold());
    println!("{}", t!("ping.help.usage_line"));
    println!();
    println!("{}", t!("ping.help.options").bold());
    println!("  {}", t!("ping.help.opt_count"));
    println!("  {}", t!("ping.help.opt_forever"));
    println!("  {}", t!("ping.help.opt_interval"));
    println!("  {}", t!("ping.help.opt_timeout"));
    println!("  {}", t!("ping.help.opt_size"));
    println!("  {}", t!("ping.help.opt_ttl"));
    println!("  {}", t!("ping.help.opt_family"));
    println!("  {}", t!("ping.help.opt_quiet"));
    println!("  {}", t!("ping.help.opt_help"));
    println!();
    println!("{}", t!("ping.help.examples").bold());
    println!("  ping example.com");
    println!("  ping 127.0.0.1 -c 10");
    println!("  ping example.com --size 64 -w 2000");
}

fn print_trace_help() {
    println!("{}", t!("trace.help.title").bold());
    println!();
    println!("{}", t!("trace.help.usage").bold());
    println!("{}", t!("trace.help.usage_line"));
    println!();
    println!("{}", t!("trace.help.options").bold());
    println!("  {}", t!("trace.help.opt_max_hops"));
    println!("  {}", t!("trace.help.opt_timeout"));
    println!("  {}", t!("trace.help.opt_queries"));
    println!("  {}", t!("trace.help.opt_numeric"));
    println!("  {}", t!("trace.help.opt_family"));
    println!("  {}", t!("trace.help.opt_no_fallback"));
    println!("  {}", t!("trace.help.opt_help"));
    println!();
    println!("{}", t!("trace.help.examples").bold());
    println!("  trace example.com");
    println!("  trace 8.8.8.8 -m 15");
    println!("  trace example.com --no-fallback");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_argv_maps_every_flag_on_windows() {
        let opts = PingOptions {
            count: Some(3),
            interval_ms: Some(500),
            timeout_ms: Some(2000),
            size: Some(64),
            ttl: Some(7),
            family: Some(Family::V4),
            forever: false,
            quiet: true,
        };
        let (prog, args) = ping_argv(Os::Windows, &opts, "example.com");
        assert_eq!(prog, "ping");
        assert_eq!(
            args,
            vec![
                "-4",
                "-n",
                "3",
                "-w",
                "2000",
                "-l",
                "64",
                "-i",
                "7",
                "example.com"
            ]
        );
        // Interval and quiet have no Windows spelling: they must warn.
        assert_eq!(
            ping_unsupported(Os::Windows, &opts),
            vec!["--interval", "--quiet"]
        );
    }

    #[test]
    fn ping_argv_maps_every_flag_on_unix() {
        let opts = PingOptions {
            count: Some(3),
            interval_ms: Some(1000),
            timeout_ms: Some(2000),
            size: Some(64),
            ttl: Some(7),
            family: Some(Family::V6),
            forever: false,
            quiet: true,
        };
        let (prog, args) = ping_argv(Os::Unix, &opts, "example.com");
        assert_eq!(prog, "ping");
        assert_eq!(
            args,
            vec![
                "-6",
                "-c",
                "3",
                "-i",
                "1",
                "-W",
                "2.000",
                "-s",
                "64",
                "-m",
                "7",
                "-q",
                "example.com"
            ]
        );
        assert!(ping_unsupported(Os::Unix, &opts).is_empty());
    }

    #[test]
    fn ping_forever_never_passes_a_count() {
        let opts = PingOptions {
            forever: true,
            ..PingOptions::default()
        };
        let (_, args) = ping_argv(Os::Windows, &opts, "h");
        assert_eq!(args, vec!["-t", "h"]);
        let (_, args) = ping_argv(Os::Unix, &opts, "h");
        assert_eq!(args, vec!["h"]);
        // Default count stays 4 when nothing was asked for.
        let (_, args) = ping_argv(Os::Windows, &PingOptions::default(), "h");
        assert_eq!(args, vec!["-n", "4", "h"]);
        let (_, args) = ping_argv(Os::Unix, &PingOptions::default(), "h");
        assert_eq!(args, vec!["-c", "4", "h"]);
    }

    #[test]
    fn trace_argv_per_tool() {
        let opts = TraceOptions {
            max_hops: Some(15),
            timeout_ms: Some(1500),
            queries: Some(2),
            numeric: true,
            family: Some(Family::V4),
            no_fallback: false,
        };
        let (prog, args) = trace_argv(Os::Windows, "tracert", &opts, "example.com");
        assert_eq!(prog, "tracert");
        assert_eq!(
            args,
            vec!["-d", "-h", "15", "-w", "1500", "-4", "example.com"]
        );

        let (_, args) = trace_argv(Os::Unix, "traceroute", &opts, "example.com");
        assert_eq!(
            args,
            vec!["-n", "-m", "15", "-w", "1.5", "-q", "2", "example.com"]
        );

        // tracepath only understands the basics.
        let (_, args) = trace_argv(Os::Unix, "tracepath", &opts, "example.com");
        assert_eq!(args, vec!["-n", "example.com"]);
    }

    #[test]
    fn fallback_ping_uses_the_ttl_for_one_packet() {
        let (prog, args) = fallback_ping_argv(Os::Windows, 5, "example.com");
        assert_eq!(prog, "ping");
        assert_eq!(
            args,
            vec!["-n", "1", "-w", "1000", "-i", "5", "example.com"]
        );
        let (_, args) = fallback_ping_argv(Os::Unix, 5, "example.com");
        assert_eq!(args, vec!["-c", "1", "-W", "1", "-m", "5", "example.com"]);
    }

    #[test]
    fn ping_parser_rejects_bad_input() {
        assert!(parse_ping_args("").is_err());
        assert!(parse_ping_args("--count").is_err());
        assert!(parse_ping_args("--count x host").is_err());
        assert!(parse_ping_args("--forever -c 3 host").is_err());
        assert!(parse_ping_args("--bogus host").is_err());
        assert!(parse_ping_args("a b").is_err());
        let (opts, host) = parse_ping_args("-c 2 -4 host").unwrap();
        assert_eq!(opts.count, Some(2));
        assert_eq!(opts.family, Some(Family::V4));
        assert_eq!(host, "host");
    }

    #[test]
    fn trace_parser_rejects_bad_input() {
        assert!(parse_trace_args("").is_err());
        assert!(parse_trace_args("--max-hops").is_err());
        assert!(parse_trace_args("--queries x host").is_err());
        let (opts, host) = parse_trace_args("-m 9 -n --no-fallback host").unwrap();
        assert_eq!(opts.max_hops, Some(9));
        assert!(opts.numeric);
        assert!(opts.no_fallback);
        assert_eq!(host, "host");
    }

    #[test]
    fn ttl_reply_reads_only_expiry_lines() {
        // Windows: the header carries the destination, the reply carries the hop.
        let win = "Pinging example.com [93.184.216.34] with 32 bytes of data:\r\nReply from 10.0.0.1: TTL expired in transit.\r\n";
        assert_eq!(ttl_reply_ip(win).as_deref(), Some("10.0.0.1"));
        // Linux: "From <ip> ... Time to live exceeded".
        let unix = "From 192.168.1.1 icmp_seq=1 Time to live exceeded\n";
        assert_eq!(ttl_reply_ip(unix).as_deref(), Some("192.168.1.1"));
        // Nothing to read: no TTL line at all.
        assert_eq!(ttl_reply_ip("Reply from 93.184.216.34: time=12ms"), None);
        // IPv6 replies are recognised too.
        let v6 = "From 2001:db8::1 icmp_seq=1 Time to live exceeded\n";
        assert_eq!(ttl_reply_ip(v6).as_deref(), Some("2001:db8::1"));
    }
}
