use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ping.title".into(), "PING 命令：".into());

    m.insert(
        "help.ping.usage".into(),
        "ping <主机> ## ICMP ping（-c 次数）".into(),
    );

    m.insert(
        "help.ping.flags".into(),
        "ping <主机> [-c N] [--forever] [-w 毫秒] [-s 字节] [-t TTL] [-4|-6] ## 带参数的 ICMP ping"
            .into(),
    );

    m.insert(
        "help.ping.example1".into(),
        "ping google.com ## 对 google.com 执行 ping".into(),
    );
}
