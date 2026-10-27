use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.mode.admin".into(),
        "mode admin ## 退出并以管理员身份重新启动ntc".into(),
    );

    m.insert(
        "help.mode.normal".into(),
        "mode normal ## 退出并以非提升模式重新启动ntc".into(),
    );

    m.insert("help.mode.title".into(), "Windows模式（Windows）：".into());

    m.insert(
        "help.mode.example1".into(),
        "mode admin ## 以管理员身份重新启动".into(),
    );
}
