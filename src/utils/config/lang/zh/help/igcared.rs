use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcared.title".into(), "IGCARED 命令：".into());

    m.insert(
        "help.igcared.usage".into(),
        "igcared ## 显示所有忽略/关注设置".into(),
    );

    m.insert(
        "help.igcared.example1".into(),
        "igcared ## 显示所有忽略/关注设置".into(),
    );
}
