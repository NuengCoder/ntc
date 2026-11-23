use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmd.title".into(), "RMD 命令：".into());

    m.insert(
        "help.rmd.usage".into(),
        "rmd <目录>[, ...] ## 删除目录；通配符确认后删除所有匹配项".into(),
    );

    m.insert(
        "help.rmd.example1".into(),
        "rmd build ## 删除 build 目录".into(),
    );
}
