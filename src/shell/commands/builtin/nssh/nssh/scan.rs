//! TCP port scanner for `nssh scan`: dependency-free (std + rayon).
//!
//! The host is resolved once, then every port is probed with
//! `TcpStream::connect_timeout` in parallel. No SYN stealth — a full TCP
//! connect, same as opening the port in a browser.

use rayon::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

/// One open port plus its best-known service label.
#[derive(Debug, Clone, PartialEq)]
pub struct ScanHit {
    pub port: u16,
    pub service: &'static str,
}

/// Curated default set for `--top 20` (most common remote-access, web, db,
// cache and dev ports).
pub const TOP_PORTS: &[u16] = &[
    22, 80, 443, 21, 25, 53, 110, 143, 445, 3306, 3389, 5432, 5900, 6379, 8080, 8443, 3000, 8000,
    27017, 1433,
];

/// Well-known service label for display; `None` means "unknown".
pub fn service_name(port: u16) -> Option<&'static str> {
    Some(match port {
        21 => "ftp",
        22 => "ssh",
        23 => "telnet",
        25 => "smtp",
        53 => "dns",
        80 => "http",
        110 => "pop3",
        143 => "imap",
        443 => "https",
        445 => "smb",
        1433 => "mssql",
        3000 => "dev",
        3306 => "mysql",
        3389 => "rdp",
        5432 => "postgres",
        5900 => "vnc",
        6379 => "redis",
        8000 => "dev",
        8080 => "http-alt",
        8443 => "https-alt",
        11211 => "memcached",
        27017 => "mongo",
        _ => return None,
    })
}

/// Parse `--ports 22,80,8000-8010`: comma/space separated numbers and
/// inclusive `lo-hi` ranges. Invalid tokens are returned as an error naming
/// the first offender.
pub fn parse_ports(spec: &str) -> Result<Vec<u16>, String> {
    let mut ports: Vec<u16> = Vec::new();
    for token in spec.split([',', ' ', ';']).filter(|t| !t.trim().is_empty()) {
        let token = token.trim();
        if let Some((lo, hi)) = token.split_once('-') {
            let lo: u16 = lo
                .trim()
                .parse()
                .map_err(|_| format!("bad port range '{}'", token))?;
            let hi: u16 = hi
                .trim()
                .parse()
                .map_err(|_| format!("bad port range '{}'", token))?;
            if lo == 0 || hi == 0 || lo > hi {
                return Err(format!("bad port range '{}'", token));
            }
            if hi - lo > 2000 {
                return Err(format!("range '{}' too wide (max 2000 ports)", token));
            }
            ports.extend(lo..=hi);
        } else {
            let p: u16 = token.parse().map_err(|_| format!("bad port '{}'", token))?;
            if p == 0 {
                return Err(format!("bad port '{}'", token));
            }
            ports.push(p);
        }
    }
    ports.sort_unstable();
    ports.dedup();
    if ports.is_empty() {
        return Err("no ports given".to_string());
    }
    Ok(ports)
}

/// Resolve `host` to IP addresses once (via the first port as anchor).
pub fn resolve_host(host: &str, anchor_port: u16) -> Result<Vec<IpAddr>, String> {
    let addrs: Vec<SocketAddr> = format!("{}:{}", host, anchor_port)
        .to_socket_addrs()
        .map_err(|e| format!("cannot resolve '{}': {}", host, e))?
        .collect();
    if addrs.is_empty() {
        return Err(format!("cannot resolve '{}'", host));
    }
    let mut ips: Vec<IpAddr> = addrs.into_iter().map(|a| a.ip()).collect();
    ips.sort();
    ips.dedup();
    Ok(ips)
}

/// Probe every port on every resolved IP; returns open ports sorted.
pub fn scan_ports(ips: &[IpAddr], ports: &[u16], timeout: Duration) -> Vec<ScanHit> {
    let probes: Vec<(IpAddr, u16)> = ips
        .iter()
        .flat_map(|ip| ports.iter().map(|p| (*ip, *p)))
        .collect();
    let mut hits: Vec<ScanHit> = probes
        .par_iter()
        .filter_map(|(ip, port)| {
            let addr = SocketAddr::new(*ip, *port);
            TcpStream::connect_timeout(&addr, timeout)
                .ok()
                .map(|_| ScanHit {
                    port: *port,
                    service: service_name(*port).unwrap_or("unknown"),
                })
        })
        .collect();
    hits.sort_by_key(|h| h.port);
    hits.dedup_by_key(|h| h.port);
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ports_lists_and_ranges() {
        assert_eq!(parse_ports("22,80,443").unwrap(), vec![22, 80, 443]);
        assert_eq!(parse_ports("8000-8002").unwrap(), vec![8000, 8001, 8002]);
        assert_eq!(parse_ports("22, 22,80").unwrap(), vec![22, 80]);
        assert!(parse_ports("0").is_err());
        assert!(parse_ports("abc").is_err());
        assert!(parse_ports("10-5").is_err());
        assert!(parse_ports("").is_err());
        assert!(parse_ports("1-5000").is_err());
    }

    #[test]
    fn scan_finds_open_listener() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let ips = vec!["127.0.0.1".parse().unwrap()];
        let hits = scan_ports(&ips, &[port], Duration::from_secs(2));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].port, port);
        let _ = listener;
    }

    #[test]
    fn scan_closed_port_is_silent() {
        // Port 1 on loopback is (practically) never open; short timeout.
        let ips = vec!["127.0.0.1".parse().unwrap()];
        let hits = scan_ports(&ips, &[1], Duration::from_millis(300));
        assert!(hits.is_empty());
    }
}
