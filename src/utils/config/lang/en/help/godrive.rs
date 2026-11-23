use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.godrive.title".into(), "GODRIVE COMMAND:".into());

    m.insert(
        "help.godrive.usage".into(),
        "godrive ## List and select drives (Windows) or /mnt mounts (Linux)".into(),
    );

    m.insert(
        "help.godrive.example1".into(),
        "godrive ## Pick a drive or mount".into(),
    );

    m.insert(
        "help.godrive.example2".into(),
        "godrive c ## Jump to drive C: on Windows or /mnt/c on Linux".into(),
    );
}
