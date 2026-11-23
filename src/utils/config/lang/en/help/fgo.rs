use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fgo.title".into(), "FGO COMMAND:".into());

    m.insert(
        "help.fgo.usage".into(),
        "fgo <pattern> [-d <n>] ## Find files, navigate to selected file's parent dir".into(),
    );

    m.insert(
        "help.fgo.example1".into(),
        "fgo *.rs ## Find Rust files and jump to one".into(),
    );
}
