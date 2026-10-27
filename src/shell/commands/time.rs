use super::{capture_system_command, run_system_command};
use crate::navigator::Navigator;
use crate::output::{
    print_error, print_info, print_success, print_warning, with_captured_output_bytes,
};
use anyhow::{anyhow, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub(super) fn cmd_time(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() || args == "help" || args == "--help" || args == "-h" {
        print_help();
        return Ok(false);
    }

    if let Some(rest) = args.strip_prefix("count ") {
        return countdown(rest.trim());
    }
    if args == "count" {
        print_error(&crate::t!("cmd.time.count_usage"));
        return Ok(false);
    }

    if let Some(rest) = args.strip_prefix("compare ") {
        return compare_mode(rest.trim(), nav);
    }
    if args == "compare" {
        print_error(&crate::t!("cmd.time.compare_usage"));
        return Ok(false);
    }

    if let Some(rest) = args.strip_prefix("bench ") {
        return bench_mode(rest.trim(), nav);
    }
    if args == "bench" {
        print_error(&crate::t!("cmd.time.bench_usage"));
        return Ok(false);
    }

    let command = if let Some(rest) = args.strip_prefix("run ") {
        rest.trim()
    } else if let Some(rest) = args.strip_prefix("--run ") {
        rest.trim()
    } else if let Some(rest) = args.strip_prefix("-r ") {
        rest.trim()
    } else {
        print_help();
        return Ok(false);
    };

    if command.is_empty() {
        print_error(&crate::t!("cmd.time.run_usage"));
        return Ok(false);
    }
    if matches!(command, "help" | "--help" | "-h") {
        print_help();
        return Ok(false);
    }

    if let Some((left, right, ignore_ws)) = split_and_clean_comparison(command) {
        let left = expand_line(&left);
        let right = expand_line(&right);
        return compare_commands(&left, &right, ignore_ws, nav);
    }

    run_timed(&expand_line(command), nav)
}

fn compare_mode(raw: &str, nav: &mut Navigator) -> Result<bool> {
    if raw.is_empty() {
        print_error(&crate::t!("cmd.time.compare_usage"));
        return Ok(false);
    }
    let Some((left, right, ignore_ws)) = split_and_clean_comparison(raw) else {
        print_error(&crate::t!("cmd.time.compare_usage"));
        return Ok(false);
    };
    let left = expand_line(&left);
    let right = expand_line(&right);
    compare_commands(&left, &right, ignore_ws, nav)
}

fn bench_mode(raw: &str, nav: &mut Navigator) -> Result<bool> {
    let (runs, command) = match parse_bench_args(raw) {
        Ok(parsed) => parsed,
        Err(message) => {
            print_error(&message);
            return Ok(false);
        }
    };
    let command = expand_line(&command);
    println!("{}", crate::t!("cmd.time.bench_header", runs, command));

    let mut durations = Vec::with_capacity(runs);
    let mut failed = false;
    let mut exit_shell = false;
    for index in 1..=runs {
        let outcome = run_once(&command, nav, true)?;
        durations.push(outcome.elapsed);
        println!(
            "{}",
            crate::t!(
                "cmd.time.bench_run",
                index,
                format_elapsed(outcome.elapsed),
                outcome.status
            )
        );
        failed |= !outcome.success;
        exit_shell |= outcome.exit_shell;
    }

    let stats = summarize_runs(&durations);
    println!("{}", crate::t!("cmd.time.bench_summary", runs));
    println!(
        "{}",
        crate::t!("cmd.time.bench_min", format_elapsed(stats.min))
    );
    println!(
        "{}",
        crate::t!("cmd.time.bench_max", format_elapsed(stats.max))
    );
    println!(
        "{}",
        crate::t!("cmd.time.bench_mean", format_elapsed(stats.mean))
    );
    println!(
        "{}",
        crate::t!("cmd.time.bench_median", format_elapsed(stats.median))
    );
    print_warning(&crate::t!("cmd.time.bench_note"));

    if failed {
        crate::shell::pipeline::mark_failed();
    }
    Ok(exit_shell)
}

fn expand_line(line: &str) -> String {
    crate::shell::alias::expand_aliases_line_deep(line)
}

/// Commands that need NTC's own executor (built-ins, aliases that resolve to
/// built-ins, or pipelines containing one). Pure external command lines keep
/// their numeric exit status through the system runner.
fn uses_ntc(line: &str) -> bool {
    crate::shell::pipeline::split_commands(line)
        .iter()
        .filter_map(|(_, segment)| first_token(segment))
        .any(|token| crate::shell::hinter::is_builtin_command(&token))
}

fn first_token(segment: &str) -> Option<String> {
    let token = segment.split_whitespace().next()?;
    Some(token.trim_matches(|c| c == '"' || c == '\'').to_lowercase())
}

struct RunOutcome {
    elapsed: Duration,
    stdout: String,
    stderr: String,
    status: String,
    success: bool,
    exit_shell: bool,
}

fn run_once(command: &str, nav: &mut Navigator, capture: bool) -> Result<RunOutcome> {
    if uses_ntc(command) {
        run_ntc(command, nav, capture)
    } else {
        run_external(command, nav, capture)
    }
}

fn native_status(success: bool) -> String {
    if success {
        crate::t!("cmd.time.status_ok")
    } else {
        crate::t!("cmd.time.status_failed")
    }
}

fn run_ntc(command: &str, nav: &mut Navigator, capture: bool) -> Result<RunOutcome> {
    if capture {
        let mut result: Result<(bool, bool)> = Ok((false, true));
        let start = Instant::now();
        let bytes = with_captured_output_bytes(|| {
            result = super::execute_command_with_status(command, nav, &None);
        });
        let elapsed = start.elapsed();
        let stdout =
            String::from_utf8_lossy(&crate::shell::pipeline::strip_ansi_bytes(&bytes)).into_owned();
        match result {
            Ok((exit_shell, success)) => Ok(RunOutcome {
                elapsed,
                stdout,
                stderr: String::new(),
                status: native_status(success),
                success,
                exit_shell,
            }),
            Err(err) => Ok(RunOutcome {
                elapsed,
                stdout,
                stderr: format!("{err}"),
                status: crate::t!("cmd.time.status_failed"),
                success: false,
                exit_shell: false,
            }),
        }
    } else {
        let start = Instant::now();
        let result = super::execute_command_with_status(command, nav, &None);
        let elapsed = start.elapsed();
        match result {
            Ok((exit_shell, success)) => Ok(RunOutcome {
                elapsed,
                stdout: String::new(),
                stderr: String::new(),
                status: native_status(success),
                success,
                exit_shell,
            }),
            Err(err) => {
                print_error(&format!("{}: {err}", crate::t!("cmd.time.status_failed")));
                Ok(RunOutcome {
                    elapsed,
                    stdout: String::new(),
                    stderr: String::new(),
                    status: crate::t!("cmd.time.status_failed"),
                    success: false,
                    exit_shell: false,
                })
            }
        }
    }
}

fn run_external(command: &str, nav: &mut Navigator, capture: bool) -> Result<RunOutcome> {
    let cwd = nav.current_path().to_path_buf();
    if capture {
        let start = Instant::now();
        let output = capture_system_command(command, &cwd)?;
        let elapsed = start.elapsed();
        Ok(RunOutcome {
            elapsed,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            status: status_text(&output),
            success: output.status.success(),
            exit_shell: false,
        })
    } else {
        let start = Instant::now();
        let status = run_system_command(command, &cwd)?;
        let elapsed = start.elapsed();
        Ok(RunOutcome {
            elapsed,
            stdout: String::new(),
            stderr: String::new(),
            status: status_code_text(&status),
            success: status.success(),
            exit_shell: false,
        })
    }
}

fn run_timed(command: &str, nav: &mut Navigator) -> Result<bool> {
    if command.is_empty() {
        print_error(&crate::t!("cmd.time.run_usage"));
        return Ok(false);
    }
    print_info(&crate::t!("cmd.time.executing", command));
    let outcome = run_once(command, nav, false)?;
    println!();
    print_info(&crate::t!(
        "cmd.time.elapsed",
        format_elapsed(outcome.elapsed)
    ));
    print_info(&crate::t!("cmd.time.exit_status", outcome.status));
    if !outcome.success {
        crate::shell::pipeline::mark_failed();
    }
    Ok(outcome.exit_shell)
}

fn compare_commands(left: &str, right: &str, ignore_ws: bool, nav: &mut Navigator) -> Result<bool> {
    if left.is_empty() || right.is_empty() {
        print_error(&crate::t!("cmd.time.compare_usage"));
        return Ok(false);
    }
    println!("{}", crate::t!("cmd.time.first_command", left));
    let first = run_once(left, nav, true)?;

    println!("{}", crate::t!("cmd.time.second_command", right));
    let second = run_once(right, nav, true)?;

    let mut first_text = combined_output(&first);
    let mut second_text = combined_output(&second);
    if ignore_ws {
        print_info(&crate::t!("cmd.time.ignoring_whitespace"));
        first_text = normalize_whitespace(&first_text);
        second_text = normalize_whitespace(&second_text);
    }

    println!(
        "\n{}",
        crate::t!(
            "cmd.time.first_result",
            format_elapsed(first.elapsed),
            first.status
        )
    );
    println!(
        "{}",
        crate::t!(
            "cmd.time.second_result",
            format_elapsed(second.elapsed),
            second.status
        )
    );
    if first_text == second_text {
        print_success(&crate::t!("cmd.time.outputs_match"));
    } else {
        print_warning(&crate::t!("cmd.time.outputs_differ"));
        print_diff(&first_text, &second_text);
    }
    if !first.success || !second.success {
        crate::shell::pipeline::mark_failed();
    }
    Ok(first.exit_shell || second.exit_shell)
}

fn combined_output(outcome: &RunOutcome) -> String {
    let mut text = outcome.stdout.clone();
    if !outcome.stderr.is_empty() {
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str("[stderr]\n");
        text.push_str(&outcome.stderr);
    }
    text
}

fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn status_text(output: &std::process::Output) -> String {
    status_code_text(&output.status)
}

fn status_code_text(status: &std::process::ExitStatus) -> String {
    status
        .code()
        .map(|code| code.to_string())
        .unwrap_or_else(|| crate::t!("cmd.time.terminated"))
}

fn print_diff(left: &str, right: &str) {
    let a: Vec<&str> = left.lines().collect();
    let b: Vec<&str> = right.lines().collect();
    let width = b.len() + 1;
    if a.len().saturating_add(1).saturating_mul(width) > 2_000_000 {
        println!("{}", crate::t!("cmd.time.large_diff"));
        for line in a {
            println!("- {}", line);
        }
        for line in b {
            println!("+ {}", line);
        }
        return;
    }
    let mut lcs = vec![0usize; (a.len() + 1) * width];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i * width + j] = if a[i] == b[j] {
                lcs[(i + 1) * width + j + 1] + 1
            } else {
                lcs[(i + 1) * width + j].max(lcs[i * width + j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            println!("  {}", a[i]);
            i += 1;
            j += 1;
        } else if i < a.len()
            && (j == b.len() || lcs[(i + 1) * width + j] >= lcs[i * width + j + 1])
        {
            println!("- {}", a[i]);
            i += 1;
        } else {
            println!("+ {}", b[j]);
            j += 1;
        }
    }
}

/// Find an unquoted `--compare` or `--with` separator and return both sides.
fn split_comparison(input: &str) -> Option<(&str, &str)> {
    let bytes = input.as_bytes();
    let (mut double, mut single) = (false, false);
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if double && i + 1 < bytes.len() => i += 2,
            b'"' if !single => {
                double = !double;
                i += 1;
            }
            b'\'' if !double => {
                single = !single;
                i += 1;
            }
            _ if !double && !single && (i == 0 || bytes[i - 1].is_ascii_whitespace()) => {
                let marker = ["--compare", "--with"].iter().find(|m| {
                    input[i..].starts_with(**m)
                        && (i + m.len() == bytes.len() || bytes[i + m.len()].is_ascii_whitespace())
                });
                if let Some(marker) = marker {
                    return Some((
                        input[..i].trim_end(),
                        input[i + marker.len()..].trim_start(),
                    ));
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    None
}

fn split_and_clean_comparison(input: &str) -> Option<(String, String, bool)> {
    let (left, right) = split_comparison(input)?;
    let (left, left_long) = remove_flags(left, &["--ignore-whitespace"]);
    let (left, left_leading) = take_leading_token_flag(&left, &["-w"]);
    let (left, left_trailing) = remove_trailing_token_flag(&left, &["-w"]);

    let (right, right_long) = remove_flags(right, &["--ignore-whitespace"]);
    let (right, right_leading) = take_leading_token_flag(&right, &["-w"]);
    let (right, right_trailing) = remove_trailing_token_flag(&right, &["-w"]);

    Some((
        left,
        right,
        left_long || left_leading || left_trailing || right_long || right_leading || right_trailing,
    ))
}

fn remove_flags(input: &str, flags: &[&str]) -> (String, bool) {
    let spans = token_spans(input);
    let mut out = String::new();
    let mut last = 0;
    let mut found = false;
    for (start, end) in spans {
        if flags.contains(&&input[start..end]) {
            out.push_str(&input[last..start]);
            last = end;
            found = true;
        }
    }
    out.push_str(&input[last..]);
    (out.trim().to_string(), found)
}

fn take_leading_token_flag(input: &str, flags: &[&str]) -> (String, bool) {
    let mut rest = input.trim();
    let mut found = false;
    loop {
        let (token, tail) = take_token(rest);
        if token.is_empty() || !flags.contains(&token) {
            break;
        }
        found = true;
        rest = tail;
    }
    (rest.to_string(), found)
}

fn remove_trailing_token_flag(input: &str, flags: &[&str]) -> (String, bool) {
    let spans = token_spans(input);
    if let Some(&(start, end)) = spans.last() {
        if flags.contains(&&input[start..end]) {
            return (input[..start].trim_end().to_string(), true);
        }
    }
    (input.to_string(), false)
}

fn token_spans(input: &str) -> Vec<(usize, usize)> {
    let bytes = input.as_bytes();
    let (mut double, mut single) = (false, false);
    let mut spans = Vec::new();
    let mut start: Option<usize> = None;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if double && i + 1 < bytes.len() => {
                i += 2;
                continue;
            }
            b'"' if !single => double = !double,
            b'\'' if !double => single = !single,
            _ => {}
        }
        if !double && !single && bytes[i].is_ascii_whitespace() {
            if let Some(s) = start.take() {
                spans.push((s, i));
            }
        } else if start.is_none() {
            start = Some(i);
        }
        i += 1;
    }
    if let Some(s) = start {
        spans.push((s, bytes.len()));
    }
    spans
}

fn take_token(input: &str) -> (&str, &str) {
    let input = input.trim_start();
    match input.find(char::is_whitespace) {
        Some(index) => (&input[..index], input[index..].trim_start()),
        None => (input, ""),
    }
}

fn parse_bench_args(raw: &str) -> std::result::Result<(usize, String), String> {
    let mut rest = raw.trim();
    let mut runs = 5usize;
    loop {
        let (token, tail) = take_token(rest);
        if token.is_empty() {
            break;
        }
        if let Some(value) = token.strip_prefix("--runs=") {
            runs = parse_runs(value)?;
            rest = tail;
            continue;
        }
        if token == "--runs" || token == "-n" {
            let (value, after) = take_token(tail);
            runs = parse_runs(value)?;
            rest = after;
            continue;
        }
        break;
    }
    if rest.is_empty() {
        Err(crate::t!("cmd.time.bench_usage"))
    } else {
        Ok((runs, rest.to_string()))
    }
}

fn parse_runs(raw: &str) -> std::result::Result<usize, String> {
    match raw.trim().parse::<usize>() {
        Ok(runs) if runs > 0 => Ok(runs),
        _ => Err(crate::t!("cmd.time.invalid_runs", raw)),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RunStats {
    min: Duration,
    max: Duration,
    mean: Duration,
    median: Duration,
}

fn summarize_runs(runs: &[Duration]) -> RunStats {
    if runs.is_empty() {
        return RunStats {
            min: Duration::ZERO,
            max: Duration::ZERO,
            mean: Duration::ZERO,
            median: Duration::ZERO,
        };
    }
    let min = *runs.iter().min().expect("runs is not empty");
    let max = *runs.iter().max().expect("runs is not empty");
    let total: u128 = runs.iter().map(Duration::as_nanos).sum();
    let mean_ns = total / runs.len() as u128;

    let mut sorted = runs.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    let median_ns = if sorted.len() % 2 == 1 {
        sorted[mid].as_nanos()
    } else {
        (sorted[mid - 1].as_nanos() + sorted[mid].as_nanos()) / 2
    };

    RunStats {
        min,
        max,
        mean: Duration::from_nanos(mean_ns.min(u64::MAX as u128) as u64),
        median: Duration::from_nanos(median_ns.min(u64::MAX as u128) as u64),
    }
}

fn countdown(raw: &str) -> Result<bool> {
    let duration =
        parse_duration(raw).ok_or_else(|| anyhow!(crate::t!("cmd.time.invalid_duration", raw)))?;
    let cancelled = Arc::new(AtomicBool::new(false));
    super::register_shutdown_flag(Arc::clone(&cancelled));
    super::init_ctrlc_handler();
    let start = Instant::now();
    let mut shown = u64::MAX;
    loop {
        if cancelled.load(Ordering::SeqCst) {
            println!("\n{}", crate::t!("cmd.time.cancelled"));
            break;
        }
        let elapsed = start.elapsed();
        if elapsed >= duration {
            print_success(&crate::t!("cmd.time.finished"));
            break;
        }
        let remaining = (duration - elapsed).as_secs().saturating_add(1);
        if remaining != shown {
            print!("\r{}", crate::t!("cmd.time.remaining", remaining));
            let _ = std::io::Write::flush(&mut std::io::stdout());
            shown = remaining;
        }
        std::thread::sleep((duration - elapsed).min(Duration::from_millis(100)));
    }
    super::unregister_shutdown_flag(&cancelled);
    Ok(false)
}

fn parse_duration(raw: &str) -> Option<Duration> {
    let raw = raw.trim();
    if let Ok(seconds) = raw.parse::<f64>() {
        return (seconds.is_finite() && seconds >= 0.0 && seconds < u64::MAX as f64)
            .then(|| Duration::from_secs_f64(seconds));
    }
    let bytes = raw.as_bytes();
    let mut i = 0;
    let mut total = 0.0f64;
    let mut found = false;
    while i < bytes.len() {
        let start = i;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
            i += 1;
        }
        if start == i {
            return None;
        }
        let amount = raw[start..i].parse::<f64>().ok()?;
        let unit_start = i;
        while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
            i += 1;
        }
        let multiplier = match &raw[unit_start..i] {
            "s" | "sec" | "secs" => 1.0,
            "m" | "min" | "mins" => 60.0,
            "h" | "hr" | "hrs" => 3600.0,
            _ => return None,
        };
        total += amount * multiplier;
        found = true;
    }
    (found && total.is_finite() && total < u64::MAX as f64).then(|| Duration::from_secs_f64(total))
}

fn format_elapsed(duration: Duration) -> String {
    let ms = duration.as_millis();
    if ms < 1000 {
        format!("{ms}ms")
    } else if duration.as_secs() < 60 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_secs() < 3600 {
        format!(
            "{}m {:.1}s",
            duration.as_secs() / 60,
            duration.as_secs_f64() % 60.0
        )
    } else {
        format!(
            "{}h {}m {:.1}s",
            duration.as_secs() / 3600,
            (duration.as_secs() % 3600) / 60,
            duration.as_secs_f64() % 60.0
        )
    }
}

fn print_help() {
    let _ = crate::shell::help::print_topic_help("time");
}

#[cfg(test)]
mod tests {
    use super::{
        format_elapsed, parse_bench_args, parse_duration, remove_flags, split_and_clean_comparison,
        split_comparison, summarize_runs, take_leading_token_flag, RunStats,
    };
    use std::time::Duration;

    #[test]
    fn parses_seconds_and_compound_durations() {
        assert_eq!(parse_duration("30"), Some(Duration::from_secs(30)));
        assert_eq!(parse_duration("90s"), Some(Duration::from_secs(90)));
        assert_eq!(parse_duration("1h30m"), Some(Duration::from_secs(5400)));
        assert_eq!(parse_duration("2.5s"), Some(Duration::from_millis(2500)));
        assert_eq!(parse_duration("-2s"), None);
        assert_eq!(parse_duration("2fortnights"), None);
    }

    #[test]
    fn comparison_separator_ignores_quoted_markers() {
        assert_eq!(
            split_comparison("say \"--compare\" --compare say done"),
            Some(("say \"--compare\"", "say done"))
        );
        assert_eq!(split_comparison("say '--with'"), None);
    }

    #[test]
    fn comparison_flags_stay_out_of_commands() {
        let (left, right, ignore) =
            split_and_clean_comparison("grep -w foo --with grep -w bar").unwrap();
        assert_eq!(left, "grep -w foo");
        assert_eq!(right, "grep -w bar");
        assert!(!ignore);
    }

    #[test]
    fn comparison_flags_can_be_leading_or_trailing() {
        let (left, right, ignore) =
            split_and_clean_comparison("-w echo hi --with echo bye -w").unwrap();
        assert_eq!(left, "echo hi");
        assert_eq!(right, "echo bye");
        assert!(ignore);
    }

    #[test]
    fn long_ignore_flag_is_removed_inside_compare() {
        let (left, right, ignore) =
            split_and_clean_comparison("echo hi --ignore-whitespace --with echo bye").unwrap();
        assert_eq!(left, "echo hi");
        assert_eq!(right, "echo bye");
        assert!(ignore);
    }

    #[test]
    fn remove_flags_keeps_quoted_tokens() {
        let (clean, found) =
            remove_flags("echo \"--ignore-whitespace\" hi", &["--ignore-whitespace"]);
        assert_eq!(clean, "echo \"--ignore-whitespace\" hi");
        assert!(!found);
    }

    #[test]
    fn leading_flag_removal_only_uses_first_token() {
        let (clean, found) = take_leading_token_flag("-w grep -w foo", &["-w"]);
        assert_eq!(clean, "grep -w foo");
        assert!(found);
    }

    #[test]
    fn parses_bench_runs_and_command() {
        assert_eq!(
            parse_bench_args("echo hi").unwrap(),
            (5, "echo hi".to_string())
        );
        assert_eq!(
            parse_bench_args("--runs 3 echo hi").unwrap(),
            (3, "echo hi".to_string())
        );
        assert_eq!(
            parse_bench_args("--runs=7 echo hi").unwrap(),
            (7, "echo hi".to_string())
        );
        assert_eq!(
            parse_bench_args("-n 2 echo hi").unwrap(),
            (2, "echo hi".to_string())
        );
        assert!(parse_bench_args("--runs 0 echo hi").is_err());
        assert!(parse_bench_args("--runs").is_err());
    }

    #[test]
    fn summarizes_run_durations() {
        let stats = summarize_runs(&[
            Duration::from_millis(10),
            Duration::from_millis(30),
            Duration::from_millis(20),
        ]);
        assert_eq!(
            stats,
            RunStats {
                min: Duration::from_millis(10),
                max: Duration::from_millis(30),
                mean: Duration::from_millis(20),
                median: Duration::from_millis(20),
            }
        );
    }

    #[test]
    fn formats_elapsed_time_by_scale() {
        assert_eq!(format_elapsed(Duration::from_millis(950)), "950ms");
        assert_eq!(format_elapsed(Duration::from_millis(1500)), "1.50s");
        assert_eq!(format_elapsed(Duration::from_secs(65)), "1m 5.0s");
    }
}
