use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rev.title".into(), "REV 命令：".into());

    m.insert(
        "help.rev.usage".into(),
        "rev [-u] [file] ## 反转每行的字符，或用 -u 上下翻转行顺序".into(),
    );

    m.insert(
        "help.rev.example1".into(),
        "rev notes.txt ## 反转每行的字符顺序".into(),
    );

    m.insert(
        "help.rev.example2".into(),
        "rev -u notes.txt ## 上下翻转：最后一行变第一行，第一行变最后一行".into(),
    );
}
