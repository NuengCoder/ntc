use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.igcare.create_failed".into(),
        "ไม่สามารถสร้างไฟล์: {}".into(),
    );

    m.insert(
        "cmd.igcare.example_export_all".into(),
        "  igcare export --all myproject        # สร้าง myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.example_export_select".into(),
        "  igcare export --select myproject     # เลือกว่าจะส่งออกหมวดหมู่ใด".into(),
    );

    m.insert(
        "cmd.igcare.example_import".into(),
        "  igcare import myproject.ntc.igcare   # นำเข้าไปยังการกำหนดค่าปัจจุบัน".into(),
    );

    m.insert("cmd.igcare.examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.igcare.export_cmd".into(),
        "igcare export --all <ชื่อ> ส่งออกการตั้งค่าทั้งหมดไปยัง <ชื่อ>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_name_empty".into(),
        "ชื่อสำหรับส่งออกต้องไม่ว่างเปล่า.".into(),
    );

    m.insert(
        "cmd.igcare.export_select_cmd".into(),
        "igcare export --select <ชื่อ> เลือกหมวดหมู่เพื่อส่งออกไปยัง <ชื่อ>.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.export_usage".into(),
        "การใช้งาน: igcare export --all <ชื่อ> | igcare export --select <ชื่อ>".into(),
    );

    m.insert("cmd.igcare.file_exists".into(), "ไฟล์มีอยู่แล้ว: {}".into());

    m.insert(
        "cmd.igcare.import_cmd".into(),
        "igcare import <ไฟล์> นำเข้าการตั้งค่าจากไฟล์ .ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.import_example".into(),
        "ตัวอย่าง: igcare import myproject.ntc.igcare".into(),
    );

    m.insert(
        "cmd.igcare.import_usage".into(),
        "การใช้งาน: igcare import <ไฟล์>".into(),
    );

    m.insert("cmd.igcare.main_title".into(), "ตัวจัดการ Ignore/Care".into());

    m.insert(
        "cmd.igcare.template_created".into(),
        "สร้างเทมเพลต igcare แล้ว: {}".into(),
    );

    m.insert(
        "cmd.igcare.template_hint".into(),
        "นี่คือไฟล์เทมเพลตในเครื่อง. ใช้ 'igcare import' เพื่อนำการตั้งค่าไปใช้.".into(),
    );

    m.insert(
        "cmd.igcare.type_help".into(),
        "พิมพ์ 'igcare' เพื่อดูความช่วยเหลือ.".into(),
    );

    m.insert(
        "cmd.igcare.unknown_flag".into(),
        "แฟลกส่งออกไม่รู้จัก: {}. ใช้ --all หรือ --select".into(),
    );

    m.insert(
        "cmd.igcare.unknown_subcmd".into(),
        "ไม่รู้จักคำสั่งย่อยของ igcare: {}".into(),
    );
}
