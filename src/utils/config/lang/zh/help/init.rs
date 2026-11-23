use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.init.title".into(), "INIT 命令：".into());

    m.insert(
        "help.init.usage".into(),
        "init [all|ran|local|opencode|project] [-f|--force] ## 初始化项目文件（NTCRANFILE、ntconfig.toml、.opencode/、项目脚手架）".into(),
    );

    m.insert(
        "help.init.example1".into(),
        "init ran ## 初始化 NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.init.example2".into(),
        "init project my-app --no lc,oc,docker ## 不生成 LICENSE、OpenCode、Docker 的项目脚手架"
            .into(),
    );
}
