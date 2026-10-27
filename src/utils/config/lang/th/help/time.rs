use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.title".into(),
        "คำสั่ง TIME - จับเวลาและนับถอยหลัง:".into(),
    );

    m.insert("help.time.run".into(), "  time run <คำสั่ง>                       เรียก NTC command, alias หรือคำสั่งภายนอกและรายงานเวลา".into());

    m.insert(
        "help.time.alias".into(),
        "  time -r|--run <คำสั่ง>                  รูปแบบย่อของ run".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "  time -r <คำสั่ง> --compare <คำสั่ง>     เรียกทั้งสองคำสั่งและเปรียบเทียบผลลัพธ์".into(),
    );

    m.insert("help.time.compare_with".into(), "  time compare <คำสั่ง A> --with <คำสั่ง B> [-w]  เปรียบเทียบผลลัพธ์สองคำสั่ง (alias: -r A --compare B)".into());

    m.insert("help.time.bench".into(), "  time bench [--runs N] <คำสั่ง>          เรียกซ้ำ N ครั้ง (ค่าเริ่มต้น 5) และแสดง min/max/mean/median".into());

    m.insert(
        "help.time.count".into(),
        "  time count <ระยะเวลา>                   นับถอยหลัง (เช่น 30, 90s, 5m, 1h30m)".into(),
    );

    m.insert(
        "help.time.help".into(),
        "  time help                               แสดงวิธีใช้นี้".into(),
    );

    m.insert("help.time.note".into(), "คำสั่งทำงานตามลำดับในไดเรกทอรีปัจจุบัน รองรับ NTC command และ alias โหมดเปรียบเทียบ/bench จะเก็บผลลัพธ์ แต่การเปลี่ยนแปลงไฟล์ยังคงอยู่ กด Ctrl+C เพื่อยกเลิกการนับถอยหลัง".into());
}
