use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ntpkg.available".into(),
        "ntpkg available [语言] ## 列出语言或可下载版本".into(),
    );

    m.insert(
        "help.ntpkg.help".into(),
        "ntpkg help ## 显示 ntpkg 用法".into(),
    );

    m.insert(
        "help.ntpkg.info".into(),
        "ntpkg info <语言> ## 显示包信息（版本、位置、二进制文件）".into(),
    );

    m.insert(
        "help.ntpkg.install".into(),
        "ntpkg install <语言> [版本] ## 下载并安装运行时".into(),
    );

    m.insert(
        "help.ntpkg.list".into(),
        "ntpkg list ## 显示已安装的运行时".into(),
    );

    m.insert(
        "help.ntpkg.ntpx".into(),
        "ntpx <包> [二进制] [参数] ## 运行已安装包的二进制文件".into(),
    );

    m.insert(
        "help.ntpkg.search".into(),
        "ntpkg search [关键词] ## 搜索可用包".into(),
    );

    m.insert(
        "help.ntpkg.title".into(),
        "包管理器（ntpkg）+ ntpx：".into(),
    );

    m.insert(
        "help.ntpkg.uninstall".into(),
        "ntpkg uninstall <语言> [版本] ## 删除已安装的运行时".into(),
    );

    m.insert(
        "help.ntpkg.use".into(),
        "ntpkg use <语言> <版本> ## 激活特定版本".into(),
    );

    m.insert(
        "help.ntpkg.which".into(),
        "ntpkg which <语言> ## 显示活动运行时的路径".into(),
    );

    m.insert(
        "help.ntpkg.example1".into(),
        "ntpkg install node 18 ## 安装 Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example2".into(),
        "ntpkg list ## 显示已安装的运行时".into(),
    );

    m.insert(
        "help.ntpkg.example3".into(),
        "ntpkg use node 18 ## 激活 Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example4".into(),
        "ntpkg which node ## 显示 Node.js 路径".into(),
    );

    m.insert(
        "help.ntpkg.example5".into(),
        "ntpkg search python ## 搜索 Python 包".into(),
    );

    m.insert(
        "help.ntpkg.example6".into(),
        "ntpkg info node ## 显示 Node.js 包信息".into(),
    );

    m.insert(
        "help.ntpkg.example7".into(),
        "ntpkg available ## 列出可下载的语言".into(),
    );

    m.insert(
        "help.ntpkg.example8".into(),
        "ntpx node npm init ## 通过 ntpx 运行 npm".into(),
    );

    m.insert(
        "help.ntpkg.example9".into(),
        "ntpkg help ## 显示 ntpkg 用法".into(),
    );
}
