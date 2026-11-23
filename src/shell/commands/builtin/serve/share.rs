//! `--share` advertisement block, LAN discovery and public URL building.
//!
//! `serve --share` is advertisement-only: it never changes auth or routing.
//! Step 1: quick TCP reachability probe (port 53/DNS, ~900ms each).
//! Step 2: only when online, discover the primary LAN IPv4 via the
//! default-route UDP trick and print it plus a terminal QR.

use crate::output::{print_info, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use std::net::{Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;

/// Candidate endpoints for the internet reachability probe.
const SHARE_PROBE_TARGETS: &[&str] = &["8.8.8.8:53", "1.1.1.1:53"];
const SHARE_PROBE_TIMEOUT: Duration = Duration::from_millis(900);

/// Step 1: true when at least one probe target answers a TCP connect.
fn has_internet() -> bool {
    for target in SHARE_PROBE_TARGETS {
        if let Ok(addr) = target.parse::<SocketAddr>() {
            if TcpStream::connect_timeout(&addr, SHARE_PROBE_TIMEOUT).is_ok() {
                return true;
            }
        }
    }
    false
}

/// True for RFC1918 private ranges: 10/8, 172.16/12, 192.168/16.
fn is_rfc1918(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    o[0] == 10 || o[0] == 192 && o[1] == 168 || o[0] == 172 && (16..=31).contains(&o[1])
}

/// Priority for QR selection: 192.168 > 10 > 172.16-31 > other unicast.
fn lan_priority(ip: Ipv4Addr) -> u8 {
    let o = ip.octets();
    if o[0] == 192 && o[1] == 168 {
        0
    } else if o[0] == 10 {
        1
    } else if o[0] == 172 && (16..=31).contains(&o[1]) {
        2
    } else {
        3
    }
}

/// Step 2: primary LAN IPv4 via the default-route UDP trick (std-only,
/// cross-platform incl. Termux). No packet is actually sent; `connect` on a
/// UDP socket only selects the outbound interface, then `local_addr` reveals it.
fn primary_lan_ip() -> Option<Ipv4Addr> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    // 8.8.8.8:80 never receives traffic here — connect() just picks the route.
    sock.connect("8.8.8.8:80").ok()?;
    match sock.local_addr().ok()?.ip() {
        std::net::IpAddr::V4(v4) if !v4.is_loopback() && !v4.is_link_local() => Some(v4),
        _ => None,
    }
}

fn print_share_qr(url: &str) {
    match qrcode::QrCode::new(url.as_bytes()) {
        Ok(code) => {
            let rendered = code
                .render::<qrcode::render::unicode::Dense1x2>()
                .module_dimensions(2, 1)
                .build();
            println!();
            println!("{}", rendered);
        }
        Err(e) => print_warning(&t!("cmd.serve.share_qr_fail", e)),
    }
}

/// Print the `--share` block: internet check first, LAN + QR only when online.
pub(crate) fn print_share_block(port: u16, bind: &str, share_path: Option<&str>, show_qr: bool) {
    println!("{}", t!("cmd.serve.share_checking").dimmed());
    if !has_internet() {
        print_warning(&t!("cmd.serve.share_offline"));
        println!("  {}", format!("http://127.0.0.1:{}", port).cyan());
        return;
    }
    print_info(&t!("cmd.serve.share_online"));

    if bind == "127.0.0.1" || bind == "localhost" {
        print_warning(&t!("cmd.serve.share_bound_loopback"));
    }

    match primary_lan_ip() {
        Some(ip) => {
            let prio = lan_priority(ip);
            let url = format!("http://{}:{}", ip, port);
            if prio == 3 {
                print_warning(&t!("cmd.serve.share_not_private", ip));
            }
            println!(
                "  {} {}",
                t!("cmd.serve.share_lan_title").cyan().bold(),
                url.cyan()
            );
            if !is_rfc1918(ip) {
                print_warning(&t!("cmd.serve.share_not_private_hint"));
            }
            if let Some(p) = share_path {
                println!("  {} {}", t!("cmd.serve.share_path_title").dimmed(), p);
            }
            if show_qr {
                print_share_qr(&url);
            }
        }
        None => {
            print_warning(&t!("cmd.serve.share_no_lan"));
            println!("  {}", format!("http://127.0.0.1:{}", port).cyan());
        }
    }
}

/// URL a client should use: the bound address when it is concrete, the LAN
/// address when binding to every interface, loopback otherwise.
pub(crate) fn public_base_url(bind: &str, port: u16) -> String {
    let host = match bind {
        "0.0.0.0" | "::" | "[::]" | "*" => primary_lan_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "127.0.0.1".to_string()),
        other => other.to_string(),
    };
    format!("http://{}:{}", host, port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc1918_ranges_detected() {
        assert!(is_rfc1918(Ipv4Addr::new(10, 0, 0, 5)));
        assert!(is_rfc1918(Ipv4Addr::new(10, 255, 255, 255)));
        assert!(is_rfc1918(Ipv4Addr::new(192, 168, 1, 10)));
        assert!(is_rfc1918(Ipv4Addr::new(172, 16, 0, 1)));
        assert!(is_rfc1918(Ipv4Addr::new(172, 31, 255, 254)));
        assert!(!is_rfc1918(Ipv4Addr::new(172, 15, 0, 1)));
        assert!(!is_rfc1918(Ipv4Addr::new(172, 32, 0, 1)));
        assert!(!is_rfc1918(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(!is_rfc1918(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn lan_priority_prefers_private_ranges() {
        let lan192 = lan_priority(Ipv4Addr::new(192, 168, 0, 2));
        let lan10 = lan_priority(Ipv4Addr::new(10, 1, 2, 3));
        let lan172 = lan_priority(Ipv4Addr::new(172, 20, 0, 4));
        let pub_ip = lan_priority(Ipv4Addr::new(8, 8, 8, 8));
        assert!(lan192 < lan10);
        assert!(lan10 < lan172);
        assert!(lan172 < pub_ip);
    }

    #[test]
    fn primary_lan_ip_never_panics() {
        // Environment-dependent (CI may have no route); must not panic.
        let _ = primary_lan_ip();
    }

    #[test]
    fn public_base_url_keeps_a_concrete_bind() {
        assert_eq!(public_base_url("127.0.0.1", 9000), "http://127.0.0.1:9000");
    }
}
