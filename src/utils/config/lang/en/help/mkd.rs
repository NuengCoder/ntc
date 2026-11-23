use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkd.title".into(), "MKD COMMAND:".into());

    m.insert(
        "help.mkd.usage".into(),
        "mkd <dir>[, ...] ## Create a new directory".into(),
    );

    m.insert(
        "help.mkd.example1".into(),
        "mkd src/components ## Create directories".into(),
    );
}
