use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "msg.ignore.dir_added".into(),
        "Now ignoring directory: {}".into(),
    );

    m.insert(
        "msg.ignore.dir_removed".into(),
        "No longer ignoring directory: {}".into(),
    );

    m.insert(
        "msg.ignore.ext_added".into(),
        "Now ignoring .{} files".into(),
    );

    m.insert(
        "msg.ignore.ext_removed".into(),
        "Now caring about .{} files".into(),
    );

    m.insert(
        "msg.ignore.file_added".into(),
        "Now ignoring file: {}".into(),
    );

    m.insert(
        "msg.ignore.file_removed".into(),
        "Now caring about file: {}".into(),
    );

    m.insert("msg.ignored.dirs".into(), "Ignored directories: {}".into());

    m.insert(
        "msg.ignored.extensions".into(),
        "Ignored extensions: {}".into(),
    );

    m.insert(
        "msg.ignored.extra_extensions".into(),
        "Extra supported extensions: {}".into(),
    );

    m.insert(
        "msg.ignored.extra_files".into(),
        "Extra supported files: {}".into(),
    );

    m.insert("msg.ignored.files".into(), "Ignored files: {}".into());

    m.insert(
        "msg.image.converted".into(),
        "Image converted and saved to: {}".into(),
    );

    m.insert("msg.image.not_found".into(), "Image not found: {}".into());

    m.insert(
        "msg.init.template_created".into(),
        "Created template: {}".into(),
    );

    m.insert("msg.list.entry_format".into(), " {}".into());

    m.insert(
        "msg.list.title".into(),
        "ntc {} - Available command-line functions:\n".into(),
    );

    m.insert(
        "msg.report.binary_clipboard_warning".into(),
        "{} report cannot be copied to clipboard (binary format)".into(),
    );

    m.insert(
        "msg.report.copied_to_clipboard".into(),
        "{} report copied to clipboard!".into(),
    );

    m.insert("msg.report.file_saved".into(), "File saved to: {}".into());

    m.insert(
        "msg.report.path_not_found".into(),
        "Path not found: {}".into(),
    );

    m.insert(
        "msg.report.skipped_unsupported".into(),
        "Skipped (not support format): {}".into(),
    );

    m.insert(
        "msg.setA.error".into(),
        "Invalid value for setA: {}. Use ON or OFF.".into(),
    );

    m.insert("msg.setA.off".into(), "Autosuggest: OFF".into());

    m.insert("msg.setA.on".into(), "Autosuggest: ON".into());

    m.insert(
        "msg.setA.status".into(),
        "Autosuggest (ghost text): {}".into(),
    );

    m.insert(
        "msg.setC.error".into(),
        "Invalid value for setC: {}. Use ON or OFF.".into(),
    );

    m.insert("msg.setC.status".into(), "Color output: {}".into());

    m.insert("msg.setC.success".into(), "Color: {}".into());

    m.insert(
        "msg.setD.error_out_of_range".into(),
        "Invalid depth value: {}. Must be between 1 and 20.".into(),
    );

    m.insert(
        "msg.setD.error_parse".into(),
        "Invalid depth value: {}. Must be a positive integer between 1 and 20.".into(),
    );

    m.insert("msg.setD.status".into(), "Current max depth: {}".into());

    m.insert("msg.setD.success".into(), "Max depth set to: {}".into());

    m.insert("msg.setH.off".into(), "History: OFF".into());

    m.insert("msg.setH.on".into(), "History: ON".into());

    m.insert("msg.setH.path".into(), "History path: {}".into());

    m.insert(
        "msg.setH.path_default".into(),
        "History path: default".into(),
    );

    m.insert(
        "msg.setH.path_reset".into(),
        "History path reset to default".into(),
    );

    m.insert("msg.setH.path_set".into(), "History path set to: {}".into());

    m.insert("msg.setH.status".into(), "History: {}".into());

    m.insert(
        "msg.setL.error".into(),
        "Invalid value for setL: {}. Use ON or OFF.".into(),
    );

    m.insert("msg.setL.status".into(), "Line numbers: {}".into());

    m.insert("msg.setL.success".into(), "Line numbers: {}".into());

    m.insert("msg.setO.status".into(), "Current output path: {}".into());

    m.insert("msg.setO.success".into(), "Output path set to: {}".into());

    m.insert(
        "msg.setT.error_out_of_range".into(),
        "Invalid thread count: {}. Must be between 1 and 64.".into(),
    );

    m.insert(
        "msg.setT.error_parse".into(),
        "Invalid thread count: {}. Must be a positive integer between 1 and 64.".into(),
    );

    m.insert("msg.setT.status".into(), "Current threads: {}".into());

    m.insert("msg.setT.success".into(), "Threads set to: {}".into());

    m.insert("msg.showcg.box_bottom".into(), "\u{2514}{}\u{2518}".into());

    m.insert("msg.showcg.box_divider".into(), "\u{251C}{}\u{2524}".into());

    m.insert("msg.showcg.box_top".into(), "\u{250C}{}\u{2510}".into());

    m.insert("msg.showcg.label_color".into(), "Color:".into());

    m.insert("msg.showcg.label_history".into(), "History:".into());

    m.insert(
        "msg.showcg.label_line_numbers".into(),
        "Line Numbers:".into(),
    );

    m.insert("msg.showcg.label_max_depth".into(), "Max Depth:".into());

    m.insert("msg.showcg.label_output_path".into(), "Output Path:".into());

    m.insert("msg.showcg.label_threads".into(), "Threads:".into());

    m.insert("msg.showcg.label_watcher".into(), "Watcher:".into());

    m.insert("msg.showcg.label_watch_limit".into(), "Watch limit:".into());

    m.insert("msg.showcg.label_dir_view".into(), "Dir View:".into());

    m.insert("msg.showcg.label_file_view".into(), "File View:".into());

    m.insert("msg.showcg.label_auto_size".into(), "Auto Size:".into());

    m.insert(
        "msg.showcg.auto_size_value".into(),
        "files {}, dirs {}".into(),
    );

    m.insert("msg.showcg.label_serve_files".into(), "Serve files:".into());

    m.insert("msg.showcg.label_serve_http".into(), "Serve http:".into());

    m.insert("msg.showcg.label_serve_chess".into(), "Serve chess:".into());

    m.insert("msg.showcg.title".into(), "Current Configuration".into());

    m.insert("msg.size.bytes_label".into(), "Bytes:".into());

    m.insert("msg.size.human_label".into(), "Human:".into());

    m.insert("msg.size.path_label".into(), "Path: {}".into());

    m.insert("msg.size.time_label".into(), "Time:".into());

    m.insert("msg.size.title".into(), "Current Directory Size".into());

    m.insert(
        "msg.teleport.error".into(),
        "Failed to teleport to '{}': {}".into(),
    );

    m.insert(
        "msg.teleport.hint_list".into(),
        "Use 'ntc --tp-list' to see all savepoints.".into(),
    );

    m.insert(
        "msg.teleport.not_found".into(),
        "Teleport point not found: '{}'".into(),
    );

    m.insert(
        "msg.teleport.success".into(),
        "\u{2713} Teleported to '{}' -> {}".into(),
    );

    m.insert("msg.version.format".into(), "ntc {}".into());

    m.insert(
        "msg.watch.invalid".into(),
        "Use --watch ON or --watch OFF".into(),
    );

    m.insert(
        "msg.watch.off".into(),
        "File watcher: OFF (restart ntc to deactivate)".into(),
    );

    m.insert(
        "msg.watch.on".into(),
        "File watcher: ON (restart ntc to activate)".into(),
    );

    m.insert("msg.watch.status".into(), "File watcher: {}".into());

    m.insert(
        "msg.watch.limit_status".into(),
        "Watch display limit: {}".into(),
    );

    m.insert(
        "msg.watch.limit_success".into(),
        "Watch display limit set to {}".into(),
    );

    m.insert(
        "msg.watch.limit_error".into(),
        "Invalid watch limit: '{}'. Use a positive number.".into(),
    );

    m.insert(
        "msg.where.current_dir".into(),
        "Current directory: {}".into(),
    );

    m.insert("msg.where.executable".into(), "ntc executable: {}".into());

    m.insert("msg.setUC.status".into(), "Update check: {}".into());

    m.insert(
        "msg.setUC.on".into(),
        "Update check enabled (daily new-release notice).".into(),
    );

    m.insert("msg.setUC.off".into(), "Update check disabled.".into());

    m.insert(
        "msg.setUC.error".into(),
        "Usage: config update_check ON|OFF".into(),
    );

    m.insert(
        "msg.showcg.label_update_check".into(),
        "Update check".into(),
    );

    m.insert(
        "msg.update.available".into(),
        "New NTC {} available (you have {}) — run 'ntc --upgrade' in {} or see {}".into(),
    );

    m.insert("msg.setGit.status".into(), "Git integration is: {}".into());

    m.insert("msg.setGit.on".into(), "Git integration enabled.".into());

    m.insert("msg.setGit.off".into(), "Git integration disabled.".into());

    m.insert("msg.setGit.error".into(), "Usage: config git ON|OFF".into());

    m.insert("msg.setIV.status".into(), "Intro view is: {}".into());

    m.insert(
        "msg.setIV.on".into(),
        "Intro view enabled (logo, ntpkg and watcher notices).".into(),
    );

    m.insert(
        "msg.setIV.off".into(),
        "Intro view disabled (no logo, no ntpkg or watcher notices).".into(),
    );

    m.insert(
        "msg.setIV.error".into(),
        "Usage: config intro_view ON|OFF".into(),
    );

    m.insert("msg.showcg.label_intro_view".into(), "Intro View:".into());

    m.insert("msg.setDV.status".into(), "Dir view is: {}".into());

    m.insert("msg.setDV.on".into(), "Dir view enabled.".into());

    m.insert("msg.setDV.off".into(), "Dir view disabled.".into());

    m.insert(
        "msg.setDV.error".into(),
        "Usage: config dir_view ON|OFF".into(),
    );

    m.insert("msg.setFV.status".into(), "File view is: {}".into());

    m.insert(
        "msg.setFV.on".into(),
        "File view enabled. Files are shown after directory changes.".into(),
    );

    m.insert("msg.setFV.off".into(), "File view disabled.".into());

    m.insert(
        "msg.setFV.error".into(),
        "Usage: config file_view ON|OFF".into(),
    );

    m.insert("msg.setTD.status".into(), "Todo format is: {}".into());

    m.insert(
        "msg.setTD.set".into(),
        "Todo format set to {} (file: future_todo.{}).".into(),
    );

    m.insert(
        "msg.setTD.error".into(),
        "Usage: config todo format <txt|md|json|toml|yaml|html>".into(),
    );

    m.insert(
        "msg.setSZ.status".into(),
        "Auto size: files {}, dirs {}".into(),
    );

    m.insert(
        "msg.setSZ.target_status".into(),
        "Auto size for {} is: {}".into(),
    );

    m.insert("msg.setSZ.on".into(), "Auto size enabled for {}.".into());

    m.insert("msg.setSZ.off".into(), "Auto size disabled for {}.".into());

    m.insert(
        "msg.setSZ.error".into(),
        "Invalid size setting. Usage: config size all|file|dir ON|OFF".into(),
    );

    m.insert(
        "msg.setSZ.slow_warn".into(),
        "Warning: auto size uses --care; every cd/teleport may be much slower.".into(),
    );

    m.insert(
        "msg.setSZ.usage".into(),
        "Usage: config size all|file|dir ON|OFF".into(),
    );
}
