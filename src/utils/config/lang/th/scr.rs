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

    // ── ภาษาเขียนสคริปต์ ────────────────────────────────────────────────
    m.insert("scr.help_syntax_header".into(), "ไวยากรณ์:".into());

    m.insert(
        "scr.help_direct".into(),
        " <file.ntc> [args...] เรียกสคริปต์โดยไม่ต้องพิมพ์ scr".into(),
    );

    m.insert(
        "scr.desc_expr".into(),
        " set NAME = <expr> ค่า การคำนวณ การเรียกฟังก์ชัน".into(),
    );

    m.insert(
        "scr.desc_if".into(),
        " if <เงื่อนไข> / else if / else / end บล็อกเงื่อนไข".into(),
    );

    m.insert(
        "scr.desc_while".into(),
        " while <เงื่อนไข> / end วนซ้ำขณะเงื่อนไขเป็นจริง".into(),
    );

    m.insert(
        "scr.desc_for".into(),
        " for VAR in <รายการ> / end วนซ้ำตามคำ".into(),
    );

    m.insert(
        "scr.desc_loop".into(),
        " loop [จำนวน] / end ทำซ้ำตามจำนวน (ค่าเริ่มต้น: จนกว่าจะ break)".into(),
    );

    m.insert(
        "scr.desc_break".into(),
        " break / continue ออกจากลูป หรือข้ามไปรอบถัดไป".into(),
    );

    m.insert(
        "scr.desc_func".into(),
        " func NAME(พารามิเตอร์) / end ประกาศฟังก์ชัน".into(),
    );

    m.insert(
        "scr.desc_call".into(),
        " call NAME(อาร์กิวเมนต์) เรียกฟังก์ชัน".into(),
    );

    m.insert(
        "scr.desc_return".into(),
        " return [ค่า] ออกจากฟังก์ชันพร้อมค่า".into(),
    );

    m.insert(
        "scr.desc_input".into(),
        " input NAME [ข้อความ] อ่านหนึ่งบรรทัดเข้าตัวแปร".into(),
    );

    m.insert("scr.desc_exit".into(), " exit หยุดสคริปต์".into());

    m.insert("scr.desc_functions_header".into(), "ฟังก์ชันในนิพจน์:".into());

    m.insert(
        "scr.desc_fn_nprintf".into(),
        " nprintf(fmt, ...) จัดรูปแบบเหมือนคำสั่ง nprintf".into(),
    );

    m.insert(
        "scr.desc_fn_strings".into(),
        " len / upper / lower / trim / str ฟังก์ชันข้อความ".into(),
    );

    m.insert(
        "scr.desc_fn_math".into(),
        " int / abs ฟังก์ชันตัวเลข".into(),
    );

    m.insert(
        "scr.desc_fn_env".into(),
        " env(NAME) อ่านตัวแปรสภาพแวดล้อม".into(),
    );

    m.insert("scr.input_prompt".into(), "> ".into());

    m.insert(
        "scr.input_eof".into(),
        "ไม่มีข้อมูลสำหรับ '{}' — หยุดสคริปต์".into(),
    );

    m.insert("scr.parse_error".into(), "{}:{} — {}".into());

    m.insert(
        "scr.parse_failed".into(),
        "สคริปต์ '{}' มีข้อผิดพลาดไวยากรณ์ {} จุด จึงไม่ถูกเรียก".into(),
    );

    m.insert("scr.err_expr".into(), "นิพจน์ไม่ถูกต้อง '{}': {}".into());

    m.insert("scr.err_loop_limit".into(), "ลูปเกิน {} รอบ จึงหยุด".into());

    m.insert(
        "scr.err_func_missing".into(),
        "ไม่รู้จักฟังก์ชัน '{}'".into(),
    );

    m.insert(
        "scr.err_func_args".into(),
        "'{}' ต้องการ {} อาร์กิวเมนต์ แต่ได้ {}".into(),
    );

    m.insert(
        "scr.err_recursion".into(),
        "'{}' เรียกซ้อนเกิน {} ชั้น".into(),
    );

    m.insert(
        "scr.err_break_outside_loop".into(),
        "`break` / `continue` ออกจากฟังก์ชัน '{}' ไม่ได้".into(),
    );

    m.insert(
        "scr.nested_too_deep".into(),
        "สคริปต์ซ้อนกันเกิน {} ชั้น จึงหยุด".into(),
    );

    m.insert(
        "scr.pipe_refused".into(),
        "'{}' ตั้ง `set-pipe = fail` จึงใช้กับไปป์หรือเปลี่ยนเส้นทางไม่ได้".into(),
    );

    m.insert(
        "scr.desc_pipe".into(),
        " set-pipe = on|off อนุญาตให้สคริปต์นี้ถูกไปป์หรือเปลี่ยนเส้นทางหรือไม่".into(),
    );

    m.insert(
        "scr.desc_showexc".into(),
        " set-showexc = on|off แสดงบรรทัด [n] Executing: หรือไม่".into(),
    );

}
