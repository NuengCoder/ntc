use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("doctor.title".into(), "ntc doctor".into());

    m.insert("doctor.platform".into(), "ntc {} ({}) บน {}/{}".into());

    m.insert(
        "doctor.no_config_dir".into(),
        "ระบบนี้ไม่มีไดเรกทอรี config".into(),
    );

    m.insert(
        "doctor.config_new".into(),
        "ยังไม่ได้สร้างไดเรกทอรี config: {} (จะสร้างเมื่อบันทึกครั้งแรก)".into(),
    );

    m.insert("doctor.config_ok".into(), "การตั้งค่า: {}".into());

    m.insert(
        "doctor.config_unwritable".into(),
        "ไดเรกทอรี config เขียนไม่ได้: {} ({})".into(),
    );

    m.insert("doctor.settings".into(), "ภาษา: {}, ธีม: {}".into());

    m.insert("doctor.install".into(), "{} อยู่ใน PATH: {}".into());

    m.insert(
        "doctor.install_missing".into(),
        "{} ไม่ได้อยู่ใน PATH \u{2014} มีเพียงไดเรกทอรี build นี้ที่เรียกได้".into(),
    );

    m.insert(
        "doctor.ran_none".into(),
        "ran: ไม่มี ranfile ใน {} (ไม่มีอะไรให้รันที่นี่)".into(),
    );

    m.insert("doctor.ran_ok".into(), "ran: {} มี {} target".into());

    m.insert(
        "doctor.ran_env".into(),
        "ran: {} environment variable, {} ตัวแปร".into(),
    );

    m.insert(
        "doctor.ran_problems".into(),
        "ran: {} มี {} ปัญหา \u{2014} รัน `{}`".into(),
    );

    m.insert("doctor.ran_error".into(), "ran ผิดพลาด: {}: {}".into());

    m.insert(
        "doctor.ntpkg_none".into(),
        "ntpkg: ยังไม่ได้ติดตั้ง runtime ใด ๆ".into(),
    );

    m.insert(
        "doctor.ntpkg_ok".into(),
        "ntpkg: ติดตั้ง {} เวอร์ชัน, ใช้งานอยู่ {}".into(),
    );

    m.insert("doctor.ntpkg_error".into(), "ntpkg ผิดพลาด: {}: {}".into());

    m.insert(
        "doctor.ntpkg_path".into(),
        "ntpkg: {} ไดเรกทอรี bin ที่ใช้งานอยู่หายไปจาก PATH".into(),
    );

    m.insert("doctor.ok".into(), "ไม่พบปัญหา".into());

    m.insert("doctor.issues".into(), "มี {} เรื่องที่ควรตรวจสอบ".into());
}
