use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "git.no_git_binary".into(),
        "ยังไม่ได้ติดตั้ง Git. ใช้ `ntpkg install git` เพื่อติดตั้ง".into(),
    );

    m.insert(
        "git.not_repo".into(),
        "ไม่ใช่ git repository (หรือไดเรกทอรีแม่)".into(),
    );

    m.insert("git.executing".into(), "กำลังดำเนินการ: {}".into());

    m.insert("git.completed".into(), "คำสั่ง git เสร็จสิ้น".into());

    m.insert("git.exit_code".into(), "คำสั่งออกด้วยรหัส: {}".into());

    m.insert("git.terminated".into(), "คำสั่งถูกยกเลิก".into());

    m.insert("git.failed".into(), "ไม่สามารถดำเนินการคำสั่งได้: {}".into());

    m.insert("git.checking_status".into(), "กำลังตรวจสอบสถานะ".into());

    m.insert("git.status_failed".into(), "ไม่สามารถอ่านสถานะ git ได้".into());

    m.insert("git.branch_header".into(), "บน branch".into());

    m.insert("git.upstream".into(), "ติดตาม {}".into());

    m.insert("git.ahead".into(), "ข้างหน้า {}".into());

    m.insert("git.behind".into(), "ข้างหลัง {}".into());

    m.insert("git.detached_header".into(), "Detached HEAD".into());

    m.insert("git.section_conflicts".into(), "พาธที่ยังไม่รวม:".into());

    m.insert(
        "git.section_staged".into(),
        "การเปลี่ยนแปลงที่พร้อม commit:".into(),
    );

    m.insert(
        "git.section_unstaged".into(),
        "การเปลี่ยนแปลงที่ยังไม่ staged:".into(),
    );

    m.insert("git.section_untracked".into(), "ไฟล์ที่ไม่ได้ติดตาม:".into());

    m.insert("git.clean".into(), "working tree สะอาด".into());

    m.insert(
        "git.branch_failed".into(),
        "ไม่สามารถแสดงรายการ branch ได้".into(),
    );

    m.insert("git.log_failed".into(), "ไม่สามารถอ่าน git log ได้".into());

    m.insert("git.log_empty".into(), "ไม่พบ commit".into());

    m.insert("git.help_title".into(), "GIT INTEGRATION:".into());

    m.insert(
        "git.help_status".into(),
        "git status ## แสดง branch + staged/unstaged/untracked".into(),
    );

    m.insert(
        "git.help_branch".into(),
        "git branch ## รายการ branch".into(),
    );

    m.insert("git.help_log".into(), "git log ## commit ล่าสุดแบบมีสี".into());

    m.insert(
        "git.help_diff".into(),
        "git diff ## diff ของ working tree".into(),
    );

    m.insert(
        "git.help_passthrough".into(),
        "git <ใดก็ได้> ## ที่เหลือส่งต่อไปยัง git จริง".into(),
    );

    m.insert("git.examples_header".into(), "ตัวอย่าง:".into());

    m.insert("git.help_example1".into(), "git status".into());

    m.insert("git.help_example2".into(), "git checkout main".into());
}
