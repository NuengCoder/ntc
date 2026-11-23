use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.betw.title".into(), "BETW คำสั่ง:".into());

    m.insert(
        "help.betw.usage".into(),
        "betw A:B [file] ## แสดงบรรทัดที่ A ถึง B (เริ่มที่ 1 รวมปลาย)".into(),
    );

    m.insert(
        "help.betw.example1".into(),
        "betw 10:20 app.log ## แสดงบรรทัด 10-20 ของ app.log".into(),
    );
}
