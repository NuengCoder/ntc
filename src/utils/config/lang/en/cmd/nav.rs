use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.nav.back.invalid_number".into(),
        "Invalid number: {}. Usage: back [n]".into(),
    );

    m.insert(
        "cmd.nav.back.invalid_step".into(),
        "Null parent at step {} - nowhere to go back".into(),
    );

    m.insert("cmd.nav.cd.example1".into(), "Example: cd C:\\Users".into());

    m.insert("cmd.nav.cd.example2".into(), " cd subdir".into());

    m.insert(
        "cmd.nav.cd.example3".into(),
        " cd to work # Teleport to 'work' savepoint".into(),
    );

    m.insert("cmd.nav.cd.navigated".into(), "Navigated to: {}".into());

    m.insert(
        "cmd.nav.cd.tp_list".into(),
        "Use 'tp list' to see all savepoints.".into(),
    );

    m.insert(
        "cmd.nav.cd.tp_not_found".into(),
        "Teleport point not found: '{}'".into(),
    );

    m.insert(
        "cmd.nav.cd.usage1".into(),
        "Usage: cd <directory_path>".into(),
    );

    m.insert(
        "cmd.nav.cd.usage2".into(),
        " cd to <tp_name> Teleport to savepoint".into(),
    );

    m.insert(
        "cmd.nav.drive_windows_only".into(),
        "Drive navigation is only supported on Windows.".into(),
    );

    m.insert("cmd.nav.fgo.cancel".into(), "0. Cancel".into());

    m.insert("cmd.nav.fgo.cancelled".into(), "Cancelled.".into());

    m.insert(
        "cmd.nav.fgo.example1".into(),
        " fgo main.rs # search then pick file to navigate to its parent".into(),
    );

    m.insert(
        "cmd.nav.fgo.example2".into(),
        " fgo main.rs -d 5 # search up to 5 levels deep".into(),
    );

    m.insert(
        "cmd.nav.fgo.found".into(),
        "\u{1f50d} Found {} file(s) for \"{}\":".into(),
    );

    m.insert(
        "cmd.nav.fgo.select".into(),
        "Select file to navigate to its parent directory: ".into(),
    );

    m.insert(
        "cmd.nav.fgo_usage".into(),
        "Usage: fgo <pattern> [-d <depth>]".into(),
    );

    m.insert("cmd.nav.gos.empty".into(), "(no subdirectories)".into());

    m.insert("cmd.nav.gos.exit".into(), "exit".into());

    m.insert("cmd.nav.gos.staying".into(), "Staying here.".into());

    m.insert(
        "cmd.nav.invalid_drive".into(),
        "Invalid drive number.".into(),
    );

    m.insert("cmd.nav.invalid_input".into(), "Invalid input.".into());

    m.insert("cmd.nav.invalid_number".into(), "Invalid number.".into());

    m.insert(
        "cmd.nav.invalid_selection".into(),
        "Invalid selection.".into(),
    );

    m.insert(
        "cmd.nav.where.config_file".into(),
        "\u{2699}\u{fe0f} Config file:".into(),
    );

    m.insert(
        "cmd.nav.where.config_missing".into(),
        "\u{26a0} Config file not found (will be created on first save)".into(),
    );

    m.insert(
        "cmd.nav.where.config_size".into(),
        "\u{1f4cf} Config size:".into(),
    );

    m.insert(
        "cmd.nav.where.current_dir".into(),
        "\u{1f4c2} Current dir:".into(),
    );

    m.insert(
        "cmd.nav.where.executable".into(),
        "\u{1f4c1} Executable:".into(),
    );

    m.insert(
        "cmd.nav.where.not_found".into(),
        "\u{2717} '{}' not found in PATH".into(),
    );

    m.insert(
        "cmd.nav.where.path_title".into(),
        "\u{1f50d} PATH matches for '{}':".into(),
    );

    m.insert("cmd.nav.where.title".into(), "ntc Location Info".into());
}
