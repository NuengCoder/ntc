use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ping.title".into(), "PING COMMAND:".into());

    m.insert(
        "help.ping.usage".into(),
        "ping <host> ## ICMP ping (-c count)".into(),
    );

    m.insert(
        "help.ping.flags".into(),
        "ping <host> [-c N] [--forever] [-w MS] [-s BYTES] [-t TTL] [-4|-6] ## ICMP echo with flags".into(),
    );

    m.insert(
        "help.ping.example1".into(),
        "ping google.com ## Ping google.com".into(),
    );
}
