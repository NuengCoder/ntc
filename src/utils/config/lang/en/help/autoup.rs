use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.autoup.add".into(),
        "autoup add <name> \"<cmd>\" ## Add a startup entry to HKCU".into(),
    );

    m.insert(
        "help.autoup.disable".into(),
        "autoup disable <name> ## Disable entry (uses StartupApproved)".into(),
    );

    m.insert(
        "help.autoup.enable".into(),
        "autoup enable <name> ## Re-enable a disabled entry".into(),
    );

    m.insert(
        "help.autoup.list".into(),
        "autoup list ## List all startup entries (CU/LM/SF sources)".into(),
    );

    m.insert(
        "help.autoup.rm".into(),
        "autoup rm <name> ## Remove a startup entry".into(),
    );

    m.insert(
        "help.autoup.sources".into(),
        "[CU]=HKCU [LM]=HKLM [SF]=Startup Folder".into(),
    );

    m.insert(
        "help.autoup.title".into(),
        "STARTUP MANAGER (Windows):".into(),
    );

    m.insert(
        "help.autoup.example1".into(),
        "autoup list ## List startup entries".into(),
    );

    m.insert(
        "help.autoup.example2".into(),
        "autoup add ntc \"C:\\ntc\\ntc.exe\" ## Add to startup".into(),
    );

    m.insert(
        "help.autoup.example3".into(),
        "autoup rm ntc ## Remove startup entry".into(),
    );

    m.insert(
        "help.autoup.example4".into(),
        "autoup disable ntc ## Disable startup entry".into(),
    );
}
