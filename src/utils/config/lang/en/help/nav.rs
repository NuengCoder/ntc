use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.nav.back".into(),
        "back [n] ## Go back to parent directory (or n levels)".into(),
    );

    m.insert(
        "help.nav.cd".into(),
        "cd <path> ## Navigate to a directory".into(),
    );

    m.insert(
        "help.nav.cd_tp".into(),
        "cd to <tp_name> ## Teleport to a saved teleport point".into(),
    );

    m.insert(
        "help.nav.fgo".into(),
        "fgo <pattern> [-d <n>] ## Find files, navigate to selected file's parent dir".into(),
    );

    m.insert(
        "help.nav.godrive".into(),
        "godrive ## List and select Windows drives".into(),
    );

    m.insert(
        "help.nav.gos".into(),
        "gos ## List subdirectories and pick one".into(),
    );

    m.insert(
        "help.nav.gosc".into(),
        "gosc ## Continuous navigation (0 to exit, v/s/vs for view)".into(),
    );

    m.insert("help.nav.title".into(), "NAVIGATION COMMANDS:".into());

    m.insert(
        "help.nav.where".into(),
        "where [<binary>] ## Show ntc locations, or find a binary on PATH".into(),
    );

    m.insert(
        "help.nav.example1".into(),
        "cd src ## Navigate to the src directory".into(),
    );

    m.insert(
        "help.nav.example2".into(),
        "back ## Go back to parent directory".into(),
    );

    m.insert(
        "help.nav.example3".into(),
        "gos ## Interactive subdirectory selection".into(),
    );

    m.insert(
        "help.nav.example4".into(),
        "cd to web ## Teleport to 'web' savepoint".into(),
    );

    m.insert(
        "help.nav.example5".into(),
        "where git ## Show where a binary lives on PATH".into(),
    );
}
