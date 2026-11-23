use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clip.title".into(), "剪贴板命令：".into());

    m.insert("help.clip.show".into(), "clip ## 显示剪贴板历史".into());

    m.insert(
        "help.clip.list".into(),
        "clip list ## 显示剪贴板历史".into(),
    );

    m.insert(
        "help.clip.select".into(),
        "clip select <N> ## 将条目N复制回剪贴板".into(),
    );

    m.insert(
        "help.clip.clear".into(),
        "clip clear ## 清除剪贴板历史".into(),
    );

    m.insert("help.clip.example1".into(), "clip ## 显示剪贴板历史".into());

    m.insert(
        "help.clip.example2".into(),
        "clip select 1 ## 将条目 #1 复制回剪贴板".into(),
    );
}
