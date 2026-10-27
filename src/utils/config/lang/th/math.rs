use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "math.builtin_agg".into(),
        "  agg:  sum, min, max, avg".into(),
    );

    m.insert(
        "math.builtin_constants".into(),
        "  constants: PI, E, PHI, TAU".into(),
    );

    m.insert(
        "math.builtin_conv".into(),
        "  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB".into(),
    );

    m.insert("math.builtin_header".into(), "ฟังก์ชันในตัว (Built-in):".into());

    m.insert(
        "math.builtin_inv_trig".into(),
        "  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc"
            .into(),
    );

    m.insert(
        "math.builtin_math".into(),
        "  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10".into(),
    );

    m.insert(
        "math.builtin_print".into(),
        "  print: พิมพ์ค่า (เหมือน Python)".into(),
    );

    m.insert("math.builtin_rand".into(), "  rand: rand(min,max)".into());

    m.insert(
        "math.builtin_trig".into(),
        "  trig: sin, cos, tan, cot, sec, csc".into(),
    );

    m.insert("math.compiled_header".into(), "คอมไพล์แล้ว: {}".into());

    m.insert("math.countdown".into(), "นับถอยหลัง: {} วินาที".into());

    m.insert("math.error".into(), "  {} ที่บรรทัด {}: {}".into());

    m.insert(
        "math.example_add".into(),
        "  math 3+4*5                           → 23".into(),
    );

    m.insert(
        "math.example_fun_call".into(),
        "  math square(5)                        → 25".into(),
    );

    m.insert(
        "math.example_fun_def".into(),
        "  math fun add square(x) = x^2".into(),
    );

    m.insert(
        "math.example_hexbin".into(),
        "  math 0xFF + 0b1010                   → 265".into(),
    );

    m.insert(
        "math.example_rand".into(),
        "  math rand(1,100)                      → 42".into(),
    );

    m.insert(
        "math.example_sin".into(),
        "  math sin(PI/2)                       → 1".into(),
    );

    m.insert(
        "math.example_sqrt".into(),
        "  math sqrt(144)                        → 12".into(),
    );

    m.insert(
        "math.example_timer".into(),
        "  math timer 10                        → นับถอยหลัง 10 วินาที".into(),
    );

    m.insert(
        "math.example_tobinary".into(),
        "  math toBinary(255)                    → 11111111".into(),
    );

    m.insert(
        "math.example_tohb".into(),
        "  math toHB(1048576)                    → 1.00 MB".into(),
    );

    m.insert(
        "math.example_tohex".into(),
        "  math toHex(255)                       → FF".into(),
    );

    m.insert("math.examples_header".into(), "ตัวอย่าง:".into());

    m.insert(
        "math.exec_end".into(),
        "\u{255A}\u{2550} จบ: {} \u{2550}\u{255D}".into(),
    );

    m.insert("math.failed_read".into(), "ไม่สามารถอ่าน '{}': {}".into());

    m.insert("math.file_not_found".into(), "ไม่พบไฟล์: '{}'".into());

    m.insert("math.fun_commands_header".into(), "คำสั่ง math fun:".into());

    m.insert(
        "math.fun_list_header".into(),
        "ฟังก์ชันคณิตศาสตร์ที่ผู้ใช้กำหนด:".into(),
    );

    m.insert(
        "math.fun_none".into(),
        " ไม่มีฟังก์ชันที่ผู้ใช้กำหนด. ใช้ 'math fun add <ชื่อ>(<params>) = <body>'".into(),
    );

    m.insert("math.fun_removed".into(), " ลบแล้ว: {}".into());

    m.insert(
        "math.help_expr".into(),
        "  math <นิพจน์>       ประเมินนิพจน์คณิตศาสตร์".into(),
    );

    m.insert(
        "math.help_file".into(),
        "  math <ไฟล์>.ntc.math    คอมไพล์และเรียกไฟล์คณิตศาสตร์".into(),
    );

    m.insert(
        "math.help_fun_add".into(),
        "  math fun add <ชื่อ>(<params>) = <body>   กำหนดฟังก์ชัน".into(),
    );

    m.insert(
        "math.help_fun_edit".into(),
        "  math fun edit <ชื่อ> = <body>  อัปเดตฟังก์ชัน".into(),
    );

    m.insert(
        "math.help_fun_info".into(),
        "  math fun info <ชื่อ>    แสดงรายละเอียดฟังก์ชัน".into(),
    );

    m.insert(
        "math.help_fun_ls".into(),
        "  math fun ls             แสดงฟังก์ชันที่ผู้ใช้กำหนดทั้งหมด".into(),
    );

    m.insert(
        "math.help_fun_rm".into(),
        "  math fun rm <ชื่อ>      ลบฟังก์ชัน".into(),
    );

    m.insert(
        "math.help_func_call".into(),
        "  math <func>(<args>)     เรียกฟังก์ชันในตัวหรือที่ผู้ใช้กำหนด".into(),
    );

    m.insert(
        "math.help_timer".into(),
        "  math timer [วินาที]    จับเวลารอบหรือนับถอยหลังพร้อมสัญญาณเตือน".into(),
    );

    m.insert("math.help_title".into(), "โมดูลคณิตศาสตร์ - การใช้งาน:".into());

    m.insert(
        "math.is_directory".into(),
        "'{}' เป็นไดเรกทอรี ไม่ใช่ไฟล์".into(),
    );

    m.insert("math.lap_time".into(), "รอบ: {} วินาที".into());

    m.insert("math.time_remaining".into(), "เวลาที่เหลือ: {}".into());

    m.insert("math.timer_started".into(), "เริ่มจับเวลาแล้ว...".into());

    m.insert(
        "math.timer_usage".into(),
        "การใช้งาน: math timer <วินาที>".into(),
    );

    m.insert("math.times_up".into(), "หมดเวลา!".into());
}
