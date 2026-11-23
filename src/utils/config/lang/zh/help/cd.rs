use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cd.title".into(), "CD 命令：".into());

    m.insert("help.cd.usage".into(), "cd <路径> ## 导航到目录".into());

    m.insert(
        "help.cd.cd_tp".into(),
        "cd to <tp名称> ## 传送到已保存的传送点".into(),
    );

    m.insert(
        "help.cd.example1".into(),
        "cd src ## 导航到 src 目录".into(),
    );

    m.insert(
        "help.cd.example2".into(),
        "cd to web ## 传送到 'web' 保存点".into(),
    );
}
