use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ds.title".into(), "DS COMMAND:".into());

    m.insert(
        "help.ds.usage".into(),
        "ds <pattern> [-d <n>] [--no-fuzzy] ## Search directories by name (exact/partial only with --no-fuzzy)".into(),
    );

    m.insert(
        "help.ds.example1".into(),
        "ds src ## Find directories named 'src'".into(),
    );
}
