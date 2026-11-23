use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.version.title".into(), "VERSION COMMAND:".into());

    m.insert(
        "help.version.usage".into(),
        "version ## Show version".into(),
    );

    m.insert(
        "help.version.example1".into(),
        "version ## Show version info".into(),
    );
}
