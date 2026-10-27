use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "dl.cancel_hint".into(),
        "Use 'dl --cancel <number|url>' to remove a record.".into(),
    );

    m.insert("dl.completed".into(), "Downloaded {} ({} in {}, {})".into());

    m.insert(
        "dl.dns_host_required".into(),
        "Host is required. Usage: dns <host>".into(),
    );

    m.insert(
        "dl.dns_looking_up".into(),
        "Looking up {} {} records via system tools...".into(),
    );

    m.insert(
        "dl.dns_lookup_failed".into(),
        "DNS lookup failed (exit code: {})".into(),
    );

    m.insert(
        "dl.dns_missing_type".into(),
        "Missing record type after -t".into(),
    );

    m.insert("dl.dns_no_records".into(), " {} No {} records found".into());

    m.insert(
        "dl.dns_tool_failed".into(),
        "Failed to run DNS tool: {}. Install dig or nslookup for {} record support.".into(),
    );

    m.insert("dl.downloading".into(), "Downloading {}".into());

    m.insert(
        "dl.failed_append".into(),
        "Failed to open {} for append".into(),
    );

    m.insert("dl.failed_create".into(), "Failed to create {}".into());

    m.insert("dl.failed_download".into(), "Failed to download {}".into());

    m.insert("dl.failed_read".into(), "Failed to read {}".into());

    m.insert(
        "dl.file_still_exists".into(),
        "File '{}' still exists. Remove manually if desired.".into(),
    );

    m.insert("dl.help.examples".into(), "Examples:".into());

    m.insert(
        "dl.help.opt_cancel".into(),
        " --cancel <number|url> Remove download record".into(),
    );

    m.insert(
        "dl.help.opt_help".into(),
        " -h, --help Show this help".into(),
    );

    m.insert(
        "dl.help.opt_o".into(),
        " -o, --output <file> Save as filename".into(),
    );

    m.insert(
        "dl.help.opt_progress".into(),
        " --progress Show download progress bar".into(),
    );

    m.insert(
        "dl.help.opt_resume".into(),
        " --resume Resume partial download".into(),
    );

    m.insert(
        "dl.help.opt_stop".into(),
        " --stop Show download history".into(),
    );

    m.insert("dl.help.options".into(), "Options:".into());

    m.insert("dl.help.title".into(), "Download (dl)".into());

    m.insert("dl.help.usage".into(), "Usage:".into());

    m.insert("dl.help.usage_line".into(), " dl <url> [options]".into());

    m.insert("dl.history_title".into(), "Download History:".into());

    m.insert("dl.http_error".into(), "HTTP {}: {}".into());

    m.insert(
        "dl.invalid_header".into(),
        "Invalid header format, expected \"Key: Value\", got: {}".into(),
    );

    m.insert("dl.invalid_index".into(), "Invalid index".into());

    m.insert("dl.invalid_request".into(), "Invalid request: {}".into());

    m.insert(
        "dl.missing_cancel".into(),
        "Missing URL/name after --cancel".into(),
    );

    m.insert("dl.missing_data".into(), "Missing data after -d".into());

    m.insert(
        "dl.missing_header".into(),
        "Missing header value after -H".into(),
    );

    m.insert("dl.missing_o".into(), "Missing filename after -o".into());

    m.insert("dl.no_records".into(), "No downloads recorded yet.".into());

    m.insert(
        "dl.not_found".into(),
        "No download record found for: {}".into(),
    );

    m.insert("dl.not_found_404".into(), "URL not found (404): {}".into());

    m.insert(
        "dl.ping_exit_code".into(),
        "ping exited with code: {}".into(),
    );

    m.insert("dl.ping_failed".into(), "Failed to run ping: {}".into());

    m.insert(
        "dl.ping_host_required".into(),
        "Host is required. Usage: ping <host>".into(),
    );

    m.insert("dl.ping_invalid_count".into(), "Invalid count".into());

    m.insert(
        "dl.ping_missing_count".into(),
        "Missing count after -c".into(),
    );

    m.insert(
        "dl.removed_record".into(),
        "Removed download record: {}".into(),
    );

    m.insert("dl.request_failed".into(), "Request to {} failed".into());

    m.insert(
        "dl.up_url_required".into(),
        "URL is required. Usage: up <file> <url>".into(),
    );

    m.insert("dl.upload_failed".into(), "Upload to {} failed".into());

    m.insert("dl.uploading".into(), "Uploading {} ({})".into());

    m.insert(
        "dl.url_required".into(),
        "URL is required. Usage: dl <url>".into(),
    );
}
