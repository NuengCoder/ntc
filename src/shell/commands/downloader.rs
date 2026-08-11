use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

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

pub fn cmd_dl(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_dl_help();
        return Ok(false);
    }

    let mut i = 0;
    let mut url = String::new();
    let mut output: Option<String> = None;
    let mut progress = false;
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
            "--progress" => progress = true,
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

    if stop {
        return cmd_dl_stop();
    }
    if cancel {
        return cmd_dl_cancel(&url);
    }
    if url.is_empty() {
        anyhow::bail!("{}", t!("dl.url_required"));
    }

    do_download(&url, output.as_deref(), progress, resume)
}

fn do_download(url: &str, output: Option<&str>, show_progress: bool, resume: bool) -> Result<bool> {
    let filename = output
        .map(|s| s.to_string())
        .unwrap_or_else(|| url.rsplit('/').next().unwrap_or("download").to_string());
    let path = PathBuf::from(&filename);
    let part_path = path.with_extension("part");

    let existing_size = if resume {
        if part_path.exists() {
            fs::metadata(&part_path).map(|m| m.len()).unwrap_or(0)
        } else if path.exists() {
            fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
        } else {
            0
        }
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

    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0)
        + existing_size;

    let download_path = if resume { &part_path } else { &path };

    let mut file = fs::File::create(download_path)
        .with_context(|| t!("dl.failed_create", download_path.display()))?;

    if existing_size > 0 && status == 206 {
        // Append mode for resume
        file = fs::OpenOptions::new()
            .append(true)
            .open(download_path)
            .with_context(|| t!("dl.failed_append", download_path.display()))?;
    }

    let pb = if show_progress {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{wide_bar} {bytes}/{total_bytes} ({eta})")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("##-"),
        );
        bar.set_message(t!("dl.downloading", &filename));
        if existing_size > 0 {
            bar.set_position(existing_size);
        }
        Some(bar)
    } else {
        None
    };

    let start = Instant::now();
    let mut downloaded = existing_size;
    let mut chunk = vec![0u8; 65536];
    let mut body = response.into_body();
    let mut reader = body.as_reader();

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
        if let Some(ref bar) = pb {
            bar.set_position(downloaded);
        }
    }

    if let Some(bar) = pb {
        bar.finish_and_clear();
    }

    if resume && status == 206 {
        // Rename .part to final name on successful resume
        fs::rename(&part_path, &path).context("Failed to finalize resumed download")?;
    }

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

    // Save record
    let mut records = load_records();
    records.push(DownloadRecord {
        url: url.to_string(),
        file: filename,
        size: downloaded,
        status: "completed".to_string(),
        time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    });
    save_records(&records);

    Ok(false)
}

fn cmd_dl_stop() -> Result<bool> {
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

pub fn cmd_up(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_up_help();
        return Ok(false);
    }

    let mut i = 0;
    let mut url = String::new();
    let mut file_path: Option<String> = None;
    let mut data: Option<String> = None;
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut method = "PUT";

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-H" | "--header" => {
                i += 1;
                let h = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.missing_header")))?;
                if let Some((k, v)) = h.split_once(':') {
                    headers.push((k.trim().to_string(), v.trim().to_string()));
                } else {
                    anyhow::bail!("{}", t!("dl.invalid_header", h));
                }
            }
            "-d" | "--data" => {
                i += 1;
                data = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.missing_data")))?
                        .clone(),
                );
            }
            "--post" => method = "POST",
            "--" => break,
            _ => {
                if file_path.is_none() {
                    file_path = Some(tokens[i].clone());
                } else if url.is_empty() {
                    url = tokens[i].clone();
                }
            }
        }
        i += 1;
    }

    if url.is_empty() {
        anyhow::bail!("{}", t!("dl.up_url_required"));
    }

    do_upload(
        &url,
        file_path.as_deref(),
        data.as_deref(),
        &headers,
        method,
    )
}

fn do_upload(
    url: &str,
    file_path: Option<&str>,
    data: Option<&str>,
    headers: &[(String, String)],
    method: &str,
) -> Result<bool> {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(300)))
        .http_status_as_error(false)
        .build()
        .new_agent();

    let mut req_builder = ureq::http::Request::builder().method(method).uri(url);

    for (k, v) in headers {
        req_builder = req_builder.header(k.as_str(), v.as_str());
    }

    let start = Instant::now();

    let response = if let Some(path) = file_path {
        let file_data = fs::read(path).with_context(|| t!("dl.failed_read", path))?;
        let total = file_data.len();
        if !headers
            .iter()
            .any(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        {
            req_builder = req_builder.header("Content-Type", guess_mime(path));
        }
        let req = req_builder
            .body(file_data)
            .map_err(|e| anyhow::anyhow!("{}", t!("dl.invalid_request", e)))?;
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{wide_bar} {bytes}/{total_bytes}")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("##-"),
        );
        pb.set_message(t!("dl.uploading", path, format_size(total as u64)));
        let resp = agent
            .run(req)
            .with_context(|| t!("dl.upload_failed", url))?;
        pb.finish_and_clear();
        resp
    } else if let Some(body) = data {
        if !headers
            .iter()
            .any(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        {
            req_builder = req_builder.header("Content-Type", "text/plain");
        }
        let req = req_builder
            .body(body.to_string())
            .map_err(|e| anyhow::anyhow!("{}", t!("dl.invalid_request", e)))?;
        agent
            .run(req)
            .with_context(|| t!("dl.request_failed", url))?
    } else {
        let req = req_builder
            .body(())
            .map_err(|e| anyhow::anyhow!("{}", t!("dl.invalid_request", e)))?;
        agent
            .run(req)
            .with_context(|| t!("dl.request_failed", url))?
    };

    let elapsed = start.elapsed();
    let status = response.status();
    let status_text = status.canonical_reason().unwrap_or("Unknown");

    let status_line = format!("{} {} {}  {}", method, url, status, status_text);
    if status.is_success() {
        println!("{}", status_line.green().bold());
    } else if status.is_server_error() {
        println!("{}", status_line.red().bold());
    } else if status.is_client_error() {
        println!("{}", status_line.yellow().bold());
    } else {
        println!("{}", status_line.cyan().bold());
    }

    let ms = elapsed.as_millis();
    let time_str = if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    };
    println!("{}", time_str.dimmed());

    let body = response.into_body().read_to_string().unwrap_or_default();
    if !body.is_empty() {
        println!();
        if body.trim().starts_with('{') || body.trim().starts_with('[') {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Ok(pretty) = serde_json::to_string_pretty(&v) {
                    println!("{}", pretty);
                } else {
                    println!("{}", body);
                }
            } else {
                println!("{}", body);
            }
        } else {
            println!("{}", body);
        }
    }

    Ok(false)
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
    println!("{}", t!("dl.help.opt_progress"));
    println!("{}", t!("dl.help.opt_resume"));
    println!("{}", t!("dl.help.opt_stop"));
    println!("{}", t!("dl.help.opt_cancel"));
    println!("{}", t!("dl.help.opt_help"));
    println!();
    println!("{}", t!("dl.help.examples").bold());
    println!("  dl https://example.com/file.zip");
    println!("  dl https://example.com/file.zip --progress");
    println!("  dl https://example.com/file.zip -o myfile.zip");
    println!("  dl https://example.com/file.zip --resume");
    println!("  dl --stop");
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
    println!("{}", t!("up.help.opt_post"));
    println!("{}", t!("up.help.opt_help"));
    println!();
    println!("{}", t!("up.help.examples").bold());
    println!("  up myfile.txt https://example.com/upload");
    println!("  up data.json https://api.example.com/data --post");
    println!(
        "  up -d \"hello world\" https://echo.free.beeceptor.com -H \"Content-Type: text/plain\""
    );
}

// ── Ping ──────────────────────────────────────────────────────────────────

pub fn cmd_ping(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() || tokens[0] == "help" || tokens[0] == "--help" || tokens[0] == "-h" {
        print_ping_help();
        return Ok(false);
    }

    let mut host = String::new();
    let mut count: u32 = 4;
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "-c" | "--count" => {
                i += 1;
                count = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("dl.ping_missing_count")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("dl.ping_invalid_count")))?;
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
        anyhow::bail!("{}", t!("dl.ping_host_required"));
    }

    #[cfg(windows)]
    let status = std::process::Command::new("ping")
        .args(["-n", &count.to_string(), &host])
        .spawn()
        .and_then(|mut c| c.wait());

    #[cfg(not(windows))]
    let status = std::process::Command::new("ping")
        .args(["-c", &count.to_string(), &host])
        .spawn()
        .and_then(|mut c| c.wait());

    match status {
        Ok(exit) => {
            if let Some(code) = exit.code() {
                if code != 0 && code != 1 {
                    print_warning(&t!("dl.ping_exit_code", code));
                }
            }
        }
        Err(e) => {
            print_error(&t!("dl.ping_failed", e));
        }
    }

    Ok(false)
}

fn print_ping_help() {
    println!("{}", t!("ping.help.title").bold());
    println!();
    println!("{}", t!("ping.help.usage").bold());
    println!("{}", t!("ping.help.usage_line"));
    println!();
    println!("{}", t!("ping.help.options").bold());
    println!("{}", t!("ping.help.opt_count"));
    println!("{}", t!("ping.help.opt_help"));
    println!();
    println!("{}", t!("ping.help.examples").bold());
    println!("  ping example.com");
    println!("  ping 127.0.0.1 -c 10");
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
        "A" | "AAAA" => resolve_a(&host, &record),
        "ALL" => {
            resolve_a(&host, "A")?;
            resolve_a(&host, "AAAA")?;
            Ok(false)
        }
        _ => {
            // Fallback to system dig/nslookup for non-A records
            system_dns_lookup(&host, &record)
        }
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
    println!("  dns example.com -t ALL");
    println!("  dns example.com -t MX");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_parse_defaults() {
        // tokenize doesn't parse, it just splits — parsing is done in cmd_ping
        let t = tokenize("example.com");
        assert_eq!(t, vec!["example.com"]);
    }

    #[test]
    fn test_ping_parse_with_flags() {
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
}
