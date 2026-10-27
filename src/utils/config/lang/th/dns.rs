use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("dns.help.examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "dns.help.opt_help".into(),
        "  -h, --help แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "dns.help.opt_type".into(),
        "  -t, --type <type> ประเภทระเบียน: A, AAAA, ALL, MX, NS, CNAME เป็นต้น (ค่าเริ่มต้น A)".into(),
    );

    m.insert(
        "dns.help.opt_type_sub".into(),
        "  MX/NS/CNAME/etc ใช้ dig/nslookup ของระบบ".into(),
    );

    m.insert("dns.help.options".into(), "ตัวเลือก:".into());

    m.insert("dns.help.title".into(), "DNS Lookup".into());

    m.insert("dns.help.usage".into(), "การใช้งาน:".into());

    m.insert(
        "dns.help.usage_line".into(),
        "  dns <host> [options]".into(),
    );
}
