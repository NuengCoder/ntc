use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("dns.help.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "dns.help.opt_help".into(),
        " -h, --help ສະແດງ help ນີ້".into(),
    );

    m.insert(
        "dns.help.opt_type".into(),
        " -t, --type <type> Record type: A, AAAA, ALL, MX, NS, CNAME ແລະອື່ນໆ (ເລີ່ມຕົ້ນ A)".into(),
    );

    m.insert(
        "dns.help.opt_type_sub".into(),
        " MX/NS/CNAME ແລະອື່ນໆ ໃຊ້ dig/nslookup ຂອງລະບົບ".into(),
    );

    m.insert("dns.help.options".into(), "ຕົວເລືອກ:".into());

    m.insert("dns.help.title".into(), "DNS Lookup".into());

    m.insert("dns.help.usage".into(), "ວິທີໃຊ້:".into());

    m.insert("dns.help.usage_line".into(), " dns <host> [options]".into());
}
