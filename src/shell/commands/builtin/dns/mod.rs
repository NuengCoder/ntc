//! `dns` — resolve typed DNS records and show system fallback output.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::net::ToSocketAddrs;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::net::dns;
use super::super::support::net::tokenize;

// ── DNS ───────────────────────────────────────────────────────────────────

pub fn cmd_dns(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_dns_help();
        return Ok(false);
    }

    let mut host = String::new();
    let mut record = String::from("A");
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-t" | "--type" => {
                i += 1;
                record = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.dns_missing_type")))?
                    .clone();
            }
            "--" => break,
            _ => {
                if host.is_empty() {
                    host = tokens[i].clone();
                }
            }
        }
        i += 1;
    }

    if host.is_empty() {
        anyhow::bail!("{}", t!("dl.dns_host_required"));
    }

    match record.to_uppercase().as_str() {
        "A" => {
            resolve_a(&host, "A")?;
        }
        "AAAA" => {
            resolve_a(&host, "AAAA")?;
        }
        "ALL" => {
            resolve_a(&host, "A")?;
            resolve_a(&host, "AAAA")?;
            for rtype in [
                dns::RRType::Mx,
                dns::RRType::Ns,
                dns::RRType::Txt,
                dns::RRType::Soa,
            ] {
                print_dns_records(&host, rtype);
            }
        }
        other => match dns::RRType::from_label(other) {
            Some(rtype) => print_dns_records(&host, rtype),
            None => print_error(&t!("dl.dns_unsupported_type", record, dns::SUPPORTED_TYPES)),
        },
    }

    Ok(false)
}

/// Queries one typed record and prints it; `PTR` also accepts an IP literal.
fn print_dns_records(host: &str, rtype: dns::RRType) {
    let label = rtype.label();
    let query = if rtype == dns::RRType::Ptr {
        dns::reverse_name(host).unwrap_or_else(|| host.to_string())
    } else {
        host.to_string()
    };

    println!("{} {} records:", host.bold(), label.cyan());

    match dns::resolve(&query, rtype) {
        Ok(records) => {
            let matching: Vec<dns::Record> = records
                .into_iter()
                .filter(|record| record.rtype == rtype)
                .collect();
            if matching.is_empty() {
                println!("{}", t!("dl.dns_no_records", "─".dimmed(), label));
            } else {
                for record in matching {
                    println!("  {}  {}", "OK".green().bold(), record.render());
                }
            }
        }
        Err(dns::DnsError::Network(message)) => {
            // No answer at all: only then fall back to dig/nslookup.
            print_warning(&t!("dl.dns_io", message));
            print_warning(&t!("dl.dns_system_fallback"));
            if let Err(e) = system_dns_lookup(host, label) {
                print_error(&format!("{}", e));
            }
        }
        Err(error) => println!("  {}  {}", "FAIL".red().bold(), dns_error_text(&error)),
    }
}

fn dns_error_text(error: &dns::DnsError) -> String {
    match error {
        dns::DnsError::Rcode(code) => {
            t!("dl.dns_rcode", code, dns::rcode_name(*code))
        }
        dns::DnsError::Timeout => t!("dl.dns_timeout"),
        dns::DnsError::Malformed => t!("dl.dns_malformed"),
        dns::DnsError::Network(message) => t!("dl.dns_io", message),
    }
}

fn resolve_a(host: &str, rtype: &str) -> Result<bool> {
    let hint = if rtype.eq_ignore_ascii_case("AAAA") {
        " (IPv6)"
    } else {
        " (IPv4)"
    };
    println!(
        "{}{}",
        format_args!("{} {} records:", host.bold(), rtype.cyan()),
        hint.dimmed()
    );

    let addr = format!("{}:1", host);
    let addrs: Vec<std::net::SocketAddr> = match addr.to_socket_addrs() {
        Ok(a) => a.collect(),
        Err(e) => {
            println!("  {}  {}", "FAIL".red().bold(), e);
            return Ok(false);
        }
    };

    let found: Vec<&std::net::SocketAddr> = if rtype.eq_ignore_ascii_case("AAAA") {
        addrs.iter().filter(|a| a.is_ipv6()).collect()
    } else {
        addrs.iter().filter(|a| a.is_ipv4()).collect()
    };

    if found.is_empty() {
        println!("{}", t!("dl.dns_no_records", "─".dimmed(), rtype));
    } else {
        for addr in &found {
            println!("  {}  {}", "OK".green().bold(), addr.ip());
        }
    }

    Ok(false)
}

fn system_dns_lookup(host: &str, rtype: &str) -> Result<bool> {
    println!(
        "{}",
        t!("dl.dns_looking_up", host.bold(), rtype.cyan()).dimmed()
    );

    #[cfg(windows)]
    let output = std::process::Command::new("nslookup")
        .arg(format!("-type={}", rtype.to_lowercase()))
        .arg(host)
        .output();

    #[cfg(not(windows))]
    let output = std::process::Command::new("dig")
        .arg(host)
        .arg(&rtype.to_uppercase())
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if out.status.success() {
                println!("{}", stdout);
            } else {
                print!("{}", stdout);
                eprintln!("{}", stderr);
                anyhow::bail!(
                    "{}",
                    t!("dl.dns_lookup_failed", format!("{:?}", out.status.code()))
                );
            }
        }
        Err(e) => {
            anyhow::bail!("{}", t!("dl.dns_tool_failed", e, rtype));
        }
    }

    Ok(false)
}

fn print_dns_help() {
    println!("{}", t!("dns.help.title").bold());
    println!();
    println!("{}", t!("dns.help.usage").bold());
    println!("{}", t!("dns.help.usage_line"));
    println!();
    println!("{}", t!("dns.help.options").bold());
    println!("{}", t!("dns.help.opt_type"));
    println!("{}", t!("dns.help.opt_type_sub"));
    println!("{}", t!("dns.help.opt_help"));
    println!();
    println!("{}", t!("dns.help.examples").bold());
    println!("  dns example.com");
    println!("  dns example.com -t AAAA");
    println!("  dns example.com -t MX");
    println!("  dns example.com -t TXT");
    println!("  dns example.com -t SOA");
    println!("  dns 8.8.8.8 -t PTR");
    println!("  dns example.com -t ALL");
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "dns",
    aliases: &[],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_dns),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_resolve_localhost() {
        let addrs: Vec<std::net::SocketAddr> = "127.0.0.1:1".to_socket_addrs().unwrap().collect();
        assert!(!addrs.is_empty());
        assert!(addrs[0].ip().is_loopback());
    }

    #[test]
    fn test_dns_resolve_hostname() {
        let addrs: Vec<std::net::SocketAddr> = "localhost:1".to_socket_addrs().unwrap().collect();
        assert!(!addrs.is_empty());
    }

    #[test]
    fn test_resolve_a_ipv4_filter() {
        let host = "127.0.0.1";
        let addr = format!("{}:1", host);
        let addrs: Vec<std::net::SocketAddr> = addr.to_socket_addrs().unwrap().collect();
        let ipv4: Vec<&std::net::SocketAddr> = addrs.iter().filter(|a| a.is_ipv4()).collect();
        assert!(!ipv4.is_empty());
    }
}
