use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.deadlock.title".into(),
        "DEADLOCK - vault รหัสผ่านที่เข้ารหัส:".into(),
    );

    m.insert(
        "help.deadlock.launch".into(),
        "  deadlock                          เปิด vault (ถามรหัสผ่านหลัก)".into(),
    );

    m.insert(
        "help.deadlock.ls".into(),
        "  ls                                แสดง pass-env ทั้งหมดพร้อมรหัสผ่านจริง".into(),
    );

    m.insert(
        "help.deadlock.add".into(),
        "  add <pass-env> <real-password>    บันทึกหรืออัปเดตความลับ (รหัสผ่านใส่เครื่องหมายคำพูดได้)".into(),
    );

    m.insert(
        "help.deadlock.rmpw".into(),
        "  rmpw <pass-env>                   ลบความลับที่บันทึกไว้".into(),
    );

    m.insert(
        "help.deadlock.master".into(),
        "  master-pass                       เปลี่ยนรหัสผ่านหลัก".into(),
    );

    m.insert(
        "help.deadlock.forget".into(),
        "  forget                            ลบรหัสผ่านหลักออกจากที่เก็บข้อมูลรับรองของระบบปฏิบัติการ".into(),
    );

    m.insert(
        "help.deadlock.destroy".into(),
        "  destroy                           ลบ vault และความลับทั้งหมด".into(),
    );

    m.insert(
        "help.deadlock.exit".into(),
        "  exit                              ปิด vault และกลับสู่ shell ของ ntc".into(),
    );

    m.insert("help.deadlock.note".into(), "ความลับถูกเข้ารหัสด้วย AES-256-GCM โดยใช้คีย์ที่มาจากรหัสผ่านหลักด้วย Argon2id รหัสผ่านหลักเริ่มต้นคือ 12345678 และ deadlock จะบังคับให้เปลี่ยนเมื่อเข้าสู่ระบบครั้งแรก ไฟล์ vault ถูกจำกัดสิทธิ์ให้เฉพาะบัญชีผู้ใช้ของคุณและได้รับการป้องกันการลบจากระบบปฏิบัติการ (Windows deny-DELETE ACL / macOS uchg; Linux ต้องใช้ root) รหัสผ่านหลักถูกลงทะเบียนในที่เก็บข้อมูลรับรองของระบบปฏิบัติการเพื่อให้ nssh แก้ชื่อ pass-env ได้โดยไม่ต้องเปิด shell นี้".into());

    m.insert("help.deadlock.example1".into(), "  deadlock".into());

    m.insert(
        "help.deadlock.example2".into(),
        "  deadlock > add prod_pass CatLoveFish360@house".into(),
    );
}
