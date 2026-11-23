use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sys.title".into(), "系统命令：".into());

    m.insert(
        "help.sys.monitor".into(),
        "sys, top [-l|--log] [-d|--delay [秒数:-1]] ## 实时系统监视器（CPU、内存、磁盘、进程）".into(),
    );

    m.insert(
        "help.sys.passthrough".into(),
        "<任意命令> ## NTC将未知命令传递给系统".into(),
    );

    m.insert(
        "help.sys.native".into(),
        "shell -- echo, git, dir/ls, ping 等。 所有命令均可与NTC命令一起本地运行。".into(),
    );

    m.insert("help.sys.example1".into(), "sys ## 显示系统监视器".into());

    m.insert(
        "help.sys.example2".into(),
        "sys --log --delay 5 ## 每 5 秒将 CPU/内存/磁盘使用情况追加到 log/ntc_sys.log（默认 15 秒）".into(),
    );

    m.insert(
        "help.sys.example3".into(),
        "sys -ld 5 ## 等同 'sys -l -d 5'：-l/--log 打开日志，-d/--delay 指定秒数".into(),
    );
}
