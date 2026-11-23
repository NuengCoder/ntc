use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.doctor.title".into(), "DOCTOR 命令：".into());

    m.insert(
        "help.doctor.usage".into(),
        "doctor ## 检查配置、PATH、ranfile 和 ntpkg 的问题".into(),
    );

    m.insert(
        "help.doctor.example1".into(),
        "doctor ## 只读报告，不会做任何更改".into(),
    );
}
