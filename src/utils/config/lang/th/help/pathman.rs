use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.pathman.syspath_add".into(),
        "syspath add <path> ## เพิ่มไดเรกทอรีไปยัง PATH ของระบบ".into(),
    );

    m.insert(
        "help.pathman.syspath_add_cwd".into(),
        "syspath add --cwd | --here ## เพิ่มไดเรกทอรีปัจจุบันไปยัง PATH ของระบบ".into(),
    );

    m.insert(
        "help.pathman.syspath_move".into(),
        "syspath move <n> <m> ## ย้ายรายการ #n ไปยังตำแหน่ง #m".into(),
    );

    m.insert(
        "help.pathman.syspath_rm".into(),
        "syspath rm <path-or-number> ## ลบจาก PATH ของระบบ (ตามเส้นทางหรือดัชนี)".into(),
    );

    m.insert(
        "help.pathman.syspath_show".into(),
        "syspath show ## แสดงรายการ PATH ระดับระบบ".into(),
    );

    m.insert(
        "help.pathman.title".into(),
        "ตัวจัดการ PATH (Windows):".into(),
    );

    m.insert(
        "help.pathman.usrpath_add".into(),
        "usrpath add <path> ## เพิ่มไดเรกทอรีไปยัง PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.pathman.usrpath_add_cwd".into(),
        "usrpath add --cwd | --here ## เพิ่มไดเรกทอรีปัจจุบันไปยัง PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.pathman.usrpath_move".into(),
        "usrpath move <n> <m> ## ย้ายรายการ #n ไปยังตำแหน่ง #m".into(),
    );

    m.insert(
        "help.pathman.usrpath_rm".into(),
        "usrpath rm <path-or-number> ## ลบจาก PATH ของผู้ใช้ (ตามเส้นทางหรือดัชนี)".into(),
    );

    m.insert(
        "help.pathman.usrpath_show".into(),
        "usrpath show ## แสดงรายการ PATH ระดับผู้ใช้".into(),
    );

    m.insert(
        "help.pathman.example1".into(),
        "usrpath show ## แสดงรายการ PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.pathman.example2".into(),
        "usrpath add C:\\tools ## เพิ่มไปยัง PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.pathman.example3".into(),
        "usrpath rm 2 ## ลบรายการ #2 จาก PATH ของผู้ใช้".into(),
    );

    m.insert(
        "help.pathman.example4".into(),
        "syspath show ## แสดงรายการ PATH ของระบบ".into(),
    );

    m.insert(
        "help.pathman.example5".into(),
        "syspath add C:\\bin ## เพิ่มไปยัง PATH ของระบบ".into(),
    );

    m.insert(
        "help.pathman.example6".into(),
        "syspath move 3 1 ## ย้ายรายการไปยังตำแหน่ง 1".into(),
    );
}
