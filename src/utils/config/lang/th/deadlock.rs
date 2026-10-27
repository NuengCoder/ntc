use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "deadlock.no_args".into(),
        "deadlock เปิด shell ของตัวเองและไม่รับอาร์กิวเมนต์ (ใช้ --help เพื่อดูคำสั่ง)".into(),
    );

    m.insert("deadlock.created".into(), "สร้าง vault ใหม่ที่ {} แล้ว".into());

    m.insert(
        "deadlock.default_master_hint".into(),
        "รหัสผ่านหลักเริ่มต้น: {} - ระบบจะให้คุณเปลี่ยน".into(),
    );

    m.insert("deadlock.master_prompt".into(), "master-pass: ".into());

    m.insert(
        "deadlock.wrong_master".into(),
        "รหัสผ่านหลักไม่ถูกต้อง (ครั้งที่ {}/3)".into(),
    );

    m.insert(
        "deadlock.too_many_attempts".into(),
        "ใส่รหัสผ่านหลักผิดหลายครั้งเกินไป - ยกเลิก".into(),
    );

    m.insert(
        "deadlock.force_change_notice".into(),
        "ยังใช้รหัสผ่านหลักเริ่มต้นอยู่ กรุณาตั้งรหัสใหม่ตอนนี้".into(),
    );

    m.insert(
        "deadlock.new_master_prompt".into(),
        "new master-pass: ".into(),
    );

    m.insert(
        "deadlock.confirm_master_prompt".into(),
        "confirm new master-pass: ".into(),
    );

    m.insert("deadlock.empty_master".into(), "รหัสผ่านหลักต้องไม่ว่าง".into());

    m.insert(
        "deadlock.default_master_rejected".into(),
        "กรุณาเลือกรหัสผ่านหลักอื่นที่ไม่ใช่ค่าเริ่มต้น".into(),
    );

    m.insert(
        "deadlock.master_mismatch".into(),
        "รหัสผ่านไม่ตรงกัน ลองอีกครั้ง".into(),
    );

    m.insert(
        "deadlock.master_changed".into(),
        "เปลี่ยนรหัสผ่านหลักแล้ว และเข้ารหัส vault ใหม่แล้ว".into(),
    );

    m.insert(
        "deadlock.old_master_prompt".into(),
        "old master-pass: ".into(),
    );

    m.insert(
        "deadlock.wrong_master_once".into(),
        "รหัสผ่านหลักไม่ถูกต้อง".into(),
    );

    m.insert("deadlock.unlocked".into(), "ปลดล็อก vault แล้ว".into());

    m.insert(
        "deadlock.add_usage".into(),
        "วิธีใช้: add <pass-env> <real-password>".into(),
    );

    m.insert(
        "deadlock.invalid_name".into(),
        "ชื่อ pass-env '{}' ไม่ถูกต้อง (ใช้ได้เฉพาะตัวอักษร ตัวเลข _ - .)".into(),
    );

    m.insert("deadlock.value_prompt".into(), "real-password: ".into());

    m.insert("deadlock.empty_password".into(), "รหัสผ่านต้องไม่ว่าง".into());

    m.insert("deadlock.added".into(), "บันทึก pass-env '{}' แล้ว".into());

    m.insert("deadlock.updated".into(), "อัปเดต pass-env '{}' แล้ว".into());

    m.insert("deadlock.ls_title".into(), "PASS-ENV -> รหัสผ่านจริง:".into());

    m.insert(
        "deadlock.ls_empty".into(),
        "vault ว่างอยู่ เพิ่มด้วย: add <pass-env> <real-password>".into(),
    );

    m.insert("deadlock.rmpw_usage".into(), "วิธีใช้: rmpw <pass-env>".into());

    m.insert("deadlock.removed".into(), "ลบ pass-env '{}' แล้ว".into());

    m.insert("deadlock.not_found".into(), "ไม่พบ pass-env ชื่อ '{}'".into());

    m.insert(
        "deadlock.unknown_cmd".into(),
        "ไม่รู้จักคำสั่ง deadlock '{}'".into(),
    );

    m.insert(
        "deadlock.hint_help".into(),
        "พิมพ์ 'help' เพื่อดูคำสั่งของ deadlock".into(),
    );

    m.insert("deadlock.bye".into(), "ปิด deadlock แล้ว".into());

    m.insert(
        "deadlock.editor_fail".into(),
        "ไม่สามารถเริ่มตัวแก้ไขบรรทัด: {}".into(),
    );

    m.insert("deadlock.keychain_registered".into(), "ลงทะเบียนรหัสผ่านหลักในที่เก็บข้อมูลรับรองของระบบปฏิบัติการแล้ว (nssh ปลดล็อก vault ได้; ใช้ 'forget' เพื่อลบ)".into());

    m.insert(
        "deadlock.keychain_unavailable".into(),
        "ที่เก็บข้อมูลรับรองของระบบปฏิบัติการไม่พร้อมใช้งาน - nssh จะถามรหัสผ่านหลัก".into(),
    );

    m.insert(
        "deadlock.forget_done".into(),
        "ลบรหัสผ่านหลักออกจากที่เก็บข้อมูลรับรองของระบบปฏิบัติการแล้ว".into(),
    );

    m.insert(
        "deadlock.forget_failed".into(),
        "ไม่สามารถลบรหัสผ่านหลักออกจากที่เก็บข้อมูลรับรองของระบบปฏิบัติการได้: {}".into(),
    );

    m.insert(
        "deadlock.destroy_confirm".into(),
        "ลบ vault และความลับทั้งหมดอย่างถาวรหรือไม่?".into(),
    );

    m.insert(
        "deadlock.destroy_cancelled".into(),
        "ยกเลิกการลบ vault แล้ว".into(),
    );

    m.insert("deadlock.destroyed".into(), "ลบ vault แล้ว".into());

    m.insert(
        "deadlock.destroy_failed".into(),
        "ไม่สามารถลบ vault ได้: {}".into(),
    );

    m.insert(
        "deadlock.os_protected".into(),
        "ไฟล์ vault ได้รับการป้องกันการลบจากระบบปฏิบัติการแล้ว".into(),
    );

    m.insert(
        "deadlock.os_unprotected".into(),
        "ระบบปฏิบัติการ/บัญชีนี้ไม่รองรับการป้องกันการลบ (สิทธิ์ไฟล์ยังมีผลอยู่)".into(),
    );

    m.insert(
        "deadlock.vault_missing".into(),
        "ยังไม่มี vault ของ deadlock - รัน `deadlock` หนึ่งครั้งเพื่อสร้าง".into(),
    );
}
