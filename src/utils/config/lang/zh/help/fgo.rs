use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fgo.title".into(), "FGO 命令：".into());

    m.insert(
        "help.fgo.usage".into(),
        "fgo <模式> [-d <n>] ## 查找文件，导航到所选文件的父目录".into(),
    );

    m.insert(
        "help.fgo.example1".into(),
        "fgo *.rs ## 查找 Rust 文件并跳转".into(),
    );
}
