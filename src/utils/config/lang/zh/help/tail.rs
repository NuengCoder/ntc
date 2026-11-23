use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tail.title".into(), "TAIL 命令：".into());

    m.insert(
        "help.tail.usage".into(),
        "tail [N] [file] ## 打印后 N 行（默认 10）".into(),
    );

    m.insert(
        "help.tail.example1".into(),
        "tail 20 app.log ## app.log 最后 20 行".into(),
    );
}
