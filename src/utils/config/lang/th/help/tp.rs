use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tp.title".into(), "คำสั่งเทเลพอร์ต:".into());

    m.insert("help.tp.menu".into(), "tp ## เมนูแบบโต้ตอบ".into());

    m.insert(
        "help.tp.add".into(),
        "tp add <name> ## บันทึกไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.tp.add_path".into(),
        "tp add <name> <path> ## บันทึกเส้นทางเฉพาะ".into(),
    );

    m.insert(
        "help.tp.jump".into(),
        "tp jump <name> ## เทเลพอร์ตไปยังตำแหน่งที่บันทึก".into(),
    );

    m.insert(
        "help.tp.jump_num".into(),
        "tp jump <number> ## เทเลพอร์ตตามหมายเลข".into(),
    );

    m.insert(
        "help.tp.list".into(),
        "tp list ## แสดงจุดเทเลพอร์ตทั้งหมด".into(),
    );

    m.insert(
        "help.tp.info".into(),
        "tp info <name> ## แสดงรายละเอียดจุดบันทึก".into(),
    );

    m.insert("help.tp.rm".into(), "tp rm <name> ## ลบจุดเทเลพอร์ต".into());

    m.insert(
        "help.tp.rnm".into(),
        "tp rnm <old> to <new> ## เปลี่ยนชื่อจุดบันทึก".into(),
    );

    m.insert("help.tp.cls".into(), "tp cls ## ล้างจุดบันทึกทั้งหมด".into());

    m.insert("help.tp.at".into(), "@<name> ## ทางลัดเทเลพอร์ตเร็ว".into());

    m.insert(
        "help.tp.example1".into(),
        "tp add web ## บันทึกไดเรกทอรีปัจจุบันเป็น 'web'".into(),
    );

    m.insert(
        "help.tp.example2".into(),
        "tp add docs ~/Documents ## บันทึกเส้นทางเฉพาะ".into(),
    );

    m.insert(
        "help.tp.example3".into(),
        "tp jump web ## เทเลพอร์ตไปยัง 'web'".into(),
    );

    m.insert(
        "help.tp.example4".into(),
        "tp list ## แสดงจุดบันทึกทั้งหมด".into(),
    );

    m.insert("help.tp.example5".into(), "@web ## ทางลัดเทเลพอร์ตเร็ว".into());
}
