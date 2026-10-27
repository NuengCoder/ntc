use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.arc.create_usage".into(),
        "การใช้งาน: arc create <out.zip|tar.gz|tgz|tar> <ไฟล์...>".into(),
    );

    m.insert(
        "cmd.arc.extract_usage".into(),
        "การใช้งาน: arc extract <ไฟล์> [ปลายทาง]".into(),
    );

    m.insert(
        "cmd.arc.list_usage".into(),
        "การใช้งาน: arc list <ไฟล์>".into(),
    );

    m.insert(
        "cmd.arc.bad_ext".into(),
        "ไม่รองรับไฟล์ประเภทนี้ (ใช้ .zip, .tar.gz/.tgz, .tar)".into(),
    );

    m.insert("cmd.arc.not_found".into(), "ไม่พบ: {}".into());

    m.insert("cmd.arc.no_inputs".into(), "ไม่มีไฟล์อินพุตสำหรับบีบอัด".into());

    m.insert(
        "cmd.arc.created".into(),
        "สร้างไฟล์แล้ว: {} ({} รายการ)".into(),
    );

    m.insert("cmd.arc.extracted".into(), "แตกไฟล์ {} ไฟล์ไปยัง {}".into());

    m.insert("cmd.arc.list_title".into(), "ไฟล์บีบอัด: {}".into());

    m.insert("cmd.arc.total".into(), "ทั้งหมด: {} รายการ".into());

    m.insert(
        "cmd.arc.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อยของ arc: '{}'".into(),
    );

    m.insert("cmd.arc.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.arc.help_create".into(),
        "  arc create <ออก> <ไฟล์...>  สร้างไฟล์บีบอัด (.zip, .tar.gz, .tar)".into(),
    );

    m.insert(
        "cmd.arc.help_extract".into(),
        "  arc extract <ไฟล์> [ปลายทาง]   แตกไฟล์".into(),
    );

    m.insert(
        "cmd.arc.help_list".into(),
        "  arc list <ไฟล์>              แสดงเนื้อหา".into(),
    );

    m.insert("cmd.arc.examples_title".into(), "ตัวอย่าง:".into());
}
