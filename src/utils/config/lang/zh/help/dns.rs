use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.dns.title".into(), "DNS 命令：".into());

    m.insert(
        "help.dns.usage".into(),
        "dns <主机> ## DNS查询（-t A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL）".into(),
    );

    m.insert(
        "help.dns.example1".into(),
        "dns example.com -t MX ## 查询 DNS MX 记录".into(),
    );
}
