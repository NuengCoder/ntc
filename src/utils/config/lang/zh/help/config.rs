use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.config.autosuggest".into(),
        "config autosuggest on|off ## 显示/设置自动建议（幽灵文本）".into(),
    );

    m.insert(
        "help.config.color".into(),
        "config color on|off ## 显示/设置颜色输出".into(),
    );

    m.insert(
        "help.config.depth".into(),
        "config depth [n] ## 显示/设置最大深度（1-20）".into(),
    );

    m.insert(
        "help.config.history".into(),
        "config history [on|off|路径] ## 显示/设置历史记录".into(),
    );

    m.insert(
        "help.config.lang".into(),
        "config lang [语言] ## 显示/设置界面语言".into(),
    );

    m.insert(
        "help.config.linenumbers".into(),
        "config linenumbers on|off ## 显示/设置行号".into(),
    );

    m.insert(
        "help.config.local_deinit".into(),
        "local deinit ## 从当前目录删除 ntconfig.toml".into(),
    );

    m.insert(
        "help.config.local_help".into(),
        "local help ## 显示本地命令用法".into(),
    );

    m.insert(
        "help.config.local_init".into(),
        "local init ## 创建 ntconfig.toml 模板（已注释）".into(),
    );

    m.insert(
        "help.config.opencg".into(),
        "opencg ## 在编辑器中打开 config.toml".into(),
    );

    m.insert(
        "help.config.output".into(),
        "config output [路径] ## 显示/设置输出路径".into(),
    );

    m.insert(
        "help.config.qsc".into(),
        "config qsc \"#c 0\" ## 设置QSC全局热键".into(),
    );

    m.insert(
        "help.config.qsc_off".into(),
        "config qsc off ## 禁用QSC热键".into(),
    );

    m.insert(
        "help.config.resetcg".into(),
        "resetcg ## 重置配置为默认值（可选备份）".into(),
    );

    m.insert(
        "help.config.restorecg".into(),
        "restorecg ## 从备份恢复配置".into(),
    );

    m.insert(
        "help.config.showcg".into(),
        "showcg ## 显示当前配置概览".into(),
    );

    m.insert(
        "help.config.showenv".into(),
        "showenv ## 显示环境变量（前5个PATH条目）".into(),
    );

    m.insert(
        "help.config.showenv_all".into(),
        "showenv --all ## 显示所有PATH条目".into(),
    );

    m.insert(
        "help.config.showenv_d".into(),
        "showenv -d <n> ## 显示前n个PATH条目".into(),
    );

    m.insert(
        "help.config.threads".into(),
        "config threads [n] ## 显示/设置线程数".into(),
    );

    m.insert("help.config.title".into(), "配置命令：".into());

    m.insert(
        "help.config.watch".into(),
        "config watch on|off ## 启用/禁用文件监视器".into(),
    );

    m.insert(
        "help.config.watch_trigger".into(),
        "config watch trigger <别名> ## 文件监视器触发时自动运行别名".into(),
    );

    m.insert(
        "help.config.watch_trigger_off".into(),
        "config watch trigger off ## 禁用自动运行".into(),
    );

    m.insert(
        "help.config.watch_limit".into(),
        "config watch limit <数量> ## 监视器更改框中显示的行数".into(),
    );

    m.insert(
        "help.config.wi".into(),
        "wi [路径] ## 实时文件更改流（q=退出）".into(),
    );

    m.insert(
        "help.config.example1".into(),
        "config output ~/Desktop ## 设置输出路径到桌面".into(),
    );

    m.insert(
        "help.config.example2".into(),
        "config depth 3 ## 设置最大深度为 3".into(),
    );

    m.insert(
        "help.config.example3".into(),
        "config color ON ## 启用彩色输出".into(),
    );

    m.insert(
        "help.config.example4".into(),
        "config threads 8 ## 设置线程数为 8".into(),
    );

    m.insert(
        "help.config.example5".into(),
        "showcg ## 显示当前配置概览".into(),
    );

    m.insert(
        "help.config.git".into(),
        "config git ON|OFF ## 启用/禁用 git 集成（提示符和符号）".into(),
    );

    m.insert(
        "help.config.dir_view".into(),
        "config dir_view ON|OFF ## 在shell中显示/隐藏目录树".into(),
    );

    m.insert(
        "help.config.update_check".into(),
        "config update_check ON|OFF ## 显示/隐藏每日新版本提醒（别名：uc）".into(),
    );

    m.insert("help.config.serve".into(), "config serve [服务] [--port-default 端口] [--bind-default 地址] ## serve 各服务默认值（别名：serve config）".into());

    m.insert(
        "help.config.intro_view".into(),
        "config intro_view ON|OFF ## 显示/隐藏启动 logo、ntpkg 与 watcher 提示（别名: iv）".into(),
    );
}
