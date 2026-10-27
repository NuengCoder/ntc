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
}
