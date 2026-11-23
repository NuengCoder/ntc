use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clear.title".into(), "CLEAR COMMAND:".into());

    m.insert("help.clear.usage".into(), "clear ## Clear screen".into());

    m.insert(
        "help.clear.example1".into(),
        "clear ## Clear the screen".into(),
    );
}
