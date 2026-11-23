use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.dl.title".into(), "DL COMMAND:".into());

    m.insert(
        "help.dl.usage".into(),
        "dl <url> ## Download file (--resume, --stop, --history, --cancel)".into(),
    );

    m.insert(
        "help.dl.example1".into(),
        "dl https://example.com/file.zip ## Download with progress".into(),
    );
}
