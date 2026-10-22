//! `nms` command — NTC Music: a TUI sound player plus a media scanner.
//!
//! - `nms play <file>` opens a fullscreen player (real waveform + level meter,
//!   seek, volume, pause). Video files play their audio track only.
//! - `nms scan [-d|--depth <n>] [-s|--size] [-l|--long <dur>] [--care]` lists
//!   audio + video files under the current directory, in the same style as
//!   `scan ext`; `-l`/`--long` keeps only media longer than a duration.
//! - `nms` / `nms help` prints the usage guide.
//!
//! On a machine without a usable sound driver the player reports a clear error
//! and returns to the shell instead of crashing.

use std::path::{Path, PathBuf};

use crate::explorer::human_readable_size;
use crate::navigator::Navigator;
use crate::nms::player::PlayError;
use crate::nms::scan::ScanOpts;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::Result;
use colored::*;

/// Tokenize on whitespace, honoring double quotes (`nms play "my song.mp3"`).
fn tokenize(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Resolve `name` against `cwd` unless it is already absolute.
fn resolve(cwd: &Path, name: &str) -> PathBuf {
    let p = Path::new(name);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}

pub fn cmd_nms(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_usage();
        return Ok(false);
    }
    match tokens[0].to_lowercase().as_str() {
        "play" | "p" => cmd_nms_play(&tokens[1..], nav),
        "scan" | "list" | "ls" => cmd_nms_scan(&tokens[1..], nav),
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(false)
        }
        other => {
            print_error(&t!("cmd.nms.unknown_sub", other));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

fn print_usage() {
    println!("{}", t!("cmd.nms.usage_title").cyan().bold());
    println!("  {}", t!("cmd.nms.help_play"));
    println!("  {}", t!("cmd.nms.help_scan"));
    println!("  {}", t!("cmd.nms.help_help"));
    println!();
    println!("{}", t!("cmd.nms.options_title").green());
    println!("{}", t!("cmd.nms.help_depth"));
    println!("{}", t!("cmd.nms.help_size"));
    println!("{}", t!("cmd.nms.help_long"));
    println!("{}", t!("cmd.nms.help_care"));
    println!();
    println!("{}", t!("cmd.nms.keys_title").green());
    println!("{}", t!("cmd.nms.help_keys"));
    println!();
    println!("{}", t!("cmd.nms.examples_title").green());
    println!("    {}", t!("cmd.nms.example_play"));
    println!("    {}", t!("cmd.nms.example_video"));
    println!("    {}", t!("cmd.nms.example_scan"));
    println!("    {}", t!("cmd.nms.example_long"));
}

/// `nms play <file>`
fn cmd_nms_play(tokens: &[String], nav: &Navigator) -> Result<bool> {
    let Some(first) = tokens.first() else {
        print_error(&t!("cmd.nms.play_missing_file"));
        return Ok(false);
    };
    if first.starts_with('-') {
        print_error(&t!("cmd.nms.unknown_option", first));
        return Ok(false);
    }
    // A quoted path arrives as one token; unquoted spaced paths are rejoined.
    let name = tokens.join(" ");
    let path = resolve(nav.current_path(), &name);

    if !path.exists() {
        print_error(&t!("cmd.nms.file_not_found", name));
        return Ok(false);
    }
    if !path.is_file() {
        print_error(&t!("cmd.nms.not_media", name));
        return Ok(false);
    }
    if !crate::nms::is_media(&path) {
        print_error(&t!("cmd.nms.not_media", name));
        return Ok(false);
    }

    match crate::nms::play(&path) {
        Ok(()) => Ok(false),
        Err(PlayError::NoDevice) => {
            print_error(&t!("cmd.nms.no_sound_device"));
            Ok(false)
        }
        Err(PlayError::Decode(e)) => {
            print_error(&t!("cmd.nms.play_failed", name, e));
            Ok(false)
        }
    }
}

/// `nms scan [-d|--depth <n>] [-s|--size] [-l|--long <dur>] [--care]`
fn cmd_nms_scan(tokens: &[String], nav: &Navigator) -> Result<bool> {
    let mut max_depth: Option<usize> = None;
    let mut show_size = false;
    let mut care = false;
    let mut min_duration = None;

    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens[i].as_str();

        // Support the `--long=<dur>` spelling in addition to `--long <dur>`.
        if let Some(val) = tok.strip_prefix("--long=") {
            match crate::nms::parse_duration(val) {
                Some(d) => min_duration = Some(d),
                None => {
                    print_error(&t!("cmd.nms.invalid_long", val));
                    return Ok(false);
                }
            }
            i += 1;
            continue;
        }

        match tok.to_lowercase().as_str() {
            "-d" | "--depth" => {
                let Some(val) = tokens.get(i + 1) else {
                    print_error(&t!("cmd.nms.depth_usage"));
                    return Ok(false);
                };
                match val.parse::<usize>() {
                    Ok(n) => max_depth = Some(n),
                    Err(_) => {
                        print_error(&t!("cmd.nms.invalid_depth", val));
                        return Ok(false);
                    }
                }
                i += 2;
            }
            "-s" | "--size" => {
                show_size = true;
                i += 1;
            }
            "-l" | "--long" => {
                let Some(val) = tokens.get(i + 1) else {
                    print_error(&t!("cmd.nms.long_usage"));
                    return Ok(false);
                };
                match crate::nms::parse_duration(val) {
                    Some(d) => min_duration = Some(d),
                    None => {
                        print_error(&t!("cmd.nms.invalid_long", val));
                        return Ok(false);
                    }
                }
                i += 2;
            }
            "--care" => {
                care = true;
                i += 1;
            }
            other => {
                print_error(&t!("cmd.nms.unknown_option", other));
                return Ok(false);
            }
        }
    }

    let opts = ScanOpts {
        max_depth,
        care,
        min_duration,
    };
    let entries = crate::nms::scan::scan_media(nav.current_path(), &opts);

    if entries.is_empty() {
        if let Some(thr) = min_duration {
            print_info(&t!("cmd.nms.long_none", crate::nms::tui::format_time(thr)));
        } else {
            print_info(&t!("cmd.nms.no_files"));
        }
        return Ok(false);
    }

    println!("{}", t!("cmd.nms.list_title").cyan().bold());
    let show_dur = min_duration.is_some();
    for e in &entries {
        let dur = e.duration.map(crate::nms::tui::format_time);
        let detail = match (show_dur, show_size) {
            (true, true) => Some(format!(
                "{}, {}",
                dur.unwrap_or_else(|| "--:--".into()),
                human_readable_size(e.size)
            )),
            (true, false) => Some(dur.unwrap_or_else(|| "--:--".into())),
            (false, true) => Some(human_readable_size(e.size)),
            (false, false) => None,
        };
        match detail {
            Some(d) => println!("  {} ({})", e.rel, d),
            None => println!("  {}", e.rel),
        }
    }
    println!("{}", "─".repeat(40));
    if show_size {
        let total: u64 = entries.iter().map(|e| e.size).sum();
        print_success(&t!(
            "cmd.nms.total_files_size",
            entries.len(),
            human_readable_size(total)
        ));
    } else {
        print_success(&t!("cmd.nms.total_files", entries.len()));
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_respects_quotes() {
        assert_eq!(tokenize("play a.mp3"), vec!["play", "a.mp3"]);
        assert_eq!(
            tokenize("play \"my song.mp3\""),
            vec!["play", "my song.mp3"]
        );
    }

    #[test]
    fn resolve_joins_relative_against_cwd() {
        let cwd = Path::new("/tmp/ntc");
        assert_eq!(resolve(cwd, "a.mp3"), PathBuf::from("/tmp/ntc/a.mp3"));
        assert_eq!(resolve(cwd, "/abs/a.mp3"), PathBuf::from("/abs/a.mp3"));
    }
}
