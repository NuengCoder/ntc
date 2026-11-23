//! `serve arc` — browser UI for creating, listing and extracting archives.
//!
//! A single embedded page (no React build step) served from the directory the
//! command was started in. Every action returns the equivalent `arc` command
//! so the CLI stays the source of truth.

use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::builtin::arc::{
    create_archive, detect_kind, extract_archive, extract_dest_for, list_archive,
};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::{respond, url_decode};
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

const MAX_BODY: usize = 1024 * 1024;
const PAGE: &str = include_str!("assets/arc_web.html");

pub fn cmd_serve_arc(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_arc_help();
        return Ok(false);
    }

    // Defaults come from `serve config arc` (alias: `config serve arc`).
    let mut port = Config::global_get_serve_port(ServeService::Arc);
    let mut bind = Config::global_get_serve_bind(ServeService::Arc);
    let mut dir_arg: Option<String> = None;
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
            other if other.starts_with('-') => {
                anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
            }
            other => {
                if dir_arg.is_some() {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
                }
                dir_arg = Some(other.to_string());
            }
        }
        i += 1;
    }

    let root = match dir_arg {
        Some(d) => {
            let p = PathBuf::from(&d);
            if p.is_absolute() {
                p
            } else {
                nav.current_path().join(p)
            }
        }
        None => nav.current_path().to_path_buf(),
    };
    let root = root
        .canonicalize()
        .with_context(|| t!("cmd.serve.arc_no_dir", root.display()))?;
    if !root.is_dir() {
        anyhow::bail!("{}", t!("cmd.serve.arc_no_dir", root.display()));
    }

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&t!(
        "cmd.serve.arc_serving",
        display_path(&root),
        format!("http://{}", addr)
    ));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.arc_local_warning"));
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    crate::shell::commands::init_ctrlc_handler();
    crate::shell::commands::register_shutdown_flag(shutdown.clone());
    listener.set_nonblocking(true)?;
    let root = Arc::new(root);
    while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let root = root.clone();
                std::thread::spawn(move || handle(stream, &root));
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

// ── Connection ────────────────────────────────────────────────────────────

fn handle(mut stream: TcpStream, root: &Path) {
    let _ = stream.set_nonblocking(false);
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim_end().to_string();
    let mut parts = request_line.split_whitespace();
    let Some(method) = parts.next().map(|s| s.to_string()) else {
        return;
    };
    let Some(target) = parts.next().map(|s| s.to_string()) else {
        return;
    };
    let (raw_path, query) = target.split_once('?').unwrap_or((&target, ""));

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

    route(&mut stream, &method, raw_path, query, &body, root);
}

fn route(
    stream: &mut TcpStream,
    method: &str,
    raw_path: &str,
    query: &str,
    body: &[u8],
    root: &Path,
) {
    let path = url_decode(raw_path);
    match (method, path.as_str()) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/api/list") => api_list(stream, query, root),
        ("GET", "/api/entries") => api_entries(stream, query, root),
        ("POST", "/api/create") => api_create(stream, body, root),
        ("POST", "/api/extract") => api_extract(stream, body, root),
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

// ── API ───────────────────────────────────────────────────────────────────

fn api_list(stream: &mut TcpStream, query: &str, root: &Path) {
    let rel = query_param(query, "path").unwrap_or_default();
    let Some(dir) = resolve_existing(root, &rel) else {
        return json_resp(stream, 404, json!({ "error": "directory not found" }));
    };
    if !dir.is_dir() {
        return json_resp(stream, 400, json!({ "error": "not a directory" }));
    }
    let mut entries: Vec<Value> = Vec::new();
    if let Ok(read) = std::fs::read_dir(&dir) {
        for entry in read.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let meta = entry.metadata().ok();
            let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
            let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            entries.push(json!({
                "name": name,
                "is_dir": is_dir,
                "size": if is_dir { 0 } else { size },
                "is_archive": !is_dir && detect_kind(&name).is_some(),
            }));
        }
    }
    entries.sort_by(|a, b| {
        let ad = a["is_dir"].as_bool().unwrap_or(false);
        let bd = b["is_dir"].as_bool().unwrap_or(false);
        bd.cmp(&ad).then_with(|| {
            a["name"]
                .as_str()
                .unwrap_or("")
                .to_lowercase()
                .cmp(&b["name"].as_str().unwrap_or("").to_lowercase())
        })
    });
    let parent = if rel.is_empty() {
        Value::Null
    } else {
        json!(rel.rsplit_once('/').map(|(p, _)| p).unwrap_or(""))
    };
    json_resp(
        stream,
        200,
        json!({ "path": rel, "parent": parent, "entries": entries }),
    );
}

fn api_entries(stream: &mut TcpStream, query: &str, root: &Path) {
    let rel = query_param(query, "path").unwrap_or_default();
    let Some(src) = resolve_existing(root, &rel) else {
        return json_resp(stream, 404, json!({ "error": "archive not found" }));
    };
    if !src.is_file() || detect_kind(&src.to_string_lossy()).is_none() {
        return json_resp(stream, 400, json!({ "error": "unsupported archive" }));
    }
    match list_archive(&src) {
        Ok(entries) => {
            let total = entries.len();
            let list: Vec<Value> = entries
                .iter()
                .map(|e| json!({ "name": e.name, "size": e.size, "is_dir": e.is_dir }))
                .collect();
            json_resp(
                stream,
                200,
                json!({ "path": rel, "total": total, "entries": list }),
            );
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_create(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    let dir_rel = value.get("dir").and_then(|v| v.as_str()).unwrap_or("");
    let Some(dir) = resolve_existing(root, dir_rel) else {
        return json_resp(stream, 404, json!({ "error": "directory not found" }));
    };
    if !dir.is_dir() {
        return json_resp(stream, 400, json!({ "error": "not a directory" }));
    }
    let output = value
        .get("output")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if !is_safe_name(&output) {
        return json_resp(stream, 400, json!({ "error": "invalid archive name" }));
    }
    if detect_kind(&output).is_none() {
        return json_resp(
            stream,
            400,
            json!({ "error": "extension must be .zip, .tar.gz, .tgz or .tar" }),
        );
    }
    let names: Vec<String> = value
        .get("names")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|n| n.as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    if names.is_empty() {
        return json_resp(
            stream,
            400,
            json!({ "error": "select at least one file or folder" }),
        );
    }
    if names
        .iter()
        .any(|n| !is_safe_name(n) || !dir.join(n).exists())
    {
        return json_resp(stream, 400, json!({ "error": "invalid selection" }));
    }

    let out = dir.join(&output);
    match create_archive(&out, &names, &dir) {
        Ok(count) => {
            let mut command = format!("arc create {}", shell_quote(&output));
            for name in &names {
                command.push(' ');
                command.push_str(&shell_quote(name));
            }
            json_resp(
                stream,
                200,
                json!({ "ok": true, "count": count, "output": output, "command": command }),
            );
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_extract(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    let archive_rel = value.get("archive").and_then(|v| v.as_str()).unwrap_or("");
    let Some(src) = resolve_existing(root, archive_rel) else {
        return json_resp(stream, 404, json!({ "error": "archive not found" }));
    };
    if !src.is_file() || detect_kind(&src.to_string_lossy()).is_none() {
        return json_resp(stream, 400, json!({ "error": "unsupported archive" }));
    }
    let dest_name = value
        .get("dest")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let dest = if dest_name.is_empty() {
        extract_dest_for(&src)
    } else {
        if !is_safe_name(dest_name) {
            return json_resp(stream, 400, json!({ "error": "invalid destination" }));
        }
        src.parent().unwrap_or(root).join(dest_name)
    };

    match extract_archive(&src, &dest) {
        Ok(count) => {
            let dest_rel = rel_to(root, &dest);
            let command = format!(
                "arc extract {} {}",
                shell_quote(archive_rel),
                shell_quote(&dest_rel)
            );
            json_resp(
                stream,
                200,
                json!({ "ok": true, "count": count, "dest": dest_rel, "command": command }),
            );
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn json_resp(stream: &mut TcpStream, status: u16, value: Value) {
    let body = value.to_string();
    respond(
        stream,
        status,
        "application/json; charset=utf-8",
        body.as_bytes(),
    );
}

fn parse_body(body: &[u8]) -> Option<Value> {
    serde_json::from_slice(body).ok()
}

fn query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if url_decode(k) == key {
            return Some(url_decode(v));
        }
    }
    None
}

fn resolve_existing(root: &Path, rel: &str) -> Option<PathBuf> {
    let candidate = safe_join(root, rel)?;
    let canonical = candidate.canonicalize().ok()?;
    if canonical.starts_with(root) {
        Some(canonical)
    } else {
        None
    }
}

/// Join a `/`-separated relative path without allowing `..`, drive prefixes or
/// separators hidden inside a segment.
fn safe_join(root: &Path, rel: &str) -> Option<PathBuf> {
    let rel = rel.trim_start_matches('/');
    let mut out = root.to_path_buf();
    for seg in rel.split('/') {
        match seg {
            "" | "." => {}
            ".." => return None,
            s => {
                if s.contains('\0') || s.contains('\\') {
                    return None;
                }
                let mut components = Path::new(s).components();
                match (components.next(), components.next()) {
                    (Some(Component::Normal(_)), None) => out.push(s),
                    _ => return None,
                }
            }
        }
    }
    Some(out)
}

fn is_safe_name(name: &str) -> bool {
    if name.is_empty() || name == "." || name == ".." {
        return false;
    }
    if name.contains('\0') || name.contains('/') || name.contains('\\') {
        return false;
    }
    let mut components = Path::new(name).components();
    matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(_)), None)
    )
}

fn shell_quote(value: &str) -> String {
    let plain = !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/' | ':' | '+'));
    if plain {
        value.to_string()
    } else {
        format!("\"{}\"", value.replace('"', "\\\""))
    }
}

fn rel_to(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn is_loopback(bind: &str) -> bool {
    bind == "localhost" || bind == "::1" || bind.starts_with("127.")
}

/// Canonical Windows paths carry a `\\?\` verbatim prefix; strip it for display.
fn display_path(path: &Path) -> String {
    let shown = path.display().to_string();
    shown.strip_prefix("\\\\?\\").unwrap_or(&shown).to_string()
}

fn print_arc_help() {
    println!("{}", t!("cmd.serve.arc_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.arc_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!(
        "  {}",
        t!(
            "cmd.serve.arc_help_port",
            Config::global_get_serve_port(ServeService::Arc)
        )
    );
    println!(
        "  {}",
        t!(
            "cmd.serve.arc_help_bind",
            Config::global_get_serve_bind(ServeService::Arc)
        )
    );
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.arc_example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_traversal() {
        let root = Path::new("/tmp/ntc-root");
        assert!(safe_join(root, "../etc").is_none());
        assert!(safe_join(root, "a/../../b").is_none());
        assert!(safe_join(root, "a\\b").is_none());
        assert_eq!(
            safe_join(root, "sub/a.txt").unwrap(),
            root.join("sub").join("a.txt")
        );
    }

    #[test]
    fn safe_names_are_single_components() {
        assert!(is_safe_name("a.txt"));
        assert!(is_safe_name("backup.tar.gz"));
        assert!(!is_safe_name(""));
        assert!(!is_safe_name(".."));
        assert!(!is_safe_name("a/b"));
        assert!(!is_safe_name("a\\b"));
    }

    #[test]
    fn page_is_embedded() {
        assert!(PAGE.contains("NTC Archive"));
    }
}
