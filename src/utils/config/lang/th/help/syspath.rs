use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.syspath.title".into(), "SYSPATH คำสั่ง:".into());

    m.insert(
        "help.syspath.show".into(),
        "syspath show ## แสดงรายการ PATH ระดับระบบ".into(),
    );

    m.insert(
        "help.syspath.add".into(),
        "syspath add <path> ## เพิ่มไดเรกทอรีไปยัง PATH ของระบบ".into(),
    );

    m.insert(
        "help.syspath.add_cwd".into(),
        "syspath add --cwd | --here ## เพิ่มไดเรกทอรีปัจจุบันไปยัง PATH ของระบบ".into(),
    );

    m.insert(
        "help.syspath.rm".into(),
        "syspath rm <path-or-number> ## ลบจาก PATH ของระบบ (ตามเส้นทางหรือดัชนี)".into(),
    );

    m.insert(
        "help.syspath.move".into(),
        "syspath move <n> <m> ## ย้ายรายการ #n ไปยังตำแหน่ง #m".into(),
    );

    m.insert(
        "help.syspath.example1".into(),
        "syspath show ## แสดงรายการ PATH ของระบบ".into(),
    );

    m.insert(
        "help.syspath.example2".into(),
        "syspath add /usr/local/bin ## เพิ่มไปยัง PATH ของระบบ".into(),
    );

    m.insert(
        "help.syspath.example3".into(),
        "syspath move 3 1 ## ย้ายรายการไปยังตำแหน่ง 1".into(),
    );
}
