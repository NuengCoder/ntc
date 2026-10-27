use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.http.authorization_label".into(),
        "Authorization:".into(),
    );

    m.insert(
        "cmd.http.basic_placeholder".into(),
        "Basic <encoded>".into(),
    );

    m.insert(
        "cmd.http.bearer_placeholder".into(),
        "Bearer <token>".into(),
    );

    m.insert("cmd.http.byte_count".into(), "{} bytes".into());

    m.insert(
        "cmd.http.help_auth".into(),
        "  -a, --auth <user:pass>         Basic authentication".into(),
    );

    m.insert(
        "cmd.http.help_bearer".into(),
        "  --bearer <token>               Bearer token authentication".into(),
    );

    m.insert(
        "cmd.http.help_body".into(),
        "  -d, --data <body>              Request body".into(),
    );

    m.insert(
        "cmd.http.help_example1".into(),
        "  http GET https://api.github.com/users/octocat".into(),
    );

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert(
        "cmd.http.help_example3".into(),
        "  http GET https://api.example.com/search -q q=hello -q page=1 -s > results.json".into(),
    );

    m.insert(
        "cmd.http.help_example4".into(),
        "  http GET https://api.example.com/data --bearer mytoken -o response.json".into(),
    );

    m.insert(
        "cmd.http.help_example5".into(),
        "  http GET https://api.example.com/health -s -t 5".into(),
    );

    m.insert(
        "cmd.http.help_example6".into(),
        "  http GET https://api.example.com/users -a admin:secret123".into(),
    );

    m.insert("cmd.http.help_examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.http.help_header".into(),
        "  -H, --header <\\\"Key: Value\\\">     Add request header (repeatable)".into(),
    );

    m.insert(
        "cmd.http.help_help".into(),
        "  -h, --help                     Show this help".into(),
    );

    m.insert(
        "cmd.http.help_json".into(),
        "  --json <data>                  Send as JSON (auto-sets Content-Type)".into(),
    );

    m.insert(
        "cmd.http.help_methods".into(),
        "  GET, POST, PUT, DELETE, PATCH, HEAD".into(),
    );

    m.insert("cmd.http.help_methods_title".into(), "Methods:".into());

    m.insert(
        "cmd.http.help_no_pretty".into(),
        "  --no-pretty                    Disable JSON pretty-printing".into(),
    );

    m.insert("cmd.http.help_options_title".into(), "Options:".into());

    m.insert(
        "cmd.http.help_output".into(),
        "  -o, --output <file>            Save body to file (raw)".into(),
    );

    m.insert(
        "cmd.http.help_query".into(),
        "  -q, --query <key=value>        URL query parameter (repeatable)".into(),
    );

    m.insert(
        "cmd.http.help_silent".into(),
        "  -s, --silent                   Show body only (no status/headers)".into(),
    );

    m.insert(
        "cmd.http.help_timeout".into(),
        "  -t, --timeout <secs>           Request timeout (default 30)".into(),
    );

    m.insert("cmd.http.help_title".into(), "HTTP API Client".into());

    m.insert(
        "cmd.http.help_usage_default".into(),
        "  <url>                          (GET by default if URL is first argument)".into(),
    );

    m.insert(
        "cmd.http.help_usage_line".into(),
        "  http <method> <url> [options]".into(),
    );

    m.insert("cmd.http.help_usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.http.help_verbose".into(),
        "  -v, --verbose                  Show outgoing request details".into(),
    );

    m.insert("cmd.http.invalid_body".into(), "Invalid body: {}".into());

    m.insert(
        "cmd.http.invalid_header".into(),
        "Invalid header format, expected \"Key: Value\", got: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_query".into(),
        "Invalid query format, expected key=value, got: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_request".into(),
        "Invalid request: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_timeout".into(),
        "Invalid timeout: {}".into(),
    );

    m.insert(
        "cmd.http.missing_auth".into(),
        "Missing user:pass after -a".into(),
    );

    m.insert(
        "cmd.http.missing_bearer".into(),
        "Missing token after --bearer".into(),
    );

    m.insert(
        "cmd.http.missing_body_data".into(),
        "Missing body data after -d".into(),
    );

    m.insert(
        "cmd.http.missing_filename".into(),
        "Missing filename after -o".into(),
    );

    m.insert(
        "cmd.http.missing_header".into(),
        "Missing header value after -H".into(),
    );

    m.insert(
        "cmd.http.missing_json".into(),
        "Missing JSON data after --json".into(),
    );

    m.insert(
        "cmd.http.missing_query".into(),
        "Missing key=value after -q".into(),
    );

    m.insert(
        "cmd.http.missing_timeout".into(),
        "Missing seconds after -t".into(),
    );

    m.insert("cmd.http.request_failed".into(), "{} {} failed".into());

    m.insert("cmd.http.saved_bytes".into(), "Saved {} bytes to {}".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.unknown_status".into(), "Unknown".into());

    m.insert(
        "cmd.http.url_required".into(),
        "URL is required. Usage: http <method> <url>".into(),
    );

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.write_fail".into(), "Failed to write to {}".into());
}
