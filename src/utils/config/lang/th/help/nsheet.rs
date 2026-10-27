use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nsheet.title".into(), "คำสั่ง NSHEET:".into());

    m.insert(
        "help.nsheet.open".into(),
        "nsheet open <ไฟล์> [ตัวเลือก] ## ดูตารางแบบเต็มจอ (CSV/TSV/JSON)".into(),
    );

    m.insert(
        "help.nsheet.view".into(),
        "nsheet view <ไฟล์> [ตัวเลือก] ## พิมพ์ตาราง (ค่าเริ่มต้น 50 แถว)".into(),
    );

    m.insert(
        "help.nsheet.sort".into(),
        "nsheet view <ไฟล์> --sort <คอลัมน์> [--desc] ## เรียงตามคอลัมน์".into(),
    );

    m.insert(
        "help.nsheet.filter".into(),
        "nsheet view <ไฟล์> --filter <เงื่อนไข> ## กรองแถว (col~ข้อความ, col=ค่า, col>ตัวเลข)".into(),
    );

    m.insert(
        "help.nsheet.export".into(),
        "nsheet view <ไฟล์> --export <json|md> [-o <ไฟล์>] ## ส่งออก".into(),
    );

    m.insert(
        "help.nsheet.tui".into(),
        "ปุ่มเต็มจอ: ลูกศร/hjkl เลื่อน · / ค้นหา · n ถัดไป · s เรียง · f กรอง · e ส่งออก · Enter ดู · q ออก"
            .into(),
    );

    m.insert(
        "help.nsheet.example1".into(),
        "nsheet open users.csv ## เปิดดูตาราง".into(),
    );

    m.insert(
        "help.nsheet.example2".into(),
        "nsheet view data.json --filter \"age>30\" --sort age ## กรองและเรียง".into(),
    );

    m.insert(
        "help.nsheet.example3".into(),
        "nsheet view a.csv --export md -o out.md ## ส่งออก Markdown".into(),
    );
}
