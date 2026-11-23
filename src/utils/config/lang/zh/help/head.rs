use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.head.title".into(), "HEAD 命令：".into());

    m.insert(
        "help.head.usage".into(),
        "head [N] [file] ## 打印前 N 行（默认 10）".into(),
    );

    m.insert(
        "help.head.example1".into(),
        "head 20 app.log ## 文件（或管道输入）的前 20 行".into(),
    );
}
