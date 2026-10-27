use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.tl.dir_not_found".into(),
        "Directory not found: {}".into(),
    );

    m.insert("cmd.tl.dir_usage".into(), "Usage: tl dir <path>".into());

    m.insert(
        "cmd.tl.ext_not_supported".into(),
        ".{} not supported — blocked".into(),
    );

    m.insert(
        "cmd.tl.ext_usage".into(),
        "Usage: tl ext <extension>".into(),
    );

    m.insert("cmd.tl.file_not_found".into(), "File not found: {}".into());

    m.insert("cmd.tl.file_usage".into(), "Usage: tl file <path>".into());

    m.insert(
        "cmd.tl.help_dir".into(),
        "  tl dir <path>           Show supported extensions and total lines in directory".into(),
    );

    m.insert(
        "cmd.tl.help_ext".into(),
        "  tl ext <extension>      Show all files with extension and total lines".into(),
    );

    m.insert(
        "cmd.tl.help_file".into(),
        "  tl file <path>          Count lines in a single file".into(),
    );

    m.insert("cmd.tl.help_sort".into(), "  tl ext|dir <target> -s <-n|-l|-r>   Sort output (-n normal, -l by line asc, -r by line desc)".into());

    m.insert("cmd.tl.highest".into(), "highest: {}  {} lines".into());

    m.insert("cmd.tl.no_ext_files".into(), "No .{} files found".into());

    m.insert(
        "cmd.tl.no_supported_files".into(),
        "No supported files found in {}".into(),
    );

    m.insert("cmd.tl.not_a_file".into(), "Not a file: {}".into());

    m.insert("cmd.tl.sort_usage".into(), "Invalid sort flags. Use: -s -n (normal) | -s -l (by line, small to big) | -s -r (reverse, big to small).".into());

    m.insert(
        "cmd.tl.total_files_lines".into(),
        "total: {} files, {} lines".into(),
    );

    m.insert(
        "cmd.tl.unknown_sub".into(),
        "Unknown subcommand '{}'.".into(),
    );

    m.insert("cmd.tl.usage_title".into(), "Usage:".into());
}
