use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.where.title".into(), "WHERE คำสั่ง:".into());

    m.insert(
        "help.where.usage".into(),
        "where [-a|--all] [<โปรแกรม>] ## แสดงไฟล์ปฏิบัติการของ ntc หรือค้นหาโปรแกรมใน PATH".into(),
    );

    m.insert(
        "help.where.example1".into(),
        "where -a ## แสดงทุกพาธของ config, session, แพ็กเกจ และ vault ของ ntc".into(),
    );

    m.insert(
        "help.where.example2".into(),
        "where git ## แสดง git ตัวแรกใน PATH (-a แสดงทั้งหมด)".into(),
    );
}
