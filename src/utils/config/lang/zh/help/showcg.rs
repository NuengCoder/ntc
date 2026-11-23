use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showcg.title".into(), "SHOWCG 命令：".into());

    m.insert(
        "help.showcg.usage".into(),
        "showcg ## 显示当前配置概览".into(),
    );

    m.insert(
        "help.showcg.example1".into(),
        "showcg ## 显示当前配置概览".into(),
    );
}
