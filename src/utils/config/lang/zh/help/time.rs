use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.time.title".into(), "TIME 命令 - 计时与倒计时:".into());

    m.insert(
        "help.time.run".into(),
        "  time run <命令>                       运行 NTC 命令、别名或外部命令".into(),
    );

    m.insert(
        "help.time.alias".into(),
        "  time -r|--run <命令>                  run 的简写".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "  time -r <命令> --compare <命令>       运行两者并比较捕获的输出".into(),
    );

    m.insert(
        "help.time.compare_with".into(),
        "  time compare <命令 A> --with <命令 B> [-w]  比较两个命令（别名: -r A --compare B）"
            .into(),
    );

    m.insert(
        "help.time.bench".into(),
        "  time bench [--runs N] <命令>          重复 N 次（默认 5）并显示最小/最大/平均/中位数"
            .into(),
    );

    m.insert(
        "help.time.count".into(),
        "  time count <时长>                     倒计时（例如 30、90s、5m、1h30m）".into(),
    );

    m.insert(
        "help.time.help".into(),
        "  time help                             显示此帮助".into(),
    );

    m.insert("help.time.note".into(), "命令在当前目录中依次运行，可以是 NTC 命令、别名或外部命令。比较和 bench 会捕获输出；命令的副作用仍会发生。Ctrl+C 可取消倒计时。".into());
}
