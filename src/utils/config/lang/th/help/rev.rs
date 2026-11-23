use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rev.title".into(), "REV คำสั่ง:".into());

    m.insert(
        "help.rev.usage".into(),
        "rev [-u] [file] ## กลับตัวอักษรของแต่ละบรรทัด หรือสลับลำดับบรรทัดด้วย -u".into(),
    );

    m.insert(
        "help.rev.example1".into(),
        "rev notes.txt ## กลับลำดับตัวอักษรในแต่ละบรรทัด".into(),
    );

    m.insert(
        "help.rev.example2".into(),
        "rev -u notes.txt ## พลิกบนลงล่าง: บรรทัดสุดท้ายขึ้นก่อน บรรทัดแรกไปท้าย".into(),
    );
}
