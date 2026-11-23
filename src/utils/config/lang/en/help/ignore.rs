use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ignore.title".into(), "IGNORE COMMAND:".into());

    m.insert(
        "help.ignore.dir".into(),
        "ignore dir <name>[, ...] ## Ignore directories by name".into(),
    );

    m.insert(
        "help.ignore.ext".into(),
        "ignore ext <ext>[, ...] ## Ignore file extensions".into(),
    );

    m.insert(
        "help.ignore.file".into(),
        "ignore file <name>[, ...] ## Ignore specific files".into(),
    );

    m.insert(
        "help.ignore.example1".into(),
        "ignore dir node_modules ## Ignore node_modules".into(),
    );

    m.insert(
        "help.ignore.example2".into(),
        "ignore ext .log ## Ignore .log files".into(),
    );
}
