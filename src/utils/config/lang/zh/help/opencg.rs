use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.opencg.title".into(), "OPENCG 命令：".into());

    m.insert(
        "help.opencg.usage".into(),
        "opencg ## 在编辑器中打开 config.toml".into(),
    );

    m.insert(
        "help.opencg.example1".into(),
        "opencg ## 在编辑器中编辑 config.toml".into(),
    );
}
