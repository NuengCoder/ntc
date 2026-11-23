use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.exit.title".into(), "EXIT 命令：".into());

    m.insert("help.exit.usage".into(), "exit, quit ## 退出ntc".into());

    m.insert("help.exit.example1".into(), "exit ## 退出 ntc".into());
}
