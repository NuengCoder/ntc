use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.up.title".into(), "UP 命令：".into());

    m.insert(
        "help.up.usage".into(),
        "up <文件> <url> ## 上传文件（默认 multipart POST；--put, --raw, -a, --bearer, -F, -H, -d）".into(),
    );

    m.insert(
        "help.up.example1".into(),
        "up ./file.zip https://example.com/upload ## 上传文件".into(),
    );
}
