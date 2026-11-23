use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.resetcg.title".into(), "RESETCG 命令：".into());

    m.insert(
        "help.resetcg.usage".into(),
        "resetcg ## 重置配置为默认值（可选备份）".into(),
    );

    m.insert(
        "help.resetcg.example1".into(),
        "resetcg ## 重置配置为默认值".into(),
    );
}
