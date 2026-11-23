use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ui.title".into(), "UI 命令：".into());

    m.insert("help.ui.usage".into(), "ui ## 显示当前UI模式".into());

    m.insert("help.ui.example1".into(), "ui ## 显示当前 UI 模式".into());
}
