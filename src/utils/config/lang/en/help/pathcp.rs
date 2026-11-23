use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.pathcp.title".into(), "PATHCP COMMAND:".into());

    m.insert(
        "help.pathcp.usage".into(),
        "pathcp ## Copy current directory path to clipboard".into(),
    );

    m.insert(
        "help.pathcp.example1".into(),
        "pathcp ## Copy current directory path".into(),
    );
}
