use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gos.title".into(), "GOS COMMAND:".into());

    m.insert(
        "help.gos.usage".into(),
        "gos ## List subdirectories and pick one".into(),
    );

    m.insert(
        "help.gos.example1".into(),
        "gos ## Interactive subdirectory selection".into(),
    );
}
