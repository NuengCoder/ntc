use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.line.title".into(), "LINE 命令：".into());

    m.insert(
        "help.line.usage".into(),
        "line [-A <前缀>] [-B <分隔符>] [文件] ## 为每行编号（默认 '1: 行'）".into(),
    );

    m.insert(
        "help.line.example1".into(),
        "gs main | line -A 'line' -B ':' ## 在每行前添加 line1: ...".into(),
    );
}
