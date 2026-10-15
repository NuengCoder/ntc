use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::t;
use anyhow::{Context, Result};
use colored::*;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::SystemTime;

use super::serve::{
    auth_matches, basic_expectation, is_help_request, parse_auth, print_share_block, respond,
    respond_unauthorized, response_bytes, spawn_cloudflared, tokenize, url_decode, write_all_retry,
};

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve_http(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_http_help();
        return Ok(false);
    }

    // Defaults come from `serve config http-server` (see `config serve`).
    let mut port = Config::global_get_serve_port(ServeService::Http);
    let mut bind = Config::global_get_serve_bind(ServeService::Http);
    let mut auth: Option<(String, String)> = None;
    let mut use_tunnel = false;
    let mut share_mode = false;
    let mut share_path: Option<String> = None;
    let mut no_qr = false;
    let mut want_qr = false;
    let mut dir_arg: Option<String> = None;

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "help" | "--help" | "-h" => {
                print_http_help();
                return Ok(false);
            }
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
            "--spa" | "-w" | "--web-dir" => {
                anyhow::bail!(
                    "{} (http-server is read-only static; --spa/--web-dir belong to `serve files`)",
                    t!("cmd.serve.unknown_option", tokens[i])
                );
            }
            "--" => break,
            other => {
                if let Ok(p) = other.parse::<u16>() {
                    // Bare numeric — but only treat as port when it looks like
                    // one AND no dir was given yet that looks numeric... a dir
                    // named "3000" is pathological; port wins (documented).
                    port = p;
                } else if other.starts_with('-') {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
                } else if dir_arg.is_none() {
                    dir_arg = Some(other.to_string());
                } else {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
                }
            }
        }
        i += 1;
    }

    // Resolve root: CWD (navigator path) by default, optional [dir].
    let root = match dir_arg {
        Some(d) => {
            let p = PathBuf::from(&d);
            let joined = if p.is_absolute() {
                p
            } else {
                nav.current_path().join(&p)
            };
            joined
        }
        None => nav.current_path().to_path_buf(),
    };
    let root = root
        .canonicalize()
        .with_context(|| t!("cmd.serve.http_no_dir", root.display()))?;
    if !root.is_dir() {
        anyhow::bail!("{}", t!("cmd.serve.http_no_dir", root.display()));
    }

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    // `canonicalize` yields a `\\?\`-prefixed verbatim path on Windows;
    // strip it for display only (resolution still uses `root`).
    let shown = root.display().to_string();
    let shown = shown.strip_prefix("\\\\?\\").unwrap_or(&shown);
    print_info(&format!(
        "{} {} ({})",
        t!("cmd.serve.http_serving"),
        shown.cyan(),
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
    listener.set_nonblocking(true)?;

    let auth = auth.map(|(u, p)| Arc::new((u, p)));
    while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let root = root.clone();
                let auth = auth.clone();
                std::thread::spawn(move || handle_http_connection(stream, &root, auth));
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

fn handle_http_connection(
    mut stream: std::net::TcpStream,
    root: &Path,
    auth: Option<Arc<(String, String)>>,
) {
    let _ = stream.set_nonblocking(false);
    let peer = stream
        .peer_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "-".to_string());
    let mut reader = BufReader::new(&mut stream);

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
    let (raw_path, _query) = full_path.split_once('?').unwrap_or((full_path, ""));

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
            if k.trim().eq_ignore_ascii_case("authorization") {
                authorization = v.trim().to_string();
            }
        }
    }
    // End the BufReader borrow so we can write via `stream` again.
    drop(reader);

    // Auth gates EVERYTHING (no login page for a raw file server).
    if let Some(ref creds) = auth {
        let expected = basic_expectation(&creds.0, &creds.1);
        if !auth_matches(&authorization, &expected) {
            respond_unauthorized(&mut stream);
            log_access(&peer, method, full_path, 401);
            return;
        }
    }

    if method != "GET" && method != "HEAD" {
        respond(&mut stream, 405, "text/plain", b"405 Method Not Allowed");
        log_access(&peer, method, full_path, 405);
        return;
    }
    let is_head = method == "HEAD";

    match resolve_fs_path(root, raw_path) {
        Resolve::Forbidden => {
            respond(&mut stream, 403, "text/plain", b"403 Forbidden");
            log_access(&peer, method, full_path, 403);
        }
        Resolve::NotFound => {
            respond(&mut stream, 404, "text/plain", b"404 Not Found");
            log_access(&peer, method, full_path, 404);
        }
        Resolve::Dir(fs_dir, display) => {
            let body = render_listing(&display, &fs_dir);
            if is_head {
                // Headers only: same Content-Length a GET would send.
                let head = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
                    body.len()
                );
                let _ = write_all_retry(&mut stream, head.as_bytes());
            } else {
                let _ = write_all_retry(
                    &mut stream,
                    &response_bytes(200, "text/html; charset=utf-8", body),
                );
            }
            log_access(&peer, method, full_path, 200);
        }
        Resolve::File(fs_file) => {
            let meta = match std::fs::metadata(&fs_file) {
                Ok(m) => m,
                Err(_) => {
                    respond(&mut stream, 404, "text/plain", b"404 Not Found");
                    log_access(&peer, method, full_path, 404);
                    return;
                }
            };
            let mime = mime_guess::from_path(&fs_file)
                .first_or_octet_stream()
                .to_string();
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
                mime,
                meta.len()
            );
            let _ = write_all_retry(&mut stream, head.as_bytes());
            if is_head {
                log_access(&peer, method, full_path, 200);
                return;
            }
            if let Ok(mut f) = std::fs::File::open(&fs_file) {
                let mut buf = vec![0u8; 65536];
                loop {
                    match f.read(&mut buf) {
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
            log_access(&peer, method, full_path, 200);
        }
    }
}

/// One-line access log, python-`http.server` style: `127.0.0.1 - GET /path 200`.
/// Always on; unparsable requests (no method/path) stay silent.
fn log_access(peer: &str, method: &str, full_path: &str, status: u16) {
    println!("{} - {} {} {}", peer, method, full_path, status);
}

// ── Path resolution (traversal-safe) ───────────────────────────────────────

enum Resolve {
    Forbidden,
    NotFound,
    Dir(PathBuf, String),
    File(PathBuf),
}

fn resolve_fs_path(root: &Path, raw_path: &str) -> Resolve {
    if raw_path.contains('\0') {
        return Resolve::Forbidden;
    }
    let decoded = url_decode(raw_path);
    if decoded.contains('\0') {
        return Resolve::Forbidden;
    }
    // Strip query (already split by caller, but be safe) and leading slashes.
    let rel = decoded
        .split('?')
        .next()
        .unwrap_or("")
        .trim_start_matches('/');
    // Join segment-by-segment so `..` can't escape before canonicalization.
    let mut candidate = root.to_path_buf();
    for seg in rel.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                candidate.pop();
            }
            s => candidate.push(s),
        }
    }
    let canonical = match candidate.canonicalize() {
        Ok(c) => c,
        Err(_) => return Resolve::NotFound,
    };
    if !canonical.starts_with(root) {
        return Resolve::Forbidden;
    }
    if canonical.is_dir() {
        let display = if rel.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", rel.trim_end_matches('/'))
        };
        Resolve::Dir(canonical, display)
    } else if canonical.is_file() {
        Resolve::File(canonical)
    } else {
        Resolve::NotFound
    }
}

// ── Listing HTML (python-style) ────────────────────────────────────────────

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn fmt_size(len: u64) -> String {
    if len < 1024 {
        format!("{} B", len)
    } else if len < 1024 * 1024 {
        format!("{:.1} KB", len as f64 / 1024.0)
    } else if len < 1024 * 1024 * 1024 {
        format!("{:.1} MB", len as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", len as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn fmt_mtime(m: Option<SystemTime>) -> String {
    match m {
        Some(t) => {
            let dt: chrono::DateTime<chrono::Local> = t.into();
            dt.format("%Y-%m-%d %H:%M").to_string()
        }
        None => "-".to_string(),
    }
}

struct Entry {
    name: String,
    is_dir: bool,
    size: u64,
    mtime: Option<SystemTime>,
}

fn render_listing(display_path: &str, fs_dir: &Path) -> Vec<u8> {
    let mut entries: Vec<Entry> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(fs_dir) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let (is_dir, size, mtime) = match e.metadata() {
                Ok(m) => (m.is_dir(), m.len(), m.modified().ok()),
                Err(_) => (false, 0, None),
            };
            entries.push(Entry {
                name,
                is_dir,
                size,
                mtime,
            });
        }
    }
    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    let base = if display_path == "/" {
        String::new()
    } else {
        display_path.to_string()
    };
    let mut h = String::new();
    h.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\">");
    h.push_str(&format!(
        "<title>Index of {}</title>",
        html_escape(display_path)
    ));
    h.push_str("<style>body{font-family:sans-serif;max-width:900px;margin:2em auto;padding:0 1em}table{width:100%;border-collapse:collapse}a{text-decoration:none}td,th{padding:.35em .6em;border-bottom:1px solid #ddd;text-align:left}.num{text-align:right;white-space:nowrap}</style>");
    h.push_str("</head><body>");
    h.push_str(&format!(
        "<h1>Index of {}</h1><hr>",
        html_escape(display_path)
    ));
    h.push_str("<table><tr><th>Name</th><th class=\"num\">Size</th><th>Modified</th></tr>");
    if display_path != "/" {
        let parent = match base.rfind('/') {
            Some(0) | None => "/".to_string(),
            Some(i) => base[..i].to_string(),
        };
        h.push_str(&format!(
            "<tr><td><a href=\"{}\">../</a></td><td class=\"num\">-</td><td>-</td></tr>",
            html_escape(&parent)
        ));
    }
    for e in &entries {
        let href = if base.is_empty() {
            format!("/{}", url_encode(&e.name))
        } else {
            format!("{}/{}", base, url_encode(&e.name))
        };
        let href = if e.is_dir { format!("{}/", href) } else { href };
        let label = if e.is_dir {
            format!("{}/", html_escape(&e.name))
        } else {
            html_escape(&e.name)
        };
        let size = if e.is_dir {
            "-".to_string()
        } else {
            fmt_size(e.size)
        };
        h.push_str(&format!(
            "<tr><td><a href=\"{}\">{}</a></td><td class=\"num\">{}</td><td>{}</td></tr>",
            html_escape(&href),
            label,
            size,
            fmt_mtime(e.mtime)
        ));
    }
    h.push_str("</table><hr></body></html>");
    h.into_bytes()
}

// ── Help ───────────────────────────────────────────────────────────────────

fn print_http_help() {
    println!("{}", t!("cmd.serve.http_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.http_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!("  {}", t!("cmd.serve.help_port"));
    println!("  {}", t!("cmd.serve.help_bind"));
    println!("  {}", t!("cmd.serve.help_auth_http"));
    println!("  {}", t!("cmd.serve.help_tunnel"));
    println!("  {}", t!("cmd.serve.help_share"));
    println!("  {}", t!("cmd.serve.help_qr"));
    println!("  {}", t!("cmd.serve.help_no_qr"));
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.example_http_default"));
    println!("  {}", t!("cmd.serve.example_http_dir"));
    println!("  {}", t!("cmd.serve.example_http_auth"));
    println!();
    println!(
        "  {}",
        t!(
            "cmd.serve.current_defaults",
            Config::global_get_serve_port(ServeService::Http),
            Config::global_get_serve_bind(ServeService::Http)
        )
        .dimmed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_escape_covers_specials() {
        assert_eq!(
            html_escape("<a href=\"x\">&'"),
            "&lt;a href=&quot;x&quot;&gt;&amp;'"
        );
    }

    #[test]
    fn url_encode_leaves_unreserved() {
        assert_eq!(url_encode("abc-_.~19"), "abc-_.~19");
        assert_eq!(url_encode("a b/c"), "a%20b%2Fc");
    }

    #[test]
    fn traversal_is_forbidden_or_contained() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        std::fs::create_dir(root.join("sub")).unwrap();
        std::fs::write(root.join("sub").join("ok.txt"), b"hi").unwrap();
        // Normal file resolves.
        assert!(matches!(
            resolve_fs_path(&root, "/sub/ok.txt"),
            Resolve::File(_)
        ));
        // `..` that escapes root can only resolve inside root or be blocked.
        match resolve_fs_path(&root, "/../ok.txt") {
            Resolve::Forbidden | Resolve::NotFound => {}
            Resolve::File(p) | Resolve::Dir(p, _) => {
                assert!(p.starts_with(&root));
            }
        }
        // Encoded traversal also contained.
        match resolve_fs_path(&root, "/sub/%2e%2e/%2e%2e/ok.txt") {
            Resolve::Forbidden | Resolve::NotFound => {}
            Resolve::File(p) | Resolve::Dir(p, _) => {
                assert!(p.starts_with(&root));
            }
        }
        // Missing file is 404, not 403.
        assert!(matches!(
            resolve_fs_path(&root, "/nope.txt"),
            Resolve::NotFound
        ));
    }

    #[test]
    fn listing_sorts_dirs_first() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("z.txt"), b"z").unwrap();
        std::fs::write(dir.path().join("a.txt"), b"a").unwrap();
        std::fs::create_dir(dir.path().join("m")).unwrap();
        let root = dir.path().canonicalize().unwrap();
        let body = String::from_utf8(render_listing("/", &root)).unwrap();
        let pos_m = body.find("m/").unwrap();
        let pos_a = body.find("a.txt").unwrap();
        assert!(pos_m < pos_a);
    }
}
