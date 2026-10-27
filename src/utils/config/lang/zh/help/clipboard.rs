use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.clipboard.clear".into(),
        "clip clear ## 清除剪贴板历史".into(),
    );

    m.insert(
        "help.clipboard.list".into(),
        "clip list ## 显示剪贴板历史".into(),
    );

    m.insert(
        "help.clipboard.pathcp".into(),
        "pathcp ## 将当前目录路径复制到剪贴板".into(),
    );

    m.insert(
        "help.clipboard.select".into(),
        "clip select <N> ## 将条目N复制回剪贴板".into(),
    );

    m.insert(
        "help.clipboard.show".into(),
        "clip ## 显示剪贴板历史".into(),
    );

    m.insert("help.clipboard.title".into(), "剪贴板命令：".into());

    m.insert(
        "help.clipboard.example1".into(),
        "clip ## 显示剪贴板历史".into(),
    );

    m.insert(
        "help.clipboard.example2".into(),
        "clip select 1 ## 将条目 #1 复制回剪贴板".into(),
    );

    m.insert(
        "help.clipboard.example3".into(),
        "pathcp ## 复制当前目录路径".into(),
    );
}
