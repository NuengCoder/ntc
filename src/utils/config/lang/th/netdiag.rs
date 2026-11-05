use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("trace.help.title".into(), "ติดตามเส้นทาง".into());

    m.insert("trace.help.usage".into(), "วิธีใช้:".into());

    m.insert(
        "trace.help.usage_line".into(),
        " trace <โฮสต์|IP> [ตัวเลือก]".into(),
    );

    m.insert("trace.help.options".into(), "ตัวเลือก:".into());

    m.insert(
        "trace.help.opt_max_hops".into(),
        " -m, --max-hops <n> จำนวนฮอปสูงสุด (ค่าเริ่มต้น 30)".into(),
    );

    m.insert(
        "trace.help.opt_timeout".into(),
        " -w, --timeout <มิลลิวินาที> เวลารอต่อฮอป".into(),
    );

    m.insert(
        "trace.help.opt_queries".into(),
        " -q, --queries <n> จำนวนการทดสอบต่อฮอป (traceroute)".into(),
    );

    m.insert(
        "trace.help.opt_numeric".into(),
        " -n, --numeric ไม่แปลงชื่อโฮสต์".into(),
    );

    m.insert(
        "trace.help.opt_family".into(),
        " -4, -6 บังคับ IPv4 / IPv6".into(),
    );

    m.insert(
        "trace.help.opt_no_fallback".into(),
        " --no-fallback ไม่ใช้วิธี TTL ทีละฮอป".into(),
    );

    m.insert("trace.help.opt_help".into(), " -h, --help แสดงวิธีใช้".into());

    m.insert("trace.help.examples".into(), "ตัวอย่าง:".into());

    m.insert("netdiag.missing_value".into(), "ขาดค่าของ {}".into());

    m.insert("netdiag.invalid_value".into(), "ค่าไม่ถูกต้อง: {}".into());

    m.insert("netdiag.unknown_option".into(), "ตัวเลือกไม่รู้จัก: {}".into());

    m.insert("netdiag.unexpected_arg".into(), "อาร์กิวเมนต์เกิน: {}".into());

    m.insert("netdiag.host_required".into(), "ต้องระบุโฮสต์".into());

    m.insert(
        "netdiag.forever_or_count".into(),
        "ใช้ --forever หรือ --count อย่างใดอย่างหนึ่ง".into(),
    );

    m.insert("netdiag.spawn_failed".into(), "เรียก {} ไม่ได้: {}".into());

    m.insert(
        "netdiag.flag_unsupported".into(),
        "ping ของแพลตฟอร์มนี้ไม่รองรับ {} (ข้าม)".into(),
    );

    m.insert(
        "netdiag.trace_missing".into(),
        "ไม่พบเครื่องมือ traceroute (tracert/traceroute/tracepath)".into(),
    );

    m.insert(
        "netdiag.trace_fallback".into(),
        "ใช้วิธี TTL ทีละฮอปแทน (ping ต่อฮอป) ช้ากว่าและฮอปที่ถูกกรองจะแสดง *".into(),
    );

    m.insert(
        "netdiag.trace_title".into(),
        "กำลังติดตามเส้นทางไป {} (สูงสุด {} ฮอป):".into(),
    );
}
