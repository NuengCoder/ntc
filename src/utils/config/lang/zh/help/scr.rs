use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scr.continue".into(),
        "scr -c <file.ntc> [args...] ## 运行脚本，出错后继续".into(),
    );

    m.insert(
        "help.scr.example1".into(),
        "scr deploy.ntc ## 运行 deploy.ntc".into(),
    );

    m.insert(
        "help.scr.example2".into(),
        "scr build.ntc dev prod ## 传入参数 $1=dev, $2=prod".into(),
    );

    m.insert(
        "help.scr.example3".into(),
        "scr -c setup.ntc ## 运行并在出错后继续".into(),
    );

    m.insert("help.scr.help".into(), "scr help ## 显示 scr 用法".into());

    m.insert(
        "help.scr.init".into(),
        "scr init [name] ## 创建脚本模板".into(),
    );

    m.insert(
        "help.scr.ls".into(),
        "scr ls ## 列出当前目录中的 *.ntc 脚本".into(),
    );

    m.insert(
        "help.scr.run".into(),
        "scr <file.ntc> [args...] ## 运行自动化脚本".into(),
    );

    m.insert("help.scr.title".into(), "SCR / 自动化脚本命令：".into());
}
