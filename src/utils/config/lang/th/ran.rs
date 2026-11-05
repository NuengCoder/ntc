use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ran.already_exists".into(), "{} มีอยู่แล้วใน {}".into());

    m.insert(
        "ran.calling_not_allowed".into(),
        "ไม่สามารถเรียก 'ran' จากภายใน target ของ NTCRANFILE ได้".into(),
    );

    m.insert("ran.completed".into(), "คำสั่งเสร็จสมบูรณ์.".into());

    m.insert("ran.created".into(), "สร้าง {} แล้ว".into());

    m.insert(
        "ran.desc_aliases".into(),
        " คำสั่งจะถูกขยายผ่านระบบ run-alias ของ ntc ดังนั้น alias ทำงานได้ด้วย.".into(),
    );

    m.insert(
        "ran.desc_aliases2".into(),
        " ขึ้นต้นด้วย 'run ' เพื่อข้ามการขยาย alias และเรียกเป็นคำสั่งระบบโดยตรง.".into(),
    );

    m.insert(
        "ran.desc_deps".into(),
        " Target ขึ้นอยู่กับ target อื่นได้. Dependencies เรียกก่อนตามลำดับ.".into(),
    );

    m.insert(
        "ran.desc_note".into(),
        " ไม่อนุญาตให้เรียก 'ran' จากภายในคำสั่ง target ของ NTCRANFILE.".into(),
    );

    m.insert(
        "ran.desc_reserved".into(),
        " init, deinit, help, list, ls, check, fmt \u{2014} ชื่อเหล่านี้ใช้เป็นชื่อ target ไม่ได้.".into(),
    );

    m.insert(
        "ran.desc_vars".into(),
        " กำหนดตัวแปรภายใต้ [vars] และอ้างอิงด้วย $(VAR_NAME) ในคำสั่ง.".into(),
    );

    m.insert(
        "ran.desc_vars2".into(),
        " ทำงานเหมือนตัวแปร Make แต่ใช้ไวยากรณ์ $(...).".into(),
    );

    m.insert("ran.error_prefix".into(), "ran: {}".into());

    m.insert("ran.exec_failed".into(), "ไม่สามารถเรียกคำสั่ง: {}".into());

    m.insert("ran.executing".into(), "กำลังเรียก: {}".into());

    m.insert("ran.exit_code".into(), "คำสั่งออกด้วยรหัส: {}".into());

    m.insert(
        "ran.get_cwd_failed".into(),
        "ไม่สามารถรับไดเรกทอรีปัจจุบัน: {}".into(),
    );

    m.insert(
        "ran.help_deinit_fmt".into(),
        " ran deinit ลบ {} ออกจากไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert("ran.help_help".into(), " ran help แสดงความช่วยเหลือนี้".into());

    m.insert(
        "ran.help_init_fmt".into(),
        " ran init สร้าง {} ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "ran.help_list".into(),
        " ran list แสดง target ที่มีทั้งหมด".into(),
    );

    m.insert(
        "ran.help_standalone".into(),
        " ran เรียก target 'standalone' (ค่าเริ่มต้น)".into(),
    );

    m.insert(
        "ran.help_target".into(),
        " ran <target> เรียก target เฉพาะ".into(),
    );

    m.insert(
        "ran.help_title".into(),
        "ran \u{2014} ตัวเรียกงานของ ntc (เหมือน Make แต่เป็น ntc-native)".into(),
    );

    m.insert("ran.help_usage".into(), "การใช้งาน:".into());

    m.insert("ran.not_found".into(), "ไม่พบ {} ใน {}".into());

    m.insert("ran.remove_failed".into(), "ไม่สามารถลบ {}: {}".into());

    m.insert("ran.removed".into(), "ลบ {} แล้ว".into());

    m.insert("ran.section_aliases".into(), "Run aliases:".into());

    m.insert("ran.section_deps".into(), "Dependencies:".into());

    m.insert("ran.section_note".into(), "หมายเหตุ:".into());

    m.insert("ran.section_reserved".into(), "ชื่อ target ที่สงวนไว้:".into());

    m.insert("ran.section_vars".into(), "ตัวแปร:".into());

    m.insert("ran.terminated".into(), "คำสั่งสิ้นสุด (Ctrl+C)".into());

    m.insert("ran.check_title".into(), "ran check กำลังวิเคราะห์ {}".into());

    m.insert("ran.check_ok".into(), "{} ไม่มีปัญหา".into());

    m.insert("ran.check_problems".into(), "พบ {} ปัญหาใน {}".into());

    m.insert("ran.check_error".into(), "ข้อผิดพลาด".into());

    m.insert("ran.check_hint".into(), "คำแนะนำ".into());

    m.insert(
        "ran.help_check_fmt".into(),
        " ran check วิเคราะห์ {} และรายงานทุกปัญหาพร้อมบรรทัด".into(),
    );

    m.insert(
        "ran.help_fmt".into(),
        " ran fmt <toml|json|yaml> แปลง ranfile (-o <ไฟล์>, -y ข้ามคำถาม)".into(),
    );

    m.insert(
        "ran.fmt.usage".into(),
        "วิธีใช้: ran fmt <toml|json|yaml> [-o <ไฟล์>] [-y]".into(),
    );

    m.insert(
        "ran.fmt.unknown".into(),
        "ไม่รู้จักรูปแบบ '{}' ใช้ toml, json หรือ yaml".into(),
    );

    m.insert(
        "ran.fmt.current".into(),
        "รูปแบบ ranfile ที่ตั้งไว้: {} (ran init จะสร้าง {})".into(),
    );

    m.insert(
        "ran.fmt.no_file".into(),
        "ไม่พบ ranfile ที่นี่ ตั้งรูปแบบเป็น {} แล้ว; รัน `ran init` เพื่อสร้าง {}".into(),
    );

    m.insert("ran.fmt.already".into(), "{} เป็น {} อยู่แล้ว".into());

    m.insert(
        "ran.fmt.confirm".into(),
        "แปลง {} เป็น {} \u{2192} {}? [y|n] ".into(),
    );

    m.insert(
        "ran.fmt.skipped".into(),
        "ไม่ได้เขียนไฟล์ `ran init` จะสร้าง {}".into(),
    );

    m.insert(
        "ran.fmt.exists".into(),
        "{} มีอยู่แล้ว \u{2014} ลบก่อนหรือใช้ -o <ไฟล์>".into(),
    );

    m.insert("ran.fmt.written".into(), "เขียนแล้ว {}".into());

    m.insert(
        "ran.fmt.old_kept".into(),
        "{} ยังอยู่; `ran` จะเลือก {} ก่อน (ลบไฟล์เก่าเพื่อไม่ให้มีสอง ranfile)".into(),
    );

    m.insert(
        "ran.fmt.no_terminal".into(),
        "ไม่ใช่เทอร์มินัล: ใส่ -y เพื่อยืนยันการแปลง".into(),
    );
}
