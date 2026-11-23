use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcared.title".into(), "IGCARED COMMAND:".into());

    m.insert(
        "help.igcared.usage".into(),
        "igcared ## Show all ignore/care settings".into(),
    );

    m.insert(
        "help.igcared.example1".into(),
        "igcared ## Show all ignore/care settings".into(),
    );
}
