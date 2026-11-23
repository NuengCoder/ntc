use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ignore.title".into(), "IGNORE 命令：".into());

    m.insert(
        "help.ignore.dir".into(),
        "ignore dir <名称>[, ...] ## 按名称忽略目录".into(),
    );

    m.insert(
        "help.ignore.ext".into(),
        "ignore ext <扩展名>[, ...] ## 忽略文件扩展名".into(),
    );

    m.insert(
        "help.ignore.file".into(),
        "ignore file <名称>[, ...] ## 忽略特定文件".into(),
    );

    m.insert(
        "help.ignore.example1".into(),
        "ignore dir node_modules ## 忽略 node_modules".into(),
    );

    m.insert(
        "help.ignore.example2".into(),
        "ignore ext .log ## 忽略 .log 文件".into(),
    );
}
