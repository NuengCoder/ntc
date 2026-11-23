//! `config serve` — show and set per-service bind/port defaults.

use crate::config::{Config, ServeService};
use crate::output::{print_error, print_success};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

/// `serve config [service] [--port-default N] [--bind-default ADDR]`
/// (alias: `config serve …` — both route here). Bare call shows all
/// services; a bare service name shows that service; flags save new
/// per-service defaults (port + bind) used when `--port`/`--bind`
/// are absent at serve startup.
pub(super) fn config_serve(args: &str) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() {
        print_serve_config_all();
        return Ok(false);
    }
    if matches!(tokens[0].to_lowercase().as_str(), "help" | "--help" | "-h") {
        print_serve_config_help();
        return Ok(false);
    }
    let Some(svc) = ServeService::parse(tokens[0]) else {
        print_error(&t!("cmd.config.serve.invalid_service", tokens[0]));
        return Ok(false);
    };

    let mut port: Option<u16> = None;
    let mut bind: Option<String> = None;
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].to_lowercase().as_str() {
            "--port-default" => {
                i += 1;
                let v = tokens.get(i).copied().unwrap_or("");
                match v.parse::<u32>() {
                    Ok(n) if (1..=65535).contains(&n) => port = Some(n as u16),
                    _ => {
                        print_error(&t!("cmd.config.serve.invalid_port", v));
                        return Ok(false);
                    }
                }
            }
            "--bind-default" => {
                i += 1;
                let v = tokens.get(i).copied().unwrap_or("");
                if v.is_empty() {
                    print_error(&t!("cmd.config.serve.empty_bind"));
                    return Ok(false);
                }
                bind = Some(v.to_string());
            }
            other => {
                print_error(&t!("cmd.config.serve.unknown_option", other));
                return Ok(false);
            }
        }
        i += 1;
    }

    if port.is_none() && bind.is_none() {
        print_serve_config_one(svc);
    } else {
        if let Some(p) = port {
            Config::global_set_serve_port(svc, p);
        }
        if let Some(b) = bind {
            Config::global_set_serve_bind(svc, &b);
        }
        print_success(&t!(
            "cmd.config.serve.saved",
            svc.name(),
            Config::global_get_serve_bind(svc),
            Config::global_get_serve_port(svc)
        ));
    }
    Ok(false)
}

fn print_serve_config_one(svc: ServeService) {
    println!(
        "{}",
        t!(
            "cmd.config.serve.status",
            svc.name(),
            Config::global_get_serve_bind(svc),
            Config::global_get_serve_port(svc)
        )
    );
}

fn print_serve_config_all() {
    println!("{}", t!("cmd.config.serve.title").cyan().bold());
    print_serve_config_one(ServeService::Files);
    print_serve_config_one(ServeService::Http);
    print_serve_config_one(ServeService::Chess);
    print_serve_config_one(ServeService::Git);
    print_serve_config_one(ServeService::Arc);
    print_serve_config_one(ServeService::Theme);
    print_serve_config_one(ServeService::Ndb);
    print_serve_config_one(ServeService::Backup);
    print_serve_config_one(ServeService::Fake);
    print_serve_config_one(ServeService::Config);
    print_serve_config_one(ServeService::Mdview);
}

fn print_serve_config_help() {
    println!("{}", t!("cmd.config.serve.title").cyan().bold());
    println!();
    println!("{}", t!("cmd.config.serve.usage"));
    println!("  {}", t!("cmd.config.serve.example_show"));
    println!("  {}", t!("cmd.config.serve.example_show_one"));
    println!("  {}", t!("cmd.config.serve.example_set"));
}
