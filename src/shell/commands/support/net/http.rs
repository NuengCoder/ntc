//! Minimal HTTP response helpers and Basic-auth plumbing shared by every
//! `serve` service and by the chess online transport.

use crate::t;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub(crate) fn respond(stream: &mut TcpStream, status: u16, ct: &str, data: &[u8]) {
    let reason = reason_phrase(status);
    let resp = format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", status, reason, ct, data.len());
    let _ = write_all_retry(stream, resp.as_bytes());
    let _ = write_all_retry(stream, data);
}

/// Static asset response for pages that ship inside the binary: the bytes
/// change with the build, so they must never be cached by the browser.
pub(crate) fn respond_no_store(stream: &mut TcpStream, ct: &str, data: &[u8]) {
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        ct,
        data.len()
    );
    let _ = write_all_retry(stream, resp.as_bytes());
    let _ = write_all_retry(stream, data);
}

/// Status text for the codes the servers actually emit.
fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        408 => "Request Timeout",
        500 => "Internal Server Error",
        _ => "Error",
    }
}

/// Writes all bytes, retrying transient WouldBlock/TimedOut errors that can
/// happen when a socket still carries the listener's non-blocking mode.
pub(crate) fn write_all_retry(stream: &mut TcpStream, mut data: &[u8]) -> std::io::Result<()> {
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

pub(crate) fn response_bytes(status: u16, ct: &str, data: Vec<u8>) -> Vec<u8> {    let reason = reason_phrase(status);
    let head = format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n", status, reason, ct, data.len());
    let mut r = head.into_bytes();
    r.extend(data);
    r
}

pub(crate) fn json_error(status: u16) -> Vec<u8> {
    let reason = match status {
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        _ => "Error",
    };
    let body = format!(r#"{{"error":"{}"}}"#, reason);
    response_bytes(status, "application/json", body.into_bytes())
}

pub(crate) fn json_ok(data: Vec<u8>) -> Vec<u8> {
    response_bytes(200, "application/json", data)
}

pub(crate) fn respond_unauthorized(stream: &mut TcpStream) {
    let body = r#"{"error":"Unauthorized"}"#;
    let resp = format!(
        "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"ntc serve\"\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = write_all_retry(stream, resp.as_bytes());
}

/// Split `--auth <user:pass>` on the FIRST `:` (`@` and later `:` are
/// literal, so `admin:555@ntcstorage` → `admin` / `555@ntcstorage`).
pub(crate) fn parse_auth(value: &str) -> Result<(String, String)> {
    match value.split_once(':') {
        Some((user, pass)) if !user.is_empty() && !pass.is_empty() => {
            Ok((user.to_string(), pass.to_string()))
        }
        _ => anyhow::bail!("{}", t!("cmd.serve.invalid_auth")),
    }
}

/// Expected `Authorization` header value for these credentials.
pub(crate) fn basic_expectation(user: &str, pass: &str) -> String {
    let raw = format!("{}:{}", user, pass);
    let encoded =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, raw.as_bytes());
    format!("Basic {}", encoded)
}

/// Constant-time-ish equality so wrong passwords don't fail faster.
pub(crate) fn auth_matches(given: &str, expected: &str) -> bool {
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

pub(crate) fn parse_query(query: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for pair in query.split('&').filter(|s| !s.is_empty()) {
        if let Some((k, v)) = pair.split_once('=') {
            m.insert(url_decode(k), url_decode(v));
        }
    }
    m
}

pub(crate) fn url_decode(s: &str) -> String {
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

/// Base URL for a request: prefer the `Host` the client sent (and the
/// `X-Forwarded-Proto` scheme when a tunnel/proxy set it) so the `/info` page
/// is reachable through cloudflared and other reverse proxies. Falls back to
/// the URL computed from the bind address when `Host` is missing.
pub(crate) fn request_base_url(
    host: &str,
    forwarded_host: &str,
    forwarded_proto: &str,
    fallback: &str,
) -> String {
    // A proxy may rewrite `Host` to the backend and pass the public hostname
    // separately; the forwarded value wins when present.
    let forwarded = sanitize_host(forwarded_host);
    let host = if forwarded.is_empty() {
        sanitize_host(host)
    } else {
        forwarded
    };
    if host.is_empty() {
        return fallback.to_string();
    }
    // Proxies may append a list (`https,http`); the first value is the
    // client-facing one. Without a hint, inherit the fallback scheme.
    let scheme = match forwarded_proto
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "http" => "http",
        "https" => "https",
        _ => {
            if fallback.starts_with("https://") {
                "https"
            } else {
                "http"
            }
        }
    };
    format!("{}://{}", scheme, host)
}

/// Keep a `Host` value only when it is a plain authority (letters, digits,
/// `.`, `-`, `_`, `:` and IPv6 brackets). Anything else is dropped so the
/// page can never echo an injected HTML/URL fragment back to the browser.
pub(crate) fn sanitize_host(host: &str) -> String {
    let host = host.trim();
    if host.is_empty() || host.len() > 255 {
        return String::new();
    }
    if !host
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | ':' | '[' | ']'))
    {
        return String::new();
    }
    host.to_string()
}
