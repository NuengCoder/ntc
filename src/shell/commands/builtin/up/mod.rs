//! `up` — upload a file or data over multipart or raw HTTP.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::print_info;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::time::{Duration, Instant};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::net::{format_size, tokenize};

const UP_DEFAULT_TIMEOUT: u64 = 300;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Transport {
    /// multipart/form-data with the file in a `file` field, which is what
    /// `ntc serve files` and most upload endpoints expect.
    Multipart,
    /// The raw file bytes as the request body.
    Raw,
}

struct UpOptions {
    url: String,
    file: Option<String>,
    data: Option<String>,
    headers: Vec<(String, String)>,
    forms: Vec<(String, String)>,
    auth: Option<String>,
    bearer: Option<String>,
    transport: Transport,
    method: &'static str,
    no_fallback: bool,
    timeout: u64,
}

pub fn cmd_up(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_up_help();
        return Ok(false);
    }
    let opts = parse_up_args(&tokens)?;
    do_upload(&opts)
}

/// Parses `up` arguments order-independently: the first `http(s)://` token is
/// the URL, the other positional is the file.
fn parse_up_args(tokens: &[String]) -> Result<UpOptions> {
    let mut opts = UpOptions {
        url: String::new(),
        file: None,
        data: None,
        headers: Vec::new(),
        forms: Vec::new(),
        auth: None,
        bearer: None,
        transport: Transport::Multipart,
        method: "POST",
        no_fallback: false,
        timeout: UP_DEFAULT_TIMEOUT,
    };
    let mut raw = false;
    let mut put = false;
    let mut literal = false;
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens[i].as_str();
        let is_flag = !literal
            && matches!(
                token,
                "--" | "-H"
                    | "--header"
                    | "-d"
                    | "--data"
                    | "-a"
                    | "--auth"
                    | "--bearer"
                    | "-F"
                    | "--form"
                    | "--raw"
                    | "--put"
                    | "--post"
                    | "--no-fallback"
                    | "--timeout"
            );

        if !is_flag {
            if token.starts_with("http://") || token.starts_with("https://") {
                if opts.url.is_empty() {
                    opts.url = token.to_string();
                } else if opts.file.is_none() {
                    opts.file = Some(token.to_string());
                }
            } else if opts.file.is_none() {
                opts.file = Some(token.to_string());
            }
            i += 1;
            continue;
        }

        match token {
            "--" => literal = true,
            "-H" | "--header" => {
                i += 1;
                let header = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.missing_header")))?;
                match header.split_once(':') {
                    Some((key, value)) => opts
                        .headers
                        .push((key.trim().to_string(), value.trim().to_string())),
                    None => anyhow::bail!("{}", t!("dl.invalid_header", header)),
                }
            }
            "-d" | "--data" => {
                i += 1;
                opts.data = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.missing_data")))?
                        .clone(),
                );
            }
            "-a" | "--auth" => {
                i += 1;
                opts.auth = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("up.missing_auth")))?
                        .clone(),
                );
            }
            "--bearer" => {
                i += 1;
                opts.bearer = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("up.missing_bearer")))?
                        .clone(),
                );
            }
            "-F" | "--form" => {
                i += 1;
                let field = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("up.missing_form")))?;
                match field.split_once('=') {
                    Some((name, value)) => opts
                        .forms
                        .push((name.trim().to_string(), value.to_string())),
                    None => anyhow::bail!("{}", t!("up.invalid_form", field)),
                }
            }
            "--raw" => raw = true,
            "--put" => put = true,
            "--post" => put = false,
            "--no-fallback" => opts.no_fallback = true,
            "--timeout" => {
                i += 1;
                let value = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("up.missing_timeout")))?;
                let secs: u64 = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("up.invalid_timeout", value)))?;
                if secs == 0 {
                    anyhow::bail!("{}", t!("up.invalid_timeout", value));
                }
                opts.timeout = secs;
            }
            _ => literal = true,
        }
        i += 1;
    }

    if opts.url.is_empty() {
        anyhow::bail!("{}", t!("dl.up_url_required"));
    }
    if opts.file.is_some() && opts.data.is_some() {
        anyhow::bail!("{}", t!("up.conflict"));
    }

    if opts.file.is_some() && !raw && !put {
        opts.transport = Transport::Multipart;
    } else {
        opts.transport = Transport::Raw;
    }
    opts.method = if put { "PUT" } else { "POST" };

    Ok(opts)
}

struct UploadOutcome {
    status: u16,
    status_text: String,
    body: String,
    elapsed: Duration,
}

fn do_upload(opts: &UpOptions) -> Result<bool> {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(opts.timeout)))
        .http_status_as_error(false)
        .build()
        .new_agent();

    let file_bytes = match opts.file.as_deref() {
        Some(path) => Some(fs::read(path).with_context(|| t!("dl.failed_read", path))?),
        None => None,
    };

    let mut transport = opts.transport;
    let mut method = opts.method;
    let mut retried = false;

    loop {
        let outcome = send_upload(&agent, opts, transport, method, file_bytes.as_deref())?;
        let rejected = matches!(outcome.status, 405 | 415 | 501);
        if !retried && rejected && !opts.no_fallback && file_bytes.is_some() {
            let (next_transport, next_method) = match transport {
                Transport::Multipart => (Transport::Raw, "PUT"),
                Transport::Raw => (Transport::Multipart, "POST"),
            };
            print_info(&t!("up.fallback_retry", outcome.status, next_method));
            transport = next_transport;
            method = next_method;
            retried = true;
            continue;
        }
        print_upload_result(method, &opts.url, &outcome);
        return Ok(false);
    }
}

fn send_upload(
    agent: &ureq::Agent,
    opts: &UpOptions,
    transport: Transport,
    method: &str,
    file_bytes: Option<&[u8]>,
) -> Result<UploadOutcome> {
    let mut builder = ureq::http::Request::builder().method(method).uri(&opts.url);

    for (key, value) in &opts.headers {
        builder = builder.header(key.as_str(), value.as_str());
    }

    if let Some(token) = &opts.bearer {
        builder = builder.header("Authorization", format!("Bearer {}", token));
    } else if let Some(credentials) = &opts.auth {
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            credentials.as_str(),
        );
        builder = builder.header("Authorization", format!("Basic {}", encoded));
    }

    let has_content_type = opts
        .headers
        .iter()
        .any(|(key, _)| key.eq_ignore_ascii_case("content-type"));

    let mut label = String::from("upload");
    let body: Vec<u8> = if let Some(bytes) = file_bytes {
        let path = opts.file.as_deref().unwrap_or("upload");
        let name = file_basename(path);
        label = name.to_string();
        match transport {
            Transport::Multipart => {
                let boundary = multipart_boundary();
                if !has_content_type {
                    builder = builder.header(
                        "Content-Type",
                        format!("multipart/form-data; boundary={}", boundary),
                    );
                }
                multipart_body(&boundary, name, guess_mime(name), bytes, &opts.forms)
            }
            Transport::Raw => {
                if !has_content_type {
                    builder = builder.header("Content-Type", guess_mime(name));
                }
                bytes.to_vec()
            }
        }
    } else if let Some(data) = &opts.data {
        if !has_content_type {
            builder = builder.header("Content-Type", "text/plain");
        }
        label = String::from("data");
        data.clone().into_bytes()
    } else {
        Vec::new()
    };

    let total = body.len();
    let pb = if total > 0 {
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{wide_bar} {bytes}/{total_bytes}")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("##-"),
        );
        pb.set_message(t!("dl.uploading", label, format_size(total as u64)));
        Some(pb)
    } else {
        None
    };

    let start = Instant::now();
    let request = builder
        .body(body)
        .map_err(|e| anyhow::anyhow!("{}", t!("dl.invalid_request", e)))?;
    let response = agent
        .run(request)
        .with_context(|| t!("dl.upload_failed", &opts.url))?;
    if let Some(pb) = pb {
        pb.finish_and_clear();
    }
    let elapsed = start.elapsed();
    let status = response.status();
    let status_text = status.canonical_reason().unwrap_or("Unknown").to_string();
    let body = response.into_body().read_to_string().unwrap_or_default();

    Ok(UploadOutcome {
        status: status.as_u16(),
        status_text,
        body,
        elapsed,
    })
}

fn print_upload_result(method: &str, url: &str, outcome: &UploadOutcome) {
    let status_line = format!(
        "{} {} {}  {}",
        method, url, outcome.status, outcome.status_text
    );
    if (200..300).contains(&outcome.status) {
        println!("{}", status_line.green().bold());
    } else if outcome.status >= 500 {
        println!("{}", status_line.red().bold());
    } else if outcome.status >= 400 {
        println!("{}", status_line.yellow().bold());
    } else {
        println!("{}", status_line.cyan().bold());
    }

    let ms = outcome.elapsed.as_millis();
    let time_str = if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    };
    println!("{}", time_str.dimmed());

    let body = &outcome.body;
    if body.is_empty() {
        return;
    }
    println!();
    let trimmed = body.trim();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if let Ok(pretty) = serde_json::to_string_pretty(&value) {
                println!("{}", pretty);
                return;
            }
        }
    }
    println!("{}", body);
}

fn multipart_boundary() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("----ntc{}{}", std::process::id(), nanos)
}

fn multipart_body(
    boundary: &str,
    filename: &str,
    mime: &str,
    bytes: &[u8],
    forms: &[(String, String)],
) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() + 320);
    for (name, value) in forms {
        out.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        out.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                sanitize_field(name)
            )
            .as_bytes(),
        );
        out.extend_from_slice(value.as_bytes());
        out.extend_from_slice(b"\r\n");
    }
    out.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    out.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n",
            sanitize_field(filename)
        )
        .as_bytes(),
    );
    out.extend_from_slice(format!("Content-Type: {}\r\n\r\n", mime).as_bytes());
    out.extend_from_slice(bytes);
    out.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());
    out
}

/// Strips the bytes that would break a multipart header line.
fn sanitize_field(value: &str) -> String {
    value
        .chars()
        .filter(|c| *c != '"' && *c != '\r' && *c != '\n')
        .collect()
}

fn file_basename(path: &str) -> &str {
    path.rsplit(|c| c == '/' || c == '\\')
        .find(|part| !part.is_empty())
        .unwrap_or(path)
}

fn guess_mime(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "json" => "application/json",
        "xml" => "application/xml",
        "html" | "htm" => "text/html",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}
fn print_up_help() {
    println!("{}", t!("up.help.title").bold());
    println!();
    println!("{}", t!("up.help.usage").bold());
    println!("{}", t!("up.help.usage_line"));
    println!("{}", t!("up.help.usage_post"));
    println!("{}", t!("up.help.usage_data"));
    println!();
    println!("{}", t!("up.help.options").bold());
    println!("{}", t!("up.help.opt_header"));
    println!("{}", t!("up.help.opt_data"));
    println!("{}", t!("up.help.opt_auth"));
    println!("{}", t!("up.help.opt_bearer"));
    println!("{}", t!("up.help.opt_form"));
    println!("{}", t!("up.help.opt_raw"));
    println!("{}", t!("up.help.opt_put"));
    println!("{}", t!("up.help.opt_timeout"));
    println!("{}", t!("up.help.opt_no_fallback"));
    println!("{}", t!("up.help.opt_post"));
    println!("{}", t!("up.help.opt_help"));
    println!();
    println!("{}", t!("up.help.examples").bold());
    println!("  up myfile.txt https://example.com/upload");
    println!("  up https://127.0.0.1:8080/api/upload myfile.txt -a user:pass");
    println!("  up myfile.bin https://host/put --put --raw");
    println!("  up myfile.txt https://host/upload -F tag=release -F owner=ci");
    println!(
        "  up -d \"hello world\" https://echo.free.beeceptor.com -H \"Content-Type: text/plain\""
    );
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "up",
    aliases: &[],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_up),
};

#[cfg(test)]
mod tests {
    use super::*;

    fn up_opts(args: &str) -> UpOptions {
        parse_up_args(&tokenize(args)).expect("valid up args")
    }

    #[test]
    fn up_accepts_url_before_file() {
        let opts = up_opts("https://example.com/upload file.txt");
        assert_eq!(opts.url, "https://example.com/upload");
        assert_eq!(opts.file.as_deref(), Some("file.txt"));
    }

    #[test]
    fn up_accepts_file_before_url() {
        let opts = up_opts("file.txt https://example.com/upload");
        assert_eq!(opts.url, "https://example.com/upload");
        assert_eq!(opts.file.as_deref(), Some("file.txt"));
    }

    #[test]
    fn up_data_with_url_is_not_a_file() {
        let opts = up_opts("-d \"hello\" https://echo.example.com");
        assert_eq!(opts.url, "https://echo.example.com");
        assert!(opts.file.is_none());
        assert_eq!(opts.data.as_deref(), Some("hello"));
    }

    #[test]
    fn up_file_defaults_to_multipart_post() {
        let opts = up_opts("file.txt https://h/up");
        assert_eq!(opts.method, "POST");
        assert_eq!(opts.transport, Transport::Multipart);
    }

    #[test]
    fn up_put_forces_raw_put() {
        let opts = up_opts("file.bin https://h/put --put --raw");
        assert_eq!(opts.method, "PUT");
        assert_eq!(opts.transport, Transport::Raw);
    }

    #[test]
    fn up_parses_auth_headers_and_forms() {
        let opts =
            up_opts("f.txt https://h/u -a user:pass --bearer tok -H \"X-A: 1\" -F tag=release");
        assert_eq!(opts.auth.as_deref(), Some("user:pass"));
        assert_eq!(opts.bearer.as_deref(), Some("tok"));
        assert_eq!(opts.headers, vec![("X-A".to_string(), "1".to_string())]);
        assert_eq!(opts.forms, vec![("tag".to_string(), "release".to_string())]);
    }

    #[test]
    fn up_rejects_bad_arguments() {
        assert!(parse_up_args(&tokenize("f.txt https://h/u --timeout 0")).is_err());
        assert!(parse_up_args(&tokenize("f.txt https://h/u --timeout abc")).is_err());
        assert!(parse_up_args(&tokenize("f.txt -d x https://h/u")).is_err());
        // no URL at all
        assert!(parse_up_args(&tokenize("f.txt")).is_err());
    }

    #[test]
    fn multipart_body_wraps_file_with_fields() {
        let body = multipart_body(
            "BOUND",
            "a.txt",
            "text/plain",
            b"hi",
            &[("tag".to_string(), "x".to_string())],
        );
        let text = String::from_utf8_lossy(&body);
        assert!(text.starts_with("--BOUND\r\n"));
        assert!(text.contains("name=\"tag\""));
        assert!(text.contains("name=\"file\"; filename=\"a.txt\""));
        assert!(text.contains("Content-Type: text/plain"));
        assert!(text.contains("\r\n\r\nhi\r\n"));
        assert!(text.ends_with("--BOUND--\r\n"));
    }

    #[test]
    fn multipart_body_sanitizes_filenames() {
        let body = multipart_body("B", "bad\"\r\nname.txt", "text/plain", b"", &[]);
        let text = String::from_utf8_lossy(&body);
        assert!(text.contains("filename=\"badname.txt\""));
    }

    #[test]
    fn file_basename_handles_both_separators() {
        assert_eq!(file_basename("a/b/c.txt"), "c.txt");
        assert_eq!(file_basename("a\\b\\c.txt"), "c.txt");
        assert_eq!(file_basename("c.txt"), "c.txt");
    }

    #[test]
    fn test_guess_mime_json() {
        assert_eq!(guess_mime("data.json"), "application/json");
        assert_eq!(guess_mime("data.JSON"), "application/json");
    }

    #[test]
    fn test_guess_mime_text() {
        assert_eq!(guess_mime("readme.txt"), "text/plain");
        assert_eq!(guess_mime("index.html"), "text/html");
        assert_eq!(guess_mime("data.csv"), "text/csv");
    }

    #[test]
    fn test_guess_mime_image() {
        assert_eq!(guess_mime("photo.png"), "image/png");
        assert_eq!(guess_mime("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime("photo.jpeg"), "image/jpeg");
        assert_eq!(guess_mime("photo.gif"), "image/gif");
        assert_eq!(guess_mime("icon.svg"), "image/svg+xml");
    }

    #[test]
    fn test_guess_mime_octet() {
        assert_eq!(guess_mime("file.bin"), "application/octet-stream");
        assert_eq!(guess_mime("data"), "application/octet-stream");
        assert_eq!(guess_mime(""), "application/octet-stream");
    }

    #[test]
    fn test_guess_mime_archive() {
        assert_eq!(guess_mime("archive.zip"), "application/zip");
        assert_eq!(guess_mime("archive.tar"), "application/x-tar");
        assert_eq!(guess_mime("archive.tar.gz"), "application/gzip");
    }
}
