use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.care.title".into(), "CARE COMMAND:".into());

    m.insert(
        "help.care.care_dir".into(),
        "care dir <name>[, ...] ## Stop ignoring directories".into(),
    );

    m.insert(
        "help.care.care_ext".into(),
        "care ext <ext>[, ...] ## Add extra supported extensions".into(),
    );

    m.insert(
        "help.care.care_file".into(),
        "care file <name>[, ...] ## Add extra supported files".into(),
    );

    m.insert(
        "help.care.example1".into(),
        "care dir src ## Un-ignore src directory".into(),
    );
}
