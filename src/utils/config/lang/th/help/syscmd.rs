use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.syscmd.any".into(),
        "<any command> ## NTC ส่งคำสั่งที่ไม่รู้จักไปยังระบบของคุณ".into(),
    );

    m.insert(
        "help.syscmd.native".into(),
        "shell -- echo, git, dir/ls, ping, เป็นต้น".into(),
    );

    m.insert(
        "help.syscmd.native2".into(),
        "ทั้งหมดทำงาน natively ควบคู่กับคำสั่ง NTC.".into(),
    );

    m.insert(
        "help.syscmd.title".into(),
        "คำสั่งระบบ (SYSTEM COMMANDS):".into(),
    );
}
