use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("dns.help.examples".into(), "示例：".into());

    m.insert("dns.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert(
        "dns.help.opt_type".into(),
        " -t, --type <类型> 记录类型：A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL（默认 A）".into(),
    );

    m.insert(
        "dns.help.opt_type_sub".into(),
        " 非 A 记录使用内置 DNS 客户端（dig/nslookup 仅作后备）".into(),
    );

    m.insert("dns.help.options".into(), "选项：".into());

    m.insert("dns.help.title".into(), "DNS 查询".into());

    m.insert("dns.help.usage".into(), "用法：".into());

    m.insert("dns.help.usage_line".into(), " dns <主机> [选项]".into());
}
