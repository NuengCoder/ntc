use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.backup.clear".into(),
        "backup clear ## ลบข้อมูลสำรองทั้งหมด (ถามยืนยัน)".into(),
    );

    m.insert(
        "help.backup.clear_force".into(),
        "backup clear --force ## ลบข้อมูลสำรองทั้งหมด (ไม่ถาม)".into(),
    );

    m.insert(
        "help.backup.create".into(),
        "backup create ## สร้างข้อมูลสำรองของโปรเจกต์ปัจจุบัน".into(),
    );

    m.insert(
        "help.backup.diff".into(),
        "backup diff [num] ## เปรียบเทียบโปรเจกต์ปัจจุบันกับข้อมูลสำรอง".into(),
    );

    m.insert(
        "help.backup.list".into(),
        "backup list ## แสดงข้อมูลสำรองทั้งหมด".into(),
    );

    m.insert(
        "help.backup.recreate".into(),
        "backup recreate ## ล้างทั้งหมด + เลิกทำ + สร้างข้อมูลสำรองใหม่".into(),
    );

    m.insert(
        "help.backup.restore".into(),
        "backup restore [num] ## คืนค่าข้อมูลสำรอง (แบบโต้ตอบถ้าไม่มี num)".into(),
    );

    m.insert(
        "help.backup.restore_choose".into(),
        "backup restore --choose ## เลือกข้อมูลสำรองจากเมนูแบบโต้ตอบ".into(),
    );

    m.insert(
        "help.backup.restore_deleted".into(),
        "backup restore --deleted ## คืนค่าเฉพาะไฟล์ที่ถูกลบ".into(),
    );

    m.insert("help.backup.title".into(), "คำสั่ง BACKUP:".into());

    m.insert(
        "help.backup.undo".into(),
        "backup undo ## เลิกทำการคืนค่าครั้งล่าสุด".into(),
    );

    m.insert(
        "help.backup.undo_cls".into(),
        "backup undo --cls ## ล้างประวัติการเลิกทำ (ถามยืนยัน)".into(),
    );

    m.insert(
        "help.backup.undo_force".into(),
        "backup undo --force ## ล้างประวัติการเลิกทำ (ไม่ถาม)".into(),
    );

    m.insert(
        "help.backup.example1".into(),
        "backup create ## สร้างข้อมูลสำรองใหม่".into(),
    );

    m.insert(
        "help.backup.example2".into(),
        "backup list ## แสดงข้อมูลสำรองทั้งหมด".into(),
    );

    m.insert(
        "help.backup.example3".into(),
        "backup restore 1 ## คืนค่าข้อมูลสำรอง #1".into(),
    );

    m.insert(
        "help.backup.example4".into(),
        "backup diff 1 ## เปรียบเทียบกับข้อมูลสำรอง #1".into(),
    );
}
