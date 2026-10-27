use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.target.not_dir".into(), "'{}' ไม่ใช่ไดเรกทอรี".into());

    m.insert("cmd.target.not_found".into(), "ไม่พบเส้นทาง '{}'".into());

    m.insert("cmd.target.tp_not_found".into(), "ไม่พบจุดบันทึก '{}'".into());
}
