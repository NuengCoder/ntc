use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sort.title".into(), "SORT คำสั่ง:".into());

    m.insert(
        "help.sort.usage".into(),
        "sort [-r] [-n] [-u] [-f] [file] ## เรียงบรรทัด (ย้อนกลับ/ตัวเลข/ไม่ซ้ำ/ไม่สนตัวพิมพ์)".into(),
    );

    m.insert(
        "help.sort.example1".into(),
        "gs error | sort -u | head 10 ## เรียงผลลัพธ์ที่ไม่ซ้ำ แล้วเอา 10 บรรทัดแรก".into(),
    );
}
