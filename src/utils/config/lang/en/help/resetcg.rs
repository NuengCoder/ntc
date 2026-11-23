use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.resetcg.title".into(), "RESETCG COMMAND:".into());

    m.insert(
        "help.resetcg.usage".into(),
        "resetcg ## Reset config to defaults (with backup option)".into(),
    );

    m.insert(
        "help.resetcg.example1".into(),
        "resetcg ## Reset config to defaults".into(),
    );
}
