use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.restorecg.title".into(), "RESTORECG 命令：".into());

    m.insert(
        "help.restorecg.usage".into(),
        "restorecg ## 从备份恢复配置".into(),
    );

    m.insert(
        "help.restorecg.example1".into(),
        "restorecg ## 从备份恢复配置".into(),
    );
}
