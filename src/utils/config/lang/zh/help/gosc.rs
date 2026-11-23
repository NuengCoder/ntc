use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gosc.title".into(), "GOSC 命令：".into());

    m.insert(
        "help.gosc.usage".into(),
        "gosc ## 连续导航（0退出，v/s/vs查看）".into(),
    );

    m.insert("help.gosc.example1".into(), "gosc ## 连续浏览子目录".into());
}
