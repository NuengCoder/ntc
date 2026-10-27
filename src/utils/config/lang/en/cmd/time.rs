use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.time.count_usage".into(),
        "Usage: time count <duration>".into(),
    );

    m.insert(
        "cmd.time.run_usage".into(),
        "Usage: time run <command>".into(),
    );

    m.insert("cmd.time.executing".into(), "Executing: {}".into());

    m.insert("cmd.time.elapsed".into(), "Elapsed: {}".into());

    m.insert("cmd.time.exit_status".into(), "Exit status: {}".into());

    m.insert(
        "cmd.time.terminated".into(),
        "Command was terminated".into(),
    );

    m.insert(
        "cmd.time.first_command".into(),
        "Running first command: {}".into(),
    );

    m.insert(
        "cmd.time.second_command".into(),
        "Running second command: {}".into(),
    );

    m.insert(
        "cmd.time.first_result".into(),
        "First:  {} (exit {})".into(),
    );

    m.insert(
        "cmd.time.second_result".into(),
        "Second: {} (exit {})".into(),
    );

    m.insert(
        "cmd.time.outputs_match".into(),
        "Command outputs match.".into(),
    );

    m.insert(
        "cmd.time.outputs_differ".into(),
        "Command outputs differ:".into(),
    );

    m.insert(
        "cmd.time.large_diff".into(),
        "Diff is large; showing removed and added lines without alignment:".into(),
    );

    m.insert(
        "cmd.time.invalid_duration".into(),
        "Invalid duration '{}'. Use seconds or forms such as 90s, 5m, or 1h30m.".into(),
    );

    m.insert("cmd.time.cancelled".into(), "Timer cancelled.".into());

    m.insert("cmd.time.finished".into(), "Time is up!".into());

    m.insert("cmd.time.remaining".into(), "{} seconds remaining ".into());

    m.insert(
        "cmd.time.compare_usage".into(),
        "Usage: time compare <command A> --with <command B> [-w]".into(),
    );

    m.insert(
        "cmd.time.bench_usage".into(),
        "Usage: time bench [--runs N] <command>".into(),
    );

    m.insert(
        "cmd.time.invalid_runs".into(),
        "time bench: invalid run count '{}'. Use a positive integer.".into(),
    );

    m.insert(
        "cmd.time.bench_header".into(),
        "Benchmarking {} run(s): {}".into(),
    );

    m.insert("cmd.time.bench_run".into(), "  Run {}: {} ({})".into());

    m.insert("cmd.time.bench_summary".into(), "Summary ({} runs):".into());

    m.insert("cmd.time.bench_min".into(), "  min:    {}".into());

    m.insert("cmd.time.bench_max".into(), "  max:    {}".into());

    m.insert("cmd.time.bench_mean".into(), "  mean:   {}".into());

    m.insert("cmd.time.bench_median".into(), "  median: {}".into());

    m.insert(
        "cmd.time.bench_note".into(),
        "Repeated runs may change files or environment; side effects are not rolled back.".into(),
    );

    m.insert("cmd.time.status_ok".into(), "ok".into());

    m.insert("cmd.time.status_failed".into(), "failed".into());

    m.insert(
        "cmd.time.ignoring_whitespace".into(),
        "Ignoring whitespace-only differences.".into(),
    );
}
