use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cd.title".into(), "CD คำสั่ง:".into());

    m.insert(
        "help.cd.usage".into(),
        "cd <path> ## นำทางไปยังไดเรกทอรี".into(),
    );

    m.insert(
        "help.cd.cd_tp".into(),
        "cd to <tp_name> ## เทเลพอร์ตไปยังจุดบันทึกที่บันทึกไว้".into(),
    );

    m.insert(
        "help.cd.example1".into(),
        "cd src ## นำทางไปยังไดเรกทอรี src".into(),
    );

    m.insert(
        "help.cd.example2".into(),
        "cd to web ## เทเลพอร์ตไปยังจุดบันทึก 'web'".into(),
    );
}
