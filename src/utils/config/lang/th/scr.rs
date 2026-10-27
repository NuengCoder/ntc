use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "scr.calling_not_allowed".into(),
        "ไม่อนุญาตให้เรียก 'scr' จากภายในสคริปต์".into(),
    );

    m.insert("scr.cmd_failed".into(), "คำสั่งล้มเหลว".into());

    m.insert(
        "scr.completed".into(),
        "สคริปต์ '{}' เสร็จสมบูรณ์ ({} คำสั่ง)".into(),
    );

    m.insert(
        "scr.continue_warn".into(),
        "ไปต่อหลังเกิดข้อผิดพลาดที่บรรทัด {}".into(),
    );

    m.insert("scr.desc_args".into(), " $1..$9, $@ อาร์กิวเมนต์ตำแหน่ง".into());

    m.insert(
        "scr.desc_force".into(),
        " @ <cmd> บังคับคำสั่งระบบ (ข้ามคำสั่ง ntc)".into(),
    );

    m.insert(
        "scr.desc_interp".into(),
        " $var / ${var} / ${var:-default} การแทนค่าตัวแปร".into(),
    );

    m.insert("scr.desc_set".into(), " set NAME = value กำหนดตัวแปร".into());

    m.insert(
        "scr.done_errors".into(),
        "สคริปต์ '{}' เสร็จสิ้น มี {} ข้อผิดพลาด".into(),
    );

    m.insert("scr.error_at".into(), "ข้อผิดพลาดที่ {}:{} \u{2014} {}".into());

    m.insert("scr.executing".into(), "[{}] กำลังเรียก: {}".into());

    m.insert(
        "scr.failed_abort".into(),
        "สคริปต์ '{}' หยุดที่บรรทัด {} ({} ข้อผิดพลาด)".into(),
    );

    m.insert(
        "scr.help_continue".into(),
        " scr -c <file.ntc> [args...] ไปต่อหลังข้อผิดพลาด".into(),
    );

    m.insert("scr.help_help".into(), " scr help แสดงความช่วยเหลือนี้".into());

    m.insert(
        "scr.help_init".into(),
        " scr init [name] สร้างเทมเพลตสคริปต์".into(),
    );

    m.insert(
        "scr.help_ls".into(),
        " scr ls แสดงรายชื่อ *.ntc ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "scr.help_run".into(),
        " scr <file.ntc> [args...] เรียกใช้สคริปต์อัตโนมัติ".into(),
    );

    m.insert(
        "scr.help_title".into(),
        "scr \u{2014} ตัวเรียกสคริปต์อัตโนมัติของ ntc".into(),
    );

    m.insert("scr.help_usage".into(), "การใช้งาน:".into());

    m.insert("scr.init_created".into(), "สร้างเทมเพลต {} แล้ว".into());

    m.insert("scr.init_exists".into(), "{} มีอยู่แล้ว".into());

    m.insert("scr.ls_empty".into(), "ไม่พบ *.ntc ใน {}".into());

    m.insert("scr.ls_title".into(), "สคริปต์อัตโนมัติ:".into());

    m.insert("scr.no_commands".into(), "{} ไม่มีคำสั่งให้เรียก".into());

    m.insert("scr.not_found".into(), "ไม่พบสคริปต์: {} ({})".into());

    m.insert("scr.start".into(), "กำลังเรียกสคริปต์ {}...".into());

    m.insert("scr.syscmd".into(), "กำลังเรียก: {}".into());

    m.insert("scr.syscmd_exit".into(), "คำสั่งออกด้วยรหัส: {}".into());

    m.insert("scr.syscmd_terminated".into(), "คำสั่งสิ้นสุด (Ctrl+C)".into());
}
