use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.spec.title".into(), "SPEC 命令：".into());

    m.insert(
        "help.spec.usage".into(),
        "spec ## 显示电脑/系统规格（CPU、显卡、内存、存储、系统、序列号）".into(),
    );

    m.insert("help.spec.example1".into(), "spec ## 显示系统规格".into());
}
