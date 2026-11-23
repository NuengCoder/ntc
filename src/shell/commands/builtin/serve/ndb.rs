//! `serve ndb` - a small SQLite database server for the current folder.
//!
//! One embedded page (no React build step) plus a small JSON API. The start
//! folder is pinned at startup and the page lists every readable database in
//! it; each request names one of those files with `db=<name>`, and
//! [`resolve_db`] keeps every other path (traversal, subfolders, symlinks out)
//! out of reach.
//!
//! Reads stay the default: every connection is opened read-only. With
//! `--write`, one row-level DML statement at a time may run through
//! [`db::is_write_sql`] after a `<name>.bak.<stamp>` copy was taken once per
//! file for the whole process.
//!
//! Size note: kept as one file (~1k lines) because the read-only guard, the
//! backup-on-first-write rule and the request routing form one security
//! boundary; splitting them would make the path checks easier to bypass.

use super::share::{print_share_block, public_base_url};
use super::tunnel::spawn_cloudflared;
use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::builtin::ndb::ndb::db::{self, Database, QueryResult};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::{
    auth_matches, basic_expectation, parse_auth, respond, respond_unauthorized, url_decode,
    write_all_retry,
};
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// The whole UI lives in one file, like `arc_web.html`.
const PAGE: &str = include_str!("assets/ndb_web.html");

const DEFAULT_LIMIT: usize = 50;
const MAX_ROW_LIMIT: usize = 5_000;
const MAX_EXPORT_ROWS: usize = 10_000;
const MAX_OFFSET: usize = 100_000_000;
/// SQL body cap for `POST /api/query`.
const MAX_BODY: usize = 256 * 1024;

// ── App state ─────────────────────────────────────────────────────────────

/// Everything one server process knows: the pinned folder, whether writes were
/// enabled, and the backups already taken per database file.
struct App {
    cwd: PathBuf,
    writable: bool,
    /// `<db path>` -> `<name>.bak.<stamp>` taken before its first write.
    backups: Mutex<HashMap<PathBuf, String>>,
}

impl App {
    fn new(cwd: PathBuf, writable: bool) -> Self {
        Self {
            cwd,
            writable,
            backups: Mutex::new(HashMap::new()),
        }
    }
}

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve_ndb(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_ndb_help();
        return Ok(false);
    }

    // Defaults come from `serve config ndb` (alias: `config serve ndb`).
    let mut port = Config::global_get_serve_port(ServeService::Ndb);
    let mut bind = Config::global_get_serve_bind(ServeService::Ndb);
    let mut auth: Option<(String, String)> = None;
    let mut use_tunnel = false;
    let mut share_mode = false;
    let mut share_path: Option<String> = None;
    let mut no_qr = false;
    let mut want_qr = false;
    let mut writable = false;

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
            "-t" | "--tunnel" => use_tunnel = true,
            "--write" => writable = true,
            "--share" => {
                share_mode = true;
                if let Some(next) = tokens.get(i + 1) {
                    if !next.starts_with('-') {
                        share_path = Some(next.clone());
                        i += 1;
                    }
                }
            }
            "--qr" => {
                want_qr = true;
                share_mode = true;
            }
            "--no-qr" => no_qr = true,
            "--auth" => {
                i += 1;
                let value = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_auth")))?;
                auth = Some(parse_auth(value)?);
            }
            "--" => break,
            other if other.starts_with('-') => {
                anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
            }
            // The positional `<file.db>` is gone: the file is picked in the page.
            other => anyhow::bail!("{}", t!("cmd.serve.ndb_flags_only", other)),
        }
        i += 1;
    }

    // The start folder is pinned for the whole process; the listing is
    // rescanned per request, but the boundary never moves.
    let cwd = std::fs::canonicalize(nav.current_path())
        .unwrap_or_else(|_| nav.current_path().to_path_buf());
    let files: Vec<_> = db::list_db_files(&cwd)
        .into_iter()
        .filter(|f| f.is_database())
        .collect();
    if files.is_empty() {
        anyhow::bail!(t!("cmd.serve.ndb_no_db", display_path(&cwd)));
    }

    // A database is content: never expose it to the LAN without a login.
    if !is_loopback(&bind) && auth.is_none() {
        anyhow::bail!("{}", t!("cmd.serve.ndb_requires_auth", bind));
    }

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    let base = public_base_url(&bind, port);
    print_info(&t!(
        "cmd.serve.ndb_serving",
        display_path(&cwd),
        base.as_str()
    ));
    if writable {
        print_info(&t!("cmd.serve.ndb_write_on"));
    } else {
        print_info(&t!("cmd.serve.ndb_readonly"));
    }
    if let Some((ref user, _)) = auth {
        print_info(&t!("cmd.serve.auth_on", user));
    }
    if is_loopback(&bind) {
        print_info(&t!("cmd.serve.ndb_local_warning"));
    }
    if share_mode {
        print_share_block(port, &bind, share_path.as_deref(), !no_qr || want_qr);
    }
    if use_tunnel {
        spawn_cloudflared(port);
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    crate::shell::commands::init_ctrlc_handler();
    crate::shell::commands::register_shutdown_flag(shutdown.clone());
    listener.set_nonblocking(true)?;

    let app = Arc::new(App::new(cwd, writable));
    let auth = auth.map(|(u, p)| Arc::new((u, p)));
    while !shutdown.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let app = app.clone();
                let auth = auth.clone();
                std::thread::spawn(move || handle(stream, &app, auth));
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

fn handle(mut stream: TcpStream, app: &App, auth: Option<Arc<(String, String)>>) {
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

    let mut authorization = String::new();
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
            let key = k.trim();
            if key.eq_ignore_ascii_case("authorization") {
                authorization = v.trim().to_string();
            } else if key.eq_ignore_ascii_case("content-length") {
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

    // With `--auth`, every route (page included) answers 401 until the browser
    // prompts natively. There is no in-page login card to fall back on.
    if let Some(creds) = auth {
        let expected = basic_expectation(&creds.0, &creds.1);
        if !auth_matches(&authorization, &expected) {
            respond_unauthorized(&mut stream);
            return;
        }
    }

    let path = url_decode(raw_path);
    route(&mut stream, &method, &path, query, &body, app);
}

fn route(stream: &mut TcpStream, method: &str, path: &str, query: &str, body: &[u8], app: &App) {
    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/api/files") => api_files(stream, app),
        ("GET", "/api/info") => api_info(stream, query, app),
        ("GET", "/api/tables") => api_tables(stream, query, app),
        ("GET", "/api/schema") => api_schema(stream, query, app),
        ("GET", "/api/rows") => api_rows(stream, query, app),
        ("POST", "/api/query") => api_query(stream, body, app),
        ("GET", "/api/export") => api_export(stream, query, app),
        // A known route with the wrong method is a 405; anything else is a
        // genuine 404 (so a typo in the client is not misreported).
        _ if is_known_route(path) => {
            json_resp(stream, 405, json!({ "error": "method not allowed" }));
        }
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

fn is_known_route(path: &str) -> bool {
    matches!(
        path,
        "/" | "/index.html"
            | "/api/files"
            | "/api/info"
            | "/api/tables"
            | "/api/schema"
            | "/api/rows"
            | "/api/query"
            | "/api/export"
    )
}

// ── Safe database resolution ──────────────────────────────────────────────

/// Turn a `db=<name>` value into a canonical path inside `cwd`.
///
/// Rejects empty names, absolute paths, drive letters, alternate data streams,
/// `/` or `\` (so no subfolders), `.`/`..` and anything that canonicalises or
/// symlinks outside the pinned folder. The `.db` / `.sqlite` / `.sqlite3`
/// extension is required, which also excludes the `.bak.<stamp>` copies.
fn resolve_db(cwd: &Path, name: &str) -> Result<PathBuf> {
    let name = name.trim();
    if name.is_empty() {
        anyhow::bail!("missing db");
    }
    if name.contains('/') || name.contains('\\') {
        anyhow::bail!("invalid database name");
    }
    if name == "." || name == ".." || name.contains(':') {
        anyhow::bail!("invalid database name");
    }
    let candidate = cwd.join(name);
    let canon = candidate
        .canonicalize()
        .with_context(|| format!("database '{}' not found", name))?;
    if canon.parent() != Some(cwd) {
        anyhow::bail!("database '{}' is outside the served folder", name);
    }
    let ext = canon
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default();
    if !db::DB_EXTENSIONS
        .iter()
        .any(|e| ext.eq_ignore_ascii_case(e))
    {
        anyhow::bail!("only .db, .sqlite and .sqlite3 files can be opened");
    }
    Ok(canon)
}

fn require_db(stream: &mut TcpStream, query: &str, app: &App) -> Option<PathBuf> {
    let name = query_param(query, "db");
    match name {
        Some(name) => match resolve_db(&app.cwd, &name) {
            Ok(path) => Some(path),
            Err(e) => {
                json_resp(stream, 400, json!({ "error": e.to_string() }));
                None
            }
        },
        None => {
            json_resp(stream, 400, json!({ "error": "missing db" }));
            None
        }
    }
}

fn open_checked(stream: &mut TcpStream, path: &Path) -> Option<Database> {
    match Database::open(path) {
        Ok(db) => Some(db),
        Err(e) => {
            json_resp(stream, 400, json!({ "error": e.to_string() }));
            None
        }
    }
}

/// Take (once per file for the process) the pre-write backup and return its
/// display name. Later writes reuse the name of the first copy.
fn ensure_backup(app: &App, path: &Path) -> Result<String> {
    let mut backups = app
        .backups
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(name) = backups.get(path) {
        return Ok(name.clone());
    }
    let dest = db::backup_file(path)?;
    let name = file_name(&dest);
    backups.insert(path.to_path_buf(), name.clone());
    Ok(name)
}

// ── API ───────────────────────────────────────────────────────────────────

fn api_files(stream: &mut TcpStream, app: &App) {
    let mut list: Vec<Value> = Vec::new();
    for file in db::list_db_files(&app.cwd)
        .into_iter()
        .filter(|f| f.is_database())
    {
        list.push(json!({
            "name": file.name,
            "size": file.size,
            "tables": file.table_count.unwrap_or(0),
            "modified": file
                .path
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }));
    }
    json_resp(
        stream,
        200,
        json!({
            "dir": display_path(&app.cwd),
            "writable": app.writable,
            "files": list,
        }),
    );
}

fn api_info(stream: &mut TcpStream, query: &str, app: &App) {
    let Some(path) = require_db(stream, query, app) else {
        return;
    };
    let Some(db) = open_checked(stream, &path) else {
        return;
    };
    let info = db.info();
    let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    json_resp(
        stream,
        200,
        json!({
            "file": file_name(&path),
            "path": display_path(&path),
            "dir": display_path(&app.cwd),
            "size": size,
            "writable": app.writable,
            "sqlite_version": info.sqlite_version,
            "page_size": info.page_size,
            "page_count": info.page_count,
            "tables": info.tables,
            "views": info.views,
            "indexes": info.indexes,
            "triggers": info.triggers,
        }),
    );
}

fn api_tables(stream: &mut TcpStream, query: &str, app: &App) {
    let Some(path) = require_db(stream, query, app) else {
        return;
    };
    let Some(db) = open_checked(stream, &path) else {
        return;
    };
    match db.tables() {
        Ok(tables) => {
            let list: Vec<Value> = tables
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "kind": t.kind,
                        "columns": t.columns,
                        "rows": t.rows,
                    })
                })
                .collect();
            json_resp(stream, 200, json!({ "tables": list }));
        }
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_schema(stream: &mut TcpStream, query: &str, app: &App) {
    let Some(table) = query_param(query, "table").filter(|t| !t.is_empty()) else {
        return json_resp(stream, 400, json!({ "error": "missing table" }));
    };
    let Some(path) = require_db(stream, query, app) else {
        return;
    };
    let Some(db) = open_checked(stream, &path) else {
        return;
    };
    let Some(create_sql) = db.create_sql(&table) else {
        return json_resp(stream, 400, json!({ "error": "unknown table" }));
    };
    let columns: Vec<Value> = db
        .columns(&table)
        .unwrap_or_default()
        .iter()
        .map(|c| {
            json!({
                "name": c.name,
                "type": c.col_type,
                "not_null": c.not_null,
                "primary_key": c.primary_key,
                "constraint": c.constraint(),
            })
        })
        .collect();
    let indexes: Vec<Value> = db
        .indexes(&table)
        .unwrap_or_default()
        .iter()
        .map(|i| json!({ "name": i.name, "unique": i.unique, "sql": i.sql }))
        .collect();
    let foreign_keys: Vec<Value> = db
        .foreign_keys(&table)
        .unwrap_or_default()
        .iter()
        .map(|f| json!({ "table": f.table, "from": f.from, "to": f.to }))
        .collect();
    let triggers: Vec<Value> = db.triggers(&table).into_iter().map(Value::String).collect();
    json_resp(
        stream,
        200,
        json!({
            "table": table,
            "create_sql": create_sql,
            "columns": columns,
            "indexes": indexes,
            "foreign_keys": foreign_keys,
            "triggers": triggers,
        }),
    );
}

fn api_rows(stream: &mut TcpStream, query: &str, app: &App) {
    let Some(table) = query_param(query, "table").filter(|t| !t.is_empty()) else {
        return json_resp(stream, 400, json!({ "error": "missing table" }));
    };
    let limit = parse_limit(query_param(query, "limit"), DEFAULT_LIMIT, MAX_ROW_LIMIT);
    let offset = parse_offset(query_param(query, "offset"));
    let Some(path) = require_db(stream, query, app) else {
        return;
    };
    let Some(db) = open_checked(stream, &path) else {
        return;
    };
    if db.create_sql(&table).is_none() {
        return json_resp(stream, 400, json!({ "error": "unknown table" }));
    }
    let sql = format!(
        "SELECT * FROM {} LIMIT {} OFFSET {}",
        db::quote_ident(&table),
        limit + 1,
        offset
    );
    match db.query(&sql, limit) {
        Ok(res) => json_resp(
            stream,
            200,
            json!({
                "table": table,
                "limit": limit,
                "offset": offset,
                "columns": res.columns,
                "rows": res.rows,
                "truncated": res.truncated,
                "elapsed_ms": res.elapsed_ms,
            }),
        ),
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

/// The write gate, kept separate so it can be tested without sockets.
/// `wants_write` comes from `mode: "write"` in the request body.
fn check_query(sql: &str, wants_write: bool, writable: bool) -> Result<(), String> {
    if !single_statement(sql) {
        return Err("run one statement at a time".to_string());
    }
    if wants_write {
        if !writable {
            return Err(
                "read-only server: start `serve ndb --write` to allow INSERT, UPDATE, DELETE and REPLACE"
                    .to_string(),
            );
        }
        if !db::is_write_sql(sql) {
            return Err("only INSERT, UPDATE, DELETE and REPLACE can be written".to_string());
        }
    } else if !db::is_read_only_sql(sql) {
        return Err(
            "read-only query: only SELECT, PRAGMA, EXPLAIN, WITH, VALUES and TABLE are allowed"
                .to_string(),
        );
    }
    Ok(())
}

fn api_query(stream: &mut TcpStream, body: &[u8], app: &App) {
    let value: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_resp(stream, 400, json!({ "error": "invalid JSON body" })),
    };
    let Some(raw_db) = value.get("db").and_then(|v| v.as_str()) else {
        return json_resp(stream, 400, json!({ "error": "missing db" }));
    };
    let sql = value
        .get("sql")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if sql.is_empty() {
        return json_resp(stream, 400, json!({ "error": "missing sql" }));
    }
    let wants_write = value.get("mode").and_then(|v| v.as_str()) == Some("write");
    if let Err(message) = check_query(sql, wants_write, app.writable) {
        return json_resp(stream, 400, json!({ "error": message }));
    }
    let path = match resolve_db(&app.cwd, raw_db) {
        Ok(p) => p,
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };
    let mut db = match Database::open(&path) {
        Ok(db) => db,
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };

    if wants_write {
        let backup = match ensure_backup(app, &path) {
            Ok(name) => name,
            Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
        };
        if let Err(e) = db.make_writable() {
            return json_resp(stream, 400, json!({ "error": e.to_string() }));
        }
        return match db.exec_write(sql) {
            Ok(n) => json_resp(stream, 200, json!({ "written": n, "backup": backup })),
            Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
        };
    }

    let limit = value
        .get("limit")
        .and_then(|v| v.as_u64())
        .map(|n| (n as usize).clamp(1, MAX_ROW_LIMIT))
        .unwrap_or(DEFAULT_LIMIT);
    match db.query(sql, limit) {
        Ok(res) => json_resp(
            stream,
            200,
            json!({
                "columns": res.columns,
                "rows": res.rows,
                "truncated": res.truncated,
                "elapsed_ms": res.elapsed_ms,
            }),
        ),
        Err(e) => json_resp(stream, 400, json!({ "error": e.to_string() })),
    }
}

fn api_export(stream: &mut TcpStream, query: &str, app: &App) {
    let Some(table) = query_param(query, "table").filter(|t| !t.is_empty()) else {
        return json_resp(stream, 400, json!({ "error": "missing table" }));
    };
    let fmt = query_param(query, "fmt")
        .unwrap_or_else(|| "csv".to_string())
        .to_lowercase();
    if fmt != "csv" && fmt != "json" {
        return json_resp(stream, 400, json!({ "error": "fmt must be csv or json" }));
    }
    let Some(path) = require_db(stream, query, app) else {
        return;
    };
    let Some(db) = open_checked(stream, &path) else {
        return;
    };
    if db.create_sql(&table).is_none() {
        return json_resp(stream, 400, json!({ "error": "unknown table" }));
    }
    let sql = format!(
        "SELECT * FROM {} LIMIT {}",
        db::quote_ident(&table),
        MAX_EXPORT_ROWS + 1
    );
    let res = match db.query(&sql, MAX_EXPORT_ROWS) {
        Ok(r) => r,
        Err(e) => return json_resp(stream, 400, json!({ "error": e.to_string() })),
    };
    let (content_type, body) = if fmt == "csv" {
        ("text/csv; charset=utf-8", to_csv(&res))
    } else {
        ("application/json; charset=utf-8", to_json(&res))
    };
    let filename = format!("{}.{}", safe_filename(&table), fmt);
    respond_download(stream, content_type, &filename, body.as_bytes());
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

fn respond_download(stream: &mut TcpStream, ct: &str, filename: &str, data: &[u8]) {
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        ct,
        data.len(),
        filename
    );
    let _ = write_all_retry(stream, head.as_bytes());
    let _ = write_all_retry(stream, data);
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

fn parse_limit(raw: Option<String>, default: usize, max: usize) -> usize {
    raw.and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default)
        .clamp(1, max)
}

fn parse_offset(raw: Option<String>) -> usize {
    raw.and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0)
        .min(MAX_OFFSET)
}

/// True when `sql` carries at most one statement. Semicolons inside string
/// literals or quoted identifiers are ignored; anything but whitespace after
/// the first separator makes it multi-statement.
fn single_statement(sql: &str) -> bool {
    let mut single = false;
    let mut double = false;
    let mut bracket = false;
    let mut backtick = false;
    let mut after_semi = false;
    let mut chars = sql.chars().peekable();
    while let Some(c) = chars.next() {
        let quoted = single || double || bracket || backtick;
        if !quoted && after_semi && !c.is_whitespace() {
            return false;
        }
        match c {
            '\'' if !double && !bracket && !backtick => {
                if single && chars.peek() == Some(&'\'') {
                    chars.next();
                } else {
                    single = !single;
                }
            }
            '"' if !single && !bracket && !backtick => {
                if double && chars.peek() == Some(&'"') {
                    chars.next();
                } else {
                    double = !double;
                }
            }
            '`' if !single && !double && !bracket => {
                if backtick && chars.peek() == Some(&'`') {
                    chars.next();
                } else {
                    backtick = !backtick;
                }
            }
            '[' if !single && !double && !backtick => bracket = true,
            ']' if bracket => bracket = false,
            ';' if !quoted => after_semi = true,
            _ => {}
        }
    }
    true
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn to_csv(res: &QueryResult) -> String {
    let mut out = String::new();
    out.push_str(
        &res.columns
            .iter()
            .map(|c| csv_escape(c))
            .collect::<Vec<_>>()
            .join(","),
    );
    out.push('\n');
    for row in &res.rows {
        let cells: Vec<String> = row
            .iter()
            .map(|v| v.as_deref().map(csv_escape).unwrap_or_default())
            .collect();
        out.push_str(&cells.join(","));
        out.push('\n');
    }
    out
}

fn to_json(res: &QueryResult) -> String {
    let rows: Vec<Value> = res
        .rows
        .iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            for (i, col) in res.columns.iter().enumerate() {
                let value = row
                    .get(i)
                    .cloned()
                    .flatten()
                    .map(Value::String)
                    .unwrap_or(Value::Null);
                obj.insert(col.clone(), value);
            }
            Value::Object(obj)
        })
        .collect();
    serde_json::to_string_pretty(&Value::Array(rows)).unwrap_or_else(|_| "[]".to_string())
}

fn safe_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.is_empty() {
        "table".to_string()
    } else {
        cleaned
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string())
}

fn is_loopback(bind: &str) -> bool {
    bind == "localhost" || bind == "::1" || bind.starts_with("127.")
}

/// Canonical Windows paths carry a `\\?\` verbatim prefix; strip it for display.
fn display_path(path: &Path) -> String {
    let shown = path.display().to_string();
    shown.strip_prefix("\\\\?\\").unwrap_or(&shown).to_string()
}

fn print_ndb_help() {
    println!("{}", t!("cmd.serve.ndb_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.ndb_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!(
        "  {}",
        t!(
            "cmd.serve.ndb_help_port",
            Config::global_get_serve_port(ServeService::Ndb)
        )
    );
    println!(
        "  {}",
        t!(
            "cmd.serve.ndb_help_bind",
            Config::global_get_serve_bind(ServeService::Ndb)
        )
    );
    println!("  {}", t!("cmd.serve.help_auth_http"));
    println!("  {}", t!("cmd.serve.ndb_help_write"));
    println!("  {}", t!("cmd.serve.help_tunnel"));
    println!("  {}", t!("cmd.serve.help_share"));
    println!("  {}", t!("cmd.serve.help_qr"));
    println!("  {}", t!("cmd.serve.help_no_qr"));
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.ndb_readonly"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.ndb_example"));
    println!();
    println!(
        "  {}",
        t!(
            "cmd.serve.current_defaults",
            Config::global_get_serve_port(ServeService::Ndb),
            Config::global_get_serve_bind(ServeService::Ndb)
        )
        .dimmed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_result() -> QueryResult {
        QueryResult {
            columns: vec!["id".to_string(), "name".to_string()],
            rows: vec![
                vec![Some("1".to_string()), Some("Ann".to_string())],
                vec![Some("2".to_string()), None],
            ],
            truncated: false,
            elapsed_ms: 0,
        }
    }

    fn make_db(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
             INSERT INTO users (name) VALUES ('ann'), ('bob');",
        )
        .unwrap();
        path
    }

    #[test]
    fn single_statement_accepts_one_and_trailing_semicolon() {
        assert!(single_statement("SELECT 1"));
        assert!(single_statement("SELECT 1;"));
        assert!(single_statement("SELECT 1;   \n"));
        assert!(single_statement("SELECT ';' FROM t"));
        assert!(single_statement("SELECT \"a;b\" FROM t"));
    }

    #[test]
    fn single_statement_rejects_a_second_statement() {
        assert!(!single_statement("SELECT 1; DROP TABLE users"));
        assert!(!single_statement("SELECT 1; SELECT 2"));
        assert!(!single_statement("SELECT 1;;"));
    }

    #[test]
    fn resolve_db_rejects_unsafe_names() {
        let tmp = tempfile::tempdir().unwrap();
        let cwd = tmp.path().canonicalize().unwrap();
        make_db(&cwd, "app.db");
        for name in [
            "",
            " ",
            ".",
            "..",
            "../x.db",
            "sub/x.db",
            "sub\\x.db",
            "C:\\x.db",
            "C:app.db",
            "app.db.bak.20260101-000000",
            "notes.txt",
        ] {
            assert!(
                resolve_db(&cwd, name).is_err(),
                "should be rejected: {name:?}"
            );
        }
        let ok = resolve_db(&cwd, "app.db").unwrap();
        assert_eq!(ok, cwd.join("app.db"));
    }

    #[test]
    fn listing_filter_skips_non_databases() {
        let tmp = tempfile::tempdir().unwrap();
        let cwd = tmp.path().canonicalize().unwrap();
        make_db(&cwd, "app.db");
        std::fs::write(cwd.join("notes.txt"), b"not a db").unwrap();
        std::fs::write(cwd.join("bogus.db"), b"not sqlite").unwrap();
        let listed: Vec<_> = db::list_db_files(&cwd)
            .into_iter()
            .filter(|f| f.is_database())
            .collect();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].name, "app.db");
    }

    #[test]
    fn backup_is_taken_once_per_file() {
        let tmp = tempfile::tempdir().unwrap();
        let cwd = tmp.path().canonicalize().unwrap();
        let path = make_db(&cwd, "app.db");
        let app = App::new(cwd.clone(), true);

        let first = ensure_backup(&app, &path).unwrap();
        assert!(first.starts_with("app.db.bak."));
        let second = ensure_backup(&app, &path).unwrap();
        assert_eq!(first, second, "later writes reuse the first backup");

        let backups = std::fs::read_dir(&cwd)
            .unwrap()
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().contains(".bak."))
            .count();
        assert_eq!(backups, 1, "only one backup copy per file and process");
    }

    #[test]
    fn write_gate_requires_write_flag_and_dml() {
        let insert = "INSERT INTO users (name) VALUES ('x')";
        // Read-only server refuses a write outright.
        assert!(check_query(insert, true, false).is_err());
        // `--write` accepts row-level DML only.
        assert!(check_query(insert, true, true).is_ok());
        assert!(check_query("DELETE FROM users", true, true).is_ok());
        assert!(check_query("UPDATE users SET name='x'", true, true).is_ok());
        assert!(check_query("REPLACE INTO users (id) VALUES (1)", true, true).is_ok());
        assert!(check_query("DROP TABLE users", true, true).is_err());
        assert!(check_query("ALTER TABLE users ADD COLUMN z TEXT", true, true).is_err());
        assert!(check_query("VACUUM", true, true).is_err());
        assert!(check_query(
            "INSERT INTO users (name) VALUES ('x'); DROP TABLE users",
            true,
            true
        )
        .is_err());
        // Write SQL is never accepted through the read console.
        assert!(check_query(insert, false, true).is_err());
        assert!(check_query("SELECT * FROM users", false, false).is_ok());
    }

    #[test]
    fn csv_escapes_quotes_commas_and_newlines() {
        assert_eq!(csv_escape("plain"), "plain");
        assert_eq!(csv_escape("a,b"), "\"a,b\"");
        assert_eq!(csv_escape("a\"b"), "\"a\"\"b\"");
        assert_eq!(csv_escape("a\nb"), "\"a\nb\"");
    }

    #[test]
    fn csv_renders_null_as_empty_cell() {
        assert_eq!(to_csv(&sample_result()), "id,name\n1,Ann\n2,\n");
    }

    #[test]
    fn json_renders_null_as_null() {
        let rendered = to_json(&sample_result());
        let parsed: Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(parsed[0]["name"], "Ann");
        assert_eq!(parsed[1]["name"], Value::Null);
    }

    #[test]
    fn export_filename_is_ascii_safe() {
        assert_eq!(safe_filename("users"), "users");
        assert_eq!(safe_filename("a/b c"), "a_b_c");
        assert_eq!(safe_filename(""), "table");
    }

    #[test]
    fn page_has_picker_and_no_login_form() {
        assert!(PAGE.contains("NTC Database"));
        assert!(PAGE.contains("picker"));
        assert!(PAGE.contains("Change database"));
        assert!(!PAGE.contains("login-form"));
        assert!(!PAGE.contains("sessionStorage"));
    }

    #[test]
    fn page_is_ascii_only() {
        assert!(PAGE.is_ascii(), "the embedded page must stay ASCII-only");
    }
}
