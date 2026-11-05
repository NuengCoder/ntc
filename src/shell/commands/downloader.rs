use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::dns_client;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use std::net::ToSocketAddrs;

#[derive(Serialize, Deserialize)]
struct DownloadRecord {
    url: String,
    file: String,
    size: u64,
    status: String,
    time: String,
    /// Process running the transfer, so `dl --stop` can end it. Absent on
    /// records written before this field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pid: Option<u32>,
}

fn downloads_db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("downloads.json")
}

fn load_records() -> Vec<DownloadRecord> {
    let path = downloads_db_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_records(records: &[DownloadRecord]) {
    if let Ok(json) = serde_json::to_string_pretty(records) {
        if let Some(parent) = downloads_db_path().parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(downloads_db_path(), &json);
    }
}

/// Staging path for `file`: the transfer is written here and renamed into
/// place only once it completes.
fn part_path_for(file: &str) -> PathBuf {
    PathBuf::from(file).with_extension("part")
}

/// Bytes already staged for `file`, or 0 when nothing was downloaded yet.
fn staged_size(file: &str) -> u64 {
    fs::metadata(part_path_for(file))
        .map(|m| m.len())
        .unwrap_or(0)
}

/// Insert or update the record for `url` + `file`. Downloads acknowledge
/// themselves as `downloading` first, then `completed` / `failed` / `stopped`,
/// which is what the `--stop` and `--resume` pickers read.
fn set_record(url: &str, file: &str, status: &str, size: u64, pid: Option<u32>) {
    let mut records = load_records();
    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match records.iter_mut().find(|r| r.url == url && r.file == file) {
        Some(r) => {
            r.status = status.to_string();
            r.size = size;
            r.time = time;
            r.pid = pid;
        }
        None => records.push(DownloadRecord {
            url: url.to_string(),
            file: file.to_string(),
            size,
            status: status.to_string(),
            time,
            pid,
        }),
    }
    save_records(&records);
}

/// Move the staging file onto its final name. Windows refuses to rename over
/// an existing file, so a stale destination is removed first.
fn finalize_part(part: &std::path::Path, dest: &std::path::Path) -> std::io::Result<()> {
    match fs::rename(part, dest) {
        Ok(()) => Ok(()),
        Err(_) => {
            let _ = fs::remove_file(dest);
            fs::rename(part, dest)
        }
    }
}

/// One-line interactive picker over `count` entries. `None` on empty input,
/// `q`, EOF or an out-of-range number.
fn prompt_choice(count: usize, prompt_key: &str) -> Option<usize> {
    print!("{}", t!(prompt_key));
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err() {
        return None;
    }
    let line = line.trim();
    if line.is_empty() || line.eq_ignore_ascii_case("q") {
        return None;
    }
    match line.parse::<usize>() {
        Ok(n) if n >= 1 && n <= count => Some(n - 1),
        _ => {
            print_error(&t!("dl.invalid_choice"));
            None
        }
    }
}

/// End a download running in another ntc process.
#[cfg(unix)]
fn terminate_pid(pid: u32) -> bool {
    // SAFETY: `kill` takes a plain pid and signal number; no pointers.
    unsafe { libc::kill(pid as i32, libc::SIGTERM) == 0 }
}

#[cfg(windows)]
fn terminate_pid(pid: u32) -> bool {
    std::process::Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(not(any(unix, windows)))]
fn terminate_pid(_pid: u32) -> bool {
    false
}

pub fn cmd_dl(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_dl_help();
        return Ok(false);
    }

    let mut i = 0;
    let mut url = String::new();
    let mut output: Option<String> = None;
    let mut history = false;
    let mut resume = false;
    let mut stop = false;
    let mut cancel = false;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                output = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.missing_o")))?
                        .clone(),
                );
            }
            // Accepted and ignored: progress is always shown now.
            "--progress" => {}
            "--history" => history = true,
            "--resume" => resume = true,
            "--stop" => stop = true,
            "--cancel" | "--cancle" => {
                i += 1;
                cancel = true;
                url = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.missing_cancel")))?
                    .clone();
            }
            "--" => break,
            _ => {
                if url.is_empty() {
                    url = tokens[i].clone();
                }
            }
        }
        i += 1;
    }

    if history {
        return cmd_dl_history();
    }
    if stop {
        return cmd_dl_stop();
    }
    if cancel {
        return cmd_dl_cancel(&url);
    }
    if url.is_empty() {
        if resume {
            return cmd_dl_resume();
        }
        anyhow::bail!("{}", t!("dl.url_required"));
    }

    do_download(&url, output.as_deref(), resume)
}

fn do_download(url: &str, output: Option<&str>, resume: bool) -> Result<bool> {
    let filename = output
        .map(|s| s.to_string())
        .unwrap_or_else(|| url.rsplit('/').next().unwrap_or("download").to_string());
    let path = PathBuf::from(&filename);
    let part_path = part_path_for(&filename);

    // The transfer always lands in `<name>.part` and is renamed on success, so
    // an interrupted download stays resumable under its real name.
    let existing_size = if resume && part_path.exists() {
        fs::metadata(&part_path).map(|m| m.len()).unwrap_or(0)
    } else {
        0
    };

    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(300)))
        .http_status_as_error(false)
        .build()
        .new_agent();

    let mut req = agent.get(url);
    if existing_size > 0 {
        req = req.header("Range", &format!("bytes={}-", existing_size));
    }

    let response = req.call().with_context(|| t!("dl.failed_download", url))?;
    let status = response.status();

    if status == 404 {
        print_error(&t!("dl.not_found_404", url));
        return Ok(false);
    }
    if !status.is_success() && status != 206 {
        anyhow::bail!("{}", t!("dl.http_error", status.as_u16(), url));
    }

    // A server that ignored the `Range` header replies 200 with the whole
    // body, so a partial file has to restart instead of being appended to.
    let start_size = if status == 206 { existing_size } else { 0 };
    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0)
        + start_size;

    let mut file = if start_size > 0 {
        fs::OpenOptions::new().append(true).open(&part_path)
    } else {
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&part_path)
    }
    .with_context(|| t!("dl.failed_create", part_path.display()))?;

    // Always shown now; `--progress` is a no-op kept for old notes.
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{wide_bar} {bytes}/{total_bytes} ({eta})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("##-"),
    );
    pb.set_message(t!("dl.downloading", &filename));
    if start_size > 0 {
        pb.set_position(start_size);
    }

    let start = Instant::now();
    let mut downloaded = start_size;
    let mut chunk = vec![0u8; 65536];
    let mut body = response.into_body();
    let mut reader = body.as_reader();

    // Claim the transfer before the first byte: an interrupted download keeps
    // a `downloading` row with this pid, which `dl --stop` acts on.
    set_record(
        url,
        &filename,
        "downloading",
        start_size,
        Some(std::process::id()),
    );

    let transferred = (|| -> Result<u64> {
        loop {
            let n = reader
                .read(&mut chunk)
                .context("Failed to read download stream")?;
            if n == 0 {
                break;
            }
            file.write_all(&chunk[..n])
                .context("Failed to write downloaded data")?;
            downloaded += n as u64;
            pb.set_position(downloaded);
        }
        Ok(downloaded)
    })();
    if let Err(e) = transferred {
        drop(file);
        let staged = staged_size(&filename);
        set_record(url, &filename, "failed", staged, None);
        return Err(e);
    }

    pb.finish_and_clear();
    // Close the handle before renaming: Windows will not move an open file.
    drop(file);
    finalize_part(&part_path, &path).context("Failed to finalize download")?;

    let elapsed = start.elapsed();
    let ms = elapsed.as_millis();
    let speed = if ms > 0 {
        downloaded as f64 / ms as f64 * 1000.0
    } else {
        0.0
    };
    let speed_str = if speed >= 1_000_000.0 {
        format!("{:.1} MB/s", speed / 1_000_000.0)
    } else if speed >= 1000.0 {
        format!("{:.1} KB/s", speed / 1000.0)
    } else {
        format!("{:.0} B/s", speed)
    };

    let time_str = if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    };

    print_success(&t!(
        "dl.completed",
        filename,
        format_size(downloaded),
        time_str,
        speed_str
    ));

    set_record(url, &filename, "completed", downloaded, None);

    Ok(false)
}

/// `dl --history` — the record table (previously `--stop`).
fn cmd_dl_history() -> Result<bool> {
    let records = load_records();
    if records.is_empty() {
        print_info(&t!("dl.no_records"));
        return Ok(false);
    }
    println!("{}", t!("dl.history_title").bold());
    println!();
    for (i, r) in records.iter().enumerate() {
        let status_color = match r.status.as_str() {
            "completed" => "OK".green(),
            _ => r.status.yellow(),
        };
        println!(
            " {}. {}  {}  {}  {}",
            i + 1,
            status_color,
            format_size(r.size),
            r.time,
            r.file.dimmed()
        );
    }
    println!();
    print_info(&t!("dl.cancel_hint"));
    Ok(false)
}

/// `dl --stop` — list the transfers that are still marked `downloading` and
/// end the one the user picks. The staged `.part` file is kept so
/// `dl --resume` can continue it.
fn cmd_dl_stop() -> Result<bool> {
    let records = load_records();
    let active: Vec<usize> = records
        .iter()
        .enumerate()
        .filter(|(_, r)| r.status == "downloading")
        .map(|(i, _)| i)
        .collect();
    if active.is_empty() {
        print_info(&t!("dl.no_active"));
        return Ok(false);
    }
    println!("{}", t!("dl.active_title").bold());
    println!();
    for (n, idx) in active.iter().enumerate() {
        let r = &records[*idx];
        let done = staged_size(&r.file).max(r.size);
        println!(
            " {}. {}  {}  {}",
            n + 1,
            r.file.dimmed(),
            format_size(done),
            r.time
        );
    }
    println!();
    let Some(choice) = prompt_choice(active.len(), "dl.select_stop") else {
        print_info(&t!("dl.cancelled"));
        return Ok(false);
    };
    let rec = &records[active[choice]];
    let ended = rec
        .pid
        .filter(|pid| *pid != std::process::id())
        .map(terminate_pid)
        .unwrap_or(false);
    let file = rec.file.clone();
    let mut records = load_records();
    if let Some(r) = records.get_mut(active[choice]) {
        r.status = "stopped".to_string();
        r.pid = None;
        r.size = staged_size(&r.file).max(r.size);
    }
    save_records(&records);
    if ended {
        print_success(&t!("dl.stopped_ok", file));
    } else {
        print_warning(&t!("dl.stopped_stale", file));
    }
    Ok(false)
}

/// `dl --resume` with no URL — list the unfinished transfers and continue the
/// one the user picks.
fn cmd_dl_resume() -> Result<bool> {
    let records = load_records();
    let pending: Vec<usize> = records
        .iter()
        .enumerate()
        .filter(|(_, r)| r.status != "completed")
        .map(|(i, _)| i)
        .collect();
    if pending.is_empty() {
        print_info(&t!("dl.nothing_incomplete"));
        return Ok(false);
    }
    println!("{}", t!("dl.incomplete_title").bold());
    println!();
    for (n, idx) in pending.iter().enumerate() {
        let r = &records[*idx];
        println!(
            " {}. {}  {}  {}  {}",
            n + 1,
            r.status.yellow(),
            format_size(staged_size(&r.file)),
            r.time,
            r.file.dimmed()
        );
    }
    println!();
    let Some(choice) = prompt_choice(pending.len(), "dl.select_resume") else {
        print_info(&t!("dl.cancelled"));
        return Ok(false);
    };
    let rec = &records[pending[choice]];
    if rec.url.is_empty() {
        print_error(&t!("dl.not_found", &rec.file));
        return Ok(false);
    }
    println!(
        "{}",
        t!(
            "dl.resuming",
            &rec.file,
            format_size(staged_size(&rec.file))
        )
        .dimmed()
    );
    let (url, file) = (rec.url.clone(), rec.file.clone());
    do_download(&url, Some(&file), true)
}

fn cmd_dl_cancel(target: &str) -> Result<bool> {
    let mut records = load_records();
    let idx: Option<usize> = target.parse().ok().map(|n: usize| n.saturating_sub(1));

    let removed = if let Some(i) = idx {
        if i < records.len() {
            Some(records.remove(i))
        } else {
            print_error(&t!("dl.invalid_index"));
            return Ok(false);
        }
    } else {
        let pos = records
            .iter()
            .position(|r| r.url == target || r.file == target);
        if let Some(i) = pos {
            Some(records.remove(i))
        } else {
            print_error(&t!("dl.not_found", target));
            return Ok(false);
        }
    };

    if let Some(rec) = removed {
        // Remove partial file if it exists
        let path = PathBuf::from(&rec.file);
        let part = path.with_extension("part");
        if part.exists() {
            let _ = fs::remove_file(&part);
        }
        if path.exists() {
            print_warning(&t!("dl.file_still_exists", rec.file));
        }
        save_records(&records);
        print_success(&t!("dl.removed_record", rec.file));
    }

    Ok(false)
}

// ── Upload ────────────────────────────────────────────────────────────────

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

// ── Shared helpers ────────────────────────────────────────────────────────

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    for ch in input.chars() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            ' ' | '\t' if !in_single && !in_double => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len().saturating_sub(1) {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{}{}", bytes, UNITS[unit])
    } else {
        format!("{:.1}{}", size, UNITS[unit])
    }
}

// ── Help ──────────────────────────────────────────────────────────────────

fn print_dl_help() {
    println!("{}", t!("dl.help.title").bold());
    println!();
    println!("{}", t!("dl.help.usage").bold());
    println!("{}", t!("dl.help.usage_line"));
    println!();
    println!("{}", t!("dl.help.options").bold());
    println!("{}", t!("dl.help.opt_o"));
    println!("{}", t!("dl.help.opt_resume"));
    println!("{}", t!("dl.help.opt_stop"));
    println!("{}", t!("dl.help.opt_history"));
    println!("{}", t!("dl.help.opt_cancel"));
    println!("{}", t!("dl.help.opt_help"));
    println!();
    println!("{}", t!("dl.help.examples").bold());
    println!("  dl https://example.com/file.zip");
    println!("  dl https://example.com/file.zip -o myfile.zip");
    println!("  dl https://example.com/file.zip --resume");
    println!("  dl --resume");
    println!("  dl --stop");
    println!("  dl --history");
    println!("  dl --cancel 2");
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

// ── DNS ───────────────────────────────────────────────────────────────────

pub fn cmd_dns(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_dns_help();
        return Ok(false);
    }

    let mut host = String::new();
    let mut record = String::from("A");
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-t" | "--type" => {
                i += 1;
                record = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.dns_missing_type")))?
                    .clone();
            }
            "--" => break,
            _ => {
                if host.is_empty() {
                    host = tokens[i].clone();
                }
            }
        }
        i += 1;
    }

    if host.is_empty() {
        anyhow::bail!("{}", t!("dl.dns_host_required"));
    }

    match record.to_uppercase().as_str() {
        "A" => {
            resolve_a(&host, "A")?;
        }
        "AAAA" => {
            resolve_a(&host, "AAAA")?;
        }
        "ALL" => {
            resolve_a(&host, "A")?;
            resolve_a(&host, "AAAA")?;
            for rtype in [
                dns_client::RRType::Mx,
                dns_client::RRType::Ns,
                dns_client::RRType::Txt,
                dns_client::RRType::Soa,
            ] {
                print_dns_records(&host, rtype);
            }
        }
        other => match dns_client::RRType::from_label(other) {
            Some(rtype) => print_dns_records(&host, rtype),
            None => print_error(&t!(
                "dl.dns_unsupported_type",
                record,
                dns_client::SUPPORTED_TYPES
            )),
        },
    }

    Ok(false)
}

/// Queries one typed record and prints it; `PTR` also accepts an IP literal.
fn print_dns_records(host: &str, rtype: dns_client::RRType) {
    let label = rtype.label();
    let query = if rtype == dns_client::RRType::Ptr {
        dns_client::reverse_name(host).unwrap_or_else(|| host.to_string())
    } else {
        host.to_string()
    };

    println!("{} {} records:", host.bold(), label.cyan());

    match dns_client::resolve(&query, rtype) {
        Ok(records) => {
            let matching: Vec<dns_client::Record> = records
                .into_iter()
                .filter(|record| record.rtype == rtype)
                .collect();
            if matching.is_empty() {
                println!("{}", t!("dl.dns_no_records", "─".dimmed(), label));
            } else {
                for record in matching {
                    println!("  {}  {}", "OK".green().bold(), record.render());
                }
            }
        }
        Err(dns_client::DnsError::Network(message)) => {
            // No answer at all: only then fall back to dig/nslookup.
            print_warning(&t!("dl.dns_io", message));
            print_warning(&t!("dl.dns_system_fallback"));
            if let Err(e) = system_dns_lookup(host, label) {
                print_error(&format!("{}", e));
            }
        }
        Err(error) => println!("  {}  {}", "FAIL".red().bold(), dns_error_text(&error)),
    }
}

fn dns_error_text(error: &dns_client::DnsError) -> String {
    match error {
        dns_client::DnsError::Rcode(code) => {
            t!("dl.dns_rcode", code, dns_client::rcode_name(*code))
        }
        dns_client::DnsError::Timeout => t!("dl.dns_timeout"),
        dns_client::DnsError::Malformed => t!("dl.dns_malformed"),
        dns_client::DnsError::Network(message) => t!("dl.dns_io", message),
    }
}

fn resolve_a(host: &str, rtype: &str) -> Result<bool> {
    let hint = if rtype.eq_ignore_ascii_case("AAAA") {
        " (IPv6)"
    } else {
        " (IPv4)"
    };
    println!(
        "{}{}",
        format_args!("{} {} records:", host.bold(), rtype.cyan()),
        hint.dimmed()
    );

    let addr = format!("{}:1", host);
    let addrs: Vec<std::net::SocketAddr> = match addr.to_socket_addrs() {
        Ok(a) => a.collect(),
        Err(e) => {
            println!("  {}  {}", "FAIL".red().bold(), e);
            return Ok(false);
        }
    };

    let found: Vec<&std::net::SocketAddr> = if rtype.eq_ignore_ascii_case("AAAA") {
        addrs.iter().filter(|a| a.is_ipv6()).collect()
    } else {
        addrs.iter().filter(|a| a.is_ipv4()).collect()
    };

    if found.is_empty() {
        println!("{}", t!("dl.dns_no_records", "─".dimmed(), rtype));
    } else {
        for addr in &found {
            println!("  {}  {}", "OK".green().bold(), addr.ip());
        }
    }

    Ok(false)
}

fn system_dns_lookup(host: &str, rtype: &str) -> Result<bool> {
    println!(
        "{}",
        t!("dl.dns_looking_up", host.bold(), rtype.cyan()).dimmed()
    );

    #[cfg(windows)]
    let output = std::process::Command::new("nslookup")
        .arg(format!("-type={}", rtype.to_lowercase()))
        .arg(host)
        .output();

    #[cfg(not(windows))]
    let output = std::process::Command::new("dig")
        .arg(host)
        .arg(&rtype.to_uppercase())
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if out.status.success() {
                println!("{}", stdout);
            } else {
                print!("{}", stdout);
                eprintln!("{}", stderr);
                anyhow::bail!(
                    "{}",
                    t!("dl.dns_lookup_failed", format!("{:?}", out.status.code()))
                );
            }
        }
        Err(e) => {
            anyhow::bail!("{}", t!("dl.dns_tool_failed", e, rtype));
        }
    }

    Ok(false)
}

fn print_dns_help() {
    println!("{}", t!("dns.help.title").bold());
    println!();
    println!("{}", t!("dns.help.usage").bold());
    println!("{}", t!("dns.help.usage_line"));
    println!();
    println!("{}", t!("dns.help.options").bold());
    println!("{}", t!("dns.help.opt_type"));
    println!("{}", t!("dns.help.opt_type_sub"));
    println!("{}", t!("dns.help.opt_help"));
    println!();
    println!("{}", t!("dns.help.examples").bold());
    println!("  dns example.com");
    println!("  dns example.com -t AAAA");
    println!("  dns example.com -t MX");
    println!("  dns example.com -t TXT");
    println!("  dns example.com -t SOA");
    println!("  dns 8.8.8.8 -t PTR");
    println!("  dns example.com -t ALL");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_splits_a_bare_host() {
        // `tokienize` only splits; each command parses its own flags.
        let t = tokenize("example.com");
        assert_eq!(t, vec!["example.com"]);
    }

    #[test]
    fn tokenize_keeps_host_and_flags_in_order() {
        let t = tokenize("example.com -c 5 -p 443 -t 1");
        assert_eq!(t, vec!["example.com", "-c", "5", "-p", "443", "-t", "1"]);
    }

    #[test]
    fn test_dns_resolve_localhost() {
        let addrs: Vec<std::net::SocketAddr> = "127.0.0.1:1".to_socket_addrs().unwrap().collect();
        assert!(!addrs.is_empty());
        assert!(addrs[0].ip().is_loopback());
    }

    #[test]
    fn test_dns_resolve_hostname() {
        let addrs: Vec<std::net::SocketAddr> = "localhost:1".to_socket_addrs().unwrap().collect();
        assert!(!addrs.is_empty());
    }

    #[test]
    fn test_resolve_a_ipv4_filter() {
        let host = "127.0.0.1";
        let addr = format!("{}:1", host);
        let addrs: Vec<std::net::SocketAddr> = addr.to_socket_addrs().unwrap().collect();
        let ipv4: Vec<&std::net::SocketAddr> = addrs.iter().filter(|a| a.is_ipv4()).collect();
        assert!(!ipv4.is_empty());
    }

    #[test]
    fn test_tokenize_simple() {
        let t = tokenize("https://example.com/file.zip -o out.zip");
        assert_eq!(t, vec!["https://example.com/file.zip", "-o", "out.zip"]);
    }

    #[test]
    fn test_tokenize_quoted() {
        let t = tokenize("-H \"Content-Type: application/json\" -d 'body data'");
        assert_eq!(
            t,
            vec!["-H", "Content-Type: application/json", "-d", "body data"]
        );
    }

    #[test]
    fn test_tokenize_empty() {
        assert!(tokenize("").is_empty());
    }

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0B");
        assert_eq!(format_size(1), "1B");
        assert_eq!(format_size(1023), "1023B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.0KB");
        assert_eq!(format_size(1536), "1.5KB");
        assert_eq!(format_size(1024 * 1023), "1023.0KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1024 * 1024), "1.0MB");
        assert_eq!(format_size(1024 * 1024 * 10), "10.0MB");
        assert_eq!(format_size(1024 * 1024 + 512 * 1024), "1.5MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0GB");
        assert_eq!(format_size(5 * 1024 * 1024 * 1024), "5.0GB");
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

    #[test]
    fn test_download_record_serde() {
        let rec = DownloadRecord {
            url: "https://example.com/file.zip".into(),
            file: "file.zip".into(),
            size: 1024,
            status: "completed".into(),
            time: "2024-01-01 12:00:00".into(),
            pid: None,
        };
        let json = serde_json::to_string(&rec).unwrap();
        let parsed: DownloadRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.url, rec.url);
        assert_eq!(parsed.file, rec.file);
        assert_eq!(parsed.size, rec.size);
        assert_eq!(parsed.status, rec.status);
    }

    #[test]
    fn test_load_records_never_panics() {
        // Should always return a Vec (possibly empty), never panic
        let records = load_records();
        assert!(records.len() < 1000);
    }
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
}
