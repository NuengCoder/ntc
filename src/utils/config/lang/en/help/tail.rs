use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tail.title".into(), "TAIL COMMAND:".into());

    m.insert(
        "help.tail.usage".into(),
        "tail [N] [file] ## Print the last N lines (default 10)".into(),
    );

    m.insert(
        "help.tail.example1".into(),
        "tail 20 app.log ## Last 20 lines of app.log".into(),
    );
}
