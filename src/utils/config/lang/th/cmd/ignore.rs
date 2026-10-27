use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ignore.scope_local".into(), "config local".into());

    m.insert("cmd.ignore.scope_global".into(), "config global".into());

    m.insert(
        "cmd.ignore.scope_help".into(),
        "ใช้ '-g' / '--global' เพื่อเขียนไปยัง config global (ในโฟลเดอร์ที่มี ntconfig.toml)".into(),
    );

    m.insert("cmd.ignore.scope_target".into(), "เป้าหมาย: {}".into());

    m.insert("cmd.ignore.applied".into(), "{} {} รายการ".into());

    m.insert(
        "cmd.ignore.care_example_dir".into(),
        "  care dir build,dist".into(),
    );

    m.insert(
        "cmd.ignore.care_example_ext".into(),
        "  care ext txt,md,json".into(),
    );

    m.insert(
        "cmd.ignore.care_example_file".into(),
        "  care file .env".into(),
    );

    m.insert(
        "cmd.ignore.care_examples_title".into(),
        "ตัวอย่าง care:".into(),
    );

    m.insert(
        "cmd.ignore.check_hint".into(),
        "เรียก 'igcared' เพื่อดูการตั้งค่า ignore/care ปัจจุบัน".into(),
    );

    m.insert(
        "cmd.ignore.dir_desc".into(),
        "ignore dir <ชื่อ>[, ...] ละเว้นไดเรกทอรี".into(),
    );

    m.insert(
        "cmd.ignore.dir_usage".into(),
        "การใช้งาน: ignore dir <ชื่อ>[, <ชื่อ>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.example_dir".into(),
        "  ignore dir temp,logs".into(),
    );

    m.insert(
        "cmd.ignore.example_ext".into(),
        "  ignore ext log,tmp,bak".into(),
    );

    m.insert(
        "cmd.ignore.example_file".into(),
        "  ignore file Cargo.lock".into(),
    );

    m.insert("cmd.ignore.examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.ignore.ext_desc".into(),
        "ignore ext <นามสกุล>[, ...] ละเว้นนามสกุลไฟล์".into(),
    );

    m.insert(
        "cmd.ignore.ext_usage".into(),
        "การใช้งาน: ignore ext <นามสกุล>[, <นามสกุล>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.file_desc".into(),
        "ignore file <ชื่อ>[, ...] ละเว้นไฟล์เฉพาะ".into(),
    );

    m.insert(
        "cmd.ignore.file_usage".into(),
        "การใช้งาน: ignore file <ชื่อ>[, <ชื่อ>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.title".into(),
        "ignore \u{2014} จัดการรายการที่ถูกละเว้น".into(),
    );
}
