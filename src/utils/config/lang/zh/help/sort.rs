use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sort.title".into(), "SORT 命令：".into());

    m.insert(
        "help.sort.usage".into(),
        "sort [-r] [-n] [-u] [-f] [file] ## 排序行（倒序/数值/去重/忽略大小写）".into(),
    );

    m.insert(
        "help.sort.example1".into(),
        "gs error | sort -u | head 10 ## 排序去重后的匹配，取前 10 行".into(),
    );
}
