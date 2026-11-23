use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.head.title".into(), "HEAD COMMAND:".into());

    m.insert(
        "help.head.usage".into(),
        "head [N] [file] ## Print the first N lines (default 10)".into(),
    );

    m.insert(
        "help.head.example1".into(),
        "head 20 app.log ## First 20 lines of a file (or piped input)".into(),
    );
}
