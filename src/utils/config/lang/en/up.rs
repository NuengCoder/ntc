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
        " --post Use POST (the default for uploads)".into(),
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
        " up <file> <url> Upload file as multipart POST".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        " up <file> <url> --put --raw Send the raw bytes with PUT".into(),
    );

    m.insert(
        "up.missing_auth".into(),
        "Missing credentials after -a (expected user:pass)".into(),
    );

    m.insert(
        "up.missing_bearer".into(),
        "Missing token after --bearer".into(),
    );

    m.insert(
        "up.missing_form".into(),
        "Missing field after -F (expected name=value)".into(),
    );

    m.insert(
        "up.invalid_form".into(),
        "Invalid form field: {} (expected name=value)".into(),
    );

    m.insert(
        "up.missing_timeout".into(),
        "Missing seconds after --timeout".into(),
    );

    m.insert("up.invalid_timeout".into(), "Invalid timeout: {}".into());

    m.insert(
        "up.conflict".into(),
        "Use either a file or -d data, not both".into(),
    );

    m.insert(
        "up.fallback_retry".into(),
        "Server answered {} - retrying as {}".into(),
    );

    m.insert(
        "up.help.opt_auth".into(),
        " -a, --auth <user:pass> Basic auth credentials".into(),
    );

    m.insert(
        "up.help.opt_bearer".into(),
        " --bearer <token> Bearer token".into(),
    );

    m.insert(
        "up.help.opt_form".into(),
        " -F, --form <name=value> Extra multipart field (repeatable)".into(),
    );

    m.insert(
        "up.help.opt_raw".into(),
        " --raw Send the file bytes as the body".into(),
    );

    m.insert(
        "up.help.opt_put".into(),
        " --put Send a raw PUT instead of multipart POST".into(),
    );

    m.insert(
        "up.help.opt_timeout".into(),
        " --timeout <seconds> Request timeout (default 300)".into(),
    );

    m.insert(
        "up.help.opt_no_fallback".into(),
        " --no-fallback Do not retry the other transport".into(),
    );
}
