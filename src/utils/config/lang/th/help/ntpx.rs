use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ntpx.title".into(), "NTPX คำสั่ง:".into());

    m.insert(
        "help.ntpx.usage".into(),
        "ntpx <pkg> [bin] [args] ## เรียกไบนารีของแพ็กเกจที่ติดตั้ง".into(),
    );

    m.insert(
        "help.ntpx.example1".into(),
        "ntpx node npm init ## เรียก npm ผ่าน ntpx".into(),
    );
}
