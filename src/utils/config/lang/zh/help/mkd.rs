use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkd.title".into(), "MKD 命令：".into());

    m.insert(
        "help.mkd.usage".into(),
        "mkd <目录>[, ...] ## 创建新目录".into(),
    );

    m.insert(
        "help.mkd.example1".into(),
        "mkd src/components ## 创建目录".into(),
    );
}
