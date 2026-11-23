use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gosc.title".into(), "GOSC คำสั่ง:".into());

    m.insert(
        "help.gosc.usage".into(),
        "gosc ## นำทางต่อเนื่อง (0 เพื่อออก v/s/vs เพื่อดู)".into(),
    );

    m.insert(
        "help.gosc.example1".into(),
        "gosc ## ไล่ดูไดเรกทอรีย่อยต่อเนื่อง".into(),
    );
}
