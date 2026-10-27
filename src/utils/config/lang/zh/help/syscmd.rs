use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.syscmd.any".into(),
        "<任意命令> ## NTC将未知命令传递给系统".into(),
    );

    m.insert(
        "help.syscmd.native".into(),
        "shell -- echo, git, dir/ls, ping 等。".into(),
    );

    m.insert(
        "help.syscmd.native2".into(),
        "所有命令均可与NTC命令一起本地运行。".into(),
    );

    m.insert("help.syscmd.title".into(), "系统命令：".into());
}
