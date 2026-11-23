//! `serve backup [path]` - browser UI for the project backup system.
//!
//! The page pins the folder the command started in (or the given `path`) and
//! lists that project's backups. It covers the same actions as the CLI:
//! create, list, diff, restore (full or deleted-only), undo the last restore,
//! clear all backups and recreate a fresh one.
//!
//! Restoring replaces files, so every mutating request needs `confirm: true`
//! and the page asks first. `BackupManager` still writes its own undo snapshot
//! before a restore, so `Undo` remains available afterwards.

use super::share::public_base_url;
use crate::backup::{compute_project_hash, BackupManager};
use crate::backup_manifest::{display_path, BackupIndex, BackupManifest};
use crate::config::{Config, ServeService};
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::respond;
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use walkdir::WalkDir;

const MAX_BODY: usize = 64 * 1024;
const MAX_DIFF_FILES: usize = 500;
const PAGE: &str = include_str!("assets/backup_web.html");

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve_backup(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_backup_help();
        return Ok(false);
    }

    // Defaults come from `config serve backup` (alias: `serve backup`).
    let mut port = Config::global_get_serve_port(ServeService::Backup);
    let mut bind = Config::global_get_serve_bind(ServeService::Backup);
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

    // A bare `serve backup` manages the folder the shell is in.
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
        .with_context(|| t!("cmd.serve.backup_no_dir", root.display()))?;
    if !root.is_dir() {
        anyhow::bail!("{}", t!("cmd.serve.backup_no_dir", root.display()));
    }

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&t!(
        "cmd.serve.backup_serving",
        display_path(&root),
        public_base_url(&bind, port)
    ));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.backup_local_warning"));
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

fn is_loopback(bind: &str) -> bool {
    matches!(bind, "127.0.0.1" | "localhost" | "::1" | "[::1]")
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
    let Some(method) = parts.next().map(str::to_string) else {
        return;
    };
    let Some(target) = parts.next().map(str::to_string) else {
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
    match (method, raw_path) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/api/state") => json_resp(stream, 200, state_json(root)),
        ("GET", "/api/diff") => {
            let number = query_param(query, "number").and_then(|n| n.parse::<usize>().ok());
            match number {
                Some(n) if backup_exists(root, n) => match diff_json(root, n) {
                    Ok(value) => json_resp(stream, 200, value),
                    Err(e) => json_resp(stream, 400, json!({ "error": e })),
                },
                _ => json_resp(stream, 400, json!({ "error": "unknown backup number" })),
            }
        }
        ("POST", "/api/create") => match BackupManager::create_backup(root) {
            Ok(path) => {
                let mut state = state_json(root);
                state["message"] = json!(format!("Backup created: {}", display_path(&path)));
                json_resp(stream, 200, state);
            }
            Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
        },
        ("POST", "/api/restore") => api_restore(stream, body, root),
        ("POST", "/api/undo") => api_undo(stream, body, root),
        ("POST", "/api/clear") => api_clear(stream, body, root),
        ("POST", "/api/recreate") => api_recreate(stream, body, root),
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

// ── API ───────────────────────────────────────────────────────────────────

fn state_json(root: &Path) -> Value {
    let backups = BackupManager::list_backups(root).unwrap_or_default();
    let project_hash = compute_project_hash(root);
    let backup_dir = BackupIndex::get_project_backup_dir(&project_hash);
    let undo_available = BackupIndex::get_undo_state_path(&project_hash).exists();
    let list: Vec<Value> = backups
        .iter()
        .map(|(number, date, size, files)| {
            json!({
                "number": number,
                "date": date,
                "size": size,
                "files": files,
            })
        })
        .collect();
    json!({
        "root": display_path(root),
        "backup_dir": display_path(&backup_dir),
        "undo_available": undo_available,
        "backups": list,
    })
}

fn backup_exists(root: &Path, number: usize) -> bool {
    BackupManager::list_backups(root)
        .map(|list| list.iter().any(|(n, ..)| *n == number))
        .unwrap_or(false)
}

fn parse_body(body: &[u8]) -> Option<Value> {
    serde_json::from_slice(body).ok()
}

fn confirmed(value: &Value) -> bool {
    value
        .get("confirm")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn api_restore(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !confirmed(&value) {
        return json_resp(stream, 400, json!({ "error": "confirmation required" }));
    }
    let Some(number) = value
        .get("number")
        .and_then(Value::as_u64)
        .map(|n| n as usize)
    else {
        return json_resp(stream, 400, json!({ "error": "missing backup number" }));
    };
    if !backup_exists(root, number) {
        return json_resp(stream, 404, json!({ "error": "backup not found" }));
    }
    let deleted_only = value
        .get("deleted_only")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    // The page already asked; `interactive = false` keeps the server from
    // trying to read stdin.
    let result = if deleted_only {
        BackupManager::restore_deleted_only(root, number, false)
    } else {
        BackupManager::restore_backup(root, number, false)
    };
    match result {
        Ok(()) => {
            let mut state = state_json(root);
            state["message"] = json!(if deleted_only {
                format!("Restored deleted files from backup #{number}")
            } else {
                format!("Restored backup #{number}")
            });
            json_resp(stream, 200, state);
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_undo(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !confirmed(&value) {
        return json_resp(stream, 400, json!({ "error": "confirmation required" }));
    }
    match BackupManager::undo_last_restore(root) {
        Ok(()) => {
            let mut state = state_json(root);
            state["message"] = json!("Last restore undone");
            json_resp(stream, 200, state);
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_clear(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !confirmed(&value) {
        return json_resp(stream, 400, json!({ "error": "confirmation required" }));
    }
    match BackupManager::clear_backups(root) {
        Ok(()) => {
            let mut state = state_json(root);
            state["message"] = json!("All backups deleted");
            json_resp(stream, 200, state);
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_recreate(stream: &mut TcpStream, body: &[u8], root: &Path) {
    let Some(value) = parse_body(body) else {
        return json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
    };
    if !confirmed(&value) {
        return json_resp(stream, 400, json!({ "error": "confirmation required" }));
    }
    let result = BackupManager::clear_backups(root)
        .and_then(|_| BackupManager::clear_undo_history(root))
        .and_then(|_| BackupManager::create_backup(root));
    match result {
        Ok(_) => {
            let mut state = state_json(root);
            state["message"] = json!("Backups cleared and a fresh one created");
            json_resp(stream, 200, state);
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

// ── Diff ──────────────────────────────────────────────────────────────────

/// Hash every file the backup rules care about, keyed by `/`-separated path.
fn current_hashes(root: &Path) -> HashMap<String, String> {
    let ignored_dirs = Config::get_ignored_dirs_combined();
    let fmt_cfg = FormatConfig::from_both();
    let mut out = HashMap::new();

    let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
        if e.depth() == 0 {
            return true;
        }
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy().to_lowercase();
            return !ignored_dirs.contains(&name);
        }
        true
    });

    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = match entry.path().strip_prefix(root) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().to_string();
        if fmt_cfg.ignored_files.contains(&name) {
            continue;
        }
        let ext = entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if fmt_cfg.ignored_extensions.contains(&ext) {
            continue;
        }
        if let Ok(bytes) = std::fs::read(entry.path()) {
            out.insert(rel, format!("{:x}", Sha256::digest(&bytes)));
        }
    }
    out
}

/// Compare the current tree with one backup snapshot. Only file paths and
/// statuses are returned; the page renders the summary.
fn diff_json(root: &Path, number: usize) -> std::result::Result<Value, String> {
    let project_hash = compute_project_hash(root);
    let manifest_path = BackupIndex::get_backup_path(&project_hash, number).join(".manifest.json");
    if !manifest_path.exists() {
        return Err(format!("backup #{} not found", number));
    }
    let manifest = BackupManifest::load(&manifest_path).map_err(|e| e.to_string())?;

    let mut backup: HashMap<String, String> = HashMap::new();
    for entry in &manifest.files {
        backup.insert(
            entry.rel_path.to_string_lossy().replace('\\', "/"),
            entry.hash.clone(),
        );
    }
    let current = current_hashes(root);

    let mut modified: Vec<String> = Vec::new();
    let mut deleted: Vec<String> = Vec::new();
    let mut new: Vec<String> = Vec::new();
    let mut unchanged = 0usize;

    for (path, hash) in &backup {
        match current.get(path) {
            Some(current_hash) if current_hash == hash => unchanged += 1,
            Some(_) => modified.push(path.clone()),
            None => deleted.push(path.clone()),
        }
    }
    for path in current.keys() {
        if !backup.contains_key(path) {
            new.push(path.clone());
        }
    }
    modified.sort();
    deleted.sort();
    new.sort();

    let truncated = modified.len() + deleted.len() + new.len() > MAX_DIFF_FILES;
    Ok(json!({
        "number": number,
        "counts": {
            "modified": modified.len(),
            "new": new.len(),
            "deleted": deleted.len(),
            "unchanged": unchanged,
        },
        "modified": modified.into_iter().take(MAX_DIFF_FILES).collect::<Vec<_>>(),
        "new": new.into_iter().take(MAX_DIFF_FILES).collect::<Vec<_>>(),
        "deleted": deleted.into_iter().take(MAX_DIFF_FILES).collect::<Vec<_>>(),
        "truncated": truncated,
    }))
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

fn query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == key {
            return Some(v.replace("%20", " "));
        }
    }
    None
}

fn print_backup_help() {
    println!("{}", t!("cmd.serve.backup_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.backup_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!(
        "  {}",
        t!(
            "cmd.serve.backup_help_port",
            Config::global_get_serve_port(ServeService::Backup)
        )
    );
    println!(
        "  {}",
        t!(
            "cmd.serve.backup_help_bind",
            Config::global_get_serve_bind(ServeService::Backup)
        )
    );
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.backup_example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_is_the_backup_gui() {
        assert!(PAGE.contains("ntc backup"));
        assert!(PAGE.contains("/api/restore"));
        assert!(PAGE.contains("/api/diff"));
        assert!(!PAGE.contains("<script src="));
    }

    #[test]
    fn state_reports_the_pinned_folder() {
        let dir = tempfile::tempdir().unwrap();
        let state = state_json(dir.path());
        assert!(state["root"].as_str().unwrap().len() > 0);
        assert!(state["backups"].as_array().unwrap().is_empty());
        assert_eq!(state["undo_available"], false);
    }

    #[test]
    fn diff_rejects_missing_backup() {
        let dir = tempfile::tempdir().unwrap();
        assert!(diff_json(dir.path(), 99).is_err());
    }

    #[test]
    fn confirmation_is_required() {
        assert!(!confirmed(&json!({})));
        assert!(!confirmed(&json!({ "confirm": false })));
        assert!(confirmed(&json!({ "confirm": true })));
    }
}
