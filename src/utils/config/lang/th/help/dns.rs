use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.dns.title".into(), "DNS คำสั่ง:".into());

    m.insert(
        "help.dns.usage".into(),
        "dns <host> ## ค้นหา DNS (-t A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL)".into(),
    );

    m.insert(
        "help.dns.example1".into(),
        "dns example.com -t MX ## ค้นหา DNS MX record".into(),
    );
}
