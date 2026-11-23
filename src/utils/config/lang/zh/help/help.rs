use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.help.title".into(), "HELP 命令：".into());

    m.insert("help.help.usage".into(), "help ## 显示此帮助".into());

    m.insert(
        "help.help.example1".into(),
        "help cd ## 显示单个命令的帮助".into(),
    );
    m.insert(
        "help.help.command".into(),
        "help <命令> ## 显示单个命令的说明".into(),
    );
    m.insert(
        "help.help.all".into(),
        "help --all ## 打印所有命令的说明".into(),
    );
}
