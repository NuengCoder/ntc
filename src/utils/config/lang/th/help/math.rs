use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.math.eval".into(),
        "math <expr> ## ประเมินนิพจน์คณิตศาสตร์".into(),
    );

    m.insert(
        "help.math.file".into(),
        "math file.ntc.math ## คอมไพล์และเรียกใช้ไฟล์คณิตศาสตร์".into(),
    );

    m.insert(
        "help.math.fun".into(),
        "math fun add/rm/edit/info/ls ## จัดการฟังก์ชันคณิตศาสตร์แบบกำหนดเอง".into(),
    );

    m.insert(
        "help.math.timer".into(),
        "math timer [sec] ## จับเวลารอบหรือนับถอยหลังพร้อมสัญญาณเตือน".into(),
    );

    m.insert("help.math.title".into(), "คำสั่งคณิตศาสตร์ (MATH):".into());

    m.insert(
        "help.math.example1".into(),
        "math 3+4*5 ## ประเมินนิพจน์เลขคณิต".into(),
    );

    m.insert(
        "help.math.example2".into(),
        "math sin(PI/2) ## คณิตศาสตร์พร้อมฟังก์ชัน".into(),
    );

    m.insert(
        "help.math.example3".into(),
        "math timer 30 ## เริ่มนับถอยหลัง 30 วินาที".into(),
    );

    m.insert(
        "help.math.example4".into(),
        "math fun add myfn \"x+1\" ## เพิ่มฟังก์ชันแบบกำหนดเอง".into(),
    );

    m.insert(
        "help.math.example5".into(),
        "math fun ls ## แสดงฟังก์ชันแบบกำหนดเอง".into(),
    );

    m.insert(
        "help.math.example6".into(),
        "math calc.ntc.math ## เรียกไฟล์คณิตศาสตร์".into(),
    );
}
