use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "check.help".into(),
        " check [文件] - 解释 {} 并报告所有问题".into(),
    );

    m.insert("check.hint".into(), "提示".into());

    m.insert("check.no_files".into(), "{} 中没有 {} 文件".into());

    m.insert("check.not_found".into(), "找不到 {}".into());

    m.insert("check.ok".into(), "{}：没有发现问题".into());

    m.insert("check.read_fail".into(), "无法读取 {}：{}".into());

    m.insert("check.summary".into(), "{} 个问题，位于 {}".into());

    m.insert("check.title".into(), "检查 {}".into());
}
