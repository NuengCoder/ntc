use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.title".into(),
        "TIME COMMAND - command timing and countdowns:".into(),
    );

    m.insert(
        "help.time.run".into(),
        "  time run <command>                     Run an NTC command, alias, or external command"
            .into(),
    );

    m.insert(
        "help.time.alias".into(),
        "  time -r|--run <command>                Shorthand for run".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "  time -r <command> --compare <command>  Run both and compare captured output".into(),
    );

    m.insert("help.time.compare_with".into(), "  time compare <command A> --with <command B> [-w]  Compare two commands (alias: -r A --compare B)".into());

    m.insert("help.time.bench".into(), "  time bench [--runs N] <command>        Repeat N times (default 5) and show min/max/mean/median".into());

    m.insert(
        "help.time.count".into(),
        "  time count <duration>                  Countdown (e.g. 30, 90s, 5m, 1h30m)".into(),
    );

    m.insert(
        "help.time.help".into(),
        "  time help                              Show this help".into(),
    );

    m.insert("help.time.note".into(), "Commands run sequentially in the current directory and may be NTC commands, aliases, or external commands. Comparison and bench capture output; command side effects remain. Ctrl+C cancels a countdown.".into());
}
