use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fs.title".into(), "FS 命令：".into());

    m.insert(
        "help.fs.usage".into(),
        "fs <模式> [-d <n>] [--no-fuzzy] ## 按名称搜索文件（默认模糊匹配，--no-fuzzy 关闭）".into(),
    );

    m.insert("help.fs.example1".into(), "fs *.rs ## 查找 .rs 文件".into());
}
