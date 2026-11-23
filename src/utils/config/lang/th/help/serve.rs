use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.serve.title".into(), "SERVE คำสั่ง:".into());

    m.insert(
        "help.serve.service".into(),
        "serve <service> ## เลือกบริการ (`serve` เดี่ยวจะแสดงรายการ)".into(),
    );

    m.insert(
        "help.serve.chess".into(),
        "serve chess [--online] ## หมากรุก hotseat/AI/โค้ช (--online: ล็อบบี้ + ห้อง)".into(),
    );

    m.insert(
        "help.serve.git".into(),
        "serve git [โฟลเดอร์] ## Git GUI: สถานะ, diff, stage, commit, branch, tag, stash, push/pull"
            .into(),
    );

    m.insert(
        "help.serve.arc".into(),
        "serve arc [โฟลเดอร์] ## Archive GUI: สร้าง, ดู และแตกไฟล์บีบอัด".into(),
    );

    m.insert(
        "help.serve.ndb".into(),
        "serve ndb ## เรียกดู SQLite: เลือกฐานข้อมูลในโฟลเดอร์นี้, ดูโครงสร้าง/ข้อมูล, เลือกใช้ --write".into(),
    );

    m.insert(
        "help.serve.theme".into(),
        "serve theme ## Theme GUI: แก้ไข ดูตัวอย่าง บันทึก และใช้งานธีม".into(),
    );

    m.insert(
        "help.serve.fake".into(),
        "serve fake ## UI ข้อมูลจำลอง: ชนิดหรือกลุ่มที่บันทึกไว้, ดูตัวอย่าง txt/json/csv/sql".into(),
    );

    m.insert(
        "help.serve.config".into(),
        "serve config ## UI การตั้งค่า: ค่าต่างๆ และค่าเริ่มต้นของ serve (CLI ยังอยู่ที่ config serve)".into(),
    );

    m.insert(
        "help.serve.backup".into(),
        "serve backup [path] ## UI สำรองข้อมูล: สร้าง, diff, กู้คืน, ย้อนกลับ, ล้างของโฟลเดอร์นี้".into(),
    );

    m.insert(
        "help.serve.example1".into(),
        "serve files --port 8080 ## เริ่มเซิร์ฟเวอร์ file-drive บนพอร์ต 8080".into(),
    );
}
