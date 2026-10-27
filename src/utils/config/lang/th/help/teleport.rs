use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.teleport.add".into(),
        "tp add <name> ## บันทึกไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.teleport.add_path".into(),
        "tp add <name> <path> ## บันทึกเส้นทางเฉพาะ".into(),
    );

    m.insert(
        "help.teleport.at".into(),
        "@<name> ## ทางลัดเทเลพอร์ตเร็ว".into(),
    );

    m.insert(
        "help.teleport.cls".into(),
        "tp cls ## ล้างจุดบันทึกทั้งหมด".into(),
    );

    m.insert(
        "help.teleport.info".into(),
        "tp info <name> ## แสดงรายละเอียดจุดบันทึก".into(),
    );

    m.insert(
        "help.teleport.jump".into(),
        "tp jump <name> ## เทเลพอร์ตไปยังตำแหน่งที่บันทึก".into(),
    );

    m.insert(
        "help.teleport.jump_num".into(),
        "tp jump <number> ## เทเลพอร์ตตามหมายเลข".into(),
    );

    m.insert(
        "help.teleport.list".into(),
        "tp list ## แสดงจุดเทเลพอร์ตทั้งหมด".into(),
    );

    m.insert("help.teleport.menu".into(), "tp ## เมนูแบบโต้ตอบ".into());

    m.insert(
        "help.teleport.rm".into(),
        "tp rm <name> ## ลบจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "help.teleport.rnm".into(),
        "tp rnm <old> to <new> ## เปลี่ยนชื่อจุดบันทึก".into(),
    );

    m.insert("help.teleport.title".into(), "คำสั่งเทเลพอร์ต:".into());

    m.insert(
        "help.teleport.tpb".into(),
        "tpb ## เทเลพอร์ตกลับไปยังตำแหน่งก่อนหน้า (เลิกทำ tp)".into(),
    );

    m.insert(
        "help.teleport.tpb_clear".into(),
        "tpb clear ## ล้างประวัติเทเลพอร์ต".into(),
    );

    m.insert(
        "help.teleport.tpb_history".into(),
        "tpb history ## แสดงประวัติการนำทางเทเลพอร์ต".into(),
    );

    m.insert(
        "help.teleport.example1".into(),
        "tp add web ## บันทึกไดเรกทอรีปัจจุบันเป็น 'web'".into(),
    );

    m.insert(
        "help.teleport.example2".into(),
        "tp add docs ~/Documents ## บันทึกเส้นทางเฉพาะ".into(),
    );

    m.insert(
        "help.teleport.example3".into(),
        "tp jump web ## เทเลพอร์ตไปยัง 'web'".into(),
    );

    m.insert(
        "help.teleport.example4".into(),
        "tp list ## แสดงจุดบันทึกทั้งหมด".into(),
    );

    m.insert(
        "help.teleport.example5".into(),
        "@web ## ทางลัดเทเลพอร์ตเร็ว".into(),
    );
}
