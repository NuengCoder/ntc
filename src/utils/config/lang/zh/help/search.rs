use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.search.ds".into(),
        "ds <模式> [-d <n>] ## 按名称搜索目录".into(),
    );

    m.insert(
        "help.search.fs".into(),
        "fs <模式> [-d <n>] ## 按名称搜索文件（模糊匹配）".into(),
    );

    m.insert(
        "help.search.fsc".into(),
        "fsc <模式> [-d <n>] ## 搜索文件，选择后显示并可选择复制".into(),
    );

    m.insert("help.search.gs".into(), "gs <模式> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-i] ## 搜索文件内容（简单grep）".into());

    m.insert(
        "help.search.gs_interactive".into(),
        "gs -i <模式> [-d <n>] ## 交互式grep（每页10条，Enter打开）".into(),
    );

    m.insert(
        "help.search.locate".into(),
        "locate <模式> [-d <n>] ## 同时搜索文件和目录".into(),
    );

    m.insert(
        "help.search.pipe".into(),
        "cmd | gs <模式> ## 搜索管道输入的内容（也支持 qur -s <模式>）".into(),
    );

    m.insert("help.search.title".into(), "搜索命令：".into());

    m.insert(
        "help.search.example1".into(),
        "gs fn main ## 在文件中搜索 'fn main'".into(),
    );

    m.insert(
        "help.search.example2".into(),
        "gs -i config ## 交互式搜索结果".into(),
    );

    m.insert(
        "help.search.example3".into(),
        "fs *.rs ## 查找 .rs 文件".into(),
    );

    m.insert(
        "help.search.example4".into(),
        "ds src ## 查找名为 'src' 的目录".into(),
    );

    m.insert(
        "help.search.example5".into(),
        "locate README ## 搜索文件与目录中的 'README'".into(),
    );

    m.insert(
        "help.search.example6".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + 类似 grep 的显示".into(),
    );

    m.insert(
        "help.search.example7".into(),
        "gs TODO < app.log ## 搜索管道/重定向输入的内容".into(),
    );

    m.insert(
        "help.search.example8".into(),
        "gs error\\|warning ## 或搜索：匹配任意一个词 (a\\|b)".into(),
    );
}
