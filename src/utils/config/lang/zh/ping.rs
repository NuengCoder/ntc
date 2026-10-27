use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ping.help.examples".into(), "示例：".into());

    m.insert(
        "ping.help.opt_count".into(),
        " -c, --count <n> Ping 次数（默认 4）".into(),
    );

    m.insert("ping.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert("ping.help.options".into(), "选项：".into());

    m.insert("ping.help.title".into(), "Ping".into());

    m.insert("ping.help.usage".into(), "用法：".into());

    m.insert("ping.help.usage_line".into(), " ping <主机> [选项]".into());
}
