use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ping.help.examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "ping.help.opt_count".into(),
        "  -c, --count <n> จำนวนการ ping (ค่าเริ่มต้น 4)".into(),
    );

    m.insert(
        "ping.help.opt_help".into(),
        "  -h, --help แสดงความช่วยเหลือนี้".into(),
    );

    m.insert("ping.help.options".into(), "ตัวเลือก:".into());

    m.insert("ping.help.title".into(), "Ping".into());

    m.insert("ping.help.usage".into(), "การใช้งาน:".into());

    m.insert(
        "ping.help.usage_line".into(),
        "  ping <host> [options]".into(),
    );

    m.insert(
        "ping.help.opt_forever".into(),
        " --forever ping ต่อเนื่องจนกว่าจะหยุด (Ctrl+C)".into(),
    );

    m.insert(
        "ping.help.opt_interval".into(),
        " -i, --interval <มิลลิวินาที> ระยะห่างระหว่าง ping (Unix เท่านั้น)".into(),
    );

    m.insert(
        "ping.help.opt_timeout".into(),
        " -w, --timeout <มิลลิวินาที> เวลารอคำตอบ".into(),
    );

    m.insert(
        "ping.help.opt_size".into(),
        " -s, --size <ไบต์> ขนาดเพย์โหลด".into(),
    );

    m.insert(
        "ping.help.opt_ttl".into(),
        " -t, --ttl <n> เวลามีชีวิต (TTL)".into(),
    );

    m.insert(
        "ping.help.opt_family".into(),
        " -4, -6 บังคับ IPv4 / IPv6".into(),
    );

    m.insert(
        "ping.help.opt_quiet".into(),
        " -q, --quiet แสดงผลแบบเงียบ (Unix เท่านั้น)".into(),
    );
}
