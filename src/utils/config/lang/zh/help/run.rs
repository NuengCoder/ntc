use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.run.title".into(), "RUN 命令：".into());

    m.insert(
        "help.run.usage".into(),
        "run <别名> ## 使用 'run' 执行别名".into(),
    );

    m.insert("help.run.direct".into(), "<别名> ## 直接执行别名".into());

    m.insert(
        "help.run.param".into(),
        "<别名>(<参数>) ## 执行带参数的别名".into(),
    );

    m.insert("help.run.example1".into(), "build ## 直接运行别名".into());
}
