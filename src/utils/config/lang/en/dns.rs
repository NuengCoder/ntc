use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("dns.help.examples".into(), "Examples:".into());

    m.insert(
        "dns.help.opt_help".into(),
        " -h, --help Show this help".into(),
    );

    m.insert(
        "dns.help.opt_type".into(),
        " -t, --type <type> Record type: A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL (default A)"
            .into(),
    );

    m.insert(
        "dns.help.opt_type_sub".into(),
        " Non-A records use the built-in DNS client (dig/nslookup only as fallback)".into(),
    );

    m.insert("dns.help.options".into(), "Options:".into());

    m.insert("dns.help.title".into(), "DNS Lookup".into());

    m.insert("dns.help.usage".into(), "Usage:".into());

    m.insert("dns.help.usage_line".into(), " dns <host> [options]".into());
}
