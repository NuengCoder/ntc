use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkf.title".into(), "MKF COMMAND:".into());

    m.insert(
        "help.mkf.usage".into(),
        "mkf <file>[, ...] ## Create a new file".into(),
    );

    m.insert(
        "help.mkf.example1".into(),
        "mkf index.html ## Create a new file".into(),
    );
}
