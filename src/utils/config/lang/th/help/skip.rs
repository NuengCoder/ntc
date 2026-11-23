use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.skip.title".into(), "SKIP คำสั่ง:".into());

    m.insert(
        "help.skip.usage1".into(),
        "skip [N] [file] ## แสดงทุกบรรทัดหลัง N บรรทัดแรก (ค่าเริ่มต้น 10)".into(),
    );

    m.insert(
        "help.skip.usage2".into(),
        "skip -x <lines> [file] ## แสดงทุกบรรทัดยกเว้นบรรทัดที่ระบุ (นับจาก 1)".into(),
    );

    m.insert(
        "help.skip.note".into(),
        "ตัวเลขใน -x คั่นด้วยจุลภาคหรือช่องว่าง (2,5,8 หรือ 2 5 8); อินพุตมาจากไฟล์หรือ pipe".into(),
    );

    m.insert(
        "help.skip.example1".into(),
        "skip 2 app.log ## แสดง app.log โดยไม่มี 2 บรรทัดแรก".into(),
    );

    m.insert(
        "help.skip.example2".into(),
        "gs TODO | skip -x 2,5,8 ## ข้ามบรรทัดที่ 2, 5 และ 8 ของผลการค้นหา".into(),
    );
}
