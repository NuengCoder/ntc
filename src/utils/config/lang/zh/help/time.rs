use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.bench".into(),
        "time --bench <n> <命令> ## 重复 n 次并显示最小/最大/平均/中位数".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "time <命令 A> --compare <命令 B> [-w] ## 运行两者并比较退出状态与输出".into(),
    );

    m.insert(
        "help.time.count".into(),
        "time --count <时长> ## 倒计时（例如 30、90s、5m、1h30m）".into(),
    );

    m.insert("help.time.help".into(), "time --help ## 显示此帮助".into());

    m.insert(
        "help.time.note".into(),
        "命令在当前目录中运行，可以是 NTC 命令、别名或外部命令。--compare 会捕获输出；--bench 会重复执行命令，副作用也会重复。Ctrl+C 可取消倒计时。".into(),
    );

    m.insert(
        "help.time.run".into(),
        "time <命令> ## 运行命令并显示耗时".into(),
    );

    m.insert(
        "help.time.title".into(),
        "TIME 命令 - 计时、比较与倒计时:".into(),
    );
}
