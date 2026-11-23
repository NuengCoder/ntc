use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.opencg.title".into(), "OPENCG COMMAND:".into());

    m.insert(
        "help.opencg.usage".into(),
        "opencg ## Open config.toml in editor".into(),
    );

    m.insert(
        "help.opencg.example1".into(),
        "opencg ## Edit config.toml in the editor".into(),
    );
}
