use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.count.title".into(), "COUNT คำสั่ง:".into());

    m.insert(
        "help.count.usage".into(),
        "count <needle> [text] [-l|--line] ## นับจำนวนครั้งที่ <needle> ปรากฏ; -l|--line นับจำนวนบรรทัดที่ pipe เข้ามา".into(),
    );

    m.insert(
        "help.count.example1".into(),
        "gs TODO | count TODO ## นับจำนวนที่ตรงในข้อความที่ pipe เข้ามา".into(),
    );

    m.insert(
        "help.count.example2".into(),
        "gs TODO | count -l ## นับจำนวนบรรทัดทั้งหมดของผลลัพธ์ที่ pipe เข้ามา".into(),
    );
}
