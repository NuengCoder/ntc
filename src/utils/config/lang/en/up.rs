use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("up.help.examples".into(), "Examples:".into());

    m.insert(
        "up.help.opt_data".into(),
        " -d, --data <body> Upload inline data".into(),
    );

    m.insert(
        "up.help.opt_header".into(),
        " -H, --header <\"Key: Value\"> Add request header (repeatable)".into(),
    );

    m.insert(
        "up.help.opt_help".into(),
        " -h, --help Show this help".into(),
    );

    m.insert(
        "up.help.opt_post".into(),
        " --post Use POST instead of PUT".into(),
    );

    m.insert("up.help.options".into(), "Options:".into());

    m.insert("up.help.title".into(), "Upload (up)".into());

    m.insert("up.help.usage".into(), "Usage:".into());

    m.insert(
        "up.help.usage_data".into(),
        " up -d \"data\" <url> Upload inline text data".into(),
    );

    m.insert(
        "up.help.usage_line".into(),
        " up <file> <url> Upload file via PUT".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        " up <file> <url> --post Upload via POST".into(),
    );
}
