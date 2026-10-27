use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.views.title".into(),
        "VIEWS 命令（简短平铺列表）：".into(),
    );

    m.insert(
        "help.views.usage".into(),
        "views [<dir|@tp|@tp/dir/>] ## 像 ls 一样平铺列出单层目录".into(),
    );

    m.insert(
        "help.views.size".into(),
        "views -s, --size [最小大小] ## 显示大小；带值时（1024、10KB、5MB）隐藏更小的项".into(),
    );

    m.insert(
        "help.views.sort".into(),
        "views --sort <name|ext|size> ## 排序条目（默认：name）".into(),
    );

    m.insert(
        "help.views.newline".into(),
        "views -n ## 每行一个条目，而不是流动分组".into(),
    );

    m.insert(
        "help.views.note".into(),
        "没有 [Directory]/[ignored] 标签 —— 颜色表示类型。深度固定为 1，大小始终按 --care 语义计算。使用 --size 显示大小。".into(),
    );

    m.insert(
        "help.views.example1".into(),
        "views ## 平铺列出当前目录".into(),
    );

    m.insert(
        "help.views.example2".into(),
        "views src --size 1KB --sort size ## 大于等于 1KB 的条目，最大在前，显示大小".into(),
    );
}
