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

    m.insert(
        "ping.help.opt_forever".into(),
        " --forever 持续 ping 直到停止（Ctrl+C）".into(),
    );

    m.insert(
        "ping.help.opt_interval".into(),
        " -i, --interval <毫秒> 间隔时间（仅 Unix）".into(),
    );

    m.insert(
        "ping.help.opt_timeout".into(),
        " -w, --timeout <毫秒> 应答超时".into(),
    );

    m.insert(
        "ping.help.opt_size".into(),
        " -s, --size <字节> 负载大小".into(),
    );

    m.insert(
        "ping.help.opt_ttl".into(),
        " -t, --ttl <n> 生存时间 (TTL)".into(),
    );

    m.insert(
        "ping.help.opt_family".into(),
        " -4, -6 强制 IPv4 / IPv6".into(),
    );

    m.insert(
        "ping.help.opt_quiet".into(),
        " -q, --quiet 静默输出（仅 Unix）".into(),
    );
}
