use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.wi.title".into(), "WI COMMAND:".into());

    m.insert(
        "help.wi.usage".into(),
        "wi [path] ## Live file-change feed (q=quit)".into(),
    );

    m.insert(
        "help.wi.example1".into(),
        "wi ## Watch file changes live".into(),
    );
}
