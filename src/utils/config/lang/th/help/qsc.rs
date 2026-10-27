use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qsc.listen".into(),
        "qsc listen ## เริ่มเดม่อน QSC (โพรเซสเบื้องหลัง)".into(),
    );

    m.insert(
        "help.qsc.off".into(),
        "config qsc off ## ปิดการใช้งานปุ่มลัด QSC".into(),
    );

    m.insert(
        "help.qsc.set".into(),
        "config qsc \"#c 0\" ## ตั้งปุ่มลัด (Ctrl+0)".into(),
    );

    m.insert(
        "help.qsc.set_cs".into(),
        "config qsc \"#c #s a\" ## ตั้งปุ่มลัด (Ctrl+Shift+A)".into(),
    );

    m.insert(
        "help.qsc.startup".into(),
        "qsc startup on|off ## ลงทะเบียน/ยกเลิกการลงทะเบียนเป็นโปรแกรมเริ่มต้น".into(),
    );

    m.insert(
        "help.qsc.status".into(),
        "qsc status ## ตรวจสอบว่าเดม่อน QSC ทำงานหรือไม่".into(),
    );

    m.insert("help.qsc.stop".into(), "qsc stop ## หยุดเดม่อน QSC".into());

    m.insert(
        "help.qsc.title".into(),
        "คำสั่ง QSC (ปุ่มลัด):
"
        .into(),
    );

    m.insert(
        "help.qsc.example1".into(),
        "config qsc \"#c 0\" ## ตั้งปุ่มลัด Ctrl+0".into(),
    );

    m.insert(
        "help.qsc.example2".into(),
        "qsc listen ## เริ่มเดม่อน QSC".into(),
    );

    m.insert(
        "help.qsc.example3".into(),
        "qsc status ## ตรวจสอบสถานะเดม่อน".into(),
    );
}
