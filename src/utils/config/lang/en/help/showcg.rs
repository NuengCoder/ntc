use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showcg.title".into(), "SHOWCG COMMAND:".into());

    m.insert(
        "help.showcg.usage".into(),
        "showcg ## Show current configuration overview".into(),
    );

    m.insert(
        "help.showcg.example1".into(),
        "showcg ## Show current configuration overview".into(),
    );
}
