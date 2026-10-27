use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nfe.binary_file".into(), "(ไบนารี / รูปแบบที่ไม่รองรับ)".into());

    m.insert("nfe.binary_file_preview".into(), "(ไฟล์ไบนารี)".into());

    m.insert("nfe.empty_dir".into(), "(ไดเรกทอรีว่าง)".into());

    m.insert("nfe.file_truncated".into(), "... (ไฟล์ถูกตัดทอน)".into());

    m.insert(
        "nfe.hint_bar".into(),
        "h ช่วยเหลือ | b กลับ | f ค้นหา | / กรอง | q ออก".into(),
    );

    m.insert("nfe.no_matches".into(), "(ไม่มีการตรงกัน)".into());

    m.insert("nfe.preview_empty".into(), "(ว่าง)".into());

    m.insert("nfe.preview_error".into(), "(อ่านไม่ได้)".into());

    m.insert("nfe.preview_more".into(), "... (เพิ่มเติม)".into());

    m.insert("nfe.sort_date".into(), "วันที่".into());

    m.insert("nfe.sort_name".into(), "ชื่อ".into());

    m.insert("nfe.sort_size".into(), "ขนาด".into());

    m.insert("nfe.sort_type".into(), "ประเภท".into());

    m.insert(
        "nfe.status_bar".into(),
        "{} รายการ | {} ไดเรกทอรี | {} ไฟล์ | เรียง: {} {}".into(),
    );
}
