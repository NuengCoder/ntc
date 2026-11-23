use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tpb.title".into(), "TPB คำสั่ง:".into());

    m.insert(
        "help.tpb.usage".into(),
        "tpb ## เทเลพอร์ตกลับไปยังตำแหน่งก่อนหน้า (เลิกทำ tp)".into(),
    );

    m.insert(
        "help.tpb.history".into(),
        "tpb history ## แสดงประวัติการนำทางเทเลพอร์ต".into(),
    );

    m.insert(
        "help.tpb.clear".into(),
        "tpb clear ## ล้างประวัติเทเลพอร์ต".into(),
    );

    m.insert(
        "help.tpb.example1".into(),
        "tpb ## กลับไปยังตำแหน่งก่อนหน้า".into(),
    );
}
