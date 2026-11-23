use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.sys.title".into(),
        "คำสั่งระบบ (SYSTEM COMMANDS):".into(),
    );

    m.insert(
        "help.sys.monitor".into(),
        "sys, top [-l|--log] [-d|--delay [วินาที:-1]] ## ตัวตรวจสอบระบบแบบสด (CPU, หน่วยความจำ, ดิสก์, โพรเซส)".into(),
    );

    m.insert(
        "help.sys.passthrough".into(),
        "<any command> ## NTC ส่งคำสั่งที่ไม่รู้จักไปยังระบบของคุณ".into(),
    );

    m.insert(
        "help.sys.native".into(),
        "shell -- echo, git, dir/ls, ping, เป็นต้น ทั้งหมดทำงาน natively ควบคู่กับคำสั่ง NTC.".into(),
    );

    m.insert(
        "help.sys.example1".into(),
        "sys ## แสดงตัวตรวจสอบระบบ".into(),
    );

    m.insert(
        "help.sys.example2".into(),
        "sys --log --delay 5 ## เพิ่มการใช้งาน CPU/หน่วยความจำ/ดิสก์ทุก 5 วินาที (ค่าเริ่มต้น 15 วินาที) ต่อท้าย log/ntc_sys.log".into(),
    );

    m.insert(
        "help.sys.example3".into(),
        "sys -ld 5 ## เหมือน 'sys -l -d 5': -l/--log เปิดการบันทึก, -d/--delay กำหนดวินาที".into(),
    );
}
