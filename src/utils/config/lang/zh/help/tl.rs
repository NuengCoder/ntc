use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.tl.dir".into(),
        "tl dir <路径> [-s <-n|-l|-r>] ## 显示目录中支持的扩展名和总行数（可排序）".into(),
    );

    m.insert(
        "help.tl.ext".into(),
        "tl ext <扩展名> [-s <-n|-l|-r>] ## 显示所有<扩展名>文件及其总行数（遵循igcare，可排序）"
            .into(),
    );

    m.insert(
        "help.tl.file".into(),
        "tl file <路径> ## 统计单个文件的行数".into(),
    );

    m.insert("help.tl.sort".into(), "tl ext|dir <目标> -s <-n|-l|-r> ## 排序：-n 普通、-l 按行数（从小到大）、-r 反向（从大到小）".into());

    m.insert("help.tl.title".into(), "代码行数统计命令：".into());

    m.insert(
        "help.tl.example1".into(),
        "tl file main.rs ## 统计 main.rs 的行数".into(),
    );

    m.insert(
        "help.tl.example2".into(),
        "tl ext .rs ## 统计所有 .rs 文件的行数".into(),
    );

    m.insert(
        "help.tl.example3".into(),
        "tl dir . ## 统计当前目录的行数".into(),
    );

    m.insert(
        "help.tl.example4".into(),
        "tl ext .rs -s -r ## 按行数从大到小列出 .rs 文件".into(),
    );
}
