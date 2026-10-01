//! Privilege-free local network discovery for `scan ip` (+ shared
//! sweep helpers). Dependency-free: std + rayon only.
//!
//! - [`local_subnet`] — primary local /24 via the UDP-socket trick
//!   (bind `0.0.0.0:0`, "connect" a public IP, read `local_addr`:
//!   sends nothing, needs no privileges, works cross-platform).
//! - [`parse_cidr`] — `A.B.C.D/N` into a bounded host list.
//! - [`sweep_hosts`] — parallel TCP connect sweep with per-IP results
//!   (unlike `nssh::scan::scan_ports`, which collapses by port).
//! - [`sweep_hosts_async`] — same over tokio with hundreds of
//!   in-flight connects: what makes `--all` (65K ports) feasible.
//!   Progress + streaming callbacks keep big sweeps visibly alive.
//! - [`arp_table`] — OS ARP/neighbor table scrape; unioned with the
//!   sweep so hosts with fully-filtered TCP still show up. Best
//!   effort: fully silent (stealth) hosts are invisible to any
//!   unprivileged scan.

use rayon::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Fast discovery probe set: services commonly listening on LAN hosts
/// (SMB, NetBIOS, web, ssh, dns, rdp, alt-web, mDNS-adjacent).
pub const DISCOVERY_PORTS: &[u16] = &[445, 139, 135, 80, 443, 22, 53, 3389, 8080, 5357];

/// Upper bound on swept hosts per invocation (`/22`); wider ranges
/// must be narrowed by the caller with an explicit error.
pub const MAX_SWEEP_HOSTS: usize = 1024;

/// One live host: its address plus the open ports found by the sweep
/// (empty when found via ARP only).
#[derive(Debug, Clone, PartialEq)]
pub struct LiveHost {
    pub ip: IpAddr,
    pub open: Vec<crate::nssh::scan::ScanHit>,
    pub via_arp: bool,
}

/// Derive the /24 network base for a local IPv4 address.
/// Errors on IPv6, loopback, or unspecified addresses (caller must
/// demand an explicit CIDR then).
pub fn subnet24_of(ip: IpAddr) -> Result<Ipv4Addr, String> {
    match ip {
        IpAddr::V4(v4) => {
            let o = v4.octets();
            if v4.is_loopback() || v4.is_unspecified() || v4.is_multicast() {
                return Err("no usable LAN address (loopback/unspecified)".to_string());
            }
            if o[0] == 169 && o[1] == 254 {
                return Err("link-local address only — pass an explicit CIDR".to_string());
            }
            Ok(Ipv4Addr::new(o[0], o[1], o[2], 0))
        }
        IpAddr::V6(_) => Err("IPv6 local address — pass an explicit IPv4 CIDR".to_string()),
    }
}

/// Detect the primary local /24: UDP bind + sham connect to a public
/// resolver, then read back the local address. Sends no packets.
pub fn local_subnet() -> Result<(Ipv4Addr, u8), String> {
    let sock =
        UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("cannot open socket: {}", e))?;
    sock.connect("8.8.8.8:53")
        .map_err(|e| format!("cannot route: {}", e))?;
    let local = sock
        .local_addr()
        .map_err(|e| format!("cannot read local address: {}", e))?;
    let base = subnet24_of(local.ip())?;
    Ok((base, 24))
}

/// Parse `A.B.C.D/N` into the host list (network + broadcast excluded
/// for N <= 30; /31 and /32 yield their usable addresses as-is).
/// Rejects non-IPv4, bad prefix lengths, and ranges over
/// [`MAX_SWEEP_HOSTS`] hosts.
pub fn parse_cidr(spec: &str) -> Result<Vec<IpAddr>, String> {
    let (addr, prefix) = spec
        .split_once('/')
        .ok_or_else(|| format!("bad CIDR '{}' (want A.B.C.D/N)", spec))?;
    let base: Ipv4Addr = addr
        .trim()
        .parse()
        .map_err(|_| format!("bad CIDR address '{}'", spec))?;
    let n: u32 = prefix
        .trim()
        .parse()
        .map_err(|_| format!("bad CIDR prefix '{}'", spec))?;
    if !(16..=32).contains(&n) {
        return Err(format!("bad CIDR prefix '{}' (want 16-32)", spec));
    }
    let host_bits = 32 - n;
    let count = 1u64 << host_bits;
    if count > MAX_SWEEP_HOSTS as u64 + 2 {
        return Err(format!(
            "CIDR '{}' covers {} hosts (max {} — narrow the range)",
            spec,
            count,
            MAX_SWEEP_HOSTS
        ));
    }
    let base_u = u32::from(base);
    let mask = if n == 0 { 0 } else { u32::MAX << host_bits };
    let net = base_u & mask;
    let mut out = Vec::new();
    if n >= 31 {
        // /31 (point-to-point) and /32 (single host): all addresses usable.
        for i in 0..count {
            out.push(IpAddr::V4(Ipv4Addr::from(net + i as u32)));
        }
    } else {
        // Skip network (.0) and broadcast (.255-style) addresses.
        for i in 1..count - 1 {
            out.push(IpAddr::V4(Ipv4Addr::from(net + i as u32)));
        }
    }
    Ok(out)
}

/// Max concurrent in-flight TCP connects for async sweeps.
/// Rayon's CPU-count parallelism is what makes `--all` take forever
/// (65K ports × 800ms ÷ ~12 threads); 512-wide async I/O fixes it.
pub const SCAN_CONCURRENCY: usize = 512;

/// Ports above this count get the fast default timeout unless the
/// user passed `--timeout` explicitly (full sweeps shouldn't pay the
/// interactive 800ms per filtered port).
pub const FAST_TIMEOUT_ABOVE: usize = 2000;
/// Fast default timeout for huge sweeps (explicit `--timeout` wins).
pub const FAST_TIMEOUT_MS: u64 = 300;

/// Thousands-separated counter for progress lines (`12340` →
/// `"12,340"`). Pure.
pub fn fmt_count(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// Shared progress state for big sweeps: one `\r` line repainted at
/// most every 250ms, so 65K completions don't spam the terminal.
/// Thread-safe; `tick` is called once per finished probe.
///
/// The progress line is scaffolding, not output: hit rows go through
/// [`SweepProgress::print_hit`] (same lock — interleaving is
/// structurally impossible), and the finished line is *erased*, never
/// newline-terminated, so only real results remain on screen.
pub struct SweepProgress {
    label: String,
    total: usize,
    done: AtomicUsize,
    open: AtomicUsize,
    state: Mutex<ProgressPaint>,
}

/// Paint bookkeeping behind the progress lock.
struct ProgressPaint {
    last_print: Instant,
    /// Visible width of the currently displayed progress line.
    last_len: usize,
}

/// Render the progress line text (no carriage return, no padding).
/// Pure for testability.
fn render_progress(label: &str, done: usize, total: usize, open: usize) -> String {
    let pct = done * 100 / total.max(1);
    crate::t!(
        "cmd.scan.progress",
        label,
        fmt_count(done),
        fmt_count(total),
        pct,
        open
    )
}

/// Erase sequence for a progress line of `len` visible chars.
fn erase_seq(len: usize) -> String {
    format!("\r{}\r", " ".repeat(len))
}

impl SweepProgress {
    pub fn new(label: String, total: usize) -> Self {
        Self {
            label,
            total,
            done: AtomicUsize::new(0),
            open: AtomicUsize::new(0),
            state: Mutex::new(ProgressPaint {
                last_print: Instant::now(),
                last_len: 0,
            }),
        }
    }

    /// Record one finished probe (`hit` = port open). Repaints the
    /// progress line throttled; always updates the counters. When the
    /// last probe lands the line is erased — no 100% artifact stays.
    pub fn tick(&self, hit: bool) {
        let done = self.done.fetch_add(1, Ordering::Relaxed) + 1;
        if hit {
            self.open.fetch_add(1, Ordering::Relaxed);
        }
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let finished = done >= self.total;
        if finished {
            if st.last_len > 0 {
                print!("{}", erase_seq(st.last_len));
                use std::io::Write;
                let _ = std::io::stdout().flush();
                st.last_len = 0;
            }
            return;
        }
        if st.last_print.elapsed() < Duration::from_millis(250) {
            return;
        }
        st.last_print = Instant::now();
        let line = render_progress(
            &self.label,
            done,
            self.total,
            self.open.load(Ordering::Relaxed),
        );
        st.last_len = line.chars().count();
        print!("\r{}", line);
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }

    /// Print one hit row during a sweep: erases the progress line,
    /// prints the row, and forces a repaint on the next tick — so a
    /// stale progress snapshot can never fossilize above live output.
    pub fn print_hit(&self, row: String) {
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if st.last_len > 0 {
            print!("{}", erase_seq(st.last_len));
            st.last_len = 0;
        }
        // Backdate so the next tick repaints immediately.
        st.last_print = Instant::now()
            .checked_sub(Duration::from_secs(3600))
            .unwrap_or_else(Instant::now);
        println!("{}", row);
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }
}

/// Parallel TCP connect sweep: every (ip, port) pair probed with
/// `connect_timeout`; results grouped per IP, ports sorted.
/// Hosts with no open ports are omitted (ARP union adds them back).
pub fn sweep_hosts(
    ips: &[IpAddr],
    ports: &[u16],
    timeout: Duration,
) -> Vec<(IpAddr, Vec<crate::nssh::scan::ScanHit>)> {
    use std::collections::BTreeMap;
    let probes: Vec<(IpAddr, u16)> = ips
        .iter()
        .flat_map(|ip| ports.iter().map(|p| (*ip, *p)))
        .collect();
    let hits: Vec<(IpAddr, crate::nssh::scan::ScanHit)> = probes
        .par_iter()
        .filter_map(|(ip, port)| {
            let addr = SocketAddr::new(*ip, *port);
            TcpStream::connect_timeout(&addr, timeout).ok().map(|_| {
                (
                    *ip,
                    crate::nssh::scan::ScanHit {
                        port: *port,
                        service: crate::nssh::scan::service_name(*port).unwrap_or("unknown"),
                    },
                )
            })
        })
        .collect();
    let mut by_ip: BTreeMap<IpAddr, Vec<crate::nssh::scan::ScanHit>> = BTreeMap::new();
    for (ip, hit) in hits {
        by_ip.entry(ip).or_default().push(hit);
    }
    let mut out: Vec<(IpAddr, Vec<crate::nssh::scan::ScanHit>)> = by_ip.into_iter().collect();
    for (_, v) in out.iter_mut() {
        v.sort_by_key(|h| h.port);
        v.dedup_by_key(|h| h.port);
    }
    out
}

/// Async variant of [`sweep_hosts`]: tokio connects capped by a
/// semaphore (`concurrency`, e.g. [`SCAN_CONCURRENCY`]), so tens of
/// thousands of probes finish in minutes instead of hours.
/// `on_hit` fires per open port from the single collector task (safe
/// to print from — no interleaving); `progress.tick(hit)` drives the
/// shared progress line. Results identical in shape to the sync sweep.
pub fn sweep_hosts_async(
    ips: &[IpAddr],
    ports: &[u16],
    timeout: Duration,
    concurrency: usize,
    progress: &SweepProgress,
    on_hit: impl Fn(IpAddr, u16),
) -> Vec<(IpAddr, Vec<crate::nssh::scan::ScanHit>)> {
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    if ips.is_empty() || ports.is_empty() {
        return Vec::new();
    }
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("scan runtime");
    rt.block_on(async {
        let sem = Arc::new(Semaphore::new(concurrency.max(1)));
        let mut set = tokio::task::JoinSet::new();
        for ip in ips {
            for port in ports {
                let permit = sem.clone().acquire_owned().await.expect("semaphore");
                let addr = SocketAddr::new(*ip, *port);
                let (ip, port) = (*ip, *port);
                set.spawn(async move {
                    let _permit = permit;
                    let r = tokio::time::timeout(
                        timeout,
                        tokio::net::TcpStream::connect(addr),
                    )
                    .await;
                    matches!(r, Ok(Ok(_))).then_some((ip, port))
                });
            }
        }
        let mut by_ip: BTreeMap<IpAddr, Vec<crate::nssh::scan::ScanHit>> = BTreeMap::new();
        while let Some(res) = set.join_next().await {
            match res {
                Ok(Some((ip, port))) => {
                    progress.tick(true);
                    on_hit(ip, port);
                    by_ip.entry(ip).or_default().push(crate::nssh::scan::ScanHit {
                        port,
                        service: crate::nssh::scan::service_name(port).unwrap_or("unknown"),
                    });
                }
                _ => {
                    progress.tick(false);
                }
            }
        }
        let mut out: Vec<(IpAddr, Vec<crate::nssh::scan::ScanHit>)> =
            by_ip.into_iter().collect();
        for (_, v) in out.iter_mut() {
            v.sort_by_key(|h| h.port);
            v.dedup_by_key(|h| h.port);
        }
        out
    })
}

/// Scrape the OS ARP/neighbor table for live L2 entries, keeping only
/// addresses within `want` (the swept CIDR). Never fails: any parse
/// or command error yields an empty vec (TCP sweep stands alone).
pub fn arp_table(want: &[IpAddr]) -> Vec<IpAddr> {
    use std::collections::HashSet;
    let raw = arp_table_raw();
    let want_set: HashSet<IpAddr> = want.iter().copied().collect();
    let mut out: Vec<IpAddr> = raw
        .into_iter()
        .filter(|ip| want_set.contains(ip))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Raw neighbor IPs from the OS table (unfiltered). Best effort.
fn arp_table_raw() -> Vec<IpAddr> {
    #[cfg(windows)]
    {
        let out = std::process::Command::new("arp")
            .arg("-a")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default();
        parse_arp_windows(&out)
    }
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("arp")
            .arg("-a")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default();
        parse_arp_bsd(&out)
    }
    #[cfg(all(not(windows), not(target_os = "macos")))]
    {
        // Prefer `ip neigh`, fall back to /proc/net/arp.
        if let Ok(out) = std::process::Command::new("ip")
            .args(["neigh", "show"])
            .output()
        {
            let text = String::from_utf8_lossy(&out.stdout).into_owned();
            let v = parse_ip_neigh(&text);
            if !v.is_empty() {
                return v;
            }
        }
        if let Ok(text) = std::fs::read_to_string("/proc/net/arp") {
            return parse_proc_net_arp(&text);
        }
        Vec::new()
    }
}

/// Windows `arp -a`: `  192.168.1.1           aa-bb-cc-...     dynamic`.
#[cfg(any(test, windows))]
fn parse_arp_windows(text: &str) -> Vec<IpAddr> {
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 3 {
            continue;
        }
        // Dynamic entries only (static = manually configured).
        if !cols[2].eq_ignore_ascii_case("dynamic") {
            continue;
        }
        if let Ok(ip) = cols[0].parse::<IpAddr>() {
            if !is_excluded_ip(&ip) {
                out.push(ip);
            }
        }
    }
    out
}

/// BSD/macOS `arp -a`: `host (192.168.1.1) at aa:bb:.. on en0 ...`.
#[cfg(any(test, target_os = "macos"))]
fn parse_arp_bsd(text: &str) -> Vec<IpAddr> {
    let mut out = Vec::new();
    for line in text.lines() {
        let start = match line.find('(') {
            Some(i) => i + 1,
            None => continue,
        };
        let end = match line[start..].find(')') {
            Some(i) => start + i,
            None => continue,
        };
        // Skip incomplete entries ("(incomplete)").
        if line[start..end].eq_ignore_ascii_case("incomplete") {
            continue;
        }
        if let Ok(ip) = line[start..end].parse::<IpAddr>() {
            if !is_excluded_ip(&ip) {
                out.push(ip);
            }
        }
    }
    out
}

/// Linux `ip neigh show`: `192.168.1.1 dev eth0 lladdr ... REACHABLE`.
/// Only states proving L2 presence count (not FAILED/INCOMPLETE).
#[cfg(any(test, all(not(windows), not(target_os = "macos"))))]
fn parse_ip_neigh(text: &str) -> Vec<IpAddr> {
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.is_empty() {
            continue;
        }
        let state = cols.last().copied().unwrap_or("").to_uppercase();
        if !matches!(
            state.as_str(),
            "REACHABLE" | "STALE" | "DELAY" | "PROBE" | "PERMANENT"
        ) {
            continue;
        }
        if let Ok(ip) = cols[0].parse::<IpAddr>() {
            if !is_excluded_ip(&ip) {
                out.push(ip);
            }
        }
    }
    out
}

/// Linux `/proc/net/arp`: `IP address  HW type  Flags  HW address ...`.
/// Flags field `0x2` = complete entry.
#[cfg(any(test, all(not(windows), not(target_os = "macos"))))]
fn parse_proc_net_arp(text: &str) -> Vec<IpAddr> {
    let mut out = Vec::new();
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 4 {
            continue;
        }
        if cols[2] != "0x2" {
            continue;
        }
        if let Ok(ip) = cols[0].parse::<IpAddr>() {
            if !is_excluded_ip(&ip) {
                out.push(ip);
            }
        }
    }
    out
}

/// Never report multicast/unspecified addresses from neighbor tables.
fn is_excluded_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_multicast() || v4.is_unspecified(),
        IpAddr::V6(v6) => v6.is_multicast() || v6.is_unspecified(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cidr_parses_hosts_and_rejects() {
        let hosts = parse_cidr("192.168.1.0/24").unwrap();
        assert_eq!(hosts.len(), 254);
        assert_eq!(hosts[0].to_string(), "192.168.1.1");
        assert_eq!(hosts[253].to_string(), "192.168.1.254");

        let h30 = parse_cidr("10.0.0.4/30").unwrap();
        assert_eq!(h30.len(), 2);

        let h31 = parse_cidr("10.0.0.0/31").unwrap();
        assert_eq!(h31.len(), 2);

        let h32 = parse_cidr("10.0.0.7/32").unwrap();
        assert_eq!(h32, vec!["10.0.0.7".parse::<IpAddr>().unwrap()]);

        assert!(parse_cidr("192.168.1.0").is_err());
        assert!(parse_cidr("999.1.1.0/24").is_err());
        assert!(parse_cidr("192.168.1.0/15").is_err());
        assert!(parse_cidr("192.168.1.0/33").is_err());
        assert!(parse_cidr("10.0.0.0/16").is_err());
        assert!(parse_cidr("::1/128").is_err());
    }

    #[test]
    fn subnet24_math_and_rejects() {
        let base = subnet24_of("192.168.5.23".parse().unwrap()).unwrap();
        assert_eq!(base, Ipv4Addr::new(192, 168, 5, 0));
        assert!(subnet24_of("127.0.0.1".parse().unwrap()).is_err());
        assert!(subnet24_of("0.0.0.0".parse().unwrap()).is_err());
        assert!(subnet24_of("169.254.9.9".parse().unwrap()).is_err());
        assert!(subnet24_of("::1".parse().unwrap()).is_err());
    }

    #[test]
    fn arp_parsers_on_canned_outputs() {
        let win = "Interface: 192.168.1.20 --- 0x4\r\n  Internet Address      Physical Address      Type\r\n  192.168.1.1           aa-bb-cc-dd-ee-ff     dynamic\r\n  192.168.1.255         ff-ff-ff-ff-ff-ff     static\r\n  224.0.0.22            01-00-5e-00-00-16     static\r\n";
        assert_eq!(
            parse_arp_windows(win),
            vec!["192.168.1.1".parse::<IpAddr>().unwrap()]
        );

        let bsd = "router (192.168.1.1) at aa:bb:cc:dd:ee:ff on en0 ifscope [ethernet]\nphone (incomplete) at (incomplete) on en0\n";
        assert_eq!(
            parse_arp_bsd(bsd),
            vec!["192.168.1.1".parse::<IpAddr>().unwrap()]
        );

        let neigh = "192.168.1.1 dev eth0 lladdr aa:bb:cc:dd:ee:ff REACHABLE\n192.168.1.9 dev eth0  FAILED\nfe80::1 dev eth0 lladdr aa:bb STALE\n";
        assert_eq!(
            parse_ip_neigh(neigh),
            vec![
                "192.168.1.1".parse::<IpAddr>().unwrap(),
                "fe80::1".parse::<IpAddr>().unwrap()
            ]
        );

        let proc = "IP address       HW type     Flags       HW address            Mask     Device\n192.168.1.1      0x1         0x2         aa:bb:cc:dd:ee:ff     *        eth0\n192.168.1.9      0x1         0x0         00:00:00:00:00:00     *        eth0\n";
        assert_eq!(
            parse_proc_net_arp(proc),
            vec!["192.168.1.1".parse::<IpAddr>().unwrap()]
        );
    }

    #[test]
    fn sweep_finds_local_listener_per_ip() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let ips = vec!["127.0.0.1".parse().unwrap()];
        let out = sweep_hosts(&ips, &[port], Duration::from_secs(2));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0.to_string(), "127.0.0.1");
        assert_eq!(out[0].1.len(), 1);
        assert_eq!(out[0].1[0].port, port);
        let _ = listener;
    }

    #[test]
    fn sweep_omits_silent_hosts() {
        let ips = vec!["127.0.0.1".parse().unwrap()];
        let out = sweep_hosts(&ips, &[1], Duration::from_millis(300));
        assert!(out.is_empty());
    }

    #[test]
    fn fmt_count_separates_thousands() {
        assert_eq!(fmt_count(0), "0");
        assert_eq!(fmt_count(999), "999");
        assert_eq!(fmt_count(1000), "1,000");
        assert_eq!(fmt_count(12340), "12,340");
        assert_eq!(fmt_count(65535), "65,535");
        assert_eq!(fmt_count(1234567), "1,234,567");
    }

    #[test]
    fn progress_counts_every_probe() {
        let progress = SweepProgress::new("test".to_string(), 5);
        for hit in [true, false, true, false, false] {
            progress.tick(hit);
        }
        assert_eq!(progress.done.load(Ordering::Relaxed), 5);
        assert_eq!(progress.open.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn progress_line_renders_without_newline() {
        // render_progress is pure text (the \r wrapping lives in tick);
        // erase_seq must exactly cover it so no ghost chars survive.
        let line = render_progress("h", 12340, 65535, 7);
        assert!(!line.contains('\n') && !line.contains('\r'));
        let erased = erase_seq(line.chars().count());
        assert!(erased.starts_with('\r') && erased.ends_with('\r'));
        assert_eq!(erased.chars().count(), line.chars().count() + 2);
        assert!(erased[1..erased.len() - 1].chars().all(|c| c == ' '));
    }

    #[test]
    fn async_sweep_finds_local_listener() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let ips = vec!["127.0.0.1".parse().unwrap()];
        let progress = SweepProgress::new("test".to_string(), 1);
        let out = sweep_hosts_async(&ips, &[port], Duration::from_secs(2), 64, &progress, |_, _| {});
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].1.len(), 1);
        assert_eq!(out[0].1[0].port, port);
        assert_eq!(progress.done.load(Ordering::Relaxed), 1);
        assert_eq!(progress.open.load(Ordering::Relaxed), 1);
        let _ = listener;
    }

    #[test]
    fn async_sweep_silent_on_closed() {
        let ips = vec!["127.0.0.1".parse().unwrap()];
        let progress = SweepProgress::new("test".to_string(), 1);
        let out = sweep_hosts_async(&ips, &[1], Duration::from_millis(300), 64, &progress, |_, _| {});
        assert!(out.is_empty());
        assert_eq!(progress.done.load(Ordering::Relaxed), 1);
        assert_eq!(progress.open.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn async_sweep_empty_inputs_short_circuit() {
        let progress = SweepProgress::new("test".to_string(), 0);
        let ips: Vec<IpAddr> = Vec::new();
        let out = sweep_hosts_async(&ips, &[80], Duration::from_secs(1), 64, &progress, |_, _| {});
        assert!(out.is_empty());
        assert_eq!(
            sweep_hosts_async(
                &["127.0.0.1".parse().unwrap()],
                &[],
                Duration::from_secs(1),
                64,
                &progress,
                |_, _| {}
            )
            .len(),
            0
        );
    }
}
