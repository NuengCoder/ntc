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

    m.insert(
        "tutorial.already_done".into(),
        "บทเรียนนี้เสร็จแล้ว ใช้ `tutorial` เพื่อดูเมนู หรือ `tutorial restart` เพื่อทำซ้ำ".into(),
    );
    m.insert("tutorial.also_try_fmt".into(), "ลองเพิ่มเติม: {}".into());
    m.insert(
        "tutorial.blocked_cmd".into(),
        "`{}` ถูกบล็อกในโหมดบทช่วยสอน ใช้คำสั่งของบทเรียนหรือ `tutorial`".into(),
    );
    m.insert(
        "tutorial.blocked_outside".into(),
        "คุณอยู่นอกโฟลเดอร์บทช่วยสอน ใช้ `cd {}` เพื่อกลับ หรือ `tutorial exit` เพื่อออกจากโหมดบทช่วยสอน".into(),
    );
    m.insert("tutorial.goal_label".into(), "เป้าหมาย:".into());
    m.insert("tutorial.hint_label".into(), "คำใบ้:".into());
    m.insert(
        "tutorial.lesson_done".into(),
        "จบบทเรียน: {} — เลือกบทถัดไปด้วย `tutorial`".into(),
    );
    m.insert(
        "tutorial.menu_goal".into(),
        "พิมพ์ `tutorial <lesson>` หนึ่งครั้ง แล้วทำเป้าหมายด้วยตนเอง".into(),
    );
    m.insert(
        "tutorial.menu_hint".into(),
        "ติดขัด: tutorial hint · ข้าม: tutorial skip · สถานะ: tutorial status · ออก: tutorial exit"
            .into(),
    );
    m.insert(
        "tutorial.menu_title".into(),
        "บทช่วยสอน NTC — เลือกบทเรียน".into(),
    );
    m.insert(
        "tutorial.mode_off".into(),
        "ปิดโหมดบทช่วยสอนแล้ว กลับไปที่ {}".into(),
    );
    m.insert(
        "tutorial.mode_on".into(),
        "โหมดบทช่วยสอนเปิดอยู่ — คุณพิมพ์คำสั่งเอง; ntc เพียงตรวจเป้าหมาย".into(),
    );
    m.insert(
        "tutorial.not_active".into(),
        "โหมดบทช่วยสอนไม่ได้เปิดอยู่ รัน `tutorial` เพื่อเริ่ม".into(),
    );
    m.insert(
        "tutorial.not_in_lesson".into(),
        "ยังไม่ได้เลือกบทเรียน เลือกด้วย `tutorial <lesson>`".into(),
    );
    m.insert("tutorial.prompt_fmt".into(), "tutorial [{} {}/{}] ".into());
    m.insert("tutorial.prompt_none".into(), "tutorial ".into());
    m.insert("tutorial.skipped".into(), "ข้ามแล้ว".into());
    m.insert(
        "tutorial.status_fmt".into(),
        "บทเรียน {} · ขั้นตอน {}/{}".into(),
    );
    m.insert("tutorial.step_done".into(), "ขั้นตอนเสร็จแล้ว".into());
    m.insert("tutorial.step_label".into(), "ขั้นตอน {}/{}".into());
    m.insert(
        "tutorial.unknown_lesson".into(),
        "ไม่รู้จักบทเรียน '{}' บทที่ใช้ได้: {}".into(),
    );
    m.insert("tutorial.usage".into(), "วิธีใช้:\n  tutorial                 เริ่มแซนด์บ็อกซ์บทช่วยสอนและแสดงเมนูบทเรียน\n  tutorial <lesson>        เริ่มบทเรียน (คีย์หรือหมายเลข)\n  tutorial list            แสดงเมนูบทเรียน\n  tutorial hint            คำใบ้หนึ่งอย่างสำหรับขั้นตอนปัจจุบัน\n  tutorial skip            ข้ามขั้นตอนปัจจุบัน\n  tutorial restart         เริ่มบทเรียนปัจจุบันใหม่\n  tutorial status          บทเรียนและขั้นตอนปัจจุบัน\n  tutorial exit            ออกจากโหมดบทช่วยสอน\n\nบทเรียน: รัน `tutorial list` เพื่อดูเมนูทั้งหมด (ทุกคำสั่ง ntc มีบทเรียน)\nในโหมดบทช่วยสอนจะรันเฉพาะคำสั่งในแล็บ; การออกจากโฟลเดอร์บทช่วยสอน\nจะบล็อกทุกอย่างยกเว้น cd, tutorial, help และ exit".into());
    m.insert("tutorial.ws_ready".into(), "พื้นที่ทำงานบทช่วยสอน: {}".into());
}
