use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "autoup.added".into(),
        "เพิ่มรายการเริ่มต้น: {} \u{2192} {}".into(),
    );

    m.insert(
        "autoup.already_disabled".into(),
        "รายการ '{}' ถูกปิดใช้งานแล้ว.".into(),
    );

    m.insert(
        "autoup.already_enabled".into(),
        "รายการ '{}' เปิดใช้งานแล้ว.".into(),
    );

    m.insert(
        "autoup.already_exists".into(),
        "รายการ '{}' มีอยู่แล้วใน startup ของผู้ใช้.".into(),
    );

    m.insert("autoup.cmd_empty".into(), "ค่าคำสั่งต้องไม่ว่างเปล่า.".into());

    m.insert("autoup.disabled".into(), "ปิดใช้งานรายการเริ่มต้น: {}".into());

    m.insert(
        "autoup.disabled_system".into(),
        "ปิดใช้งานรายการเริ่มต้นของระบบ: {}".into(),
    );

    m.insert("autoup.enabled".into(), "เปิดใช้งานรายการเริ่มต้น: {}".into());

    m.insert(
        "autoup.enabled_system".into(),
        "เปิดใช้งานรายการเริ่มต้นของระบบ: {}".into(),
    );

    m.insert(
        "autoup.help_add".into(),
        " autoup add <ชื่อ> \"<คำสั่ง>\" เพิ่มรายการเริ่มต้น (HKCU)".into(),
    );

    m.insert(
        "autoup.help_disable".into(),
        " autoup disable <ชื่อ> ปิดใช้งานรายการเริ่มต้น".into(),
    );

    m.insert(
        "autoup.help_enable".into(),
        " autoup enable <ชื่อ> เปิดใช้งานรายการที่ปิดไว้ใหม่".into(),
    );

    m.insert(
        "autoup.help_help".into(),
        " autoup help แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "autoup.help_list".into(),
        " autoup list แสดงรายการเริ่มต้นทั้งหมดจากทุกแหล่ง".into(),
    );

    m.insert(
        "autoup.help_rm".into(),
        " autoup rm <ชื่อ> ลบรายการเริ่มต้น".into(),
    );

    m.insert(
        "autoup.hklm_hint".into(),
        "'{}' เป็นรายการระบบ (HKLM). รัน 'mode admin' ก่อน แล้วลองใหม่.".into(),
    );

    m.insert(
        "autoup.hklm_hint_short".into(),
        "'{}' เป็นรายการระบบ. รัน 'mode admin' ก่อน แล้วลองใหม่.".into(),
    );

    m.insert("autoup.list_empty".into(), "(ไม่พบรายการเริ่มต้น)".into());

    m.insert("autoup.list_title".into(), "รายการเริ่มต้นของ Windows".into());

    m.insert(
        "autoup.list_total".into(),
        "รวม: {} [A]:{} [D]:{} (CU:{} LM:{} SF:{})".into(),
    );

    m.insert("autoup.not_found".into(), "ไม่พบรายการเริ่มต้น: {}".into());

    m.insert(
        "autoup.reg_delete_fail".into(),
        "ไม่สามารถลบรายการรีจิสทรี.".into(),
    );

    m.insert(
        "autoup.reg_disable_fail".into(),
        "ไม่สามารถตั้งค่าสถานะปิดใช้งาน.".into(),
    );

    m.insert(
        "autoup.reg_exec_fail".into(),
        "ไม่สามารถรัน reg.exe: {}".into(),
    );

    m.insert(
        "autoup.reg_query_fail".into(),
        "ไม่สามารถสอบถามรีจิสทรี: {}".into(),
    );

    m.insert(
        "autoup.reg_write_fail".into(),
        "ไม่สามารถเขียนรายการรีจิสทรี. ลองรันในฐานะ Administrator.".into(),
    );

    m.insert("autoup.removed".into(), "ลบรายการเริ่มต้นแล้ว: {}".into());

    m.insert(
        "autoup.removed_system".into(),
        "ลบรายการเริ่มต้นของระบบแล้ว: {}".into(),
    );

    m.insert(
        "autoup.sf_disable_hint".into(),
        "'{}' อยู่ในโฟลเดอร์ Startup ย้ายทางลัดออกไปเพื่อปิดใช้งาน.".into(),
    );

    m.insert(
        "autoup.sf_rm_hint".into(),
        "'{}' อยู่ในโฟลเดอร์ Startup ลบไฟล์ทางลัดด้วยตนเอง.".into(),
    );

    m.insert(
        "autoup.source_cu".into(),
        " [CU] ผู้ใช้ปัจจุบัน (HKCU\\...\\Run)".into(),
    );

    m.insert(
        "autoup.source_lm".into(),
        " [LM] เครื่องท้องถิ่น (HKLM\\...\\Run) \u{2014} ต้องมีสิทธิ์ admin (mode admin)".into(),
    );

    m.insert(
        "autoup.source_sf".into(),
        " [SF] โฟลเดอร์ Startup (%APPDATA%\\...\\Startup) \u{2014} อ่านอย่างเดียว".into(),
    );

    m.insert("autoup.source_tags".into(), "แท็กแหล่งที่มา:".into());

    m.insert(
        "autoup.usage_add".into(),
        "การใช้งาน: autoup add <ชื่อ> \"<คำสั่ง>\"".into(),
    );

    m.insert(
        "autoup.usage_disable".into(),
        "การใช้งาน: autoup disable <ชื่อ>".into(),
    );

    m.insert(
        "autoup.usage_enable".into(),
        "การใช้งาน: autoup enable <ชื่อ>".into(),
    );

    m.insert("autoup.usage_examples".into(), "ตัวอย่าง:".into());

    m.insert("autoup.usage_rm".into(), "การใช้งาน: autoup rm <ชื่อ>".into());

    m.insert("autoup.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "autoup.use_rm_first".into(),
        "ใช้ 'autoup rm' ก่อนหากต้องการแทนที่.".into(),
    );

    m.insert(
        "autoup.will_run".into(),
        "รายการจะทำงานเมื่อเข้าสู่ระบบครั้งถัดไป.".into(),
    );

    m.insert(
        "autoup.windows_only".into(),
        "autoup เป็นฟีเจอร์เฉพาะ Windows.".into(),
    );
}
