use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fs.title".into(), "FS COMMAND:".into());

    m.insert(
        "help.fs.usage".into(),
        "fs <pattern> [-d <n>] [--no-fuzzy] ## Search files by name (fuzzy fallback unless --no-fuzzy)".into(),
    );

    m.insert(
        "help.fs.example1".into(),
        "fs *.rs ## Find .rs files".into(),
    );
}
