//! `scan` network port and live-host discovery.

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::commands::builtin::nssh::nssh::scan as portscan;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::time::Duration;

/// Parse the `--top N | --ports <spec> | --all` + `--timeout MS` flags
/// shared by `scan port` and `scan ip`. Returns the port list, the
/// timeout, and whether `--timeout` was passed explicitly (huge sweeps
/// without one get the fast default instead of the interactive one).
pub(super) fn parse_net_flags(
    tokens: &[String],
    default_ports: &[u16],
    default_timeout_ms: u64,
    allow_all: bool,
) -> Result<(Vec<u16>, Duration, bool), String> {
    let mut ports_spec: Option<String> = None;
    let mut top: Option<usize> = None;
    let mut all = false;
    let mut timeout_ms = default_timeout_ms;
    let mut timeout_given = false;
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--ports" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.missing_ports"))?;
                ports_spec = Some(v.clone());
                i += 2;
            }
            "--top" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.missing_top"))?;
                top = Some(v.parse().map_err(|_| t!("cmd.scan.invalid_top", v))?);
                i += 2;
            }
            "--all" => {
                if !allow_all {
                    return Err(t!("cmd.scan.unknown_option", "--all"));
                }
                all = true;
                i += 1;
            }
            "--timeout" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.scan.missing_timeout"))?;
                timeout_ms = v.parse().map_err(|_| t!("cmd.scan.invalid_timeout", v))?;
                timeout_given = true;
                i += 2;
            }
            other => {
                if other.starts_with('-') {
                    return Err(t!("cmd.scan.unknown_option", other));
                }
                return Err(t!("cmd.scan.unexpected_arg", other));
            }
        }
    }
    if timeout_ms < 50 {
        timeout_ms = 50;
    }
    let ports: Vec<u16> = if all {
        (1..=u16::MAX).collect()
    } else if let Some(spec) = ports_spec {
        portscan::parse_ports(&spec).map_err(|e| e.to_string())?
    } else {
        let n = top
            .unwrap_or(default_ports.len())
            .clamp(1, default_ports.len());
        default_ports[..n].to_vec()
    };
    Ok((ports, Duration::from_millis(timeout_ms), timeout_given))
}

/// `scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]`.
/// Default host is localhost; default range is the top 20.
pub(super) fn cmd_scan_port(tokens: &[String]) -> Result<bool> {
    let mut host = "127.0.0.1".to_string();
    let mut rest: Vec<String> = Vec::new();
    for tok in tokens {
        if rest.is_empty() && !tok.starts_with('-') && host == "127.0.0.1" {
            // First bare token is the host — but `all`/`top` style words
            // are flags, handled below; a bare host never collides since
            // flag parsing rejects unknown bare tokens anyway. Keep the
            // simple rule: first bare token wins.
            host = tok.clone();
        } else {
            rest.push(tok.clone());
        }
    }
    let (ports, timeout, timeout_given) =
        match parse_net_flags(&rest, portscan::TOP_PORTS, 800, true) {
            Ok(v) => v,
            Err(e) => {
                print_error(&e);
                return Ok(false);
            }
        };
    // Huge sweeps without an explicit --timeout get the fast default
    // (a full 65K sweep at 800ms would take over an hour).
    let timeout = if !timeout_given
        && ports.len() > crate::shell::commands::builtin::scan::netscan::FAST_TIMEOUT_ABOVE
    {
        Duration::from_millis(crate::shell::commands::builtin::scan::netscan::FAST_TIMEOUT_MS)
    } else {
        timeout
    };
    let ips = match portscan::resolve_host(&host, ports[0]) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    print_info(&t!("cmd.scan.port_start", host, ports.len()));
    let total = ips.len() * ports.len();
    let progress =
        crate::shell::commands::builtin::scan::netscan::SweepProgress::new(host.clone(), total);
    let swept = crate::shell::commands::builtin::scan::netscan::sweep_hosts_async(
        &ips,
        &ports,
        timeout,
        crate::shell::commands::builtin::scan::netscan::SCAN_CONCURRENCY,
        &progress,
        |_ip, port| {
            let service = portscan::service_name(port).unwrap_or("unknown");
            progress.print_hit(format!(
                "  {:<6} {}",
                port.to_string().green(),
                service.dimmed()
            ));
        },
    );
    let hits: Vec<_> = swept.into_iter().flat_map(|(_, v)| v).collect();
    if hits.is_empty() {
        print_warning(&t!("cmd.scan.port_none", host));
    } else {
        println!("{}", t!("cmd.scan.port_title", host).cyan().bold());
        for h in &hits {
            println!("  {:<6} {}", h.port.to_string().green(), h.service.dimmed());
        }
        print_success(&t!("cmd.scan.port_done", hits.len(), ports.len()));
    }
    Ok(false)
}

/// `scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]`.
/// Default range is the auto-detected local /24; per-host probes
/// default to the fast discovery set. Results union the TCP sweep
/// with the OS ARP table (`(arp)` rows had no open probe ports).
pub(super) fn cmd_scan_ip(tokens: &[String]) -> Result<bool> {
    let mut cidr: Option<String> = None;
    let mut rest: Vec<String> = Vec::new();
    for tok in tokens {
        if cidr.is_none() && !tok.starts_with('-') {
            cidr = Some(tok.clone());
        } else {
            rest.push(tok.clone());
        }
    }
    let (ports, timeout, timeout_given) = match parse_net_flags(
        &rest,
        crate::shell::commands::builtin::scan::netscan::DISCOVERY_PORTS,
        400,
        false,
    ) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e);
            return Ok(false);
        }
    };
    let (ips, label) = match cidr {
        Some(c) => {
            let list = match crate::shell::commands::builtin::scan::netscan::parse_cidr(&c) {
                Ok(v) => v,
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            };
            let n = list.len();
            (list, format!("{} ({} hosts)", c, n))
        }
        None => match crate::shell::commands::builtin::scan::netscan::local_subnet() {
            Ok((base, prefix)) => {
                let oct = base.octets();
                let mut list = Vec::new();
                for h in 1..=254u32 {
                    list.push(std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                        oct[0], oct[1], oct[2], h as u8,
                    )));
                }
                (
                    list,
                    format!("{}.{}.{}.0/{} (auto)", oct[0], oct[1], oct[2], prefix),
                )
            }
            Err(e) => {
                print_error(&e);
                return Ok(false);
            }
        },
    };
    print_info(&t!("cmd.scan.ip_start", label, ports.len()));
    // Big host × port matrices get the fast default unless the user
    // set --timeout explicitly (a /24 over 10 ports is 2540 probes).
    let timeout = if !timeout_given
        && ips.len().saturating_mul(ports.len())
            > crate::shell::commands::builtin::scan::netscan::FAST_TIMEOUT_ABOVE
    {
        Duration::from_millis(crate::shell::commands::builtin::scan::netscan::FAST_TIMEOUT_MS)
    } else {
        timeout
    };
    let total = ips.len() * ports.len();
    let progress =
        crate::shell::commands::builtin::scan::netscan::SweepProgress::new(label.clone(), total);
    let swept = crate::shell::commands::builtin::scan::netscan::sweep_hosts_async(
        &ips,
        &ports,
        timeout,
        crate::shell::commands::builtin::scan::netscan::SCAN_CONCURRENCY,
        &progress,
        |ip, port| {
            let service = portscan::service_name(port).unwrap_or("unknown");
            if service == "unknown" {
                progress.print_hit(format!("  {}  {}", ip, port));
            } else {
                progress.print_hit(format!(
                    "  {}  {} {}",
                    ip,
                    port.to_string().green(),
                    service.dimmed()
                ));
            }
        },
    );
    let swept_ips: std::collections::HashSet<std::net::IpAddr> =
        swept.iter().map(|(ip, _)| *ip).collect();
    let mut arp_only: Vec<std::net::IpAddr> =
        crate::shell::commands::builtin::scan::netscan::arp_table(&ips)
            .into_iter()
            .filter(|ip| !swept_ips.contains(ip))
            .collect();
    arp_only.sort();
    if swept.is_empty() && arp_only.is_empty() {
        print_warning(&t!("cmd.scan.ip_none", label));
        return Ok(false);
    }
    println!("{}", t!("cmd.scan.ip_title", label).cyan().bold());
    for (ip, open) in &swept {
        if open.is_empty() {
            println!("  {}", ip.to_string().green());
        } else {
            let ports_str = open
                .iter()
                .map(|h| {
                    if h.service == "unknown" {
                        h.port.to_string()
                    } else {
                        format!("{} {}", h.port, h.service)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            println!("  {}  {}", ip.to_string().green(), ports_str.dimmed());
        }
    }
    for ip in &arp_only {
        println!(
            "  {}  {}",
            ip.to_string().green(),
            t!("cmd.scan.ip_arp_only").dimmed()
        );
    }
    print_success(&t!(
        "cmd.scan.ip_done",
        swept.len() + arp_only.len(),
        ips.len()
    ));
    Ok(false)
}
