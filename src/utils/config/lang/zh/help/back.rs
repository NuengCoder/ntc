use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.back.title".into(), "BACK 命令：".into());

    m.insert(
        "help.back.usage".into(),
        "back [n] ## 返回父目录（或n层）".into(),
    );

    m.insert("help.back.example1".into(), "back ## 返回父目录".into());
}
