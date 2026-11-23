use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.locate.title".into(), "LOCATE 命令：".into());

    m.insert(
        "help.locate.usage".into(),
        "locate <模式> [-d <n>] [--no-fuzzy] ## 同时搜索文件和目录".into(),
    );

    m.insert(
        "help.locate.example1".into(),
        "locate README ## 搜索文件与目录中的 'README'".into(),
    );
}
