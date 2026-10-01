use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::t;
use anyhow::{Context, Result};
use colored::*;
use rusqlite::params;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(rust_embed::Embed)]
#[folder = "ntc_web/dist"]
struct Frontend;

enum ApiResponse {
    Bytes(Vec<u8>),
    Stream { head: Vec<u8>, path: PathBuf },
}

const MAX_UPLOAD: u64 = 10 * 1024 * 1024 * 1024;
// Chunked uploads split large files into pieces this size (each request must
// stay under Cloudflare's ~100 MB per-request body cap when served via a
// trycloudflare tunnel).
const CHUNK_MAX: u64 = 100 * 1024 * 1024;
// A single socket read that produces no bytes aborts after BODY_READ_TIMEOUT;
// if no bytes arrive for BODY_IDLE_TIMEOUT the upload is considered stalled.
const BODY_READ_TIMEOUT: u64 = 30;
const BODY_IDLE_TIMEOUT: u64 = 180;

// ── SQLite ────────────────────────────────────────────────────────────────

fn db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("serve.db")
}
fn files_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("serve")
        .join("files")
}
fn chunks_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("serve")
        .join("chunks")
}
fn ensure_db() -> Result<rusqlite::Connection> {
    let path = db_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::create_dir_all(files_dir())?;
    std::fs::create_dir_all(chunks_dir())?;
    let conn = rusqlite::Connection::open(&path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS files (
        id INTEGER PRIMARY KEY AUTOINCREMENT, filename TEXT NOT NULL,
        path TEXT NOT NULL DEFAULT '/', is_dir INTEGER DEFAULT 0,
        size INTEGER DEFAULT 0, disk_name TEXT, data BLOB,
        uploaded_at TEXT DEFAULT (datetime('now')))",
    )?;
    // Migration: add data column for existing databases
    let _ = conn.execute_batch("ALTER TABLE files ADD COLUMN data BLOB");
    Ok(conn)
}

// ── Tokenize ──────────────────────────────────────────────────────────────

fn tokenize(input: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut c = String::new();
    let mut s = false;
    let mut d = false;
    for ch in input.chars() {
        match ch {
            '\'' if !d => s = !s,
            '"' if !s => d = !d,
            ' ' | '\t' if !s && !d => {
                if !c.is_empty() {
                    v.push(std::mem::take(&mut c));
                }
            }
            _ => c.push(ch),
        }
    }
    if !c.is_empty() {
        v.push(c);
    }
    v
}

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || matches!(tokens[0].as_str(), "help" | "--help" | "-h") {
        print_serve_help();
        return Ok(false);
    }
    let mut port = 8080u16;
    let mut bind = "0.0.0.0".to_string();
    let mut use_tunnel = false;
    let mut spa = false;
    let mut web_dir = String::new();
    let mut auth: Option<(String, String)> = None;
    let mut share_mode = false;
    let mut share_path: Option<String> = None;
    let mut no_qr = false;
    let mut want_qr = false;

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
            "-t" | "--tunnel" => {
                use_tunnel = true;
            }
            "--spa" => {
                spa = true;
            }
            "--share" => {
                share_mode = true;
                // Optional path right after --share (must not look like a flag).
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
            "--no-qr" => {
                no_qr = true;
            }
            "--auth" => {
                i += 1;
                let value = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_auth")))?;
                auth = Some(parse_auth(value)?);
            }
            "-w" | "--web-dir" => {
                i += 1;
                web_dir = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_dir")))?
                    .clone();
            }
            "--" => break,
            _ => {
                if let Ok(p) = tokens[i].parse::<u16>() {
                    port = p;
                } else {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", tokens[i]));
                }
            }
        }
        i += 1;
    }

    let db = Arc::new(Mutex::new(ensure_db()?));
    // Chunk uploads are single-session; anything left from a previous (crashed)
    // session is garbage and can be removed now that no uploads are active.
    if let Ok(rd) = std::fs::read_dir(chunks_dir()) {
        for e in rd.flatten() {
            let _ = std::fs::remove_file(e.path());
        }
    }
    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    listener.set_nonblocking(false)?;
    print_info(&format!(
        "HTTP server on {}",
        format!("http://{}", addr).cyan()
    ));
    if let Some((ref user, _)) = auth {
        print_info(&t!("cmd.serve.auth_on", user));
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

    let web = if web_dir.is_empty() {
        None
    } else {
        Some(PathBuf::from(web_dir))
    };
    listener.set_nonblocking(true)?;

    let auth = auth.map(|(u, p)| std::sync::Arc::new((u, p)));
    while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let db = db.clone();
                let web = web.clone();
                let auth = auth.clone();
                std::thread::spawn(move || handle_connection(stream, db, spa, web, auth));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => print_warning(&format!("Accept: {}", e)),
        }
    }
    print_info(&t!("cmd.serve.stopped"));
    crate::shell::commands::unregister_shutdown_flag(&shutdown);
    crate::shell::commands::init_ctrlc_handler();
    Ok(false)
}

// ── Auth ────────────────────────────────────────────────────────────────────
//
// `serve --auth <user:pass>` protects every `/api/*` route with HTTP Basic.
// The static frontend (`/`) stays public so the login page itself always
// loads; the React app attaches the header and shows a login dialog on 401.

/// Split `--auth <user:pass>` on the FIRST `:` (`@` and later `:` are
/// literal, so `admin:555@ntcstorage` → `admin` / `555@ntcstorage`).
fn parse_auth(value: &str) -> Result<(String, String)> {
    match value.split_once(':') {
        Some((user, pass)) if !user.is_empty() && !pass.is_empty() => {
            Ok((user.to_string(), pass.to_string()))
        }
        _ => anyhow::bail!("{}", t!("cmd.serve.invalid_auth")),
    }
}

/// Expected `Authorization` header value for these credentials.
fn basic_expectation(user: &str, pass: &str) -> String {
    let raw = format!("{}:{}", user, pass);
    let encoded =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, raw.as_bytes());
    format!("Basic {}", encoded)
}

/// Constant-time-ish equality so wrong passwords don't fail faster.
fn auth_matches(given: &str, expected: &str) -> bool {
    let a = given.trim().as_bytes();
    let b = expected.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

fn respond_unauthorized(stream: &mut TcpStream) {
    let body = r#"{"error":"Unauthorized"}"#;
    let resp = format!(
        "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"ntc serve\"\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = write_all_retry(stream, resp.as_bytes());
}

// ── Share mode (internet-first, then LAN) ─────────────────────────────────
//
// `serve --share` is advertisement-only: it never changes auth or routing.
// Step 1: quick TCP reachability probe (port 53/DNS, ~900ms each).
// Step 2: only when online, discover the primary LAN IPv4 via the
// default-route UDP trick and print it plus a terminal QR.

/// Candidate endpoints for the internet reachability probe.
const SHARE_PROBE_TARGETS: &[&str] = &["8.8.8.8:53", "1.1.1.1:53"];
const SHARE_PROBE_TIMEOUT: Duration = Duration::from_millis(900);

/// Step 1: true when at least one probe target answers a TCP connect.
fn has_internet() -> bool {
    for target in SHARE_PROBE_TARGETS {
        if let Ok(addr) = target.parse::<SocketAddr>() {
            if TcpStream::connect_timeout(&addr, SHARE_PROBE_TIMEOUT).is_ok() {
                return true;
            }
        }
    }
    false
}

/// True for RFC1918 private ranges: 10/8, 172.16/12, 192.168/16.
fn is_rfc1918(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    o[0] == 10 || o[0] == 192 && o[1] == 168 || o[0] == 172 && (16..=31).contains(&o[1])
}

/// Priority for QR selection: 192.168 > 10 > 172.16-31 > other unicast.
fn lan_priority(ip: Ipv4Addr) -> u8 {
    let o = ip.octets();
    if o[0] == 192 && o[1] == 168 {
        0
    } else if o[0] == 10 {
        1
    } else if o[0] == 172 && (16..=31).contains(&o[1]) {
        2
    } else {
        3
    }
}

/// Step 2: primary LAN IPv4 via the default-route UDP trick (std-only,
/// cross-platform incl. Termux). No packet is actually sent; `connect` on a
/// UDP socket only selects the outbound interface, then `local_addr` reveals it.
fn primary_lan_ip() -> Option<Ipv4Addr> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    // 8.8.8.8:80 never receives traffic here — connect() just picks the route.
    sock.connect("8.8.8.8:80").ok()?;
    match sock.local_addr().ok()?.ip() {
        std::net::IpAddr::V4(v4) if !v4.is_loopback() && !v4.is_link_local() => Some(v4),
        _ => None,
    }
}

fn print_share_qr(url: &str) {
    match qrcode::QrCode::new(url.as_bytes()) {
        Ok(code) => {
            let rendered = code
                .render::<qrcode::render::unicode::Dense1x2>()
                .module_dimensions(2, 1)
                .build();
            println!();
            println!("{}", rendered);
        }
        Err(e) => print_warning(&t!("cmd.serve.share_qr_fail", e)),
    }
}

/// Print the `--share` block: internet check first, LAN + QR only when online.
fn print_share_block(port: u16, bind: &str, share_path: Option<&str>, show_qr: bool) {
    println!("{}", t!("cmd.serve.share_checking").dimmed());
    if !has_internet() {
        print_warning(&t!("cmd.serve.share_offline"));
        println!("  {}", format!("http://127.0.0.1:{}", port).cyan());
        return;
    }
    print_info(&t!("cmd.serve.share_online"));

    if bind == "127.0.0.1" || bind == "localhost" {
        print_warning(&t!("cmd.serve.share_bound_loopback"));
    }

    match primary_lan_ip() {
        Some(ip) => {
            let prio = lan_priority(ip);
            let url = format!("http://{}:{}", ip, port);
            if prio == 3 {
                print_warning(&t!("cmd.serve.share_not_private", ip));
            }
            println!("  {} {}", t!("cmd.serve.share_lan_title").cyan().bold(), url.cyan());
            if !is_rfc1918(ip) {
                print_warning(&t!("cmd.serve.share_not_private_hint"));
            }
            if let Some(p) = share_path {
                println!("  {} {}", t!("cmd.serve.share_path_title").dimmed(), p);
            }
            if show_qr {
                print_share_qr(&url);
            }
        }
        None => {
            print_warning(&t!("cmd.serve.share_no_lan"));
            println!("  {}", format!("http://127.0.0.1:{}", port).cyan());
        }
    }
}

// ── Connection handler ────────────────────────────────────────────────────

fn handle_connection(
    mut stream: TcpStream,
    db: Arc<Mutex<rusqlite::Connection>>,
    spa_mode: bool,
    web_dir: Option<PathBuf>,
    auth: Option<std::sync::Arc<(String, String)>>,
) {
    // The accepted socket may inherit non-blocking mode from the listener;
    // restore blocking semantics so reads/writes behave predictably.
    let _ = stream.set_nonblocking(false);
    let mut reader = std::io::BufReader::new(&mut stream);

    // Read request line
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim();
    if request_line.is_empty() {
        return;
    }

    let parts: Vec<&str> = request_line.splitn(3, ' ').collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let full_path = parts[1];
    let (path, query) = full_path.split_once('?').unwrap_or((full_path, ""));

    // Read headers
    let mut content_length: u64 = 0;
    let mut boundary = String::new();
    let mut expect_continue = false;
    let mut authorization = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_lowercase();
            let val = v.trim().to_string();
            if key == "content-length" {
                content_length = val.parse().unwrap_or(0);
            }
            if key == "content-type" && val.contains("multipart/form-data") {
                if let Some(b) = val.split("boundary=").nth(1) {
                    boundary = b.trim().to_string();
                }
            }
            if key == "expect" && val.to_lowercase().contains("100-continue") {
                expect_continue = true;
            }
            if key == "authorization" {
                authorization = val;
            }
        }
    }

    // Auth gate: every `/api/*` route needs valid Basic credentials when
    // `--auth` was given. Static files stay public so the login page loads;
    // the check runs before any body is read so unauthenticated uploads
    // can't burn bandwidth or disk.
    if path.starts_with("/api/") {
        if let Some(ref creds) = auth {
            let expected = basic_expectation(&creds.0, &creds.1);
            if !auth_matches(&authorization, &expected) {
                respond_unauthorized(&mut stream);
                return;
            }
        }
    }

    // Read body (only for methods that have one). Multipart uploads are
    // streamed to a temp file so files up to MAX_UPLOAD (10 GB) never have
    // to fit in memory; JSON payloads stay small and in-memory.
    let mut body = Vec::new();
    let mut upload_file: Option<PathBuf> = None;
    let has_body = method == "POST" || method == "PUT" || method == "PATCH";
    if has_body {
        // Clients using Expect: 100-continue wait for this before sending the
        // body; ack it so they don't stall (a stall surfaces as a network
        // failure on the client). Oversized requests are rejected up front so
        // the client doesn't transmit a body that will be refused anyway.
        let oversized = content_length > MAX_UPLOAD;
        if expect_continue && !oversized {
            let _ = write_all_retry(reader.get_mut(), b"HTTP/1.1 100 Continue\r\n\r\n");
        }
        if boundary.is_empty() {
            let max_body = MAX_UPLOAD;
            if content_length > 0 && content_length > max_body {
                print_warning(&format!("serve: JSON body too large ({})", content_length));
                respond(&mut stream, 413, "text/plain", b"413 Payload Too Large");
                return;
            }
            let mut buf = vec![0u8; 65536];
            let mut total: u64 = 0;
            let need = if content_length > 0 {
                content_length
            } else {
                u64::MAX
            };
            let mut last_activity = std::time::Instant::now();
            loop {
                reader
                    .get_ref()
                    .set_read_timeout(Some(std::time::Duration::from_secs(BODY_READ_TIMEOUT)))
                    .ok();
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        last_activity = std::time::Instant::now();
                        total += n as u64;
                        if total > max_body {
                            respond(&mut stream, 413, "text/plain", b"413 Payload Too Large");
                            return;
                        }
                        body.extend_from_slice(&buf[..n]);
                        if total >= need {
                            break;
                        }
                    }
                    Err(ref e)
                        if e.kind() == std::io::ErrorKind::WouldBlock
                            || e.kind() == std::io::ErrorKind::TimedOut =>
                    {
                        if std::time::Instant::now().duration_since(last_activity)
                            > std::time::Duration::from_secs(BODY_IDLE_TIMEOUT)
                        {
                            print_warning("serve: JSON body timed out");
                            respond(&mut stream, 408, "text/plain", b"408 Request Timeout");
                            return;
                        }
                        continue
                    }
                    Err(e) => {
                        print_error(&format!("serve: JSON body read failed: {}", e));
                        respond(&mut stream, 500, "text/plain", b"500 Internal Server Error");
                        return;
                    }
                }
            }
        } else {
            match read_body_to_file(&mut reader, content_length) {
                Ok(Some(p)) => upload_file = Some(p),
                Ok(None) => {
                    print_warning(&format!("serve: upload too large ({})", content_length));
                    respond(&mut stream, 413, "text/plain", b"413 Payload Too Large");
                    return;
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    print_warning("serve: upload timed out");
                    respond(&mut stream, 408, "text/plain", b"408 Request Timeout");
                    return;
                }
                Err(e) => {
                    print_error(&format!("serve: upload body read failed: {}", e));
                    respond(&mut stream, 500, "text/plain", b"500 Internal Server Error");
                    return;
                }
            }
        }
    }

    // Route
    let response = if path.starts_with("/api/") {
        handle_api(
            method,
            path,
            query,
            &body,
            &boundary,
            upload_file.as_deref(),
            db,
        )
    } else {
        ApiResponse::Bytes(serve_static(path, spa_mode, &web_dir))
    };

    if let Some(p) = &upload_file {
        let _ = std::fs::remove_file(p);
    }

    match response {
        ApiResponse::Bytes(b) => {
            let _ = write_all_retry(&mut stream, &b);
        }
        ApiResponse::Stream { head, path } => {
            let _ = write_all_retry(&mut stream, &head);
            if let Ok(mut file) = std::fs::File::open(&path) {
                let mut buf = vec![0u8; 65536];
                loop {
                    match file.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            if write_all_retry(&mut stream, &buf[..n]).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
        }
    }
}

/// Streams the multipart body to a temp file, enforcing MAX_UPLOAD and an
/// idle-based timeout. `Ok(Some(path))` is a fully received body; `Ok(None)`
/// means it exceeded MAX_UPLOAD (413); `Err` is a stalled or failed read.
fn read_body_to_file(
    reader: &mut std::io::BufReader<&mut TcpStream>,
    content_length: u64,
) -> std::io::Result<Option<PathBuf>> {
    if content_length > MAX_UPLOAD {
        return Ok(None);
    }
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tmp = files_dir().join(format!(".up-{}-{}", std::process::id(), nanos));
    let mut out = std::fs::File::create(&tmp)?;
    let mut buf = vec![0u8; 65536];
    let mut total: u64 = 0;
    let need = if content_length > 0 {
        content_length
    } else {
        u64::MAX
    };
    // Slow-but-flowing uploads are allowed to take as long as the link needs;
    // only a connection that stops sending entirely for BODY_IDLE_TIMEOUT is
    // aborted. MAX_UPLOAD stays the hard cap.
    let mut last_activity = std::time::Instant::now();
    loop {
        reader
            .get_ref()
            .set_read_timeout(Some(std::time::Duration::from_secs(BODY_READ_TIMEOUT)))
            .ok();
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                last_activity = std::time::Instant::now();
                total += n as u64;
                if total > MAX_UPLOAD {
                    drop(out);
                    let _ = std::fs::remove_file(&tmp);
                    return Ok(None);
                }
                if out.write_all(&buf[..n]).is_err() {
                    let _ = std::fs::remove_file(&tmp);
                    return Err(std::io::Error::other("upload write failed"));
                }
                if total >= need {
                    break;
                }
            }
            Err(ref e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                if std::time::Instant::now().duration_since(last_activity)
                    > std::time::Duration::from_secs(BODY_IDLE_TIMEOUT)
                {
                    let _ = std::fs::remove_file(&tmp);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "upload timed out",
                    ));
                }
                continue
            }
            Err(_) => {
                let _ = std::fs::remove_file(&tmp);
                return Err(std::io::Error::other("upload read failed"));
            }
        }
    }
    drop(out);
    Ok(Some(tmp))
}

fn respond(stream: &mut TcpStream, status: u16, ct: &str, data: &[u8]) {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        413 => "Payload Too Large",
        408 => "Request Timeout",
        500 => "Internal Server Error",
        _ => "Error",
    };
    let resp = format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", status, reason, ct, data.len());
    let _ = write_all_retry(stream, resp.as_bytes());
    let _ = write_all_retry(stream, data);
}

/// Writes all bytes, retrying transient WouldBlock/TimedOut errors that can
/// happen when a socket still carries the listener's non-blocking mode.
fn write_all_retry(stream: &mut TcpStream, mut data: &[u8]) -> std::io::Result<()> {
    while !data.is_empty() {
        match stream.write(data) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "write returned zero",
                ));
            }
            Ok(n) => data = &data[n..],
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn response_bytes(status: u16, ct: &str, data: Vec<u8>) -> Vec<u8> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        413 => "Payload Too Large",
        408 => "Request Timeout",
        500 => "Internal Server Error",
        _ => "Error",
    };
    let head = format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", status, reason, ct, data.len());
    let mut r = head.into_bytes();
    r.extend(data);
    r
}

fn json_error(status: u16) -> Vec<u8> {
    let reason = match status {
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        _ => "Error",
    };
    let body = format!(r#"{{"error":"{}"}}"#, reason);
    response_bytes(status, "application/json", body.into_bytes())
}

fn json_ok(data: Vec<u8>) -> Vec<u8> {
    response_bytes(200, "application/json", data)
}

// ── Static file serving ───────────────────────────────────────────────────

fn serve_static(path: &str, spa_mode: bool, web_dir: &Option<PathBuf>) -> Vec<u8> {
    let clean = if path == "/" {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    if let Some(dir) = web_dir {
        let fp = dir.join(clean);
        if fp.is_file() {
            if let Ok(d) = std::fs::read(&fp) {
                return response_bytes(200, mime_for(clean), d);
            }
        }
    }

    if let Some(f) = Frontend::get(clean) {
        return response_bytes(200, mime_for(clean), f.data.to_vec());
    }

    let alt = format!("/{}", clean).trim_start_matches('/').to_string();
    if let Some(f) = Frontend::get(&alt) {
        return response_bytes(200, mime_for(&alt), f.data.to_vec());
    }

    if spa_mode {
        if let Some(f) = Frontend::get("index.html") {
            return response_bytes(200, "text/html; charset=utf-8", f.data.to_vec());
        }
    }

    json_error(404)
}

fn mime_for(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
    {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        _ => "application/octet-stream",
    }
}

// ── API router ────────────────────────────────────────────────────────────

fn handle_api(
    method: &str,
    path: &str,
    query: &str,
    body: &[u8],
    boundary: &str,
    upload_path: Option<&Path>,
    db: Arc<Mutex<rusqlite::Connection>>,
) -> ApiResponse {
    match (method, path) {
        ("GET", "/api/files") => ApiResponse::Bytes(api_list_files(query, db)),
        ("GET", "/api/download") => api_download(query, db),
        ("POST", "/api/upload") => match upload_path {
            Some(p) => api_upload(p, boundary, db),
            None => ApiResponse::Bytes(json_error(400)),
        },
        ("POST", "/api/upload-chunk") => match upload_path {
            Some(p) => api_upload_chunk(p, boundary, db),
            None => ApiResponse::Bytes(json_error(400)),
        },
        ("POST", "/api/upload-finalize") => ApiResponse::Bytes(api_upload_finalize(body, db)),
        ("DELETE", "/api/upload-cancel") => ApiResponse::Bytes(api_upload_cancel(query, db)),
        ("POST", "/api/create-folder") => ApiResponse::Bytes(api_create_folder(body, db)),
        ("DELETE", "/api/delete") => ApiResponse::Bytes(api_delete(query, db)),
        ("PATCH", "/api/rename") => ApiResponse::Bytes(api_rename(body, db)),
        _ => ApiResponse::Bytes(json_error(404)),
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for pair in query.split('&').filter(|s| !s.is_empty()) {
        if let Some((k, v)) = pair.split_once('=') {
            m.insert(url_decode(k), url_decode(v));
        }
    }
    m
}

fn url_decode(s: &str) -> String {
    // Percent-decoded bytes are collected as raw bytes first so multi-byte
    // UTF-8 sequences (e.g. `%C3%A9` for "é") decode correctly; decoding them
    // byte-by-byte as chars would mangle non-ASCII folder/file names and make
    // delete/download miss their rows.
    let mut buf: Vec<u8> = Vec::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 3 <= bytes.len() {
            let h = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
            if let Ok(b) = u8::from_str_radix(h, 16) {
                buf.push(b);
                i += 3;
                continue;
            }
            buf.push(b'%');
            i += 1;
        } else if bytes[i] == b'+' {
            buf.push(b' ');
            i += 1;
        } else {
            buf.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&buf).into_owned()
}

fn lock(db: &Arc<Mutex<rusqlite::Connection>>) -> std::sync::MutexGuard<'_, rusqlite::Connection> {
    db.lock().unwrap_or_else(|e| e.into_inner())
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

fn api_download(query: &str, db: Arc<Mutex<rusqlite::Connection>>) -> ApiResponse {
    let q = parse_query(query);
    let id: i64 = match q.get("id").and_then(|s| s.parse().ok()) {
        Some(i) => i,
        None => return ApiResponse::Bytes(json_error(400)),
    };
    let (filename, disk_name, blob): (String, Option<String>, Option<Vec<u8>>) = {
        let db = lock(&db);
        match db.query_row(
            "SELECT filename, disk_name, data FROM files WHERE id = ?1 AND is_dir = 0",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ) {
            Ok(r) => r,
            Err(_) => return ApiResponse::Bytes(json_error(404)),
        }
    };
    let ct = mime_guess::from_path(&filename)
        .first_or_octet_stream()
        .to_string();
    match (disk_name, blob) {
        (Some(d), _) => {
            let path = files_dir().join(&d);
            match std::fs::metadata(&path) {
                Ok(m) => {
                    let head = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", ct, filename, m.len());
                    ApiResponse::Stream {
                        head: head.into_bytes(),
                        path,
                    }
                }
                Err(_) => ApiResponse::Bytes(json_error(404)),
            }
        }
        (_, Some(b)) => {
            let head = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", ct, filename, b.len());
            let mut r = head.into_bytes();
            r.extend(b);
            ApiResponse::Bytes(r)
        }
        _ => ApiResponse::Bytes(json_error(404)),
    }
}

// ── API: Upload ───────────────────────────────────────────────────────────

fn api_upload(
    upload_path: &Path,
    boundary: &str,
    db: Arc<Mutex<rusqlite::Connection>>,
) -> ApiResponse {
    if boundary.is_empty() {
        return ApiResponse::Bytes(json_error(400));
    }
    let (dest_path, files, _fields) = match parse_multipart_stream(upload_path, boundary, &files_dir()) {
        Ok(f) => f,
        Err(_) => return ApiResponse::Bytes(json_error(500)),
    };
    if files.is_empty() {
        return ApiResponse::Bytes(json_error(400));
    }
    {
        let db = lock(&db);
        for (filename, disk, size) in &files {
            if db
                .execute(
                    "INSERT INTO files (filename, path, is_dir, size, disk_name) VALUES (?1, ?2, 0, ?3, ?4)",
                    params![filename, dest_path, *size as i64, disk],
                )
                .is_err()
            {
                let _ = std::fs::remove_file(files_dir().join(disk));
                return ApiResponse::Bytes(json_error(500));
            }
        }
    }
    ApiResponse::Bytes(json_ok(
        serde_json::to_vec(&serde_json::json!({"path": dest_path, "uploaded": files.len()}))
            .unwrap(),
    ))
}

// ── API: Chunked upload ───────────────────────────────────────────────────
//
// Large files are split client-side into <= CHUNK_MAX pieces so each request
// fits under the tunnel's per-request body cap. Chunks are appended to a
// staging file and reassembled by upload-finalize.

fn valid_upload_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn chunk_part_path(id: &str) -> PathBuf {
    chunks_dir().join(format!("{}.part", id))
}

fn chunk_meta_path(id: &str) -> PathBuf {
    chunks_dir().join(format!("{}.meta", id))
}

/// Removes a chunk session's staging files (part + meta). Best-effort.
fn chunk_cleanup(id: &str) {
    let _ = std::fs::remove_file(chunk_part_path(id));
    let _ = std::fs::remove_file(chunk_meta_path(id));
}

fn api_upload_chunk(
    upload_path: &Path,
    boundary: &str,
    _db: Arc<Mutex<rusqlite::Connection>>,
) -> ApiResponse {
    if boundary.is_empty() {
        return ApiResponse::Bytes(json_error(400));
    }
    let (dest_path, files, fields) = match parse_multipart_stream(upload_path, boundary, &files_dir())
    {
        Ok(f) => f,
        Err(_) => return ApiResponse::Bytes(json_error(500)),
    };
    // The parser extracts each chunk to a disk file; those are intermediate
    // and must be removed on every path below (success or failure).
    let drop_extracted = || {
        for (_, disk, _) in &files {
            let _ = std::fs::remove_file(files_dir().join(disk));
        }
    };
    let upload_id = match fields.get("uploadId") {
        Some(id) if valid_upload_id(id) => id.clone(),
        _ => {
            drop_extracted();
            return ApiResponse::Bytes(json_error(400));
        }
    };
    let total_size: u64 = match fields.get("totalSize").and_then(|s| s.parse().ok()) {
        Some(s) => s,
        None => {
            drop_extracted();
            return ApiResponse::Bytes(json_error(400));
        }
    };
    if total_size > MAX_UPLOAD {
        drop_extracted();
        return ApiResponse::Bytes(json_error(413));
    }
    let chunk_index: u64 = fields
        .get("chunkIndex")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if files.len() != 1 {
        drop_extracted();
        chunk_cleanup(&upload_id);
        return ApiResponse::Bytes(json_error(400));
    }
    let (filename, disk, chunk_size) = &files[0];
    let chunk_file = files_dir().join(disk);
    if *chunk_size > CHUNK_MAX {
        drop_extracted();
        chunk_cleanup(&upload_id);
        return ApiResponse::Bytes(json_error(413));
    }

    // Read the current staging state (single-session, chunks arrive in order).
    let mut next_index: u64 = 0;
    let mut received: u64 = 0;
    if let Ok(meta) = std::fs::read_to_string(chunk_meta_path(&upload_id)) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&meta) {
            next_index = v["next"].as_u64().unwrap_or(0);
            received = v["received"].as_u64().unwrap_or(0);
        }
    }
    if chunk_index != next_index {
        drop_extracted();
        chunk_cleanup(&upload_id);
        return ApiResponse::Bytes(json_error(400));
    }
    if received + chunk_size > total_size {
        drop_extracted();
        chunk_cleanup(&upload_id);
        return ApiResponse::Bytes(json_error(400));
    }

    // Append this chunk's bytes to the staging file.
    let part_path = chunk_part_path(&upload_id);
    let append = || -> std::io::Result<u64> {
        let mut staged = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&part_path)?;
        let mut src = std::fs::File::open(&chunk_file)?;
        let n = std::io::copy(&mut src, &mut staged)?;
        staged.sync_all().ok();
        drop(src);
        drop(staged);
        Ok(n)
    };
    let written = match append() {
        Ok(n) => n,
        Err(_) => {
            drop_extracted();
            chunk_cleanup(&upload_id);
            return ApiResponse::Bytes(json_error(500));
        }
    };
    drop_extracted();

    // Persist staging state for the next chunk / finalize.
    received += written;
    next_index += 1;
    let meta = serde_json::json!({"next": next_index, "received": received, "total": total_size});
    if std::fs::write(chunk_meta_path(&upload_id), meta.to_string()).is_err() {
        chunk_cleanup(&upload_id);
        return ApiResponse::Bytes(json_error(500));
    }

    ApiResponse::Bytes(json_ok(
        serde_json::to_vec(&serde_json::json!({
            "ok": true,
            "received": received,
            "total": total_size,
            "path": dest_path,
            "filename": filename,
        }))
        .unwrap(),
    ))
}

fn api_upload_finalize(body: &[u8], db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let j: serde_json::Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_error(400),
    };
    let upload_id = match j["uploadId"].as_str() {
        Some(id) if valid_upload_id(id) => id.to_string(),
        _ => return json_error(400),
    };
    let dest_path = j["path"].as_str().map(clean_path).unwrap_or_else(|| "/".to_string());
    let filename = match j["filename"].as_str() {
        Some(n) if !n.trim().is_empty() => n.trim().to_string(),
        _ => return json_error(400),
    };
    let size: u64 = match j["size"].as_u64() {
        Some(s) => s,
        None => return json_error(400),
    };
    if size > MAX_UPLOAD {
        return json_error(413);
    }

    let part_path = chunk_part_path(&upload_id);
    let meta = match std::fs::read_to_string(chunk_meta_path(&upload_id)) {
        Ok(m) => m,
        Err(_) => return json_error(404),
    };
    let v: serde_json::Value = match serde_json::from_str(&meta) {
        Ok(v) => v,
        Err(_) => return json_error(404),
    };
    let received = v["received"].as_u64().unwrap_or(0);
    let total = v["total"].as_u64().unwrap_or(0);
    if total != size || received != size {
        chunk_cleanup(&upload_id);
        return json_error(400);
    }

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let disk = format!("{}-{}", nanos, filename);
    let final_path = files_dir().join(&disk);

    // Move the assembled staging file into place (same volume => rename).
    if std::fs::rename(&part_path, &final_path).is_err() {
        // Fall back to copy+remove (e.g. cross-device edge cases).
        if std::fs::copy(&part_path, &final_path).is_err() {
            chunk_cleanup(&upload_id);
            return json_error(500);
        }
        let _ = std::fs::remove_file(&part_path);
    }
    let _ = std::fs::remove_file(chunk_meta_path(&upload_id));

    {
        let db = lock(&db);
        if db
            .execute(
                "INSERT INTO files (filename, path, is_dir, size, disk_name) VALUES (?1, ?2, 0, ?3, ?4)",
                params![filename, dest_path, size as i64, disk],
            )
            .is_err()
        {
            let _ = std::fs::remove_file(&final_path);
            return json_error(500);
        }
    }

    json_ok(serde_json::to_vec(&serde_json::json!({"path": dest_path, "uploaded": 1})).unwrap())
}

fn api_upload_cancel(query: &str, _db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let q = parse_query(query);
    match q.get("uploadId") {
        Some(id) if valid_upload_id(id) => chunk_cleanup(id),
        _ => {}
    }
    json_ok(serde_json::to_vec(&serde_json::json!({"ok": true})).unwrap())
}

// ── Multipart streaming parser ────────────────────────────────────────────
//
// Parses a multipart body that was already streamed to `file_path`, writing
// each file field straight to disk so large uploads never need to fit in RAM.

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn strip_trailing_crlf(data: &[u8]) -> (&[u8], usize) {
    if data.ends_with(b"\r\n") {
        (&data[..data.len() - 2], data.len() - 2)
    } else if data.ends_with(b"\n") {
        (&data[..data.len() - 1], data.len() - 1)
    } else {
        (data, data.len())
    }
}

fn split_headers(buf: &[u8]) -> Option<usize> {
    for form in [b"\r\n\r\n".as_slice(), b"\r\n\n", b"\n\r\n", b"\n\n"] {
        if let Some(p) = find(buf, form) {
            return Some(p + form.len());
        }
    }
    None
}

fn header_blank(buf: &[u8]) -> bool {
    buf.ends_with(b"\r\n\r\n")
        || buf.ends_with(b"\r\n\n")
        || buf.ends_with(b"\n\r\n")
        || buf.ends_with(b"\n\n")
}

/// Streams bytes to `sink` up to (not including) the next `--boundary`,
/// stripping the trailing CRLF that precedes it. `pending` holds bytes that
/// were already read while looking for the end of the part headers. Returns
/// the number of bytes written.
fn stream_to_marker(
    f: &mut BufReader<std::fs::File>,
    tail: &mut Vec<u8>,
    marker: &[u8],
    sink: &mut dyn Write,
    cap: Option<u64>,
    mut pending: Vec<u8>,
) -> std::io::Result<u64> {
    let marker_len = marker.len();
    let mut w = std::mem::take(tail);
    if !pending.is_empty() {
        w.append(&mut pending);
    }
    let mut size: u64 = 0;
    loop {
        if let Some(p) = find(&w, marker) {
            let (data, n) = strip_trailing_crlf(&w[..p]);
            sink.write_all(data)?;
            size += n as u64;
            *tail = w.split_off(p);
            return Ok(size);
        }
        let keep = (marker_len - 1).min(w.len());
        let split = w.len() - keep;
        if let Some(c) = cap {
            if size + split as u64 > c {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "multipart field too large",
                ));
            }
        }
        sink.write_all(&w[..split])?;
        size += split as u64;
        w = w.split_off(split);
        let mut chunk = [0u8; 65536];
        let n = f.read(&mut chunk)?;
        if n == 0 {
            if let Some(c) = cap {
                if size + w.len() as u64 > c {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "multipart field too large",
                    ));
                }
            }
            sink.write_all(&w)?;
            size += w.len() as u64;
            *tail = Vec::new();
            return Ok(size);
        }
        w.extend_from_slice(&chunk[..n]);
    }
}

/// Returns `(dest_path, [(filename, disk_name, size)]..., {field_name: value})`.
/// Non-file multipart fields are collected so callers can read chunk metadata.
type MultipartResult =
    std::io::Result<(String, Vec<(String, String, u64)>, HashMap<String, String>)>;

fn parse_multipart_stream(file_path: &Path, boundary: &str, base: &Path) -> MultipartResult {
    let marker = format!("--{}", boundary).into_bytes();
    let marker_len = marker.len();
    let mut f = BufReader::with_capacity(1 << 20, std::fs::File::open(file_path)?);
    let mut tail: Vec<u8> = Vec::new();
    let mut created: Vec<PathBuf> = Vec::new();

    let mut run = || -> MultipartResult {
        let mut dest_path = "/".to_string();
        let mut files: Vec<(String, String, u64)> = Vec::new();
        let mut fields: HashMap<String, String> = HashMap::new();

        loop {
            // Ensure `tail` starts at the next part marker.
            if !tail.starts_with(&marker) {
                let mut found = false;
                loop {
                    if let Some(p) = find(&tail, &marker) {
                        tail.drain(..p);
                        found = true;
                        break;
                    }
                    let mut chunk = [0u8; 65536];
                    let n = f.read(&mut chunk)?;
                    if n == 0 {
                        break;
                    }
                    let keep = (marker_len - 1).min(tail.len());
                    tail.drain(..tail.len() - keep);
                    tail.extend_from_slice(&chunk[..n]);
                }
                if !found {
                    break;
                }
            }
            // Make sure we can inspect the two bytes after the marker.
            while tail.len() < marker_len + 2 {
                let mut b = [0u8; 1];
                match f.read(&mut b) {
                    Ok(0) => break,
                    Ok(_) => tail.push(b[0]),
                    Err(_) => break,
                }
            }
            if tail.len() >= marker_len + 2 && &tail[marker_len..marker_len + 2] == b"--" {
                break; // final boundary "--boundary--"
            }
            if tail.len() < marker_len {
                break;
            }
            // Consume marker + trailing CRLF; leftover bytes start the headers.
            tail.drain(..marker_len);
            let mut skip = 0;
            if tail.starts_with(b"\r\n") {
                skip = 2;
            } else if tail.starts_with(b"\n") {
                skip = 1;
            }
            tail.drain(..skip);
            let mut header: Vec<u8> = std::mem::take(&mut tail);
            let mut pending: Vec<u8> = Vec::new();
            match split_headers(&header) {
                Some(q) => {
                    pending = header.split_off(q);
                }
                None => {
                    while !header_blank(&header) {
                        let mut b = [0u8; 1];
                        match f.read(&mut b) {
                            Ok(0) => break,
                            Ok(_) => header.push(b[0]),
                            Err(_) => break,
                        }
                    }
                }
            }

            // Parse Content-Disposition.
            let header_str = String::from_utf8_lossy(&header);
            let mut field_name = String::new();
            let mut file_name = String::new();
            for line in header_str.lines() {
                let l = line.trim();
                if l.to_lowercase().starts_with("content-disposition:") {
                    for part in l.split(';') {
                        let part = part.trim();
                        if let Some(v) = part.strip_prefix("name=\"") {
                            field_name = v.trim_end_matches('"').to_string();
                        }
                        if let Some(v) = part.strip_prefix("filename=\"") {
                            file_name = v.trim_end_matches('"').to_string();
                        }
                    }
                }
            }

            if field_name == "path" {
                let mut data: Vec<u8> = Vec::new();
                stream_to_marker(
                    &mut f,
                    &mut tail,
                    &marker,
                    &mut data,
                    Some(1 << 20),
                    pending,
                )?;
                let value = String::from_utf8_lossy(&data).to_string();
                dest_path = clean_path(&value);
                fields.insert("path".to_string(), dest_path.clone());
            } else if field_name == "file" {
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                let disk = format!("{}-{}", nanos, file_name);
                let out_path = base.join(&disk);
                let mut out = std::fs::File::create(&out_path)?;
                let size = stream_to_marker(&mut f, &mut tail, &marker, &mut out, None, pending)?;
                drop(out);
                created.push(out_path);
                files.push((file_name, disk, size));
            } else {
                let mut data: Vec<u8> = Vec::new();
                stream_to_marker(
                    &mut f,
                    &mut tail,
                    &marker,
                    &mut data,
                    Some(1 << 20),
                    pending,
                )?;
                let value = String::from_utf8_lossy(&data).to_string();
                fields.insert(field_name, value);
            }
        }
        Ok((dest_path, files, fields))
    };

    match run() {
        Ok(r) => Ok(r),
        Err(e) => {
            for p in &created {
                let _ = std::fs::remove_file(p);
            }
            Err(e)
        }
    }
}

// ── API: Create folder ────────────────────────────────────────────────────

fn api_create_folder(body: &[u8], db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let j: serde_json::Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_error(400),
    };
    let name = match j["name"].as_str() {
        Some(n) => n.trim(),
        None => return json_error(400),
    };
    let req_path = j["path"]
        .as_str()
        .map(clean_path)
        .unwrap_or_else(|| "/".to_string());
    if name.is_empty() || name.contains('/') || name.contains('\\') || name == "." || name == ".." {
        return json_error(400);
    }

    let db = lock(&db);
    if db
        .query_row(
            "SELECT COUNT(*) FROM files WHERE path=?1 AND filename=?2 AND is_dir=1",
            params![req_path, name],
            |row| row.get::<_, i64>(0),
        )
        .unwrap_or(0)
        > 0
    {
        return json_error(409);
    }
    if db
        .execute(
            "INSERT INTO files (filename, path, is_dir, size) VALUES (?1, ?2, 1, 0)",
            params![name, req_path],
        )
        .is_err()
    {
        return json_error(500);
    }
    json_ok(
        serde_json::to_vec(
            &serde_json::json!({"path": format!("{}/{}", req_path.trim_end_matches('/'), name)}),
        )
        .unwrap(),
    )
}

// ── API: Delete ───────────────────────────────────────────────────────────

fn api_delete(query: &str, db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let q = parse_query(query);
    let target = q.get("path").map(|s| s.as_str()).unwrap_or("").to_string();
    let (dir_path, filename) = match split_path(&target) {
        Ok(p) => p,
        Err(_) => return json_error(400),
    };

    let db = lock(&db);
    let is_dir = db
        .query_row(
            "SELECT is_dir FROM files WHERE path=?1 AND filename=?2",
            params![dir_path, filename],
            |row| row.get::<_, i32>(0),
        )
        .map(|d| d == 1)
        .unwrap_or(false);

    if is_dir {
        let prefix = if dir_path == "/" {
            format!("/{}", filename)
        } else {
            format!("{}/{}", dir_path.trim_end_matches('/'), filename)
        };
        if let Ok(mut s) = db.prepare("SELECT path FROM files WHERE path=?1 OR path LIKE ?2") {
            let paths: Vec<String> =
                match s.query_map(params![&prefix, format!("{}/%", &prefix)], |r| r.get(0)) {
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
            let _ = db.execute(
                "DELETE FROM files WHERE (path=?1 OR path LIKE ?2)",
                params![&prefix, format!("{}/%", &prefix)],
            );
        }
        // Remove the folder row itself (its parent path is `dir_path`, so the
        // children-only DELETE above never matches it — previously the empty
        // folder was left behind).
        let _ = db.execute(
            "DELETE FROM files WHERE path=?1 AND filename=?2 AND is_dir=1",
            params![dir_path, filename],
        );
    } else {
        // Remove the disk file(s) for every matching row, not just the first,
        // so re-uploading the same name and deleting never leaves orphans.
        if let Ok(mut s) = db.prepare(
            "SELECT disk_name FROM files WHERE path=?1 AND filename=?2 AND disk_name IS NOT NULL",
        ) {
            if let Ok(rows) =
                s.query_map(params![dir_path, filename], |row| row.get::<_, String>(0))
            {
                for disk in rows.filter_map(|r| r.ok()) {
                    let _ = std::fs::remove_file(files_dir().join(&disk));
                }
            }
        }
        let _ = db.execute(
            "DELETE FROM files WHERE path=?1 AND filename=?2",
            params![dir_path, filename],
        );
    }
    json_ok(serde_json::to_vec(&serde_json::json!({"path": target})).unwrap())
}

// ── API: Rename ───────────────────────────────────────────────────────────

fn api_rename(body: &[u8], db: Arc<Mutex<rusqlite::Connection>>) -> Vec<u8> {
    let j: serde_json::Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return json_error(400),
    };
    let old_path = match j["oldPath"].as_str() {
        Some(p) => p,
        None => return json_error(400),
    };
    let new_name = match j["newName"].as_str() {
        Some(n) => n.trim().to_string(),
        None => return json_error(400),
    };
    if new_name.is_empty()
        || new_name.contains('/')
        || new_name.contains('\\')
        || new_name == "."
        || new_name == ".."
    {
        return json_error(400);
    }
    // Normalize (handles trailing slashes) so folder renames resolve.
    let old_path = clean_path(old_path);
    let (dir_path, old_name) = match split_path(&old_path) {
        Ok(p) => p,
        Err(_) => return json_error(400),
    };

    let db = lock(&db);
    let is_dir = match db.query_row(
        "SELECT is_dir FROM files WHERE path=?1 AND filename=?2",
        params![dir_path, old_name],
        |row| row.get::<_, i32>(0),
    ) {
        Ok(d) => d,
        Err(_) => return json_error(404),
    };
    if db
        .query_row(
            "SELECT COUNT(*) FROM files WHERE path=?1 AND filename=?2",
            params![dir_path, new_name],
            |row| row.get::<_, i64>(0),
        )
        .unwrap_or(0)
        > 0
    {
        return json_error(409);
    }
    if db
        .execute(
            "UPDATE files SET filename=?1 WHERE path=?2 AND filename=?3",
            params![new_name, dir_path, old_name],
        )
        .is_err()
    {
        return json_error(500);
    }

    if is_dir == 1 {
        let old_prefix = format!("{}/{}", dir_path.trim_end_matches('/'), old_name);
        let new_prefix = format!("{}/{}", dir_path.trim_end_matches('/'), new_name);
        if let Ok(mut s) = db.prepare("SELECT id, path FROM files WHERE path=?1 OR path LIKE ?2") {
            let children: Vec<(i64, String)> = match s
                .query_map(params![old_prefix, format!("{}/%", old_prefix)], |row| {
                    Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
                }) {
                Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
                Err(_) => vec![],
            };
            for (id, cp) in &children {
                let _ = db.execute(
                    "UPDATE files SET path=?1 WHERE id=?2",
                    params![cp.replacen(&old_prefix, &new_prefix, 1), id],
                );
            }
        }
    }
    json_ok(serde_json::to_vec(&serde_json::json!({"path": old_path})).unwrap())
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn clean_path(p: &str) -> String {
    let p = p.trim().trim_start_matches('/');
    if p.is_empty() {
        return "/".to_string();
    }
    let mut out = Vec::new();
    for seg in p.split('/').filter(|s| !s.is_empty() && *s != ".") {
        if seg == ".." {
            out.pop();
        } else {
            out.push(seg.to_string());
        }
    }
    if out.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", out.join("/"))
    }
}

fn split_path(full: &str) -> Result<(String, String), ()> {
    let full = full.trim();
    if full.is_empty() || full == "/" {
        return Err(());
    }
    let clean = full.trim_start_matches('/');
    if let Some((parent, name)) = clean.rsplit_once('/') {
        Ok((
            if parent.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", parent)
            },
            name.to_string(),
        ))
    } else {
        Ok(("/".to_string(), clean.to_string()))
    }
}

fn spawn_cloudflared(port: u16) {
    let url = format!("http://localhost:{}", port);
    let cf = find_cloudflared().unwrap_or_else(|| "cloudflared".to_string());
    match std::process::Command::new(&cf)
        .args(["tunnel", "--url", &url])
        .spawn()
    {
        Ok(mut c) => {
            print_info(&t!(
                "cmd.serve.cloudflared_starting",
                c.id()
            ));
            std::thread::spawn(move || {
                let _ = c.wait();
            });
        }
        Err(e) => {
            let hint = if crate::ntpkg::manager::is_termux() {
                "pkg install cloudflared"
            } else {
                "ntpkg install cloudflare"
            };
            print_error(&t!("cmd.serve.cloudflared_missing", e, hint));
        }
    }
}

fn find_cloudflared() -> Option<String> {
    if which::which("cloudflared").is_ok() {
        return Some("cloudflared".to_string());
    }
    let p = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("bin")
        .join("cloudflared");
    if p.exists() {
        return Some(p.to_string_lossy().to_string());
    }
    #[cfg(windows)]
    {
        let pe = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ntc")
            .join("bin")
            .join("cloudflared.exe");
        if pe.exists() {
            return Some(pe.to_string_lossy().to_string());
        }
    }
    None
}

fn print_serve_help() {
    println!("{}", t!("cmd.serve.help_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.help_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!("  {}", t!("cmd.serve.help_port"));
    println!("  {}", t!("cmd.serve.help_bind"));
    println!("  {}", t!("cmd.serve.help_auth"));
    println!("  {}", t!("cmd.serve.help_tunnel"));
    println!("  {}", t!("cmd.serve.help_share"));
    println!("  {}", t!("cmd.serve.help_qr"));
    println!("  {}", t!("cmd.serve.help_no_qr"));
    println!("  {}", t!("cmd.serve.help_spa"));
    println!("  {}", t!("cmd.serve.help_web_dir"));
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.example_default"));
    println!("  {}", t!("cmd.serve.example_port"));
    println!("  {}", t!("cmd.serve.example_auth"));
    println!("  {}", t!("cmd.serve.example_tunnel"));
    println!("  {}", t!("cmd.serve.example_share"));
    println!("  {}", t!("cmd.serve.example_spa"));
    println!();
    println!("{}", t!("cmd.serve.files_stored_title").cyan().bold());
    println!("  {}", files_dir().to_string_lossy().dimmed());
    println!("{}", t!("cmd.serve.db_title").cyan().bold());
    println!("  {}", db_path().to_string_lossy().dimmed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_splits_on_first_colon_only() {
        // `@` and later `:` are literal password characters.
        assert_eq!(
            parse_auth("admin:555@ntcstorage").unwrap(),
            ("admin".to_string(), "555@ntcstorage".to_string())
        );
        assert_eq!(
            parse_auth("admin:p:a:ss").unwrap(),
            ("admin".to_string(), "p:a:ss".to_string())
        );
        assert!(parse_auth("nouser").is_err());
        assert!(parse_auth(":nopass").is_err());
        assert!(parse_auth("nouser:").is_err());
        assert!(parse_auth("").is_err());
    }

    #[test]
    fn basic_header_roundtrip_matches() {
        let expected = basic_expectation("admin", "555@ntcstorage");
        assert!(expected.starts_with("Basic "));
        // Decodes back to the exact user:pass pair.
        let encoded = expected.strip_prefix("Basic ").unwrap();
        let bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            encoded,
        )
        .unwrap();
        assert_eq!(bytes, b"admin:555@ntcstorage");
    }

    #[test]
    fn auth_matches_accepts_only_exact_header() {
        let expected = basic_expectation("admin", "secret");
        assert!(auth_matches(&expected, &expected));
        // Whitespace padding is tolerated (header parsing trims already).
        assert!(auth_matches(&format!("  {}  ", expected), &expected));
        assert!(!auth_matches("Basic wrong", &expected));
        assert!(!auth_matches("", &expected));
        assert!(!auth_matches(
            &basic_expectation("admin", "other"),
            &expected
        ));
    }

    #[test]
    fn rfc1918_ranges_detected() {
        assert!(is_rfc1918(Ipv4Addr::new(10, 0, 0, 5)));
        assert!(is_rfc1918(Ipv4Addr::new(10, 255, 255, 255)));
        assert!(is_rfc1918(Ipv4Addr::new(192, 168, 1, 10)));
        assert!(is_rfc1918(Ipv4Addr::new(172, 16, 0, 1)));
        assert!(is_rfc1918(Ipv4Addr::new(172, 31, 255, 254)));
        assert!(!is_rfc1918(Ipv4Addr::new(172, 15, 0, 1)));
        assert!(!is_rfc1918(Ipv4Addr::new(172, 32, 0, 1)));
        assert!(!is_rfc1918(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(!is_rfc1918(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn lan_priority_prefers_private_ranges() {
        let lan192 = lan_priority(Ipv4Addr::new(192, 168, 0, 2));
        let lan10 = lan_priority(Ipv4Addr::new(10, 1, 2, 3));
        let lan172 = lan_priority(Ipv4Addr::new(172, 20, 0, 4));
        let pub_ip = lan_priority(Ipv4Addr::new(8, 8, 8, 8));
        assert!(lan192 < lan10);
        assert!(lan10 < lan172);
        assert!(lan172 < pub_ip);
    }

    #[test]
    fn primary_lan_ip_never_panics() {
        // Environment-dependent (CI may have no route); must not panic.
        let _ = primary_lan_ip();
    }
}
