use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.dl.title".into(), "DL 命令：".into());

    m.insert(
        "help.dl.usage".into(),
        "dl <url> ## 下载文件（--resume, --stop, --history, --cancel）".into(),
    );

    m.insert(
        "help.dl.example1".into(),
        "dl https://example.com/file.zip ## 下载并显示进度".into(),
    );
}
