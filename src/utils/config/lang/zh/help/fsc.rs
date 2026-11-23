use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fsc.title".into(), "FSC 命令：".into());

    m.insert(
        "help.fsc.usage".into(),
        "fsc <模式> [-d <n>] [--no-fuzzy] ## 搜索文件，选择后显示并可选择复制".into(),
    );

    m.insert(
        "help.fsc.example1".into(),
        "fsc config ## 查找匹配的文件并显示".into(),
    );
}
