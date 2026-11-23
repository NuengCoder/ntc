use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.local.title".into(), "LOCAL 命令：".into());

    m.insert(
        "help.local.init".into(),
        "local init ## 创建 ntconfig.toml 模板（已注释）".into(),
    );

    m.insert(
        "help.local.help".into(),
        "local help ## 显示本地命令用法".into(),
    );

    m.insert(
        "help.local.deinit".into(),
        "local deinit ## 从当前目录删除 ntconfig.toml".into(),
    );

    m.insert(
        "help.local.example1".into(),
        "local init ## 在当前目录创建 ntconfig.toml 模板".into(),
    );
}
