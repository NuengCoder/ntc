use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.bench".into(),
        "time --bench <n> <คำสั่ง> ## เรียกซ้ำ n ครั้ง และแสดง min/max/mean/median".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "time <คำสั่ง A> --compare <คำสั่ง B> [-w] ## เรียกทั้งสองและเปรียบเทียบสถานะออกกับผลลัพธ์".into(),
    );

    m.insert(
        "help.time.count".into(),
        "time --count <ระยะเวลา> ## นับถอยหลัง (เช่น 30, 90s, 5m, 1h30m)".into(),
    );

    m.insert("help.time.help".into(), "time --help ## แสดงวิธีใช้นี้".into());

    m.insert(
        "help.time.note".into(),
        "คำสั่งทำงานในไดเรกทอรีปัจจุบัน รองรับ NTC command, alias หรือคำสั่งภายนอก --compare จะเก็บผลลัพธ์ และ --bench จะเรียกคำสั่งซ้ำพร้อมผลข้างเคียงซ้ำด้วย กด Ctrl+C เพื่อยกเลิกการนับถอยหลัง".into(),
    );

    m.insert(
        "help.time.run".into(),
        "time <คำสั่ง> ## เรียกคำสั่งและรายงานเวลาที่ใช้".into(),
    );

    m.insert(
        "help.time.title".into(),
        "คำสั่ง TIME - จับเวลา เปรียบเทียบ และนับถอยหลัง:".into(),
    );
}
