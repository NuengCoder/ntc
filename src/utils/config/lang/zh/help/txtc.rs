use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtc.title".into(), "TXTC 命令：".into());

    m.insert(
        "help.txtc.usage".into(),
        "txtc [--line|--no-line] <文件> ## 将文件内容复制到剪贴板（行号开关）".into(),
    );

    m.insert(
        "help.txtc.example1".into(),
        "txtc main.rs ## 将 main.rs 复制到剪贴板".into(),
    );
}
