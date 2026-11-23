use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.say.title".into(), "SAY 命令：".into());

    m.insert(
        "help.say.usage".into(),
        "say <文本> [--loop N] [-n|-t] ## 打印文本（无需文件重定向）".into(),
    );

    m.insert(
        "help.say.example1".into(),
        "say \"hello\" ## 打印文本".into(),
    );
}
