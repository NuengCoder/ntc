use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmf.title".into(), "RNMF COMMAND:".into());

    m.insert(
        "help.rnmf.usage".into(),
        "rnmf <old> <new> ## Rename a file".into(),
    );

    m.insert(
        "help.rnmf.example1".into(),
        "rnmf a.txt b.txt ## Rename a file".into(),
    );
}
