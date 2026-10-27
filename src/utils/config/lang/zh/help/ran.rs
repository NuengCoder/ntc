use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ran.deinit".into(),
        "ran deinit ## 删除 NTCRANFILE.toml".into(),
    );

    m.insert("help.ran.help".into(), "ran help ## 显示 ran 用法".into());

    m.insert(
        "help.ran.init".into(),
        "ran init ## 创建 NTCRANFILE.toml 模板".into(),
    );

    m.insert(
        "help.ran.list".into(),
        "ran list ## 列出 NTCRANFILE.toml 中的所有目标".into(),
    );

    m.insert(
        "help.ran.run".into(),
        "ran ## 运行 'standalone' 目标（默认）".into(),
    );

    m.insert(
        "help.ran.target".into(),
        "ran <目标> ## 从 NTCRANFILE.toml 运行指定目标".into(),
    );

    m.insert("help.ran.title".into(), "RAN / 任务运行器命令：".into());

    m.insert("help.ran.example1".into(), "ran ## 运行默认独立目标".into());

    m.insert(
        "help.ran.example2".into(),
        "ran build ## 运行 'build' 目标".into(),
    );

    m.insert(
        "help.ran.example3".into(),
        "ran list ## 列出所有目标".into(),
    );
}
