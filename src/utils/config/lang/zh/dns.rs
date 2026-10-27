use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("dns.help.examples".into(), "示例：".into());

    m.insert("dns.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert(
        "dns.help.opt_type".into(),
        " -t, --type <类型> 记录类型：A, AAAA, ALL, MX, NS, CNAME 等（默认 A）".into(),
    );

    m.insert(
        "dns.help.opt_type_sub".into(),
        " MX/NS/CNAME 等使用系统 dig/nslookup".into(),
    );

    m.insert("dns.help.options".into(), "选项：".into());

    m.insert("dns.help.title".into(), "DNS 查询".into());

    m.insert("dns.help.usage".into(), "用法：".into());

    m.insert("dns.help.usage_line".into(), " dns <主机> [选项]".into());
}
