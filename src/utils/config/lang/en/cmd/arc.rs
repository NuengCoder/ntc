use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.arc.create_usage".into(),
        "Usage: arc create <out.zip|tar.gz|tgz|tar> <files...>".into(),
    );

    m.insert(
        "cmd.arc.extract_usage".into(),
        "Usage: arc extract <archive> [dest]".into(),
    );

    m.insert(
        "cmd.arc.list_usage".into(),
        "Usage: arc list <archive>".into(),
    );

    m.insert(
        "cmd.arc.bad_ext".into(),
        "Unsupported archive type (use .zip, .tar.gz/.tgz, .tar)".into(),
    );

    m.insert("cmd.arc.not_found".into(), "Not found: {}".into());

    m.insert(
        "cmd.arc.no_inputs".into(),
        "No input files to archive.".into(),
    );

    m.insert(
        "cmd.arc.created".into(),
        "Archive created: {} ({} entries)".into(),
    );

    m.insert(
        "cmd.arc.extracted".into(),
        "Extracted {} files to {}".into(),
    );

    m.insert("cmd.arc.list_title".into(), "ARCHIVE: {}".into());

    m.insert("cmd.arc.total".into(), "Total: {} entries".into());

    m.insert(
        "cmd.arc.unknown_sub".into(),
        "Unknown arc subcommand: '{}'".into(),
    );

    m.insert("cmd.arc.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.arc.help_create".into(),
        "  arc create <out> <files...>  Create archive (.zip, .tar.gz, .tar)".into(),
    );

    m.insert(
        "cmd.arc.help_extract".into(),
        "  arc extract <file> [dest]   Extract archive".into(),
    );

    m.insert(
        "cmd.arc.help_list".into(),
        "  arc list <file>              List archive contents".into(),
    );

    m.insert("cmd.arc.examples_title".into(), "Examples:".into());
}
