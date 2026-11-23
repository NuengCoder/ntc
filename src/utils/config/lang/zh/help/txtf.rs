use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtf.title".into(), "TXTF 命令：".into());

    m.insert(
        "help.txtf.usage".into(),
        "txtf [--head N|--tail N|--betw A:B] [文件|目录|@tp/文件|@tp/目录] ## 显示文件内容或从目录中选择文件（保留原始行号）".into(),
    );

    m.insert(
        "help.txtf.example1".into(),
        "txtf main.rs --head 30 ## 显示 main.rs 前 30 行".into(),
    );

    m.insert(
        "help.txtf.example2".into(),
        "txtf --betw 10:20 main.rs ## 显示 main.rs 第 10-20 行".into(),
    );

    m.insert(
        "help.txtf.example3".into(),
        "txtf @docs/notes.txt ## 显示传送点中的文件".into(),
    );

    m.insert(
        "help.txtf.example4".into(),
        "txtf src ## 从 src 目录中选择一个支持的文件显示".into(),
    );
}
