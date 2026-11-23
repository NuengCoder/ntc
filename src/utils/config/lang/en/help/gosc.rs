use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gosc.title".into(), "GOSC COMMAND:".into());

    m.insert(
        "help.gosc.usage".into(),
        "gosc ## Continuous navigation (0 to exit, v/s/vs for view)".into(),
    );

    m.insert(
        "help.gosc.example1".into(),
        "gosc ## Navigate subdirectories continuously".into(),
    );
}
