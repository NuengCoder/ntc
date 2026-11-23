use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.wi.title".into(), "WI 命令：".into());

    m.insert(
        "help.wi.usage".into(),
        "wi [路径] ## 实时文件更改流（q=退出）".into(),
    );

    m.insert("help.wi.example1".into(), "wi ## 实时监视文件变化".into());
}
