use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tutorial.title".into(), "TUTORIAL 命令：".into());

    m.insert(
        "help.tutorial.usage".into(),
        "tutorial [课程] ## 在沙盒中动手实践的课程 (cd, view, ne, ral, ran)".into(),
    );

    m.insert(
        "help.tutorial.example1".into(),
        "tutorial ## 启动交互式教程".into(),
    );
}
