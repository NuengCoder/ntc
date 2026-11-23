use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showenv.title".into(), "SHOWENV 命令：".into());

    m.insert(
        "help.showenv.usage".into(),
        "showenv ## 显示环境变量（前5个PATH条目）".into(),
    );

    m.insert(
        "help.showenv.all".into(),
        "showenv --all ## 显示所有PATH条目".into(),
    );

    m.insert(
        "help.showenv.depth".into(),
        "showenv -d <n> ## 显示前n个PATH条目".into(),
    );

    m.insert(
        "help.showenv.example1".into(),
        "showenv --all ## 显示所有 PATH 条目".into(),
    );
}
