use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.config.title".into(), "配置命令：".into());

    m.insert(
        "help.config.output".into(),
        "config output [路径] ## 显示/设置输出路径".into(),
    );

    m.insert(
        "help.config.depth".into(),
        "config depth [n] ## 显示/设置最大深度（1-20）".into(),
    );

    m.insert(
        "help.config.linenumbers".into(),
        "config linenumbers on|off ## 显示/设置行号".into(),
    );

    m.insert(
        "help.config.color".into(),
        "config color on|off ## 显示/设置颜色输出".into(),
    );

    m.insert(
        "help.config.threads".into(),
        "config threads [n] ## 显示/设置线程数".into(),
    );

    m.insert(
        "help.config.history".into(),
        "config history [on|off|路径] ## 显示/设置历史记录".into(),
    );

    m.insert(
        "help.config.autosuggest".into(),
        "config autosuggest on|off ## 显示/设置自动建议（幽灵文本）".into(),
    );

    m.insert(
        "help.config.idle".into(),
        "config idle on|off ## 显示/设置空闲幽灵输入提示".into(),
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
        "help.config.intro_view".into(),
        "config intro_view ON|OFF ## 显示/隐藏启动 logo、ntpkg 与 watcher 提示（别名: iv）".into(),
    );

    m.insert(
        "help.config.file_view".into(),
        "config file_view ON|OFF ## 在自动导航树中包含文件（别名：fv）".into(),
    );

    m.insert(
        "help.config.todo".into(),
        "config todo format <txt|md|json|toml|yaml|html> ## 设置待办文件格式".into(),
    );

    m.insert(
        "help.config.size".into(),
        "config size all|file|dir ON|OFF ## 在导航树中自动计算大小；使用 --care（别名：s）".into(),
    );

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
        "help.config.qsc".into(),
        "config qsc \"#c 0\" ## 设置QSC全局热键".into(),
    );

    m.insert(
        "help.config.qsc_off".into(),
        "config qsc off ## 禁用QSC热键".into(),
    );

    m.insert(
        "help.config.lang".into(),
        "config lang [语言] ## 显示/设置界面语言".into(),
    );

    m.insert(
        "help.config.env".into(),
        "config env NAME value [-g|-l|-os|-osg] ## 设置 NTC 环境变量（支持 NAME=value；不带参数：列出；仅名称：显示）".into(),
    );

    m.insert(
        "help.config.env_os".into(),
        "  -g 全局 | -l 本地 | -os 当前会话 | -osg 持久化到操作系统 ## 默认：存在 ntconfig.toml 时为本地，否则全局".into(),
    );

    m.insert(
        "help.config.rmenv".into(),
        "config rmenv NAME [-g|-l|-os|-osg|-all] ## 移除 NTC 环境变量（-all：所有存储 + 会话）"
            .into(),
    );

    m.insert(
        "help.config.serve".into(),
        "config serve [服务] [--port-default 端口] [--bind-default 地址] ## serve 各服务默认值（命令行；`serve config` 现为浏览器界面）".into(),
    );

    m.insert(
        "help.config.update_check".into(),
        "config update_check ON|OFF ## 显示/隐藏每日新版本提醒（别名：uc）".into(),
    );

    m.insert(
        "help.config.integration_shell".into(),
        "config integration_shell ON|OFF ## 补全已安装的 shell 名称（powershell 仅 Windows、bash 仅 Linux 桌面、cmd 等）、启动参数，并在 Windows 上启用 ntc ps 模式、在 Linux 上启用 ntc <发行版> 模式（别名: is）".into(),
    );

    m.insert(
        "help.config.code_tools".into(),
        "config code_tools ON|OFF ## 补全已安装的编程工具/编译器（g++、python、java、cargo 等）及其参数（别名: ct）".into(),
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
}
