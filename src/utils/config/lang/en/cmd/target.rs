use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.target.not_dir".into(),
        "'{}' is not a directory".into(),
    );

    m.insert("cmd.target.not_found".into(), "Path '{}' not found".into());

    m.insert(
        "cmd.target.tp_not_found".into(),
        "Savepoint '{}' not found".into(),
    );
}
