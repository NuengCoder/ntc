use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scr.continue".into(),
        "scr -c <file.ntc> [args...] ## เรียกสคริปต์ ไปต่อหลังเกิดข้อผิดพลาด".into(),
    );

    m.insert(
        "help.scr.example1".into(),
        "scr deploy.ntc ## เรียกใช้ deploy.ntc".into(),
    );

    m.insert(
        "help.scr.example2".into(),
        "scr build.ntc dev prod ## ส่ง args $1=dev, $2=prod".into(),
    );

    m.insert(
        "help.scr.example3".into(),
        "scr -c setup.ntc ## เรียกใช้และไปต่อหลังข้อผิดพลาด".into(),
    );

    m.insert(
        "help.scr.help".into(),
        "scr help ## แสดงการใช้งาน scr".into(),
    );

    m.insert(
        "help.scr.init".into(),
        "scr init [name] ## สร้างเทมเพลตสคริปต์".into(),
    );

    m.insert(
        "help.scr.ls".into(),
        "scr ls ## แสดงรายชื่อ *.ntc ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.scr.run".into(),
        "scr <file.ntc> [args...] ## เรียกใช้สคริปต์อัตโนมัติ".into(),
    );

    m.insert("help.scr.title".into(), "คำสั่ง SCR / สคริปต์อัตโนมัติ:".into());
}
