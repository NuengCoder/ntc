//! `dl` — download with resume, history, stop, and cancel.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::net::{format_size, tokenize};

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

/// One-line interactive picker over the given entries. `None` on empty input,
/// `q`, EOF or an out-of-range number. Typing a number ghosts that entry's
/// file name after the caret.
fn prompt_choice(labels: &[String], prompt_key: &str) -> Option<usize> {
    let line = crate::utils::interactive::read_number_with_hint(
        &t!(prompt_key),
        |text| text.green().to_string(),
        labels,
    )
    .ok()
    .flatten()?;
    let line = line.trim();
    if line.is_empty() || line.eq_ignore_ascii_case("q") {
        return None;
    }
    match line.parse::<usize>() {
        Ok(n) if n >= 1 && n <= labels.len() => Some(n - 1),
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
    let labels: Vec<String> = active.iter().map(|idx| records[*idx].file.clone()).collect();
    let Some(choice) = prompt_choice(&labels, "dl.select_stop") else {
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
    let labels: Vec<String> = pending.iter().map(|idx| records[*idx].file.clone()).collect();
    let Some(choice) = prompt_choice(&labels, "dl.select_resume") else {
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "dl",
    aliases: &[],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_dl),
};

#[cfg(test)]
mod tests {
    use super::*;

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
}
