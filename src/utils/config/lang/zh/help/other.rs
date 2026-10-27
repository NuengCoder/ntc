use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.other.clear".into(), "clear ## 清除屏幕".into());

    m.insert(
        "help.other.deinit".into(),
        "deinit [--ran|--local] ## 删除项目文件".into(),
    );

    m.insert("help.other.exit".into(), "exit, quit ## 退出ntc".into());

    m.insert("help.other.help".into(), "help ## 显示此帮助".into());

    m.insert("help.other.init".into(), "init [--ran|--local|--opencode] ## 初始化项目文件（NTCRANFILE.toml、ntconfig.toml、.opencode/）".into());

    m.insert(
        "help.other.print".into(),
        "print <文本> [--loop N] [-n|-t] ## say的别名".into(),
    );

    m.insert(
        "help.other.say".into(),
        "say <文本> [--loop N] [-n|-t] ## 打印文本（无需文件重定向）".into(),
    );

    m.insert("help.other.title".into(), "其他命令：".into());

    m.insert(
        "help.other.tutorial".into(),
        "tutorial ## 启动交互式分步教程".into(),
    );

    m.insert("help.other.ui".into(), "ui ## 显示当前UI模式".into());

    m.insert("help.other.version".into(), "version ## 显示版本".into());

    m.insert(
        "help.other.example1".into(),
        "init --ran ## 初始化 NTCRANFILE.toml".into(),
    );

    m.insert("help.other.example2".into(), "clear ## 清除屏幕".into());

    m.insert(
        "help.other.example3".into(),
        "version ## 显示版本信息".into(),
    );

    m.insert(
        "help.other.example4".into(),
        "tutorial ## 启动交互式教程".into(),
    );
}
