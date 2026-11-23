use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmf.title".into(), "RNMF 命令：".into());

    m.insert(
        "help.rnmf.usage".into(),
        "rnmf <旧名称> <新名称> ## 重命名文件".into(),
    );

    m.insert(
        "help.rnmf.example1".into(),
        "rnmf a.txt b.txt ## 重命名文件".into(),
    );
}
