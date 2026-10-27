use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.nav.back".into(),
        "back [n] ## กลับไปยังไดเรกทอรีหลัก (หรือ n ระดับ)".into(),
    );

    m.insert(
        "help.nav.cd".into(),
        "cd <path> ## นำทางไปยังไดเรกทอรี".into(),
    );

    m.insert(
        "help.nav.cd_tp".into(),
        "cd to <tp_name> ## เทเลพอร์ตไปยังจุดบันทึกที่บันทึกไว้".into(),
    );

    m.insert(
        "help.nav.fgo".into(),
        "fgo <pattern> [-d <n>] ## ค้นหาไฟล์ นำทางไปยังไดเรกทอรีหลักของไฟล์ที่เลือก".into(),
    );

    m.insert(
        "help.nav.godrive".into(),
        "godrive ## แสดงรายการและเลือกไดรฟ์ Windows".into(),
    );

    m.insert(
        "help.nav.gos".into(),
        "gos ## แสดงรายการไดเรกทอรีย่อยและเลือกหนึ่ง".into(),
    );

    m.insert(
        "help.nav.gosc".into(),
        "gosc ## นำทางต่อเนื่อง (0 เพื่อออก v/s/vs เพื่อดู)".into(),
    );

    m.insert("help.nav.title".into(), "คำสั่งนำทาง (NAVIGATION):".into());

    m.insert(
        "help.nav.where".into(),
        "where [<โปรแกรม>] ## แสดงตำแหน่งของ ntc หรือค้นหาโปรแกรมใน PATH".into(),
    );

    m.insert(
        "help.nav.example1".into(),
        "cd src ## นำทางไปยังไดเรกทอรี src".into(),
    );

    m.insert(
        "help.nav.example2".into(),
        "back ## กลับไปยังไดเรกทอรีหลัก".into(),
    );

    m.insert(
        "help.nav.example3".into(),
        "gos ## เลือกไดเรกทอรีย่อยแบบโต้ตอบ".into(),
    );

    m.insert(
        "help.nav.example4".into(),
        "cd to web ## เทเลพอร์ตไปยังจุดบันทึก 'web'".into(),
    );

    m.insert(
        "help.nav.example5".into(),
        "where git ## ค้นหาตำแหน่งของ git ใน PATH".into(),
    );
}
