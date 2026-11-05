use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "tutorial.complete_msg".into(),
        "ตอนนี้คุณรู้พื้นฐานของ NTC แล้ว:".into(),
    );

    m.insert(
        "tutorial.complete_title".into(),
        " บทเรียนเสร็จสมบูรณ์! ".into(),
    );

    m.insert("tutorial.demonstrating".into(), "กำลังสาธิต:".into());

    m.insert(
        "tutorial.exit_hint".into(),
        "พิมพ์ 'exit' เพื่อออกจาก NTC.".into(),
    );

    m.insert(
        "tutorial.help_hint".into(),
        "พิมพ์ 'help' สำหรับเอกสารอ้างอิงคำสั่งทั้งหมด.".into(),
    );

    m.insert(
        "tutorial.prompt_continue".into(),
        "กด Enter เพื่อดำเนินการต่อ".into(),
    );

    m.insert("tutorial.prompt_exit_label".into(), "ออก".into());

    m.insert("tutorial.prompt_skip_label".into(), "ข้าม".into());

    m.insert("tutorial.return_hint".into(), "กด Enter เพื่อกลับสู่เชลล์".into());

    m.insert(
        "tutorial.step_title".into(),
        " บทเรียน NTC \u{2014} ขั้นตอน {}/{} ".into(),
    );

    m.insert("tutorial.tip_label".into(), "เคล็ดลับ:".into());

    m.insert("tutorial.command_label".into(), "คำสั่ง:".into());

    m.insert(
        "tutorial.command_usage".into(),
        "พิมพ์เองแล้วกด Enter \u{2014} ntc จะไม่รันให้คุณ".into(),
    );
}
