use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.arc.title".into(), "ARCHIVE COMMANDS:".into());

    m.insert(
        "help.arc.create".into(),
        "arc create <out.zip|tar.gz|tar> <files...> ## Create archive".into(),
    );

    m.insert(
        "help.arc.extract".into(),
        "arc extract <file> [dest] ## Extract archive".into(),
    );

    m.insert(
        "help.arc.list".into(),
        "arc list <file> ## List archive contents".into(),
    );

    m.insert(
        "help.arc.example1".into(),
        "arc create backup.zip src Cargo.toml ## Create zip".into(),
    );

    m.insert(
        "help.arc.example2".into(),
        "arc list backup.zip ## List contents".into(),
    );

    m.insert(
        "help.arc.example3".into(),
        "arc extract backup.zip restored ## Extract".into(),
    );
}
