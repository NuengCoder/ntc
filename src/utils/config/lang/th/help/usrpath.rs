use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.usrpath.title".into(),
        "ตัวจัดการ PATH:".into(),
    );

    m.insert(
        "help.usrpath.show".into(),
        "usrpath show ## แสดงรายการ PATH ระดับผู้ใช้".into(),
    );

    m.insert(
        "help.usrpath.add".into(),
        "usrpath add <path> ## เพิ่มไดเรกทอรีไปยัง PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.usrpath.add_cwd".into(),
        "usrpath add --cwd | --here ## เพิ่มไดเรกทอรีปัจจุบันไปยัง PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.usrpath.rm".into(),
        "usrpath rm <path-or-number> ## ลบจาก PATH ของผู้ใช้ (ตามเส้นทางหรือดัชนี)".into(),
    );

    m.insert(
        "help.usrpath.move".into(),
        "usrpath move <n> <m> ## ย้ายรายการ #n ไปยังตำแหน่ง #m".into(),
    );

    m.insert(
        "help.usrpath.example1".into(),
        "usrpath show ## แสดงรายการ PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.usrpath.example2".into(),
        "usrpath add ~/bin ## เพิ่มไปยัง PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.usrpath.example3".into(),
        "usrpath rm 2 ## ลบรายการ #2 จาก PATH ของผู้ใช้".into(),
    );
}
