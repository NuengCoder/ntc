//! `serve config` - browser UI for NTC configuration.
//!
//! The CLI editor stays at `config serve` (per-service defaults) and at
//! `config <name>` (single settings). This server is the GUI side: one page
//! lists every setting and every serve service, and each change is written
//! through the same `Config` setters the CLI uses, so both views always agree.
//!
//! The page is embedded (no build step) and the API is small JSON under
//! `/api`. Values are validated before they are stored, so a bad request can
//! never leave a broken `config.toml` behind.

use super::share::public_base_url;
use crate::config::{Config, ServeService, VALID_LANGUAGES, VALID_TODO_FORMATS};
use crate::navigator::Navigator;
use crate::output::print_info;
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::respond;
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const MAX_BODY: usize = 256 * 1024;
const PAGE: &str = include_str!("assets/config_web.html");

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve_config(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_config_help();
        return Ok(false);
    }

    // Defaults come from `config serve config` (alias: `serve config`).
    let mut port = Config::global_get_serve_port(ServeService::Config);
    let mut bind = Config::global_get_serve_bind(ServeService::Config);
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
    print_info(&t!(
        "cmd.serve.config_serving",
        public_base_url(&bind, port)
    ));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.config_local_warning"));
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
            Err(e) => crate::output::print_error(&format!("Accept: {}", e)),
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
        ("GET", "/api/config") => json_resp(stream, 200, snapshot()),
        ("POST", "/api/set") => api_set(stream, body),
        ("POST", "/api/serve") => api_serve(stream, body),
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

// ── API ───────────────────────────────────────────────────────────────────

/// Current settings + serve defaults. `kind` tells the page which control to
/// draw; `options` is only present for a `select`.
fn snapshot() -> Value {
    let mut settings = Vec::new();
    let mut push = |key: &str, kind: &str, value: Value, options: Option<Vec<&str>>| {
        let mut row = json!({ "key": key, "kind": kind, "value": value });
        if let Some(options) = options {
            row["options"] = json!(options);
        }
        settings.push(row);
    };

    push(
        "output_path",
        "text",
        json!(Config::global_get_output_path().display().to_string()),
        None,
    );
    push(
        "max_depth",
        "number",
        json!(Config::global_get_max_depth()),
        None,
    );
    push(
        "num_threads",
        "number",
        json!(Config::global_get_num_threads()),
        None,
    );
    push(
        "show_line_numbers",
        "bool",
        json!(Config::global_get_show_line_numbers()),
        None,
    );
    push(
        "color_enabled",
        "bool",
        json!(Config::global_get_color_enabled()),
        None,
    );
    push(
        "autosuggest_enabled",
        "bool",
        json!(Config::global_get_autosuggest_enabled()),
        None,
    );
    push(
        "idle_hint_enabled",
        "bool",
        json!(Config::global_get_idle_hint_enabled()),
        None,
    );
    push(
        "git_integration_enabled",
        "bool",
        json!(Config::global_get_git_integration_enabled()),
        None,
    );
    push(
        "integration_shell_enabled",
        "bool",
        json!(Config::global_get_integration_shell_enabled()),
        None,
    );
    push(
        "code_tools_enabled",
        "bool",
        json!(Config::global_get_code_tools_enabled()),
        None,
    );
    push(
        "dir_view_enabled",
        "bool",
        json!(Config::global_get_dir_view_enabled()),
        None,
    );
    push(
        "file_view_enabled",
        "bool",
        json!(Config::global_get_file_view_enabled()),
        None,
    );
    push(
        "intro_view_enabled",
        "bool",
        json!(Config::global_get_intro_view_enabled()),
        None,
    );
    push(
        "update_check_enabled",
        "bool",
        json!(Config::global_get_update_check_enabled()),
        None,
    );
    push(
        "auto_size_files",
        "bool",
        json!(Config::global_get_auto_size_files()),
        None,
    );
    push(
        "auto_size_dirs",
        "bool",
        json!(Config::global_get_auto_size_dirs()),
        None,
    );
    push(
        "history_enabled",
        "bool",
        json!(Config::global_get_history_enabled()),
        None,
    );
    push(
        "file_watcher_enabled",
        "bool",
        json!(Config::global_get_file_watcher_enabled()),
        None,
    );
    push(
        "watch_limit",
        "number",
        json!(Config::global_get_watch_limit()),
        None,
    );
    push(
        "todo_format",
        "select",
        json!(Config::global_get_todo_format()),
        Some(VALID_TODO_FORMATS.to_vec()),
    );
    push(
        "language",
        "select",
        json!(Config::global_get_language()),
        Some(VALID_LANGUAGES.to_vec()),
    );

    // Listing order comes from the shared service table, so `showcg`,
    // `serve` help and this page can never drift apart.
    let services: Vec<Value> = super::service::SERVICES
        .iter()
        .map(|row| {
            let svc = row.service;
            json!({
                "name": svc.name(),
                "port": Config::global_get_serve_port(svc),
                "bind": Config::global_get_serve_bind(svc),
            })
        })
        .collect();

    json!({ "settings": settings, "services": services })
}

fn api_set(stream: &mut TcpStream, body: &[u8]) {
    let request: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_resp(stream, 400, json!({ "error": "invalid json" })),
    };
    let Some(key) = request.get("key").and_then(Value::as_str) else {
        return json_resp(stream, 400, json!({ "error": "missing key" }));
    };
    let value = request.get("value").cloned().unwrap_or(Value::Null);
    match apply_setting(key, &value) {
        Ok(()) => json_resp(stream, 200, snapshot()),
        Err(e) => json_resp(stream, 400, json!({ "error": e })),
    }
}

fn api_serve(stream: &mut TcpStream, body: &[u8]) {
    let request: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_resp(stream, 400, json!({ "error": "invalid json" })),
    };
    let Some(name) = request.get("name").and_then(Value::as_str) else {
        return json_resp(stream, 400, json!({ "error": "missing service name" }));
    };
    let Some(svc) = ServeService::parse(name) else {
        return json_resp(stream, 400, json!({ "error": "unknown service" }));
    };

    if let Some(bind) = request.get("bind").and_then(Value::as_str) {
        if bind.trim().is_empty() {
            return json_resp(stream, 400, json!({ "error": "missing bind" }));
        }
        Config::global_set_serve_bind(svc, bind.trim());
    }
    if let Some(port) = request.get("port") {
        match int_value(port) {
            Some(n) if (1..=65535).contains(&n) => Config::global_set_serve_port(svc, n as u16),
            _ => return json_resp(stream, 400, json!({ "error": "port must be 1-65535" })),
        }
    }
    json_resp(stream, 200, snapshot())
}

fn int_value(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.trim().parse().ok()))
}

fn bool_value(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(b) => Some(*b),
        Value::String(s) if s.eq_ignore_ascii_case("on") || s.eq_ignore_ascii_case("true") => {
            Some(true)
        }
        Value::String(s)
            if s.eq_ignore_ascii_case("off") || s.eq_ignore_ascii_case("false") || s.is_empty() =>
        {
            Some(false)
        }
        _ => None,
    }
}

/// Validate and write one setting. Only the keys [`snapshot`] exposes are
/// accepted, so the API is an explicit allow-list.
fn apply_setting(key: &str, value: &Value) -> std::result::Result<(), String> {
    let need_bool = || bool_value(value).ok_or_else(|| "expected true or false".to_string());
    let need_int = |min: u64, max: u64| match int_value(value) {
        Some(n) if (min..=max).contains(&n) => Ok(n),
        _ => Err(format!("expected a number in {}..={}", min, max)),
    };
    let need_text = || {
        value
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "expected text".to_string())
    };

    match key {
        "output_path" => {
            let text = need_text()?;
            if text.trim().is_empty() {
                return Err("output path cannot be empty".into());
            }
            let expanded = crate::utils::paths::expand_path_str(text.trim());
            Config::global_set_output_path(Path::new(&expanded));
        }
        "max_depth" => Config::global_set_max_depth(need_int(1, 20)? as usize),
        "num_threads" => {
            let n = need_int(1, 64)? as usize;
            if Config::parse_num_threads(&n.to_string()).is_none() {
                return Err("threads must be between 1 and 64".into());
            }
            Config::global_set_num_threads(n);
        }
        "show_line_numbers" => Config::global_set_show_line_numbers(need_bool()?),
        "color_enabled" => Config::global_set_color_enabled(need_bool()?),
        "autosuggest_enabled" => Config::global_set_autosuggest_enabled(need_bool()?),
        "idle_hint_enabled" => Config::global_set_idle_hint_enabled(need_bool()?),
        "git_integration_enabled" => Config::global_set_git_integration_enabled(need_bool()?),
        "integration_shell_enabled" => Config::global_set_integration_shell_enabled(need_bool()?),
        "code_tools_enabled" => Config::global_set_code_tools_enabled(need_bool()?),
        "dir_view_enabled" => Config::global_set_dir_view_enabled(need_bool()?),
        "file_view_enabled" => Config::global_set_file_view_enabled(need_bool()?),
        "intro_view_enabled" => Config::global_set_intro_view_enabled(need_bool()?),
        "update_check_enabled" => Config::global_set_update_check_enabled(need_bool()?),
        "auto_size_files" => {
            Config::global_set_auto_size(need_bool()?, Config::global_get_auto_size_dirs())
        }
        "auto_size_dirs" => {
            Config::global_set_auto_size(Config::global_get_auto_size_files(), need_bool()?)
        }
        "history_enabled" => Config::global_set_history_enabled(need_bool()?),
        "file_watcher_enabled" => Config::global_set_file_watcher_enabled(need_bool()?),
        "watch_limit" => Config::global_set_watch_limit(need_int(1, 100_000)? as usize),
        "todo_format" => {
            let text = need_text()?.to_lowercase();
            if !VALID_TODO_FORMATS.contains(&text.as_str()) {
                return Err("unknown todo format".into());
            }
            Config::global_set_todo_format(&text);
        }
        "language" => {
            let text = need_text()?.to_lowercase();
            if !VALID_LANGUAGES.contains(&text.as_str()) {
                return Err("unsupported language".into());
            }
            Config::global_set_language(&text);
            crate::utils::config::lang::reload();
            Config::global_set_language_active(true);
        }
        _ => return Err("unknown setting".into()),
    }
    Ok(())
}

fn json_resp(stream: &mut TcpStream, status: u16, value: Value) {
    let body = serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string());
    respond(stream, status, "application/json", body.as_bytes());
}

fn print_config_help() {
    println!("{}", t!("cmd.serve.config_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.config_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!("  {}", t!("cmd.serve.config_help_port"));
    println!("  {}", t!("cmd.serve.config_help_bind"));
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.config_example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_is_the_config_gui() {
        assert!(PAGE.contains("ntc config"));
        assert!(PAGE.contains("/api/config"));
        // No leftover login form or external asset: the page is self-contained.
        assert!(!PAGE.contains("login-form"));
        assert!(!PAGE.contains("<script src="));
    }

    #[test]
    fn rejects_unknown_keys_and_bad_values() {
        Config::with_test_config(Config::new(), || {
            assert!(apply_setting("nope", &json!(1)).is_err());
            assert!(apply_setting("max_depth", &json!(0)).is_err());
            assert!(apply_setting("max_depth", &json!(99)).is_err());
            assert!(apply_setting("num_threads", &json!(0)).is_err());
            assert!(apply_setting("show_line_numbers", &json!("maybe")).is_err());
            assert!(apply_setting("output_path", &json!("")).is_err());
            assert!(apply_setting("todo_format", &json!("nope")).is_err());
            assert!(apply_setting("language", &json!("xx")).is_err());
        });
    }

    #[test]
    fn writes_known_settings() {
        Config::with_test_config(Config::new(), || {
            apply_setting("max_depth", &json!(7)).unwrap();
            assert_eq!(Config::global_get_max_depth(), 7);
            apply_setting("show_line_numbers", &json!(true)).unwrap();
            assert!(Config::global_get_show_line_numbers());
            apply_setting("output_path", &json!("/tmp/ntc-out")).unwrap();
            assert!(Config::global_get_output_path().ends_with("ntc-out"));
        });
    }
}
