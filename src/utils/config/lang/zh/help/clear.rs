use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clear.title".into(), "CLEAR 命令：".into());

    m.insert("help.clear.usage".into(), "clear ## 清除屏幕".into());

    m.insert("help.clear.example1".into(), "clear ## 清除屏幕".into());
}
