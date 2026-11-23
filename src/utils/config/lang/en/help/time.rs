use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.bench".into(),
        "time --bench <n> <command> ## Repeat n times and show min/max/mean/median".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "time <command A> --compare <command B> [-w] ## Run both and compare exit status + captured output".into(),
    );

    m.insert(
        "help.time.count".into(),
        "time --count <duration> ## Countdown (e.g. 30, 90s, 5m, 1h30m)".into(),
    );

    m.insert(
        "help.time.help".into(),
        "time --help ## Show this help".into(),
    );

    m.insert(
        "help.time.note".into(),
        "Commands run in the current directory and may be NTC commands, aliases, or external commands. --compare captures output; --bench runs the command n times, so its side effects repeat too. Ctrl+C cancels a countdown.".into(),
    );

    m.insert(
        "help.time.run".into(),
        "time <command> ## Run a command and show how long it took".into(),
    );

    m.insert(
        "help.time.title".into(),
        "TIME COMMAND - timing, comparison and countdowns:".into(),
    );
}
