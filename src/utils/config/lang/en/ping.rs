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
}
