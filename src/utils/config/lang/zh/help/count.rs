use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.count.title".into(), "COUNT 命令：".into());

    m.insert(
        "help.count.usage".into(),
        "count <needle> [text] [-l|--line] ## 统计 <needle> 出现的次数；-l|--line 统计管道输入的总行数".into(),
    );

    m.insert(
        "help.count.example1".into(),
        "gs TODO | count TODO ## 统计管道文本中的匹配数".into(),
    );

    m.insert(
        "help.count.example2".into(),
        "gs TODO | count -l ## 统计管道输出的总行数".into(),
    );
}
