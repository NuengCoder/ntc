use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmd.title".into(), "RNMD 命令：".into());

    m.insert(
        "help.rnmd.usage".into(),
        "rnmd <旧名称> <新名称> ## 重命名目录".into(),
    );

    m.insert(
        "help.rnmd.example1".into(),
        "rnmd src lib ## 重命名目录".into(),
    );
}
