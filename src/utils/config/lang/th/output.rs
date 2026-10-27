use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("output.filename_header".into(), "{}:".into());

    m.insert("output.line_prefix".into(), "LINE {}>>".into());

    m.insert(
        "output.range_past_eof".into(),
        "(ช่วงเกินท้ายไฟล์ — ไม่มีอะไรแสดง)".into(),
    );

    m.insert(
        "output.mkdir_failed".into(),
        "ไม่สามารถสร้างไดเรกทอรี: {}".into(),
    );

    m.insert("output.mkfile_failed".into(), "ไม่สามารถสร้างไฟล์: {}".into());

    m.insert("output.print_error".into(), "\u{2717} {}".into());

    m.insert("output.print_info".into(), "\u{2139} {}".into());

    m.insert("output.print_success".into(), "\u{2713} {}".into());

    m.insert("output.print_warning".into(), "\u{26A0} {}".into());

    m.insert(
        "output.read_file_failed".into(),
        "ไม่สามารถอ่านไฟล์ '{}'".into(),
    );

    m.insert("output.write_failed".into(), "ไม่สามารถเขียนไฟล์: {}".into());
}
