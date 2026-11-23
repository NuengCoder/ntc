//! `serve fake` - browser UI for the fake-data generator.
//!
//! The page lists every `fake` type and saved group, generates rows through the
//! same engine `fake gen` uses, and previews the rendered `txt`/`json`/`csv`/
//! `sql` output. Nothing is written to disk: the browser copies or downloads
//! the preview client-side.

use super::share::public_base_url;
use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::builtin::fake::fake::format::{self, Fmt, Table};
use crate::shell::commands::builtin::fake::fake::locale::Locale;
use crate::shell::commands::builtin::fake::fake::{gen, group, types};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::respond;
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const MAX_BODY: usize = 256 * 1024;
const PAGE: &str = include_str!("assets/fake_web.html");

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve_fake(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_fake_help();
        return Ok(false);
    }
    // The folder is pinned at startup so the printed URL always matches the
    // project the shell was in (groups are still global, as in `fake gen`).
    let _cwd = nav.current_path().to_path_buf();

    let mut port = Config::global_get_serve_port(ServeService::Fake);
    let mut bind = Config::global_get_serve_bind(ServeService::Fake);
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-p" | "--port" => {
                i += 1;
                port = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_port")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.serve.invalid_port")))?;
            }
            "-b" | "--bind" => {
                i += 1;
                bind = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_bind")))?
                    .clone();
            }
            other => anyhow::bail!("{}", t!("cmd.serve.unknown_option", other)),
        }
        i += 1;
    }

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&t!("cmd.serve.fake_serving", public_base_url(&bind, port)));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.fake_local_warning"));
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    crate::shell::commands::init_ctrlc_handler();
    crate::shell::commands::register_shutdown_flag(shutdown.clone());
    listener.set_nonblocking(true)?;
    while !shutdown.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                std::thread::spawn(move || handle(stream));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => print_error(&format!("Accept: {}", e)),
        }
    }
    print_info(&t!("cmd.serve.stopped"));
    crate::shell::commands::unregister_shutdown_flag(&shutdown);
    crate::shell::commands::init_ctrlc_handler();
    Ok(false)
}

fn is_loopback(bind: &str) -> bool {
    matches!(bind, "127.0.0.1" | "localhost" | "::1" | "[::1]")
}

// ── Connection ────────────────────────────────────────────────────────────

fn handle(mut stream: TcpStream) {
    let _ = stream.set_nonblocking(false);
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim_end().to_string();
    let mut parts = request_line.split_whitespace();
    let Some(method) = parts.next().map(str::to_string) else {
        return;
    };
    let Some(target) = parts.next().map(str::to_string) else {
        return;
    };
    let (raw_path, _query) = target.split_once('?').unwrap_or((&target, ""));

    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                content_length = v.trim().parse().unwrap_or(0);
            }
        }
    }
    if content_length > MAX_BODY {
        drop(reader);
        json_resp(
            &mut stream,
            413,
            json!({ "error": "request body too large" }),
        );
        return;
    }
    let mut body = vec![0u8; content_length];
    if content_length > 0 && reader.read_exact(&mut body).is_err() {
        return;
    }
    drop(reader);

    route(&mut stream, &method, raw_path, &body);
}

fn route(stream: &mut TcpStream, method: &str, path: &str, body: &[u8]) {
    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/api/meta") => json_resp(stream, 200, meta()),
        ("POST", "/api/generate") => api_generate(stream, body),
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

// ── API ───────────────────────────────────────────────────────────────────

/// Types, saved groups, locales and text formats the page offers.
fn meta() -> Value {
    let type_list: Vec<Value> = types::TYPES
        .iter()
        .map(|def| {
            json!({
                "name": def.name,
                "category": def.category.label(),
                "about": def.about,
            })
        })
        .collect();

    let groups: Vec<Value> = group::group_names()
        .into_iter()
        .map(|name| {
            let columns: Vec<Value> = Config::get_fake_group(&name)
                .unwrap_or_default()
                .iter()
                .map(|c| {
                    json!({
                        "alias": c.alias,
                        "type": c.type_name,
                        "unique": c.unique,
                    })
                })
                .collect();
            json!({ "name": name, "columns": columns })
        })
        .collect();

    let locales: Vec<&str> = Locale::all().iter().map(|l| l.code()).collect();

    json!({
        "types": type_list,
        "groups": groups,
        "locales": locales,
        "formats": ["txt", "json", "csv", "sql"],
    })
}

fn api_generate(stream: &mut TcpStream, body: &[u8]) {
    let request: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_resp(stream, 400, json!({ "error": "invalid json" })),
    };

    let types: Vec<String> = request
        .get("types")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    let group = request
        .get("group")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string);
    let count = request
        .get("count")
        .and_then(Value::as_u64)
        .map(|n| n.clamp(1, 1000) as usize)
        .unwrap_or(10);
    let seed = request.get("seed").and_then(Value::as_u64);
    let locale = request
        .get("locale")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .and_then(Locale::parse);
    let unique = match request.get("unique") {
        Some(Value::Bool(true)) => Some(Vec::new()),
        Some(Value::Array(cols)) => Some(
            cols.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect(),
        ),
        _ => None,
    };
    let fmt = request
        .get("format")
        .and_then(Value::as_str)
        .and_then(Fmt::parse)
        .filter(|f| *f != Fmt::Xlsx)
        .unwrap_or(Fmt::Txt);

    let table = match gen::generate_table(types, group, count, seed, locale, unique) {
        Ok(t) => t,
        Err(e) => return json_resp(stream, 400, json!({ "error": e })),
    };

    json_resp(stream, 200, render(&table, fmt))
}

fn render(table: &Table, fmt: Fmt) -> Value {
    json!({
        "columns": table.columns,
        "rows": table.rows,
        "format": fmt.ext(),
        "text": format::to_text(fmt, table).unwrap_or_default(),
    })
}

fn json_resp(stream: &mut TcpStream, status: u16, value: Value) {
    let body = serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string());
    respond(stream, status, "application/json", body.as_bytes());
}

fn print_fake_help() {
    println!("{}", t!("cmd.serve.fake_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.fake_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!("  {}", t!("cmd.serve.fake_help_port"));
    println!("  {}", t!("cmd.serve.fake_help_bind"));
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.fake_example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_is_the_fake_gui() {
        assert!(PAGE.contains("ntc fake"));
        assert!(PAGE.contains("/api/generate"));
        assert!(!PAGE.contains("<script src="));
    }

    #[test]
    fn meta_lists_types_groups_and_formats() {
        Config::with_test_config(Config::new(), || {
            let m = meta();
            assert!(m["types"].as_array().unwrap().len() > 20);
            assert_eq!(m["formats"][0], "txt");
            assert!(m["locales"].as_array().unwrap().contains(&json!("en_US")));
        });
    }

    #[test]
    fn renders_every_text_format() {
        let table = Table {
            name: "users".into(),
            columns: vec!["name".into(), "email".into()],
            rows: vec![vec!["Ada Lovelace".into(), "ada@example.com".into()]],
        };
        for fmt in [Fmt::Txt, Fmt::Json, Fmt::Csv, Fmt::Sql] {
            let out = render(&table, fmt);
            assert_eq!(out["format"], fmt.ext());
            assert!(!out["text"].as_str().unwrap().is_empty());
        }
    }

    #[test]
    fn group_names_helper_is_reachable() {
        // `fake group` owns the name list; the page must use the same source.
        Config::with_test_config(Config::new(), || {
            assert!(group::group_names().is_empty());
        });
    }
}
