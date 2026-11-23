//! `scan waep` web-path enumeration and rendering.

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

/// One enumeration pass: print hits as they land, collect the rows for
/// `-o`, and return the summary.
#[allow(clippy::too_many_arguments)]
pub(super) fn waep_run(
    target: &crate::shell::commands::builtin::scan::netscan::webpath::Target,
    words: &[String],
    opts: &crate::shell::commands::builtin::scan::netscan::webpath::Options,
    accept: &[u16],
    show_all: bool,
    quiet: bool,
    // Recursion prints the full path (`/api/files`) so a hit on a deeper
    // level is not confused with a hit at the scan base (`/files`).
    show_full: bool,
    seen_urls: &mut std::collections::HashSet<String>,
    rows: &mut Vec<String>,
    hit_count: &mut usize,
) -> (
    crate::shell::commands::builtin::scan::netscan::webpath::Summary,
    Vec<String>,
) {
    use crate::shell::commands::builtin::scan::netscan::webpath::Kind;
    let accept_fn = move |status: u16| show_all || accept.contains(&status);
    let mut found: Vec<String> = Vec::new();
    let summary = crate::shell::commands::builtin::scan::netscan::webpath::enumerate(
        target,
        words,
        opts,
        &accept_fn,
        &mut |hit| {
            // Recursive levels can rediscover a URL already reported; print it
            // once.
            if !seen_urls.insert(hit.url.clone()) {
                return;
            }
            *hit_count += 1;
            found.push(hit.path.clone());
            let kind = hit.probe.kind();
            let status = hit.probe.status.to_string();
            let status = match kind {
                Kind::Ok => status.green(),
                Kind::Redirect => status.cyan(),
                Kind::Forbidden => status.yellow(),
                Kind::Method => status.magenta(),
                Kind::Error => status.red(),
                Kind::Missing => status.dimmed(),
            };
            let ctype = hit
                .probe
                .content_type
                .split(';')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            let extra = if hit.probe.location.is_empty() {
                ctype
            } else {
                format!("-> {}", hit.probe.location)
            };
            let shown = if show_full {
                crate::shell::commands::builtin::scan::netscan::webpath::join_path(
                    &target.base_path,
                    &hit.path,
                )
            } else {
                hit.path.clone()
            };
            let line = format!(
                "  {:<28} {:>4} {:>5}ms  {}",
                shown,
                status,
                hit.probe.ms,
                extra.dimmed()
            );
            if !quiet {
                println!("{}", line);
            }
            rows.push(format!(
                "{} {} {} {}",
                shown, hit.probe.status, hit.probe.ms, extra
            ));
        },
    );
    (summary, found)
}

/// Full help for `scan waep`: usage, examples, every flag and the option
/// semantics (base-relative results, accepted statuses, recursion).
pub(super) fn print_waep_help() {
    println!("{}", t!("cmd.scan.waep_usage").cyan().bold());
    println!();
    println!("{}", t!("cmd.scan.examples_title").green());
    for ex in [
        "scan waep example.com",
        "scan waep 192.168.1.5:8080 -x php,html --all",
        "scan waep http://127.0.0.1:55555/api",
        "scan waep example.com/api -r -d 2",
        "scan waep example.com -w paths.txt -s 200,301,403 -o hits.txt",
    ] {
        println!("    {}", ex);
    }
    println!();
    println!("{}", t!("cmd.scan.waep_flags_title").green());
    print_indented(t!("cmd.scan.waep_flags"));
    println!();
    println!("{}", t!("cmd.scan.waep_options_title").green());
    print_indented(t!("cmd.scan.waep_options"));
}

/// Print a multi-line block with a uniform two-space indent regardless of how
/// each translation indents its own continuation lines.
pub(super) fn print_indented(text: String) {
    for line in text.lines() {
        println!("  {}", line.trim_start());
    }
}

/// `scan waep <host|host:port|url[/base]> [flags]` — web path enumeration.
pub(super) fn cmd_scan_waep(tokens: &[String]) -> Result<bool> {
    if tokens.is_empty() || matches!(tokens[0].to_lowercase().as_str(), "help" | "-h" | "--help") {
        print_waep_help();
        return Ok(false);
    }
    let spec = tokens[0].clone();
    let target = match crate::shell::commands::builtin::scan::netscan::webpath::parse_target(&spec)
    {
        Ok(t) => t,
        Err(e) => {
            print_error(&t!("cmd.scan.waep_bad_target", e));
            return Ok(false);
        }
    };

    let mut opts = crate::shell::commands::builtin::scan::netscan::webpath::Options::default();
    let mut words: Vec<String> =
        crate::shell::commands::builtin::scan::netscan::webpath::default_words()
            .iter()
            .map(|s| s.to_string())
            .collect();
    let mut exts: Vec<String> = Vec::new();
    let mut accept: Vec<u16> =
        crate::shell::commands::builtin::scan::netscan::webpath::DEFAULT_STATUS.to_vec();
    let mut show_all = false;
    let mut recursive = false;
    let mut depth: usize = 1;
    let mut output: Option<String> = None;
    let mut quiet = false;

    let mut i = 1;
    while i < tokens.len() {
        let flag = tokens[i].as_str();
        let value = |i: &mut usize, flag: &str| -> Result<String, String> {
            *i += 1;
            tokens
                .get(*i)
                .cloned()
                .ok_or_else(|| t!("netdiag.missing_value", flag))
        };
        match flag {
            "-w" | "--wordlist" => match value(&mut i, flag) {
                Ok(path) => {
                    let expanded = crate::utils::paths::expand_path_str(&path);
                    match std::fs::read_to_string(&expanded) {
                        Ok(text) => {
                            let parsed = crate::shell::commands::builtin::scan::netscan::webpath::parse_words(&text);
                            if parsed.is_empty() {
                                print_warning(&t!("cmd.scan.waep_empty_wordlist", path));
                            } else {
                                words = parsed;
                            }
                        }
                        Err(e) => {
                            print_error(&t!("cmd.scan.waep_wordlist_failed", path, e));
                            return Ok(false);
                        }
                    }
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-x" | "--ext" => match value(&mut i, flag) {
                Ok(list) => {
                    exts = list
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-m" | "--method" => match value(&mut i, flag) {
                Ok(m) => {
                    let m = m.to_uppercase();
                    if m != "GET" && m != "HEAD" {
                        print_error(&t!("cmd.scan.waep_bad_method", m));
                        return Ok(false);
                    }
                    opts.method = m;
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-s" | "--status" => match value(&mut i, flag) {
                Ok(list) => {
                    let parsed: Vec<u16> = list
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u16>().ok())
                        .collect();
                    if parsed.is_empty() {
                        print_error(&t!("cmd.scan.waep_bad_status", list));
                        return Ok(false);
                    }
                    accept = parsed;
                }
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "--all" => show_all = true,
            "-r" | "--recursive" => recursive = true,
            "-d" | "--depth" => match value(&mut i, flag) {
                Ok(v) => match v.parse::<usize>() {
                    Ok(n) if n >= 1 => depth = n.min(4),
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_depth", v));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-c" | "--concurrency" => match value(&mut i, flag) {
                Ok(v) => match v.parse::<usize>() {
                    Ok(n) if n >= 1 => opts.concurrency = n.min(64),
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_concurrency", v));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "--timeout" => match value(&mut i, flag) {
                Ok(v) => match v.parse::<u64>() {
                    Ok(n) if n >= 50 => opts.timeout_ms = n,
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_timeout", v));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-H" | "--header" => match value(&mut i, flag) {
                Ok(h) => match h.split_once(':') {
                    Some((k, v)) if !k.trim().is_empty() => opts
                        .headers
                        .push((k.trim().to_string(), v.trim().to_string())),
                    _ => {
                        print_error(&t!("cmd.scan.waep_bad_header", h));
                        return Ok(false);
                    }
                },
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-A" | "--user-agent" => match value(&mut i, flag) {
                Ok(v) => opts.user_agent = v,
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-o" | "--output" => match value(&mut i, flag) {
                Ok(path) => output = Some(crate::utils::paths::expand_path_str(&path)),
                Err(e) => {
                    print_error(&e);
                    return Ok(false);
                }
            },
            "-q" | "--quiet" => quiet = true,
            other => {
                print_error(&t!("cmd.scan.unknown_option", other));
                return Ok(false);
            }
        }
        i += 1;
    }

    // Re-base the word list on the scanned path (`.../api` probes
    // `/api/files`, not `/api/api/files`).
    let words = crate::shell::commands::builtin::scan::netscan::webpath::adapt_words_to_base(
        &words,
        &target.base_path,
    );
    let words =
        crate::shell::commands::builtin::scan::netscan::webpath::expand_suffixes(&words, &exts);
    if words.is_empty() {
        print_error(&t!("cmd.scan.waep_empty_wordlist", target.base_url()));
        return Ok(false);
    }
    print_info(&t!("cmd.scan.waep_start", target.base_url(), words.len()));

    let mut rows: Vec<String> = Vec::new();
    let mut hit_count = 0usize;
    let mut probed = 0usize;
    let mut errors = 0usize;
    let start = std::time::Instant::now();

    // Depth 1 is the base scan; `-r -d N` adds N-1 descent levels under
    // directory-looking hits. Each level's bases are built by appending the
    // discovered (base-relative) path, so URLs never double a segment.
    let max_levels = if recursive { depth.max(1) } else { 1 };
    let mut frontier: Vec<crate::shell::commands::builtin::scan::netscan::webpath::Target> =
        vec![target.clone()];
    let mut seen_bases: std::collections::HashSet<String> = std::collections::HashSet::new();
    seen_bases.insert(target.base_path.clone());
    let mut seen_urls: std::collections::HashSet<String> = std::collections::HashSet::new();
    for level in 1..=max_levels {
        let mut next: Vec<crate::shell::commands::builtin::scan::netscan::webpath::Target> =
            Vec::new();
        for sub in &frontier {
            // A deeper level gets the same word list re-based on its own path
            // (root `-r` into `/api` must probe `/api/files`, never
            // `/api/api/files`).
            let sub_words =
                crate::shell::commands::builtin::scan::netscan::webpath::adapt_words_to_base(
                    &words,
                    &sub.base_path,
                );
            let (summary, found) = waep_run(
                sub,
                &sub_words,
                &opts,
                &accept,
                show_all,
                quiet,
                level > 1,
                &mut seen_urls,
                &mut rows,
                &mut hit_count,
            );
            probed += summary.probed;
            errors += summary.errors;
            if level >= max_levels {
                continue;
            }
            // Only directory-looking hits (no dot in the last segment) can
            // have children; a handful per level keeps a scan bounded.
            for path in found.into_iter().take(8) {
                let last = path.rsplit('/').next().unwrap_or("");
                if last.is_empty() || last.contains('.') {
                    continue;
                }
                let mut deeper = sub.clone();
                deeper.base_path =
                    crate::shell::commands::builtin::scan::netscan::webpath::join_path(
                        &sub.base_path,
                        &path,
                    );
                if deeper.base_path.len() > 200 || !seen_bases.insert(deeper.base_path.clone()) {
                    continue;
                }
                next.push(deeper);
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }

    let elapsed = start.elapsed().as_millis() as u64;
    if hit_count == 0 {
        // Every probe failing is an unreachable target, not an empty site.
        if probed > 0 && errors >= probed {
            print_error(&t!("cmd.scan.waep_unreachable", target.base_url()));
        } else {
            print_warning(&t!("cmd.scan.waep_none", target.base_url()));
        }
    } else {
        println!("{}", t!("cmd.scan.waep_summary", hit_count, probed).cyan());
    }
    if errors > 0 && errors < probed {
        print_warning(&t!("cmd.scan.waep_some_failed", errors, probed));
    }
    if elapsed > 0 {
        println!("{}", t!("cmd.scan.waep_elapsed", elapsed).dimmed());
    }
    if let Some(path) = output {
        let body = rows.join("\n") + "\n";
        match crate::output::write_file(std::path::Path::new(&path), &body) {
            Ok(()) => print_success(&t!("cmd.scan.waep_saved", path)),
            Err(e) => print_error(&t!("cmd.scan.waep_save_failed", path, e)),
        }
    }
    Ok(false)
}
