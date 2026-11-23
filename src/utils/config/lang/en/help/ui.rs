use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ui.title".into(), "UI COMMAND:".into());

    m.insert("help.ui.usage".into(), "ui ## Show current UI mode".into());

    m.insert(
        "help.ui.example1".into(),
        "ui ## Show the current UI mode".into(),
    );
}
