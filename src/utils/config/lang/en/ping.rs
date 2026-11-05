use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ping.help.examples".into(), "Examples:".into());

    m.insert(
        "ping.help.opt_count".into(),
        " -c, --count <n> Number of pings (default 4)".into(),
    );

    m.insert(
        "ping.help.opt_help".into(),
        " -h, --help Show this help".into(),
    );

    m.insert("ping.help.options".into(), "Options:".into());

    m.insert("ping.help.title".into(), "Ping".into());

    m.insert("ping.help.usage".into(), "Usage:".into());

    m.insert(
        "ping.help.usage_line".into(),
        " ping <host> [options]".into(),
    );

    m.insert(
        "ping.help.opt_forever".into(),
        " --forever Ping until stopped (Ctrl+C)".into(),
    );

    m.insert(
        "ping.help.opt_interval".into(),
        " -i, --interval <ms> Interval between pings (Unix only)".into(),
    );

    m.insert(
        "ping.help.opt_timeout".into(),
        " -w, --timeout <ms> Reply timeout".into(),
    );

    m.insert(
        "ping.help.opt_size".into(),
        " -s, --size <bytes> Payload size".into(),
    );

    m.insert(
        "ping.help.opt_ttl".into(),
        " -t, --ttl <n> Time to live".into(),
    );

    m.insert(
        "ping.help.opt_family".into(),
        " -4, -6 Force IPv4 / IPv6".into(),
    );

    m.insert(
        "ping.help.opt_quiet".into(),
        " -q, --quiet Quiet output (Unix only)".into(),
    );
}
