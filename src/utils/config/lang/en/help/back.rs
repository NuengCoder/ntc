use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.back.title".into(), "BACK COMMAND:".into());

    m.insert(
        "help.back.usage".into(),
        "back [n] ## Go back to parent directory (or n levels)".into(),
    );

    m.insert(
        "help.back.example1".into(),
        "back ## Go back to parent directory".into(),
    );
}
