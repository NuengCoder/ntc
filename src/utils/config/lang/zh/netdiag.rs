use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("trace.help.title".into(), "路由追踪".into());

    m.insert("trace.help.usage".into(), "用法：".into());

    m.insert(
        "trace.help.usage_line".into(),
        " trace <主机|IP> [选项]".into(),
    );

    m.insert("trace.help.options".into(), "选项：".into());

    m.insert(
        "trace.help.opt_max_hops".into(),
        " -m, --max-hops <n> 最大跳数（默认 30）".into(),
    );

    m.insert(
        "trace.help.opt_timeout".into(),
        " -w, --timeout <毫秒> 每跳超时时间".into(),
    );

    m.insert(
        "trace.help.opt_queries".into(),
        " -q, --queries <n> 每跳探测次数（traceroute）".into(),
    );

    m.insert(
        "trace.help.opt_numeric".into(),
        " -n, --numeric 不解析主机名".into(),
    );

    m.insert(
        "trace.help.opt_family".into(),
        " -4, -6 强制 IPv4 / IPv6".into(),
    );

    m.insert(
        "trace.help.opt_no_fallback".into(),
        " --no-fallback 不使用 TTL 逐跳回退".into(),
    );

    m.insert(
        "trace.help.opt_help".into(),
        " -h, --help 显示此帮助".into(),
    );

    m.insert("trace.help.examples".into(), "示例：".into());

    m.insert("netdiag.missing_value".into(), "缺少参数值：{}".into());

    m.insert("netdiag.invalid_value".into(), "无效的值：{}".into());

    m.insert("netdiag.unknown_option".into(), "未知选项：{}".into());

    m.insert("netdiag.unexpected_arg".into(), "多余的参数：{}".into());

    m.insert("netdiag.host_required".into(), "需要指定主机".into());

    m.insert(
        "netdiag.forever_or_count".into(),
        "--forever 与 --count 不能同时使用".into(),
    );

    m.insert("netdiag.spawn_failed".into(), "无法运行 {}：{}".into());

    m.insert(
        "netdiag.flag_unsupported".into(),
        "当前平台的 ping 不支持 {}，已忽略".into(),
    );

    m.insert(
        "netdiag.trace_missing".into(),
        "未找到路由追踪工具（tracert/traceroute/tracepath）".into(),
    );

    m.insert(
        "netdiag.trace_fallback".into(),
        "改用 TTL 逐跳探测（每跳一次 ping）；速度较慢，被过滤的跳显示 *".into(),
    );

    m.insert(
        "netdiag.trace_title".into(),
        "正在追踪到 {} 的路由（最多 {} 跳）：".into(),
    );
}
