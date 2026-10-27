use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.git.title".into(), "คำสั่ง Git Integration:".into());

    m.insert(
        "help.git.status".into(),
        "git status | st ## แสดงสถานะ repo แบบอ่านง่าย".into(),
    );

    m.insert(
        "help.git.branch".into(),
        "git branch | b ## รายการ branch (ไฮไลต์ branch ปัจจุบัน)".into(),
    );

    m.insert(
        "help.git.log".into(),
        "git log | l ## แสดง commit ล่าสุดแบบมีสี".into(),
    );

    m.insert(
        "help.git.diff".into(),
        "git diff ## แสดง diff ใน working tree".into(),
    );

    m.insert(
        "help.git.passthrough".into(),
        "git <คำสั่งใดก็ได้> ## คำสั่งย่อยอื่นๆ ส่งต่อไปยัง git จริง".into(),
    );

    m.insert(
        "help.git.example1".into(),
        "git status ## แสดง branch + staged/unstaged/untracked".into(),
    );

    m.insert(
        "help.git.example2".into(),
        "git branch -a ## รายการ branch ทั้งหมด (passthrough)".into(),
    );

    m.insert(
        "help.git.example3".into(),
        "git commit -am \"msg\" ## ใช้ git ปกติ (passthrough)".into(),
    );
}
