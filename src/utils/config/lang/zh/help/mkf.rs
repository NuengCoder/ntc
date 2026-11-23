use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkf.title".into(), "MKF 命令：".into());

    m.insert(
        "help.mkf.usage".into(),
        "mkf <文件>[, ...] ## 创建新文件".into(),
    );

    m.insert(
        "help.mkf.example1".into(),
        "mkf index.html ## 创建新文件".into(),
    );
}
