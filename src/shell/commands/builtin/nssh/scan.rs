use crate::output::{print_error, print_info, print_warning};
use crate::shell::commands::builtin::nssh::nssh::scan as portscan;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::time::Duration;

pub(super) fn cmd_scan(tokens: &[String]) -> Result<bool> {
    // nssh scan <host> [--ports <spec> | --top <n>] [--timeout <ms>]
    let mut host: Option<String> = None;
    let mut ports_spec: Option<String> = None;
    let mut top: Option<usize> = None;
    let mut timeout_ms = 800u64;
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--ports" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_ports")))?;
                ports_spec = Some(v.clone());
                i += 2;
            }
            "--top" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_top")))?;
                top = Some(
                    v.parse()
                        .map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_top", v)))?,
                );
                i += 2;
            }
            "--timeout" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_timeout")))?;
                timeout_ms = v
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_timeout", v)))?;
                i += 2;
            }
            _ => {
                if tokens[i].starts_with('-') {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                if host.is_some() {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                host = Some(tokens[i].clone());
                i += 1;
            }
        }
    }
    let host = match host {
        Some(h) => h,
        None => {
            print_error(&t!("cmd.nssh.scan_usage"));
            return Ok(false);
        }
    };
    let ports: Vec<u16> = if let Some(spec) = ports_spec {
        portscan::parse_ports(&spec).map_err(|e| anyhow::anyhow!("{}", e))?
    } else {
        let n = top.unwrap_or(20).clamp(1, portscan::TOP_PORTS.len());
        portscan::TOP_PORTS[..n].to_vec()
    };
    let ips = portscan::resolve_host(&host, ports[0]).map_err(|e| anyhow::anyhow!("{}", e))?;
    print_info(&t!("cmd.nssh.scan_start", host, ports.len().to_string()));
    let hits = portscan::scan_ports(&ips, &ports, Duration::from_millis(timeout_ms.max(50)));
    if hits.is_empty() {
        print_warning(&t!("cmd.nssh.scan_none", host));
    } else {
        println!("{}", t!("cmd.nssh.scan_title", host).cyan().bold());
        for h in &hits {
            println!("  {:<6} {}", h.port.to_string().green(), h.service.dimmed());
        }
        print_info(&t!(
            "cmd.nssh.scan_done",
            hits.len().to_string(),
            ports.len().to_string()
        ));
    }
    Ok(false)
}
