use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.skip.title".into(), "SKIP 命令：".into());

    m.insert(
        "help.skip.usage1".into(),
        "skip [N] [file] ## 打印前 N 行之后的全部内容（默认 10）".into(),
    );

    m.insert(
        "help.skip.usage2".into(),
        "skip -x <lines> [file] ## 打印除指定行号（从 1 开始）之外的所有行".into(),
    );

    m.insert(
        "help.skip.note".into(),
        "行号可用逗号或空格分隔（2,5,8 或 2 5 8）；输入来自文件或管道".into(),
    );

    m.insert(
        "help.skip.example1".into(),
        "skip 2 app.log ## 打印 app.log 但去掉前 2 行".into(),
    );

    m.insert(
        "help.skip.example2".into(),
        "gs TODO | skip -x 2,5,8 ## 跳过搜索结果中的第 2、5、8 行".into(),
    );
}
