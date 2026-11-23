use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.dns.title".into(), "DNS ຄຳສັ່ງ:".into());

    m.insert(
        "help.dns.usage".into(),
        "dns <host> ## ຄົ້ນຫາ DNS (-t A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL)".into(),
    );

    m.insert(
        "help.dns.example1".into(),
        "dns example.com -t MX ## ຄົ້ນຫາ DNS MX record".into(),
    );
}
