use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmd.title".into(), "RNMD COMMAND:".into());

    m.insert(
        "help.rnmd.usage".into(),
        "rnmd <old> <new> ## Rename a directory".into(),
    );

    m.insert(
        "help.rnmd.example1".into(),
        "rnmd src lib ## Rename a directory".into(),
    );
}
