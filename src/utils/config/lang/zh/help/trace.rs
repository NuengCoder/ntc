use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trace.title".into(), "TRACE 命令：".into());

    m.insert(
        "help.trace.usage".into(),
        "trace <主机> [-m N] [-w 毫秒] ## 逐跳路由追踪（tracert/traceroute，回退 TTL 探测）".into(),
    );

    m.insert(
        "help.trace.example1".into(),
        "trace example.com ## 跟踪到 example.com 的路由".into(),
    );
}
