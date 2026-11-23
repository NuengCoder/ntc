use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.deinit.title".into(), "DEINIT 命令：".into());

    m.insert(
        "help.deinit.usage".into(),
        "deinit [ran|local|opencode] ## 删除项目文件（每次删除前都会确认）".into(),
    );

    m.insert(
        "help.deinit.example1".into(),
        "deinit ran ## 删除 NTCRANFILE.toml".into(),
    );
}
