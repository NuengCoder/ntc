use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.autoup.add".into(),
        "autoup add <名称> \"<命令>\" ## 将启动条目添加到HKCU".into(),
    );

    m.insert(
        "help.autoup.disable".into(),
        "autoup disable <名称> ## 禁用条目（使用StartupApproved）".into(),
    );

    m.insert(
        "help.autoup.enable".into(),
        "autoup enable <名称> ## 重新启用禁用的条目".into(),
    );

    m.insert(
        "help.autoup.list".into(),
        "autoup list ## 列出所有启动条目（CU/LM/SF来源）".into(),
    );

    m.insert(
        "help.autoup.rm".into(),
        "autoup rm <名称> ## 删除启动条目".into(),
    );

    m.insert(
        "help.autoup.sources".into(),
        "[CU]=HKCU [LM]=HKLM [SF]=启动文件夹".into(),
    );

    m.insert("help.autoup.title".into(), "启动管理器（Windows）：".into());

    m.insert(
        "help.autoup.example1".into(),
        "autoup list ## 列出启动项".into(),
    );

    m.insert(
        "help.autoup.example2".into(),
        "autoup add ntc \"C:\\ntc\\ntc.exe\" ## 添加到启动项".into(),
    );

    m.insert(
        "help.autoup.example3".into(),
        "autoup rm ntc ## 删除启动项".into(),
    );

    m.insert(
        "help.autoup.example4".into(),
        "autoup disable ntc ## 禁用启动项".into(),
    );
}
