use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showenv.title".into(), "SHOWENV คำสั่ง:".into());

    m.insert(
        "help.showenv.usage".into(),
        "showenv ## แสดงตัวแปรสภาพแวดล้อม (PATH 5 รายการแรก)".into(),
    );

    m.insert(
        "help.showenv.all".into(),
        "showenv --all ## แสดง PATH ทั้งหมด".into(),
    );

    m.insert(
        "help.showenv.depth".into(),
        "showenv -d <n> ## แสดง n รายการแรกของ PATH".into(),
    );

    m.insert(
        "help.showenv.example1".into(),
        "showenv --all ## แสดงทุกรายการ PATH".into(),
    );
}
