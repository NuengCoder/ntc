use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmf.title".into(), "RMF 命令：".into());

    m.insert(
        "help.rmf.usage".into(),
        "rmf <文件>[, ...] ## 删除文件（需确认）".into(),
    );

    m.insert("help.rmf.example1".into(), "rmf old.log ## 删除文件".into());
}
