//! `ipcfg` command: ipconfig/ifconfig-style adapter listing.
//!
//! One consistent layout on every OS: per adapter (name, status, MAC,
//! IPv4 + prefix, IPv6, gateway) plus a global DNS block.
//! Adapter data comes from the `network-interface` crate (locale-proof,
//! unlike parsing `ipconfig`); gateway/DNS fall back to small OS
//! readers that degrade to empty instead of erroring.

use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::t;

use anyhow::Result;
use colored::*;
use std::net::{IpAddr, Ipv4Addr};

/// One adapter row for display.
#[derive(Debug, Clone, PartialEq)]
pub struct IfEntry {
    pub name: String,
    pub index: u32,
    pub up: bool,
    pub mac: Option<String>,
    /// (address, prefix length).
    pub v4: Vec<(Ipv4Addr, u8)>,
    pub v6: Vec<String>,
    pub gateway: Option<String>,
}

/// Prefix length of an IPv4 netmask (`255.255.255.0` -> 24).
/// Non-contiguous masks count their one-bits (best effort).
/// Desktop-only: the Android reader takes prefixes as numbers from
/// `ip -o` / fib_trie text, so nothing calls this there.
#[cfg(any(test, not(target_os = "android")))]
pub fn prefix_len(mask: Ipv4Addr) -> u8 {
    u32::from(mask).count_ones() as u8
}

/// All adapters with addresses resolved: gateways joined by interface
/// name where the OS reader provides it, else the first default route.
///
/// Android has no `getifaddrs` (bionic), so the `network-interface`
/// crate cannot link there — the `target_os = "android"` build below
/// reads the same data from sysfs/procfs instead (same `IfEntry`
/// shape, same loopback-last order).
#[cfg(not(target_os = "android"))]
pub fn list_adapters() -> Vec<IfEntry> {
    use network_interface::{NetworkInterface, NetworkInterfaceConfig};
    let gateways = default_gateways();
    let ifaces = NetworkInterface::show().unwrap_or_default();
    let mut out = Vec::new();
    for iface in ifaces {
        let mut v4 = Vec::new();
        let mut v6 = Vec::new();
        for addr in &iface.addr {
            match addr {
                network_interface::Addr::V4(a) => {
                    let prefix = a.netmask.map(prefix_len).unwrap_or(32);
                    v4.push((a.ip, prefix));
                }
                network_interface::Addr::V6(a) => {
                    v6.push(a.ip.to_string());
                }
            }
        }
        // Link-local v6 without a real address still means "present";
        // an adapter with no addresses at all reads as down.
        let up = !iface.addr.is_empty();
        // Gateway join: exact name match, else the alias segment of
        // "Alias|Index" composites (Windows), else the first route.
        let gateway = gateways
            .iter()
            .find(|(name, _)| name == &iface.name || name.split('|').next() == Some(&iface.name))
            .map(|(_, gw)| gw.clone())
            .or_else(|| gateways.first().map(|(_, gw)| gw.clone()));
        out.push(IfEntry {
            name: iface.name,
            index: iface.index,
            up,
            mac: iface.mac_addr,
            v4,
            v6,
            gateway,
        });
    }
    // Loopback last (physical adapters first, like ipconfig order).
    out.sort_by_key(|e| {
        (
            e.name.to_lowercase().starts_with("lo"),
            e.name.to_lowercase(),
        )
    });
    out
}

/// Android adapter listing without `getifaddrs` (missing from bionic):
/// names/MAC/state from `/sys/class/net`, IPv4 from the `Local:` table
/// of `/proc/net/fib_trie`, IPv6 from `/proc/net/if_inet6`. Anything
/// unreadable degrades to `-`/empty instead of erroring.
#[cfg(target_os = "android")]
pub fn list_adapters() -> Vec<IfEntry> {
    list_adapters_android(
        std::path::Path::new("/sys/class/net"),
        std::path::Path::new("/proc/net/fib_trie"),
        std::path::Path::new("/proc/net/if_inet6"),
    )
}

#[cfg(any(test, target_os = "android"))]
fn list_adapters_android(
    sys_net: &std::path::Path,
    fib_trie: &std::path::Path,
    if_inet6: &std::path::Path,
) -> Vec<IfEntry> {
    let gateways = default_gateways();
    // Best source first: `ip -o addr show` names every address with its
    // interface and prefix. Where `ip` is missing, v6 still resolves
    // per-interface via if_inet6, and v4 falls back to the fib_trie
    // `Local:` table (no interface attribution there, so those
    // addresses join the default-route interface).
    let ip_addrs = parse_ip_addr_o(&ip_addr_show());
    let inet6 = parse_if_inet6(&std::fs::read_to_string(if_inet6).unwrap_or_default());
    let fib_v4 = parse_fib_trie_local(&std::fs::read_to_string(fib_trie).unwrap_or_default());
    let default_iface = gateways.first().map(|(n, _)| n.clone());

    let mut names: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(sys_net) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                if !name.is_empty() {
                    names.push(name);
                }
            }
        }
    }

    let mut out = Vec::new();
    for name in &names {
        let dir = sys_net.join(name);
        let mac = std::fs::read_to_string(dir.join("address"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s != "00:00:00:00:00:00");
        let index = std::fs::read_to_string(dir.join("ifindex"))
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(0);
        let oper_up = std::fs::read_to_string(dir.join("operstate"))
            .ok()
            .map(|s| s.trim().eq_ignore_ascii_case("up"))
            .unwrap_or(false);
        let (v4addrs, v6addrs): (Vec<(Ipv4Addr, u8)>, Vec<String>) =
            if let Some((v4, v6)) = ip_addrs.get(name) {
                (v4.clone(), v6.clone())
            } else {
                let v6 = inet6.get(name).cloned().unwrap_or_default();
                // Unattributed fib_trie addresses belong to the
                // default-route interface (single-homed phones).
                let v4 = if default_iface.as_deref() == Some(name.as_str()) {
                    fib_v4.clone()
                } else {
                    Vec::new()
                };
                (v4, v6)
            };
        let up = oper_up || !v4addrs.is_empty() || !v6addrs.is_empty();
        let gateway = gateways
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, gw)| gw.clone())
            .or_else(|| gateways.first().map(|(_, gw)| gw.clone()));
        out.push(IfEntry {
            name: name.clone(),
            index,
            up,
            mac,
            v4: v4addrs,
            v6: v6addrs,
            gateway,
        });
    }
    // Loopback last (physical adapters first, like ipconfig order).
    out.sort_by_key(|e| {
        (
            e.name.to_lowercase().starts_with("lo"),
            e.name.to_lowercase(),
        )
    });
    out
}

/// `ip -o addr show` output, empty when `ip` is missing. Pure for
/// testability (the command runner is the one-liner below).
#[cfg(target_os = "android")]
fn ip_addr_show() -> String {
    std::process::Command::new("ip")
        .args(["-o", "addr", "show"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

/// Host-test stub: `ip` parsing is covered via `parse_ip_addr_o`;
/// the live command only exists on Android.
#[cfg(all(test, not(target_os = "android")))]
fn ip_addr_show() -> String {
    String::new()
}

/// Per-interface `(v4, v6)` addresses from `ip -o addr show` lines:
/// `2: wlan0    inet 192.168.1.5/24 …` /
/// `2: wlan0    inet6 fe80::…/64 scope link …`.
/// Unknown families and malformed rows are skipped.
#[cfg(any(test, target_os = "android"))]
fn parse_ip_addr_o(
    text: &str,
) -> std::collections::HashMap<String, (Vec<(Ipv4Addr, u8)>, Vec<String>)> {
    use std::collections::HashMap;

    let mut out: HashMap<String, (Vec<(Ipv4Addr, u8)>, Vec<String>)> = HashMap::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 4 {
            continue;
        }
        let name = cols[1].to_string();
        let (addr, plen) = match cols[3].split_once('/') {
            Some((a, p)) => (a, p.parse::<u8>().unwrap_or(0)),
            None => continue,
        };
        match cols[2] {
            "inet" => {
                if let Ok(ip) = addr.parse::<Ipv4Addr>() {
                    out.entry(name).or_default().0.push((ip, plen));
                }
            }
            "inet6" => {
                use std::net::Ipv6Addr;
                if addr.parse::<Ipv6Addr>().is_ok() {
                    out.entry(name).or_default().1.push(addr.to_string());
                }
            }
            _ => {}
        }
    }
    out
}

/// Local IPv4 addresses from the `Local:` table of `/proc/net/fib_trie`:
/// `|-- A.B.C.D` leaves followed by a `/32 host LOCAL` line
/// (`link BROADCAST` leaves are skipped). The prefix comes from the
/// nearest enclosing network `+-- x/N` line — `/31`+ entries are host
/// routes, not networks (32 when unknown). No interface attribution
/// exists at this layer — callers join these to the default-route
/// interface.
#[cfg(any(test, target_os = "android"))]
fn parse_fib_trie_local(text: &str) -> Vec<(Ipv4Addr, u8)> {
    let mut out = Vec::new();
    let mut in_local = false;
    let mut pending: Option<Ipv4Addr> = None;
    // (indent, prefix) stack of enclosing `+-- x/N` lines.
    let mut stack: Vec<(usize, u8)> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !line.starts_with([' ', '\t']) && trimmed.ends_with(':') {
            in_local = trimmed == "Local:";
            stack.clear();
            pending = None;
            continue;
        }
        if !in_local {
            continue;
        }
        // Node markers may carry leading `|` runs (`|  +-- …`).
        // Strip them first so `|-- x` and `|  |-- x` share the leaf
        // form `-- x` (`+-- x` keeps its `+` and stays distinct).
        let node = trimmed.trim_start_matches('|').trim_start();
        let indent = line.len() - trimmed.len();
        if let Some(rest) = node.strip_prefix("+-- ") {
            if let Some((_, plen)) = rest.split_once('/') {
                if let Ok(n) = plen.split_whitespace().next().unwrap_or("").parse::<u8>() {
                    while stack.last().is_some_and(|(d, _)| *d >= indent) {
                        stack.pop();
                    }
                    stack.push((indent, n));
                }
            }
            pending = None;
        } else if let Some(rest) = node.strip_prefix("-- ") {
            let ip_str = rest.split_whitespace().next().unwrap_or("");
            pending = ip_str.parse::<IpAddr>().ok().and_then(|ip| match ip {
                IpAddr::V4(v4) => Some(v4),
                _ => None,
            });
            while stack.last().is_some_and(|(d, _)| *d >= indent) {
                stack.pop();
            }
        } else if trimmed.contains("host LOCAL") {
            if let Some(ip) = pending.take() {
                // Nearest enclosing network prefix. `/31` and `/32`
                // `+-- x/N` entries are host routes sitting between the
                // leaf and its network (e.g. `127.0.0.0/31` under
                // `127.0.0.0/8`), so only `< 31` counts.
                let prefix = stack
                    .iter()
                    .rev()
                    .find_map(|(_, n)| (*n < 31).then_some(*n))
                    .unwrap_or(32);
                out.push((ip, prefix));
            }
        } else if trimmed.contains("link BROADCAST") {
            pending = None;
        }
    }
    out
}

/// IPv6 addresses per interface from `/proc/net/if_inet6`:
/// `<32-hex-digits> <ifidx-hex> <plen-hex> <scope-hex> <flags-hex> <name>`.
#[cfg(any(test, target_os = "android"))]
fn parse_if_inet6(text: &str) -> std::collections::HashMap<String, Vec<String>> {
    use std::collections::HashMap;
    use std::net::Ipv6Addr;

    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 6 {
            continue;
        }
        let Ok(raw) = u128::from_str_radix(cols[0], 16) else {
            continue;
        };
        let ip = Ipv6Addr::from(raw).to_string();
        out.entry(cols[5].to_string()).or_default().push(ip);
    }
    out
}

/// Default gateways as (interface name, gateway). Empty on failure.
pub fn default_gateways() -> Vec<(String, String)> {
    #[cfg(windows)]
    {
        gateway_windows()
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        gateway_linux()
    }
    #[cfg(target_os = "macos")]
    {
        gateway_macos()
    }
    #[cfg(not(any(
        windows,
        target_os = "linux",
        target_os = "macos",
        target_os = "android"
    )))]
    {
        Vec::new()
    }
}

/// Global DNS servers. Empty on failure.
pub fn dns_servers() -> Vec<String> {
    #[cfg(windows)]
    {
        dns_windows()
    }
    #[cfg(not(windows))]
    {
        dns_resolv_conf()
    }
}

#[cfg(windows)]
fn powershell_lines(script: &str) -> Vec<String> {
    std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Windows default gateways: `InterfaceAlias|NextHop` pairs. Alias
/// matching is best-effort (crate names are GUID-ish on Windows), so
/// callers fall back to the first route when names don't line up.
#[cfg(windows)]
fn gateway_windows() -> Vec<(String, String)> {
    let mut out = Vec::new();
    // Alias|IfIndex|NextHop triples let us join precisely.
    let lines = powershell_lines(
        "Get-NetRoute -DestinationPrefix '0.0.0.0/0' | \
         Sort-Object RouteMetric | \
         ForEach-Object { \"{0}|{1}|{2}\" -f $_.InterfaceAlias, $_.InterfaceIndex, $_.NextHop }",
    );
    for line in lines {
        let mut parts = line.splitn(3, '|');
        let (Some(alias), Some(idx), Some(gw)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        if gw.parse::<IpAddr>().is_err() {
            continue;
        }
        out.push((format!("{}|{}", alias, idx), gw.to_string()));
    }
    out
}

/// Windows DNS: deduped IPv4 server list across adapters.
#[cfg(windows)]
fn dns_windows() -> Vec<String> {
    let mut out = Vec::new();
    let lines = powershell_lines(
        "Get-DnsClientServerAddress -AddressFamily IPv4 | \
         Select-Object -ExpandProperty ServerAddresses | Sort-Object -Unique",
    );
    for line in lines {
        if line.parse::<IpAddr>().is_ok() && !out.contains(&line) {
            out.push(line);
        }
    }
    out
}

/// Linux/Android default gateways from /proc/net/route:
/// `Iface Destination Gateway Flags ...` with hex little-endian IPs.
/// The default route has Destination 00000000. (Android shares the
/// Linux kernel procfs, so the same reader applies.)
#[cfg(any(target_os = "linux", target_os = "android"))]
fn gateway_linux() -> Vec<(String, String)> {
    parse_proc_net_route(&std::fs::read_to_string("/proc/net/route").unwrap_or_default())
}

#[cfg(any(test, target_os = "linux", target_os = "android"))]
fn parse_proc_net_route(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 8 || cols[1] != "00000000" {
            continue;
        }
        let gw = u32::from_str_radix(cols[2], 16).unwrap_or(0);
        if gw == 0 {
            continue;
        }
        out.push((
            cols[0].to_string(),
            Ipv4Addr::from(u32::to_le_bytes(gw)).to_string(),
        ));
    }
    out
}

/// macOS default gateways from `netstat -rn -f inet`:
/// `default  192.168.1.1  ...  en0`.
#[cfg(target_os = "macos")]
fn gateway_macos() -> Vec<(String, String)> {
    let text = std::process::Command::new("netstat")
        .args(["-rn", "-f", "inet"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    parse_netstat_rn(&text)
}

#[cfg(any(test, target_os = "macos"))]
fn parse_netstat_rn(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 4 || cols[0] != "default" {
            continue;
        }
        if cols[1].parse::<IpAddr>().is_err() {
            continue;
        }
        out.push((cols[3].to_string(), cols[1].to_string()));
    }
    out
}

/// Unix DNS from /etc/resolv.conf `nameserver` lines.
#[cfg(not(windows))]
fn dns_resolv_conf() -> Vec<String> {
    parse_resolv_conf(&std::fs::read_to_string("/etc/resolv.conf").unwrap_or_default())
}

#[cfg(any(test, not(windows)))]
fn parse_resolv_conf(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.trim_start().strip_prefix("nameserver") {
            let ip = rest.split_whitespace().next().unwrap_or("");
            if ip.parse::<IpAddr>().is_ok() && !out.contains(&ip.to_string()) {
                out.push(ip.to_string());
            }
        }
    }
    out
}

/// Render one adapter block (ipconfig/ifconfig-like section).
fn render_adapter(e: &IfEntry) -> Vec<String> {
    let mut out = Vec::new();
    let status = if e.up { "up" } else { "down" };
    out.push(format!("{} (#{}) [{}]:", e.name, e.index, status));
    out.push(format!(
        "  MAC ......... : {}",
        e.mac.as_deref().unwrap_or("-")
    ));
    if e.v4.is_empty() {
        out.push("  IPv4 ........ : -".to_string());
    }
    for (ip, prefix) in &e.v4 {
        out.push(format!("  IPv4 ........ : {}/{}", ip, prefix));
    }
    let has_global_v6 = e.v6.iter().any(|a| !a.to_lowercase().starts_with("fe80"));
    for ip6 in &e.v6 {
        // Skip noisy link-locals when a global v6 exists.
        if !has_global_v6 || !ip6.to_lowercase().starts_with("fe80") {
            out.push(format!("  IPv6 ........ : {}", ip6));
        }
    }
    out.push(format!(
        "  Gateway ..... : {}",
        e.gateway.as_deref().unwrap_or("-")
    ));
    out
}

pub fn cmd_ipcfg(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let filter = args.trim().to_lowercase();
    let mut entries = list_adapters();
    if !filter.is_empty() {
        entries.retain(|e| e.name.to_lowercase().contains(&filter));
        if entries.is_empty() {
            print_error(&t!("cmd.ipcfg.no_such_adapter", args.trim()));
            return Ok(false);
        }
    }
    if entries.is_empty() {
        print_warning(&t!("cmd.ipcfg.no_adapters"));
        return Ok(false);
    }
    println!("{}", t!("cmd.ipcfg.title").cyan().bold());
    println!();
    for e in &entries {
        for line in render_adapter(e) {
            if line.ends_with(':') && !line.starts_with(' ') {
                println!("{}", line.cyan());
            } else {
                println!("{}", line);
            }
        }
        println!();
    }
    let dns = dns_servers();
    if dns.is_empty() {
        println!("  {} {}", t!("cmd.ipcfg.dns_label"), "-".dimmed());
    } else {
        println!("  {} {}", t!("cmd.ipcfg.dns_label"), dns.join(", ").cyan());
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_len_counts_mask_bits() {
        assert_eq!(prefix_len(Ipv4Addr::new(255, 255, 255, 0)), 24);
        assert_eq!(prefix_len(Ipv4Addr::new(255, 255, 0, 0)), 16);
        assert_eq!(prefix_len(Ipv4Addr::new(255, 255, 255, 128)), 25);
        assert_eq!(prefix_len(Ipv4Addr::new(0, 0, 0, 0)), 0);
        assert_eq!(prefix_len(Ipv4Addr::new(255, 255, 255, 255)), 32);
    }

    #[test]
    fn proc_net_route_parses_default() {
        let text = "Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\n\
                    eth0\t00000000\t0101A8C0\t0003\t0\t0\t100\t00000000\n\
                    eth0\t0001A8C0\t00000000\t0001\t0\t0\t100\t00FFFFFF\n";
        assert_eq!(
            parse_proc_net_route(text),
            vec![("eth0".to_string(), "192.168.1.1".to_string())]
        );
        assert!(parse_proc_net_route("Iface\tDestination\n").is_empty());
    }

    #[test]
    fn netstat_rn_parses_default() {
        let text =
            "Routing tables\nInternet:\nDestination        Gateway            Flags      Netif\n\
                    default            192.168.1.1        UGSc           en0\n\
                    127                127.0.0.1          UCS            lo0\n";
        assert_eq!(
            parse_netstat_rn(text),
            vec![("en0".to_string(), "192.168.1.1".to_string())]
        );
        assert!(parse_netstat_rn("nothing here\n").is_empty());
    }

    #[test]
    fn resolv_conf_parses_nameservers() {
        let text =
            "# comment\nnameserver 8.8.8.8\nnameserver 8.8.8.8\nnameserver 1.1.1.1\nsearch lan\n";
        assert_eq!(
            parse_resolv_conf(text),
            vec!["8.8.8.8".to_string(), "1.1.1.1".to_string()]
        );
    }

    #[test]
    fn render_adapter_shapes_sections() {
        let e = IfEntry {
            name: "Ethernet0".to_string(),
            index: 7,
            up: true,
            mac: Some("AA-BB-CC-DD-EE-FF".to_string()),
            v4: vec![(Ipv4Addr::new(192, 168, 1, 20), 24)],
            v6: vec!["fe80::1".to_string()],
            gateway: Some("192.168.1.1".to_string()),
        };
        let lines = render_adapter(&e);
        assert!(lines[0].contains("Ethernet0"));
        assert!(lines.iter().any(|l| l.contains("192.168.1.20/24")));
        assert!(lines.iter().any(|l| l.contains("192.168.1.1")));
        // No global v6: link-local is still shown.
        assert!(lines.iter().any(|l| l.contains("fe80::1")));
        // With a global v6, link-local is hidden as noise.
        let mut e2 = e.clone();
        e2.v6.push("2001:db8::1".to_string());
        let lines2 = render_adapter(&e2);
        assert!(lines2.iter().any(|l| l.contains("2001:db8::1")));
        assert!(!lines2.iter().any(|l| l.contains("fe80::1")));
    }

    #[test]
    fn list_adapters_runs_without_panic() {
        // Environment-dependent content; assert shape only.
        let entries = list_adapters();
        for e in &entries {
            assert!(!e.name.is_empty());
        }
        let _ = dns_servers();
        let _ = default_gateways();
    }

    #[test]
    fn fib_trie_local_takes_hosts_not_broadcasts() {
        let text = "Main:\n\
                    \x20 +-- 0.0.0.0/0 3 0 5\n\
                    Local:\n\
                    \x20 +-- 127.0.0.0/8 3 0 5\n\
                    \x20    +-- 127.0.0.0/31 2 0 4\n\
                    \x20       |-- 127.0.0.0\n\
                    \x20          /32 link BROADCAST\n\
                    \x20       |-- 127.0.0.1\n\
                    \x20          /32 host LOCAL\n\
                    \x20 +-- 192.168.1.0/24 3 0 5\n\
                    \x20    |-- 192.168.1.5\n\
                    \x20       /32 host LOCAL\n";
        // Prefix rides along from the enclosing non-/32 `+-- x/N`.
        assert_eq!(
            parse_fib_trie_local(text),
            vec![
                (Ipv4Addr::new(127, 0, 0, 1), 8),
                (Ipv4Addr::new(192, 168, 1, 5), 24),
            ]
        );
        assert!(parse_fib_trie_local("Main:\n  +-- 0.0.0.0/0\n").is_empty());
        assert!(parse_fib_trie_local("nothing here\n").is_empty());
    }

    #[test]
    fn if_inet6_parses_hex_rows_per_interface() {
        // fe80::1/64 on wlan0, ::1/128 on lo.
        let text = "fe800000000000000000000000000001 02 40 20 80 wlan0\n\
                    00000000000000000000000000000001 01 80 10 80 lo\n\
                    garbage line\n";
        let got = parse_if_inet6(text);
        assert_eq!(got["wlan0"], vec!["fe80::1".to_string()]);
        assert_eq!(got["lo"], vec!["::1".to_string()]);
    }

    #[test]
    fn ip_addr_o_parses_v4_and_v6_with_prefix() {
        let text = "2: wlan0    inet 192.168.1.5/24 brd 192.168.1.255 scope global dynamic\n\
                    2: wlan0    inet6 fe80::1/64 scope link\n\
                    1: lo    inet 127.0.0.1/8 scope host\n\
                    3: rmnet0    unknown 10.0.0.1/32 scope global\n";
        let got = parse_ip_addr_o(text);
        assert_eq!(got["wlan0"].0, vec![(Ipv4Addr::new(192, 168, 1, 5), 24)]);
        assert_eq!(got["wlan0"].1, vec!["fe80::1".to_string()]);
        assert_eq!(got["lo"].0, vec![(Ipv4Addr::new(127, 0, 0, 1), 8)]);
        // Unknown families are skipped, interface still absent.
        assert!(!got.contains_key("rmnet0"));
    }

    #[test]
    fn android_list_joins_sysfs_with_proc_tables() {
        // Fake `/sys/class/net` tree: wlan0 up with MAC, lo down.
        let dir = tempfile::tempdir().unwrap();
        let sys = dir.path().join("sys");
        for (name, addr, oper, idx) in [
            ("wlan0", "aa:bb:cc:dd:ee:ff", "up", "2"),
            ("lo", "00:00:00:00:00:00", "unknown", "1"),
        ] {
            let d = sys.join(name);
            std::fs::create_dir_all(&d).unwrap();
            std::fs::write(d.join("address"), addr).unwrap();
            std::fs::write(d.join("operstate"), oper).unwrap();
            std::fs::write(d.join("ifindex"), idx).unwrap();
        }
        let fib = dir.path().join("fib_trie");
        std::fs::write(
            &fib,
            "Local:\n \
             \x20+-- 192.168.1.0/24 3 0 5\n \
             \x20   |-- 192.168.1.5\n \
             \x20      /32 host LOCAL\n",
        )
        .unwrap();
        let inet6 = dir.path().join("if_inet6");
        std::fs::write(
            &inet6,
            "fe800000000000000000000000000001 02 40 20 80 wlan0\n",
        )
        .unwrap();

        // No default gateways on the test host ordering to rely on:
        // fib_trie v4 joins the default-route interface when there is
        // one, else stays unassigned — either way the call must not
        // panic and MACs must resolve.
        let entries = list_adapters_android(&sys, &fib, &inet6);
        assert_eq!(entries.len(), 2);
        let wlan = entries.iter().find(|e| e.name == "wlan0").unwrap();
        assert!(wlan.up);
        assert_eq!(wlan.mac.as_deref(), Some("aa:bb:cc:dd:ee:ff"));
        assert_eq!(wlan.index, 2);
        assert_eq!(wlan.v6, vec!["fe80::1".to_string()]);
        // Loopback sorts last.
        assert_eq!(entries.last().unwrap().name, "lo");
    }
}
