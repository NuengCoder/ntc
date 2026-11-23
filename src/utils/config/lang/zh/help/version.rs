use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.version.title".into(), "VERSION 命令：".into());

    m.insert("help.version.usage".into(), "version ## 显示版本".into());

    m.insert(
        "help.version.example1".into(),
        "version ## 显示版本信息".into(),
    );
}
