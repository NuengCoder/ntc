use anyhow::{Context, Result};
use colored::*;
use rusqlite::params;
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};

#[derive(rust_embed::Embed)]
#[folder = "ntc_web/dist"]
struct Frontend;

// ── SQLite ────────────────────────────────────────────────────────────────

fn db_path() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("ntc").join("serve.db")
}
fn files_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("ntc").join("serve").join("files")
}
fn ensure_db() -> Result<rusqlite::Connection> {
    let path = db_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::create_dir_all(files_dir())?;
    let conn = rusqlite::Connection::open(&path)?;
    conn.execute_batch("CREATE TABLE IF NOT EXISTS files (
        id INTEGER PRIMARY KEY AUTOINCREMENT, filename TEXT NOT NULL,
        path TEXT NOT NULL DEFAULT '/', is_dir INTEGER DEFAULT 0,
        size INTEGER DEFAULT 0, disk_name TEXT, data BLOB,
        uploaded_at TEXT DEFAULT (datetime('now')))")?;
    // Migration: add data column for existing databases
    let _ = conn.execute_batch("ALTER TABLE files ADD COLUMN data BLOB");
    Ok(conn)
}

// ── Tokenize ──────────────────────────────────────────────────────────────

fn tokenize(input: &str) -> Vec<String> {
    let mut v = Vec::new(); let mut c = String::new(); let mut s = false; let mut d = false;
    for ch in input.chars() {
        match ch {
            '\'' if !d => s = !s, '"' if !s => d = !d,
            ' ' | '\t' if !s && !d => { if !c.is_empty() { v.push(std::mem::take(&mut c)); } }
            _ => c.push(ch),
        }
    }
    if !c.is_empty() { v.push(c); }
    v
}

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || matches!(tokens[0].as_str(), "help" | "--help" | "-h") {
        print_serve_help(); return Ok(false);
    }
    let mut port = 8080u16; let mut bind = "0.0.0.0".to_string();
    let mut use_tunnel = false; let mut spa = false; let mut web_dir = String::new();

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-p" | "--port" => { i+=1; port = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing port"))?.parse().map_err(|_| anyhow::anyhow!("Invalid port"))?; }
            "-b" | "--bind" => { i+=1; bind = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing bind"))?.clone(); }
            "-t" | "--tunnel" => { use_tunnel = true; }
            "--spa" => { spa = true; }
            "-w" | "--web-dir" => { i+=1; web_dir = tokens.get(i).ok_or_else(|| anyhow::anyhow!("Missing dir"))?.clone(); }
            _ => { if let Ok(p) = tokens[i].parse::<u16>() { port = p; } else { anyhow::bail!("Unknown: {}", tokens[i]); } }
        }
        i += 1;
    }

    let db = Arc::new(Mutex::new(ensure_db()?));
    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    listener.set_nonblocking(false)?;
    print_info(&format!("HTTP server on {}", format!("http://{}", addr).cyan()));

    if use_tunnel { spawn_cloudflared(port); }

    let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let sd = shutdown.clone();
    ctrlc::set_handler(move || {
        sd.store(true, std::sync::atomic::Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");

    let web = if web_dir.is_empty() { None } else { Some(PathBuf::from(web_dir)) };
    listener.set_nonblocking(true)?;

    while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let db = db.clone();
                let web = web.clone();
                std::thread::spawn(move || handle_connection(stream, db, spa, web));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => print_warning(&format!("Accept: {}", e)),
        }
    }
    print_info("Server stopped.");
    crate::shell::commands::init_ctrlc_handler();
    Ok(false)
}

// ── Connection handler ────────────────────────────────────────────────────

fn handle_connection(
    mut stream: TcpStream,
    db: Arc<Mutex<rusqlite::Connection>>,
    spa_mode: bool,
    web_dir: Option<PathBuf>,
) {
    let _peer = stream.peer_addr().ok();
    let mut reader = std::io::BufReader::new(&mut stream);

    // Read request line
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() { return; }
    let request_line = request_line.trim();
    if request_line.is_empty() { return; }

    let parts: Vec<&str> = request_line.splitn(3, ' ').collect();
    if parts.len() < 2 { return; }
    let method = parts[0];
    let full_path = parts[1];
    let (path, query) = full_path.split_once('?').unwrap_or((full_path, ""));

    // Read headers
    let mut headers = HashMap::new();
    let mut content_length: usize = 0;
    let mut boundary = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() { return; }
        let line = line.trim();
        if line.is_empty() { break; }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_lowercase();
            let val = v.trim().to_string();
            if key == "content-length" { content_length = val.parse().unwrap_or(0); }
            if key == "content-type" && val.contains("multipart/form-data") {
                if let Some(b) = val.split("boundary=").nth(1) {
                    boundary = b.trim().to_string();
                }
            }
            headers.insert(key, val);
        }
    }

    // Read body (only for methods that have one)
    let mut body = Vec::new();
    let has_body = method == "POST" || method == "PUT" || method == "PATCH";
    if has_body {
        let max_body = MAX_UPLOAD as usize;
        if content_length > 0 && content_length > max_body {
            respond(&mut stream, 413, "text/plain", b"413 Payload Too Large");
            return;
        }
        let mut buf = vec![0u8; 65536];
        let mut total = 0usize;
        let need = if content_length > 0 { content_length } else { usize::MAX };
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
        loop {
            if std::time::Instant::now() > deadline {
                respond(&mut stream, 408, "text/plain", b"408 Request Timeout");
                return;
            }
            reader.get_ref().set_read_timeout(Some(std::time::Duration::from_secs(10))).ok();
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    total += n;
                    if total > max_body {
                        respond(&mut stream, 413, "text/plain", b"413 Payload Too Large");
                        return;
                    }
                    body.extend_from_slice(&buf[..n]);
                    if total >= need { break; }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(_) => return,
            }
        }
    }

    // Route
    let response = if path.starts_with("/api/") {
        handle_api(method, path, query, &body, &boundary, db)
    } else {
        serve_static(path, spa_mode, &web_dir)
    };

    let _ = stream.write_all(&response);
}

fn respond(stream: &mut TcpStream, status: u16, ct: &str, data: &[u8]) {
    let reason = match status { 200 => "OK", 400 => "Bad Request", 404 => "Not Found", 409 => "Conflict", 413 => "Payload Too Large", 500 => "Internal Server Error", _ => "Error" };
    let resp = format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", status, reason, ct, data.len());
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.write_all(data);
}

fn response_bytes(status: u16, ct: &str, data: Vec<u8>) -> Vec<u8> {
    let reason = match status { 200 => "OK", 400 => "Bad Request", 404 => "Not Found", 409 => "Conflict", 413 => "Payload Too Large", 500 => "Internal Server Error", _ => "Error" };
    let head = format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", status, reason, ct, data.len());
    let mut r = head.into_bytes(); r.extend(data); r
}

fn json_error(status: u16) -> Vec<u8> {
    let reason = match status { 400 => "Bad Request", 404 => "Not Found", 409 => "Conflict", 413 => "Payload Too Large", 500 => "Internal Server Error", _ => "Error" };
    let body = format!(r#"{{"error":"{}"}}"#, reason);
    response_bytes(status, "application/json", body.into_bytes())
}

fn json_ok(data: Vec<u8>) -> Vec<u8> { response_bytes(200, "application/json", data) }

// ── Static file serving ───────────────────────────────────────────────────

fn serve_static(path: &str, spa_mode: bool, web_dir: &Option<PathBuf>) -> Vec<u8> {
    let clean = if path == "/" { "index.html" } else { path.trim_start_matches('/') };

    if let Some(dir) = web_dir {
        let fp = dir.join(clean);
        if fp.is_file() { if let Ok(d) = std::fs::read(&fp) { return response_bytes(200, mime_for(clean), d); } }
    }

    if let Some(f) = Frontend::get(clean) { return response_bytes(200, mime_for(clean), f.data.to_vec()); }

    let alt = format!("/{}", clean).trim_start_matches('/').to_string();
    if let Some(f) = Frontend::get(&alt) { return response_bytes(200, mime_for(&alt), f.data.to_vec()); }

    if spa_mode { if let Some(f) = Frontend::get("index.html") { return response_bytes(200, "text/html; charset=utf-8", f.data.to_vec()); } }

    json_error(404)
}

fn mime_for(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html"|"htm" => "text/html; charset=utf-8", "css" => "text/css; charset=utf-8",
        "js"|"mjs" => "application/javascript", "json" => "application/json",
        "png" => "image/png", "jpg"|"jpeg" => "image/jpeg", "gif" => "image/gif",
        "svg" => "image/svg+xml", "ico" => "image/x-icon", "woff2" => "font/woff2",
        _ => "application/octet-stream",
    }
}

// ── API router ────────────────────────────────────────────────────────────

fn handle_api(method: &str, path: &str, query: &str, body: &[u8], boundary: &str, db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    match (method, path) {
        ("GET", "/api/files") => api_list_files(query, db),
        ("GET", "/api/download") => api_download(query, db),
        ("POST", "/api/upload") => api_upload(body, boundary, db),
        ("POST", "/api/create-folder") => api_create_folder(body, db),
        ("DELETE", "/api/delete") => api_delete(query, db),
        ("PATCH", "/api/rename") => api_rename(body, db),
        _ => json_error(404),
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for pair in query.split('&').filter(|s| !s.is_empty()) {
        if let Some((k, v)) = pair.split_once('=') { m.insert(url_decode(k), url_decode(v)); }
    }
    m
}

fn url_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut cs = s.chars();
    while let Some(c) = cs.next() {
        if c == '%' { let h: String = cs.by_ref().take(2).collect();
            if let Ok(b) = u8::from_str_radix(&h, 16) { out.push(b as char); } else { out.push('%'); out.push_str(&h); }
        } else if c == '+' { out.push(' '); } else { out.push(c); }
    }
    out
}

fn lock(db: &Arc<Mutex<rusqlite::Connection>>) -> std::sync::MutexGuard<'_, rusqlite::Connection> {
    db.lock().unwrap_or_else(|e| e.into_inner())
}

const MAX_UPLOAD: u64 = 1024 * 1024 * 1024;

fn is_executable(name: &str) -> bool {
    let low = name.to_lowercase();
    let exts = [".exe", ".dll", ".msi", ".bat", ".cmd", ".ps1", ".apk", ".ipa",
        ".sh", ".bin", ".jar", ".vb", ".vbs", ".wasm", ".appimage", ".deb",
        ".rpm", ".msix", ".msp", ".scr", ".cpl", ".gadget", ".wsf", ".pyz"];
    exts.iter().any(|e| low.ends_with(e))
}

// ── API: List files ───────────────────────────────────────────────────────

fn api_list_files(query: &str, db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let q = parse_query(query);
    let p = q.get("path").map(|s| s.as_str()).unwrap_or("/");
    let db = lock(&db);
    let mut stmt = match db.prepare("SELECT id, filename, is_dir, size FROM files WHERE path = ?1 ORDER BY is_dir DESC, filename ASC") {
        Ok(s) => s, Err(_) => return json_error(500),
    };
    let items: Vec<serde_json::Value> = match stmt.query_map(params![p], |row| {
        Ok(serde_json::json!({"id": row.get::<_,i64>(0)?, "name": row.get::<_,String>(1)?, "isDir": row.get::<_,i32>(2)? == 1, "size": row.get::<_,i64>(3)?}))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    json_ok(serde_json::to_vec(&serde_json::json!({"path": p, "items": items})).unwrap())
}

// ── API: Download ─────────────────────────────────────────────────────────

fn api_download(query: &str, db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let q = parse_query(query);
    let id: i64 = match q.get("id").and_then(|s| s.parse().ok()) { Some(i) => i, None => return json_error(400) };
    let db = lock(&db);
    let (filename, disk_name, blob): (String, Option<String>, Option<Vec<u8>>) = match db.query_row(
        "SELECT filename, disk_name, data FROM files WHERE id = ?1 AND is_dir = 0", params![id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    ) { Ok(r) => r, Err(_) => return json_error(404) };
    let data = match (disk_name, blob) {
        (Some(d), _) => match std::fs::read(files_dir().join(&d)) { Ok(d) => d, Err(_) => return json_error(404) },
        (_, Some(b)) => b,
        _ => return json_error(404),
    };
    let ct = mime_guess::from_path(&filename).first_or_octet_stream().to_string();
    let head = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", ct, filename, data.len());
    let mut r = head.into_bytes();
    r.extend(data);
    r
}

// ── API: Upload ───────────────────────────────────────────────────────────

fn api_upload(body: &[u8], boundary: &str, db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    if body.is_empty() || boundary.is_empty() { return json_error(400); }
    if body.len() as u64 > MAX_UPLOAD { return json_error(413); }

    let mut dest_path = "/".to_string();
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();

    // Manual multipart parser
    let boundary_bytes = format!("--{}", boundary);
    let _end_boundary = format!("--{}--", boundary);

    let mut pos = 0;
    while let Some(start) = find_subseq(body, boundary_bytes.as_bytes(), pos) {
        let after_boundary = start + boundary_bytes.len();
        if after_boundary + 2 >= body.len() { break; }
        // Skip \r\n after boundary
        let content_start = if body[after_boundary..].starts_with(b"\r\n") { after_boundary + 2 } else { after_boundary };
        // Check if this is the closing boundary
        if body[content_start..].starts_with(b"--") || body[content_start..].starts_with(b"\r\n--") { break; }

        // Find the double \r\n that separates headers from body
        let header_end = find_subseq(body, b"\r\n\r\n", content_start).unwrap_or(body.len());
        let header_section = std::str::from_utf8(&body[content_start..header_end]).unwrap_or("");

        // Find data end (next boundary)
        let data_end = find_subseq(body, b"\r\n--", header_end + 4).unwrap_or(body.len());

        // Get field data (strip trailing \r\n)
        let field_data = &body[header_end + 4..data_end];
        let field_data = field_data.strip_suffix(b"\r\n").unwrap_or(field_data);

        // Parse Content-Disposition
        let mut field_name = "";
        let mut file_name = "";
        for line in header_section.lines() {
            if line.to_lowercase().starts_with("content-disposition:") {
                for part in line.split(';') {
                    let part = part.trim();
                    if let Some(v) = part.strip_prefix("name=\"") { field_name = v.trim_end_matches('"'); }
                    if let Some(v) = part.strip_prefix("filename=\"") { file_name = v.trim_end_matches('"'); }
                }
            }
        }

        if field_name == "path" { dest_path = clean_path(std::str::from_utf8(field_data).unwrap_or("/")); }
        else if field_name == "file" { files.push((file_name.to_string(), field_data.to_vec())); }

        pos = data_end;
    }

    if files.is_empty() { return json_error(400); }

    let db = lock(&db);
    for (filename, data) in &files {
        if is_executable(filename) {
            if db.execute("INSERT INTO files (filename, path, is_dir, size, data) VALUES (?1, ?2, 0, ?3, ?4)",
                params![filename, dest_path, data.len() as i64, data]).is_err() { return json_error(500); }
        } else {
            let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
            let disk = format!("{}-{}", nanos, filename);
            if std::fs::write(files_dir().join(&disk), data).is_err() { return json_error(500); }
            if db.execute("INSERT INTO files (filename, path, is_dir, size, disk_name) VALUES (?1, ?2, 0, ?3, ?4)",
                params![filename, dest_path, data.len() as i64, disk]).is_err() { return json_error(500); }
        }
    }
    json_ok(serde_json::to_vec(&serde_json::json!({"path": dest_path, "uploaded": files.len()})).unwrap())
}

fn find_subseq(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    haystack[start..].windows(needle.len()).position(|w| w == needle).map(|i| i + start)
}

// ── API: Create folder ────────────────────────────────────────────────────

fn api_create_folder(body: &[u8], db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let j: serde_json::Value = match serde_json::from_slice(body) { Ok(v) => v, Err(_) => return json_error(400) };
    let name = match j["name"].as_str() { Some(n) => n.trim(), None => return json_error(400) };
    let req_path = j["path"].as_str().map(clean_path).unwrap_or_else(|| "/".to_string());
    if name.is_empty() || name.contains('/') || name.contains('\\') || name == "." || name == ".." { return json_error(400); }

    let db = lock(&db);
    if db.query_row("SELECT COUNT(*) FROM files WHERE path=?1 AND filename=?2 AND is_dir=1", params![req_path, name], |row| row.get::<_,i64>(0)).unwrap_or(0) > 0 { return json_error(409); }
    if db.execute("INSERT INTO files (filename, path, is_dir, size) VALUES (?1, ?2, 1, 0)", params![name, req_path]).is_err() { return json_error(500); }
    json_ok(serde_json::to_vec(&serde_json::json!({"path": format!("{}/{}", req_path.trim_end_matches('/'), name)})).unwrap())
}

// ── API: Delete ───────────────────────────────────────────────────────────

fn api_delete(query: &str, db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let q = parse_query(query);
    let target = q.get("path").map(|s| s.as_str()).unwrap_or("").to_string();
    let (dir_path, filename) = match split_path(&target) { Ok(p) => p, Err(_) => return json_error(400) };

    let db = lock(&db);
    let is_dir = db.query_row("SELECT is_dir FROM files WHERE path=?1 AND filename=?2", params![dir_path, filename], |row| row.get::<_,i32>(0)).map(|d| d==1).unwrap_or(false);

    if is_dir {
        let prefix = format!("{}/{}", dir_path.trim_end_matches('/'), filename);
        if let Ok(mut s) = db.prepare("SELECT path FROM files WHERE path=?1 OR path LIKE ?2") {
            let paths: Vec<String> = match s.query_map(params![&prefix, format!("{}/%", &prefix)], |r| r.get(0)) {
                Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
                Err(_) => vec![],
            };
            for cp in &paths {
                if let Ok(mut s) = db.prepare("SELECT disk_name FROM files WHERE path=?1 AND is_dir=0 AND disk_name IS NOT NULL") {
                    let disks: Vec<String> = match s.query_map(params![cp], |r| r.get::<_,String>(0)) {
                        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
                        Err(_) => vec![],
                    };
                    for disk in &disks {
                        let _ = std::fs::remove_file(files_dir().join(disk));
                    }
                }
            }
            let _ = db.execute("DELETE FROM files WHERE (path=?1 OR path LIKE ?2)", params![&prefix, format!("{}/%", &prefix)]);
        }
    } else {
        if let Ok(Some(disk)) = db.query_row("SELECT disk_name FROM files WHERE path=?1 AND filename=?2", params![dir_path, filename], |row| row.get::<_,Option<String>>(0)) {
            let _ = std::fs::remove_file(files_dir().join(&disk));
        }
        let _ = db.execute("DELETE FROM files WHERE path=?1 AND filename=?2", params![dir_path, filename]);
    }
    json_ok(serde_json::to_vec(&serde_json::json!({"path": target})).unwrap())
}

// ── API: Rename ───────────────────────────────────────────────────────────

fn api_rename(body: &[u8], db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let j: serde_json::Value = match serde_json::from_slice(body) { Ok(v) => v, Err(_) => return json_error(400) };
    let old_path = match j["oldPath"].as_str() { Some(p) => p, None => return json_error(400) };
    let new_name = match j["newName"].as_str() { Some(n) => n, None => return json_error(400) };
    if new_name.is_empty() || new_name.contains('/') || new_name.contains('\\') { return json_error(400); }
    let (dir_path, old_name) = match split_path(old_path) { Ok(p) => p, Err(_) => return json_error(400) };

    let db = lock(&db);
    let is_dir = match db.query_row("SELECT is_dir FROM files WHERE path=?1 AND filename=?2", params![dir_path, old_name], |row| row.get::<_,i32>(0)) {
        Ok(d) => d, Err(_) => return json_error(404),
    };
    if db.query_row("SELECT COUNT(*) FROM files WHERE path=?1 AND filename=?2", params![dir_path, new_name], |row| row.get::<_,i64>(0)).unwrap_or(0) > 0 { return json_error(409); }
    if db.execute("UPDATE files SET filename=?1 WHERE path=?2 AND filename=?3", params![new_name, dir_path, old_name]).is_err() { return json_error(500); }

    if is_dir == 1 {
        let old_prefix = format!("{}/{}", dir_path.trim_end_matches('/'), old_name);
        let new_prefix = format!("{}/{}", dir_path.trim_end_matches('/'), new_name);
        if let Ok(mut s) = db.prepare("SELECT id, path FROM files WHERE path=?1 OR path LIKE ?2") {
            let children: Vec<(i64, String)> = match s.query_map(params![old_prefix, format!("{}/%", old_prefix)], |row| Ok((row.get::<_,i64>(0)?, row.get::<_,String>(1)?))) {
                Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
                Err(_) => vec![],
            };
            for (id, cp) in &children { let _ = db.execute("UPDATE files SET path=?1 WHERE id=?2", params![cp.replacen(&old_prefix, &new_prefix, 1), id]); }
        }
    }
    json_ok(serde_json::to_vec(&serde_json::json!({"path": old_path})).unwrap())
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn clean_path(p: &str) -> String {
    let p = p.trim().trim_start_matches('/');
    if p.is_empty() { return "/".to_string(); }
    let mut out = Vec::new();
    for seg in p.split('/').filter(|s| !s.is_empty() && *s != ".") {
        if seg == ".." { out.pop(); } else { out.push(seg.to_string()); }
    }
    if out.is_empty() { "/".to_string() } else { format!("/{}", out.join("/")) }
}

fn split_path(full: &str) -> Result<(String, String), ()> {
    let full = full.trim();
    if full.is_empty() || full == "/" { return Err(()); }
    let clean = full.trim_start_matches('/');
    if let Some((parent, name)) = clean.rsplit_once('/') {
        Ok((if parent.is_empty() { "/".to_string() } else { format!("/{}", parent) }, name.to_string()))
    } else {
        Ok(("/".to_string(), clean.to_string()))
    }
}

fn spawn_cloudflared(port: u16) {
    let url = format!("http://localhost:{}", port);
    let cf = find_cloudflared().unwrap_or_else(|| "cloudflared".to_string());
    match std::process::Command::new(&cf).args(["tunnel", "--url", &url]).spawn() {
        Ok(mut c) => { print_info(&format!("Cloudflared tunnel starting (PID {})", c.id())); std::thread::spawn(move || { let _ = c.wait(); }); }
        Err(e) => print_error(&format!("cloudflared not found: {}. Install: ntpkg install cloudflare", e)),
    }
}

fn find_cloudflared() -> Option<String> {
    if which::which("cloudflared").is_ok() { return Some("cloudflared".to_string()); }
    let p = dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("ntc").join("bin").join("cloudflared");
    if p.exists() { return Some(p.to_string_lossy().to_string()); }
    #[cfg(windows)] {
        let pe = dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("ntc").join("bin").join("cloudflared.exe");
        if pe.exists() { return Some(pe.to_string_lossy().to_string()); }
    }
    None
}

fn print_serve_help() {
    println!("{}", "HTTP File Server".bold());
    println!();
    println!("{}", "Usage:".cyan().bold());
    println!("  serve [port] [options]");
    println!();
    println!("{}", "Options:".cyan().bold());
    println!("  --port, -p <port>     Port to listen on (default: 8080)");
    println!("  --bind, -b <addr>     Bind address (default: 0.0.0.0)");
    println!("  --tunnel, -t          Expose via cloudflared tunnel");
    println!("  --spa                 SPA mode (fallback to index.html)");
    println!("  --web-dir, -w <dir>   Serve custom frontend directory");
    println!("  --help, -h            Show this help");
    println!();
    println!("{}", "Examples:".cyan().bold());
    println!("  serve                  HTTP on port 8080");
    println!("  serve 3000             HTTP on port 3000");
    println!("  serve --tunnel         HTTP + cloudflared tunnel");
    println!("  serve --spa            SPA fallback for React Router");
    println!();
    println!("{}", "Files stored in:".cyan().bold());
    println!("  {}", files_dir().to_string_lossy().dimmed());
    println!("{}", "Database:".cyan().bold());
    println!("  {}", db_path().to_string_lossy().dimmed());
}
