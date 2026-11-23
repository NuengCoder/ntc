use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trim.title".into(), "TRIM 命令：".into());

    m.insert(
        "help.trim.usage".into(),
        "trim <pattern> [文本] ## 删除 <pattern> 的每一次出现（文本或管道输入）".into(),
    );

    m.insert(
        "help.trim.example1".into(),
        "echo nice ... bro ... | trim \" ...\" ## 从管道文本中删除模式（内联写法：trim \" ...\" \"nice ... bro ...\"）".into(),
    );
}
