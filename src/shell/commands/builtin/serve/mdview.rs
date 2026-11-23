//! `serve mdview` — markdown editor with a live preview in the browser.
//!
//! Point it at a folder of markdown and edit in place: the page lists the
//! `.md` / `.markdown` files under the root, opens one, renders it as you type
//! and writes it back with `Ctrl+S`. Images, video and audio referenced from a
//! document are streamed from the same folder through `/raw`, so a screenshot
//! or a screen recording shows up in the preview without leaving the page.
//! Everything is one embedded page plus a five-route API; no build step, no
//! external editor, no CDN.
//!
//! Safety mirrors the other `serve` services: the root is canonicalized at
//! startup, every request path is resolved inside it (no absolute paths, no
//! `..`, no symlink escapes) and only markdown is readable or writable —
//! `/raw` additionally serves image, video and audio files, nothing else. The
//! service binds loopback by default.

use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::support::args::is_help_request;
use crate::shell::commands::support::net::http::{respond, respond_no_store, url_decode};
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// Request bodies carry a whole document, so this is larger than the git API's
/// 1 MB, but still bounded.
const MAX_BODY: usize = 4 * 1024 * 1024;
/// One markdown file may not exceed this when reading or writing.
const MAX_FILE: u64 = 2 * 1024 * 1024;
/// How many files the sidebar will ever list.
const MAX_FILES: usize = 500;
/// Directory depth the listing walks.
const MAX_DEPTH: usize = 4;
/// Largest single media file `/raw` will stream. Bigger than this is almost
/// certainly not something a markdown page should embed.
const MAX_MEDIA: u64 = 512 * 1024 * 1024;
/// Chunk used when streaming media, so a 400 MB clip never lands in memory.
const MEDIA_CHUNK: usize = 64 * 1024;

const PAGE: &str = include_str!("assets/mdview/index.html");
const APP_CSS: &str = include_str!("assets/mdview/app.css");
const APP_JS: &str = include_str!("assets/mdview/app.js");

const JS_TYPE: &str = "text/javascript; charset=utf-8";

/// Extensions the service will read or write. Anything else is a 400.
const MARKDOWN_EXT: &[&str] = &["md", "markdown", "mdown", "mkd"];

/// Image / video / audio extensions `/raw` may stream, with their MIME type.
const MEDIA_TYPES: &[(&str, &str)] = &[
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("webp", "image/webp"),
    ("avif", "image/avif"),
    ("bmp", "image/bmp"),
    ("svg", "image/svg+xml"),
    ("ico", "image/x-icon"),
    ("mp4", "video/mp4"),
    ("m4v", "video/mp4"),
    ("webm", "video/webm"),
    ("ogv", "video/ogg"),
    ("mov", "video/quicktime"),
    ("mp3", "audio/mpeg"),
    ("wav", "audio/wav"),
    ("ogg", "audio/ogg"),
    ("m4a", "audio/mp4"),
    ("flac", "audio/flac"),
    ("aac", "audio/aac"),
];

fn media_type(ext: &str) -> Option<&'static str> {
    MEDIA_TYPES
        .iter()
        .find(|(name, _)| *name == ext)
        .map(|(_, mime)| *mime)
}

pub fn cmd_serve_mdview(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if is_help_request(&tokens) {
        print_mdview_help();
        return Ok(false);
    }

    let mut port = Config::global_get_serve_port(ServeService::Mdview);
    let mut bind = Config::global_get_serve_bind(ServeService::Mdview);
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

    let start = match dir_arg {
        Some(dir) => {
            let path = PathBuf::from(&dir);
            if path.is_absolute() {
                path
            } else {
                nav.current_path().join(path)
            }
        }
        None => nav.current_path().to_path_buf(),
    };
    let root = start
        .canonicalize()
        .with_context(|| t!("cmd.serve.mdview_no_dir", start.display()))?;
    if !root.is_dir() {
        anyhow::bail!("{}", t!("cmd.serve.mdview_no_dir", root.display()));
    }

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&t!(
        "cmd.serve.mdview_serving",
        root.display(),
        format!("http://{}", addr)
    ));
    if !is_loopback(&bind) {
        print_info(&t!("cmd.serve.mdview_local_warning"));
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

fn print_mdview_help() {
    println!("{}", t!("cmd.serve.mdview_title").cyan().bold());
    println!();
    println!("{}", t!("cmd.serve.mdview_usage"));
    println!(
        "{}",
        t!(
            "cmd.serve.mdview_help_port",
            Config::global_get_serve_port(ServeService::Mdview)
        )
    );
    println!(
        "{}",
        t!(
            "cmd.serve.mdview_help_bind",
            Config::global_get_serve_bind(ServeService::Mdview)
        )
    );
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.mdview_example"));
}

// ── HTTP ──────────────────────────────────────────────────────────────────

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
    let mut range: Option<String> = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            if key.trim().eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().unwrap_or(0);
            } else if key.trim().eq_ignore_ascii_case("range") {
                range = Some(value.trim().to_string());
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

    let path = url_decode(raw_path);
    let query = parse_query(query);
    route(
        &mut stream,
        root,
        &method,
        &path,
        &query,
        &body,
        range.as_deref(),
    );
}

fn route(
    stream: &mut TcpStream,
    root: &Path,
    method: &str,
    path: &str,
    query: &[(String, String)],
    body: &[u8],
    range: Option<&str>,
) {
    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            respond(stream, 200, "text/html; charset=utf-8", PAGE.as_bytes());
        }
        ("GET", "/app.css") => {
            respond_no_store(stream, "text/css; charset=utf-8", APP_CSS.as_bytes());
        }
        ("GET", "/app.js") => respond_no_store(stream, JS_TYPE, APP_JS.as_bytes()),
        // Media referenced by a document. `Range` is honoured so a video can
        // be scrubbed instead of downloaded first.
        ("GET", "/raw") | ("HEAD", "/raw") => {
            let Some(rel) = query_value(query, "path") else {
                json_resp(stream, 400, json!({ "error": "path is required" }));
                return;
            };
            serve_media(stream, root, &rel, range, method == "HEAD");
        }
        ("GET", "/api/files") => match list_files(root) {
            Ok(files) => json_resp(
                stream,
                200,
                json!({ "root": root.display().to_string(), "files": files }),
            ),
            Err(err) => json_resp(stream, 500, json!({ "error": err.to_string() })),
        },
        // The active ntc theme, so the editor highlights with the same colours
        // the terminal editor uses.
        ("GET", "/api/theme") => json_resp(stream, 200, theme_json()),
        ("GET", "/api/file") => {
            let Some(rel) = query_value(query, "path") else {
                json_resp(stream, 400, json!({ "error": "path is required" }));
                return;
            };
            match read_file(root, &rel) {
                Ok((resolved, content)) => json_resp(
                    stream,
                    200,
                    json!({
                        "path": relative_name(root, &resolved),
                        "content": content,
                    }),
                ),
                Err(err) => json_resp(stream, 400, json!({ "error": err.to_string() })),
            }
        }
        ("POST", "/api/file") => {
            let payload: Value = match serde_json::from_slice(body) {
                Ok(value) => value,
                Err(_) => {
                    json_resp(stream, 400, json!({ "error": "invalid JSON body" }));
                    return;
                }
            };
            let Some(rel) = payload.get("path").and_then(Value::as_str) else {
                json_resp(stream, 400, json!({ "error": "path is required" }));
                return;
            };
            let Some(content) = payload.get("content").and_then(Value::as_str) else {
                json_resp(stream, 400, json!({ "error": "content is required" }));
                return;
            };
            match write_file(root, rel, content) {
                Ok(resolved) => json_resp(
                    stream,
                    200,
                    json!({
                        "ok": true,
                        "path": relative_name(root, &resolved),
                        "bytes": content.len(),
                    }),
                ),
                Err(err) => json_resp(stream, 400, json!({ "error": err.to_string() })),
            }
        }
        ("GET", "/about") | ("GET", "/info") | ("GET", "/help") => {
            respond(stream, 200, "text/html; charset=utf-8", ABOUT.as_bytes());
        }
        _ => json_resp(stream, 404, json!({ "error": "not found" })),
    }
}

/// The active ntc theme as CSS-ready colours.
///
/// `syntax` maps one-to-one onto the editor's token roles, `editor` carries the
/// pane background, the default text colour and the selection colour. The page
/// turns each entry into a `--t-<role>` / `--ed-*` custom property.
fn theme_json() -> Value {
    let theme = crate::utils::theme::ThemeManager::current();
    let syntax = &theme.syntax;
    json!({
        "name": theme.name,
        "syntax": {
            "keyword": syntax.keyword.to_hex(),
            "string": syntax.string.to_hex(),
            "comment": syntax.comment.to_hex(),
            "number": syntax.number.to_hex(),
            "type": syntax.r#type.to_hex(),
            "builtin": syntax.builtin.to_hex(),
            "function": syntax.function.to_hex(),
            "operator": syntax.operator.to_hex(),
            "punctuation": syntax.punctuation.to_hex(),
            "attribute": syntax.attribute.to_hex(),
            "macro": syntax.macro_token.to_hex(),
            "tag": syntax.tag.to_hex(),
            "constant": syntax.constant.to_hex(),
            "normal": syntax.normal.to_hex(),
            "muted": syntax.muted.to_hex(),
        },
        "editor": {
            "bg": theme.editor.editor_bg.to_hex(),
            "fg": syntax.normal.to_hex(),
            "selection": theme.editor.selection_bg.to_hex(),
        },
    })
}

fn json_resp(stream: &mut TcpStream, status: u16, value: Value) {
    let body = value.to_string();
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Payload Too Large",
        _ => "Error",
    };
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        status,
        reason,
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}

fn parse_query(query: &str) -> Vec<(String, String)> {
    query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| match pair.split_once('=') {
            Some((key, value)) => (url_decode(key), url_decode(value)),
            None => (url_decode(pair), String::new()),
        })
        .collect()
}

fn query_value(query: &[(String, String)], key: &str) -> Option<String> {
    query
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.clone())
}

// ── Files ─────────────────────────────────────────────────────────────────

/// Resolve a request path against the root and refuse anything that is not a
/// markdown file inside it.
fn resolve(root: &Path, rel: &str) -> Result<PathBuf> {
    resolve_with(
        root,
        rel,
        |ext| MARKDOWN_EXT.contains(&ext),
        "only markdown files can be opened",
    )
}

/// Same checks for a media file: images, video and audio only.
fn resolve_media(root: &Path, rel: &str) -> Result<PathBuf> {
    resolve_with(
        root,
        rel,
        |ext| media_type(ext).is_some(),
        "only images, video and audio can be embedded",
    )
}

/// Resolve `rel` inside `root`, letting `allowed` decide which extensions are
/// acceptable. Everything else about the path — no absolute paths, no `..`, no
/// symlink escaping the root — is enforced here for every caller.
fn resolve_with(
    root: &Path,
    rel: &str,
    allowed: impl Fn(&str) -> bool,
    message: &str,
) -> Result<PathBuf> {
    let rel = rel.trim();
    if rel.is_empty() {
        anyhow::bail!("empty path");
    }
    if rel.contains('\0') {
        anyhow::bail!("invalid path");
    }
    let candidate = Path::new(rel);
    if candidate.is_absolute() {
        anyhow::bail!("absolute paths are not allowed");
    }
    for component in candidate.components() {
        match component {
            Component::Normal(_) => {}
            _ => anyhow::bail!("paths must stay inside the served folder"),
        }
    }
    let extension = candidate
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();
    if !allowed(&extension) {
        anyhow::bail!("{}", message);
    }
    let joined = root.join(candidate);
    // The parent must exist and, once canonicalized, still sit under the root —
    // this is what stops a symlinked folder from escaping the served tree.
    let parent = joined.parent().unwrap_or(root);
    let parent = parent
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("folder does not exist"))?;
    if !parent.starts_with(root) {
        anyhow::bail!("paths must stay inside the served folder");
    }
    Ok(joined)
}

fn relative_name(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

// ── media ─────────────────────────────────────────────────────────────────

/// Parse one `Range: bytes=…` header into an inclusive `(start, end)` pair.
///
/// Only a single range is honoured — a browser playing a video asks for one.
/// Anything unparsable returns `None`, which means "send the whole file".
fn parse_range(header: Option<&str>, size: u64) -> Option<(u64, u64)> {
    let header = header?;
    let spec = header.trim().strip_prefix("bytes=")?.trim();
    if spec.contains(',') {
        return None;
    }
    let (start, end) = spec.split_once('-')?;
    let start = start.trim();
    let end = end.trim();

    if start.is_empty() {
        // `bytes=-500` → the last 500 bytes.
        let suffix: u64 = end.parse().ok()?;
        if suffix == 0 || size == 0 {
            return None;
        }
        let suffix = suffix.min(size);
        return Some((size - suffix, size - 1));
    }

    let start: u64 = start.parse().ok()?;
    if start >= size {
        return None;
    }
    let end = if end.is_empty() {
        size - 1
    } else {
        end.parse::<u64>().ok()?.min(size - 1)
    };
    if end < start {
        return None;
    }
    Some((start, end))
}

/// Stream one media file, honouring a single byte range so video scrubbing
/// works. The file is copied in chunks; nothing is loaded whole.
fn serve_media(
    stream: &mut TcpStream,
    root: &Path,
    rel: &str,
    range: Option<&str>,
    head_only: bool,
) {
    let resolved = match resolve_media(root, rel) {
        Ok(path) => path,
        Err(err) => {
            json_resp(stream, 400, json!({ "error": err.to_string() }));
            return;
        }
    };
    let metadata = match std::fs::metadata(&resolved) {
        Ok(metadata) => metadata,
        Err(_) => {
            json_resp(stream, 404, json!({ "error": "file does not exist" }));
            return;
        }
    };
    if !metadata.is_file() {
        json_resp(stream, 404, json!({ "error": "not a file" }));
        return;
    }
    let size = metadata.len();
    if size > MAX_MEDIA {
        json_resp(
            stream,
            413,
            json!({ "error": "file is larger than 512 MB" }),
        );
        return;
    }

    let extension = resolved
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();
    let mime = media_type(&extension).unwrap_or("application/octet-stream");

    let (start, end, status, reason) = match parse_range(range, size) {
        Some((start, end)) => (start, end, 206, "Partial Content"),
        None => (0, size.saturating_sub(1), 200, "OK"),
    };
    let length = if size == 0 { 0 } else { end - start + 1 };

    let mut head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccept-Ranges: bytes\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n",
        status, reason, mime, length
    );
    if status == 206 {
        head.push_str(&format!("Content-Range: bytes {}-{}/{}\r\n", start, end, size));
    }
    head.push_str("\r\n");
    if stream.write_all(head.as_bytes()).is_err() {
        return;
    }
    if head_only || length == 0 {
        return;
    }

    let Ok(mut file) = std::fs::File::open(&resolved) else {
        return;
    };
    if file.seek(SeekFrom::Start(start)).is_err() {
        return;
    }
    let mut remaining = length;
    let mut buffer = vec![0u8; MEDIA_CHUNK];
    while remaining > 0 {
        let want = MEDIA_CHUNK.min(remaining as usize);
        let read = match file.read(&mut buffer[..want]) {
            Ok(0) | Err(_) => return,
            Ok(read) => read,
        };
        if stream.write_all(&buffer[..read]).is_err() {
            return;
        }
        remaining -= read as u64;
    }
}

fn read_file(root: &Path, rel: &str) -> Result<(PathBuf, String)> {
    let resolved = resolve(root, rel)?;
    let metadata =
        std::fs::metadata(&resolved).map_err(|_| anyhow::anyhow!("file does not exist"))?;
    if !metadata.is_file() {
        anyhow::bail!("not a file");
    }
    if metadata.len() > MAX_FILE {
        anyhow::bail!("file is larger than 2 MB");
    }
    let bytes = std::fs::read(&resolved)?;
    Ok((resolved, String::from_utf8_lossy(&bytes).to_string()))
}

/// Write through a sibling temp file so a crash cannot truncate the document.
fn write_file(root: &Path, rel: &str, content: &str) -> Result<PathBuf> {
    if content.len() as u64 > MAX_FILE {
        anyhow::bail!("file would be larger than 2 MB");
    }
    let resolved = resolve(root, rel)?;
    let temp = resolved.with_extension("mdview-tmp");
    {
        let mut file = std::fs::File::create(&temp)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
    }
    // Windows refuses to rename over an existing file.
    if std::fs::rename(&temp, &resolved).is_err() {
        let _ = std::fs::remove_file(&resolved);
        std::fs::rename(&temp, &resolved).map_err(|err| {
            let _ = std::fs::remove_file(&temp);
            anyhow::anyhow!("{}", err)
        })?;
    }
    Ok(resolved)
}

fn list_files(root: &Path) -> Result<Vec<Value>> {
    let mut files = Vec::new();
    walk(root, root, 0, &mut files);
    files.sort_by(|a, b| {
        let left = a.get("path").and_then(Value::as_str).unwrap_or("");
        let right = b.get("path").and_then(Value::as_str).unwrap_or("");
        left.cmp(right)
    });
    Ok(files)
}

fn walk(root: &Path, dir: &Path, depth: usize, out: &mut Vec<Value>) {
    if depth > MAX_DEPTH || out.len() >= MAX_FILES {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut children: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    children.sort();
    for path in children {
        if out.len() >= MAX_FILES {
            return;
        }
        let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            if matches!(name, "node_modules" | "target" | "dist" | "build") {
                continue;
            }
            walk(root, &path, depth + 1, out);
            continue;
        }
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .unwrap_or_default();
        if !MARKDOWN_EXT.contains(&extension.as_str()) {
            continue;
        }
        let Ok(metadata) = std::fs::metadata(&path) else {
            continue;
        };
        if metadata.len() > MAX_FILE {
            continue;
        }
        out.push(json!({
            "path": relative_name(root, &path),
            "size": metadata.len(),
        }));
    }
}

const ABOUT: &str = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>NTC mdview — about</title>
<style>
:root{--bg:#0f1115;--panel:#171a21;--line:#2a2f3a;--text:#e6e9ef;--muted:#8b93a3;--accent:#5aa9ff}
body{margin:0;font:14px/1.6 ui-sans-serif,system-ui,-apple-system,"Segoe UI",sans-serif;background:var(--bg);color:var(--text)}
main{max-width:760px;margin:0 auto;padding:28px 20px 60px}
h1{font-size:18px;letter-spacing:.03em}h1 span{color:var(--accent)}
h2{font-size:12px;text-transform:uppercase;letter-spacing:.07em;color:var(--muted);margin:26px 0 8px}
p,li{color:#cfd6e2}code{font-family:ui-monospace,Consolas,monospace;font-size:12.5px;background:var(--panel);border:1px solid var(--line);border-radius:4px;padding:0 4px}
a{color:var(--accent)}
</style></head><body><main>
<h1>NTC <span>mdview</span></h1>
<p>A local markdown editor: the left rail lists the <code>.md</code> / <code>.markdown</code>
files in the folder you served, the middle pane is the editor, the right pane is the
rendered document. Nothing is uploaded anywhere.</p>
<h2>Images and video</h2>
<p>Media is served from the same folder, so a document can show a screenshot or play a
screen recording:</p>
<ul>
<li><code>![shot](pics/login.png)</code> renders an image.</li>
<li><code>![demo](clips/build.mp4)</code> renders a video player with controls and seeking.</li>
<li><code>![note](audio/note.mp3)</code> renders an audio player.</li>
<li>Paths are relative to the document, so <code>../img/a.png</code> from
<code>docs/guide.md</code> resolves to <code>img/a.png</code>. A leading <code>/</code> means
the served folder, and <code>http(s)://</code> URLs are used as written.</li>
<li><code>[other](other.md)</code> opens that document in the editor.</li>
</ul>
<p>Only image, video and audio extensions are streamed, and only from inside the served
folder. A playing video keeps its position while you type.</p>
<h2>Raw HTML</h2>
<p>A README usually centres its badge with plain HTML, so those blocks are passed through
instead of escaped — <code>&lt;p align="center"&gt;</code>, <code>&lt;img&gt;</code>,
<code>&lt;video&gt;</code>, <code>&lt;a&gt;</code>, <code>&lt;b&gt;</code>, <code>&lt;br&gt;</code>,
<code>&lt;h1&gt;</code> and friends all work. It is a curated allowlist: an unknown tag is
dropped, <code>on*</code> handlers, <code>style</code>, <code>class</code> and
<code>javascript:</code> URLs never survive, and <code>&lt;script&gt;</code>,
<code>&lt;iframe&gt;</code>, <code>&lt;object&gt;</code> and <code>&lt;embed&gt;</code> are removed
entirely. Media inside a tag is routed through the same folder check.</p>
<h2>Editor highlighting</h2>
<p>The editor is a transparent textarea over a highlighted layer, so markdown is coloured while
you type: headings, list markers, quotes, inline code, links, table pipes and raw HTML tags.
The colours are not baked into the page — they come from your active ntc theme
(<code>theme list</code> / <code>theme set &lt;name&gt;</code>), the same palette
<code>ne</code> uses, and the editor background is the theme's editor background. If you have
never picked a page theme, light or dark follows the theme's brightness.</p>
<h2>Keys</h2>
<ul>
<li><code>Ctrl+S</code> / <code>Cmd+S</code> saves the current file.</li>
<li><code>Ctrl+N</code> starts a new document.</li>
<li><code>Ctrl+B</code> / <code>Ctrl+I</code> wrap the selection in bold / italic.</li>
<li><code>Tab</code> indents by two spaces instead of leaving the editor.</li>
</ul>
<h2>Rules</h2>
<ul>
<li>Only markdown files inside the served folder can be opened or saved; only images,
video and audio can be embedded.</li>
<li>Writes go through a temp file, so an interrupted save cannot truncate the document.</li>
<li>The server binds <code>127.0.0.1</code> by default. There is no authentication: only use
<code>--bind</code> on a network you trust.</li>
</ul>
<p><a href="/">Back to the editor</a></p>
</main></body></html>"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch folder with markdown, a text file, a hidden file and a
    /// dependency folder to prove the listing filters them.
    fn fixture() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let docs = dir.path().join("docs");
        std::fs::create_dir_all(docs.join("nested")).unwrap();
        std::fs::create_dir_all(docs.join("node_modules/pkg")).unwrap();
        std::fs::write(docs.join("readme.md"), "# hi").unwrap();
        std::fs::write(docs.join("nested/guide.markdown"), "# guide").unwrap();
        std::fs::write(docs.join("notes.txt"), "not markdown").unwrap();
        std::fs::write(docs.join(".hidden.md"), "hidden").unwrap();
        std::fs::write(docs.join("node_modules/pkg/readme.md"), "dep").unwrap();
        let root = docs.canonicalize().unwrap();
        (dir, root)
    }

    #[test]
    fn listing_skips_non_markdown_hidden_and_dependency_folders() {
        let (_dir, root) = fixture();
        let files = list_files(&root).unwrap();
        let names: Vec<&str> = files
            .iter()
            .filter_map(|file| file.get("path").and_then(Value::as_str))
            .collect();
        assert_eq!(names, vec!["nested/guide.markdown", "readme.md"]);
    }

    #[test]
    fn resolve_accepts_markdown_inside_the_root() {
        let (_dir, root) = fixture();
        let path = resolve(&root, "readme.md").unwrap();
        assert!(path.starts_with(&root));
        assert!(resolve(&root, "nested/guide.markdown").is_ok());
    }

    #[test]
    fn resolve_rejects_traversal_absolute_and_other_extensions() {
        let (_dir, root) = fixture();
        for bad in [
            "../escape.md",
            "nested/../../escape.md",
            "C:/Windows/win.ini",
            "/etc/passwd",
            "notes.txt",
            "readme",
            "",
        ] {
            assert!(resolve(&root, bad).is_err(), "{:?} should be rejected", bad);
        }
    }

    #[test]
    fn save_round_trips_through_a_temp_file() {
        let (_dir, root) = fixture();
        write_file(&root, "readme.md", "# changed").unwrap();
        let (_, content) = read_file(&root, "readme.md").unwrap();
        assert_eq!(content, "# changed");
        assert!(!root.join("readme.mdview-tmp").exists());
    }

    #[test]
    fn reading_a_missing_file_is_an_error_not_a_panic() {
        let (_dir, root) = fixture();
        assert!(read_file(&root, "missing.md").is_err());
    }

    #[test]
    fn embedded_page_has_the_split_editor_and_no_external_assets() {
        assert!(PAGE.contains("NTC mdview"));
        assert!(APP_JS.contains("global.md = { render:"));
        assert!(APP_JS.contains("renderPreview"));
        assert!(!PAGE.contains("http://"));
        assert!(!PAGE.contains("https://"));
    }

    /// The fixture grows a media folder: one image, one clip and a text file
    /// that must stay unreachable through `/raw`.
    fn media_fixture() -> (tempfile::TempDir, PathBuf) {
        let (dir, root) = fixture();
        std::fs::create_dir_all(root.join("pics")).unwrap();
        std::fs::write(root.join("pics/a.png"), b"\x89PNG\r\n\x1a\n").unwrap();
        std::fs::write(root.join("clips.mp4"), vec![0u8; 1024]).unwrap();
        std::fs::write(root.join("pics/notes.txt"), "no").unwrap();
        (dir, root)
    }

    #[test]
    fn media_resolution_accepts_images_and_video_inside_the_root() {
        let (_dir, root) = media_fixture();
        assert!(resolve_media(&root, "pics/a.png").is_ok());
        assert!(resolve_media(&root, "clips.mp4").is_ok());
        assert!(resolve_media(&root, "pics/A.PNG").is_ok());
    }

    #[test]
    fn media_resolution_rejects_traversal_and_other_extensions() {
        let (_dir, root) = media_fixture();
        for bad in [
            "../escape.png",
            "pics/../../escape.png",
            "C:/Windows/win.ini",
            "/etc/passwd",
            "pics/notes.txt",
            "readme.md",
            "",
        ] {
            assert!(
                resolve_media(&root, bad).is_err(),
                "{:?} should be rejected",
                bad
            );
        }
    }

    #[test]
    fn every_media_extension_has_a_content_type() {
        for (ext, mime) in MEDIA_TYPES {
            assert_eq!(media_type(ext), Some(*mime));
            assert!(mime.contains('/'), "{ext} has a bad MIME type");
        }
        assert_eq!(media_type("txt"), None);
        assert_eq!(media_type("md"), None);
    }

    #[test]
    fn range_headers_are_parsed_for_video_seeking() {
        assert_eq!(parse_range(Some("bytes=0-99"), 1000), Some((0, 99)));
        assert_eq!(parse_range(Some("bytes=500-"), 1000), Some((500, 999)));
        assert_eq!(parse_range(Some("bytes=-200"), 1000), Some((800, 999)));
        // A range that runs past the end is clamped, not rejected.
        assert_eq!(parse_range(Some("bytes=900-5000"), 1000), Some((900, 999)));
        // Out of range, multi-range and junk all mean "send the whole file".
        assert_eq!(parse_range(Some("bytes=1000-"), 1000), None);
        assert_eq!(parse_range(Some("bytes=0-1,5-6"), 1000), None);
        assert_eq!(parse_range(Some("items=0-1"), 1000), None);
        assert_eq!(parse_range(Some("bytes=abc"), 1000), None);
        assert_eq!(parse_range(None, 1000), None);
        assert_eq!(parse_range(Some("bytes=-0"), 1000), None);
        assert_eq!(parse_range(Some("bytes=0-0"), 0), None);
    }

    #[test]
    fn the_page_routes_media_through_the_raw_endpoint() {
        assert!(APP_JS.contains("/raw?path="));
        assert!(APP_JS.contains("mediaFigure(\"video\""));
        assert!(APP_JS.contains("data-md"));
    }

    #[test]
    fn raw_html_is_passed_through_a_curated_allowlist() {
        assert!(APP_JS.contains("function sanitizeHtml"));
        assert!(APP_JS.contains("function sanitizeTag"));
        assert!(APP_JS.contains("function safeUrl"));
        assert!(APP_JS.contains("HTML_TAGS"));
        // Script-bearing tags are not in the list, so `sanitizeTag` drops them.
        for tag in ["iframe", "object", "embed", "script", "style", "form"] {
            assert!(
                !APP_JS.contains(&format!(" {tag} ")),
                "{tag} must not be an allowed tag"
            );
        }
        // Event handlers never survive: `on*` is not in the attribute list.
        assert!(!APP_JS.contains(" onerror"));
        assert!(!APP_JS.contains(" onclick"));
        assert!(APP_JS.contains("javascript|vbscript"));
    }

    #[test]
    fn the_editor_paints_a_highlight_layer_behind_the_textarea() {
        assert!(PAGE.contains("id=\"hl\""));
        assert!(PAGE.contains("class=\"hl\""));
        assert!(APP_JS.contains("function highlightMarkdown"));
        assert!(APP_JS.contains("function inlineMarkdown"));
        assert!(APP_JS.contains("function paintHighlight"));
        assert!(APP_JS.contains("syncHighlightScroll"));
        // Colours come from the theme endpoint, not a hardcoded palette.
        assert!(APP_JS.contains("/api/theme"));
        assert!(APP_JS.contains("--t-"));
    }

    #[test]
    fn the_theme_endpoint_reports_css_colours() {
        let value = theme_json();
        let syntax = value
            .get("syntax")
            .and_then(Value::as_object)
            .expect("syntax object");
        for role in [
            "keyword",
            "string",
            "comment",
            "number",
            "type",
            "builtin",
            "function",
            "operator",
            "punctuation",
            "attribute",
            "macro",
            "tag",
            "constant",
            "normal",
            "muted",
        ] {
            let colour = syntax.get(role).and_then(Value::as_str).unwrap_or("");
            assert!(
                colour.len() == 7 && colour.starts_with('#'),
                "{role} = {colour}"
            );
        }
        let editor = value
            .get("editor")
            .and_then(Value::as_object)
            .expect("editor object");
        for key in ["bg", "fg", "selection"] {
            let colour = editor.get(key).and_then(Value::as_str).unwrap_or("");
            assert!(colour.len() == 7 && colour.starts_with('#'), "{key} = {colour}");
        }
    }
}
