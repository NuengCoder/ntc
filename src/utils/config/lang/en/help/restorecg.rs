use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.restorecg.title".into(), "RESTORECG COMMAND:".into());

    m.insert(
        "help.restorecg.usage".into(),
        "restorecg ## Restore config from backup".into(),
    );

    m.insert(
        "help.restorecg.example1".into(),
        "restorecg ## Restore config from a backup".into(),
    );
}
