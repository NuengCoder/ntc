use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.arc.title".into(), "压缩包命令：".into());

    m.insert(
        "help.arc.create".into(),
        "arc create <out.zip|tar.gz|tar> <文件...> ## 创建压缩包".into(),
    );

    m.insert(
        "help.arc.extract".into(),
        "arc extract <文件> [目录] ## 解压压缩包".into(),
    );

    m.insert(
        "help.arc.list".into(),
        "arc list <文件> ## 列出压缩包内容".into(),
    );

    m.insert(
        "help.arc.example1".into(),
        "arc create backup.zip src Cargo.toml ## 创建 zip".into(),
    );

    m.insert(
        "help.arc.example2".into(),
        "arc list backup.zip ## 列出内容".into(),
    );

    m.insert(
        "help.arc.example3".into(),
        "arc extract backup.zip restored ## 解压".into(),
    );
}
