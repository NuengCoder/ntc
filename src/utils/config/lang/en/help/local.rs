use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.local.title".into(), "LOCAL COMMAND:".into());

    m.insert(
        "help.local.init".into(),
        "local init ## Create ntconfig.toml template (commented)".into(),
    );

    m.insert(
        "help.local.help".into(),
        "local help ## Show local command usage".into(),
    );

    m.insert(
        "help.local.deinit".into(),
        "local deinit ## Remove ntconfig.toml from current directory".into(),
    );

    m.insert(
        "help.local.example1".into(),
        "local init ## Create an ntconfig.toml template here".into(),
    );
}
