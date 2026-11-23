use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.betw.title".into(), "BETW 命令：".into());

    m.insert(
        "help.betw.usage".into(),
        "betw A:B [file] ## 打印第 A 到 B 行（从 1 开始，含两端）".into(),
    );

    m.insert(
        "help.betw.example1".into(),
        "betw 10:20 app.log ## 显示 app.log 第 10-20 行".into(),
    );
}
