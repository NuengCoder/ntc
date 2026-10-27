use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.care.scope_help".into(),
        "ใช้ '-g' / '--global' เพื่อเขียนไปยัง config global (ในโฟลเดอร์ที่มี ntconfig.toml)".into(),
    );

    m.insert(
        "cmd.care.dir_desc".into(),
        "care dir <ชื่อ>[, ...] หยุดการละเว้นไดเรกทอรี".into(),
    );

    m.insert(
        "cmd.care.dir_usage".into(),
        "การใช้งาน: care dir <ชื่อ>[, <ชื่อ>, ...]".into(),
    );

    m.insert(
        "cmd.care.ext_desc".into(),
        "care ext <นามสกุล>[, ...] เพิ่มนามสกุลที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "cmd.care.ext_usage".into(),
        "การใช้งาน: care ext <นามสกุล>[, <นามสกุล>, ...]".into(),
    );

    m.insert(
        "cmd.care.file_desc".into(),
        "care file <ชื่อ>[, ...] เพิ่มไฟล์ที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "cmd.care.file_usage".into(),
        "การใช้งาน: care file <ชื่อ>[, <ชื่อ>, ...]".into(),
    );

    m.insert(
        "cmd.care.title".into(),
        "care \u{2014} จัดการรายการที่ต้องการ (ไม่ถูกละเว้น)".into(),
    );
}
