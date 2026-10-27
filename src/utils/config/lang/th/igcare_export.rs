use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("igcare_export.cared_dirs".into(), "ไดเรกทอรีที่ดูแล".into());

    m.insert("igcare_export.cared_extensions".into(), "นามสกุลที่ดูแล".into());

    m.insert("igcare_export.cared_files".into(), "ไฟล์ที่ดูแล".into());

    m.insert("igcare_export.all".into(), "เลือกทั้งหมด".into());

    m.insert("igcare_export.cancel".into(), "ยกเลิก".into());

    m.insert(
        "igcare_export.extra_supported_extensions".into(),
        "นามสกุลที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "igcare_export.extra_supported_files".into(),
        "ไฟล์ที่รองรับเพิ่มเติม".into(),
    );

    m.insert("igcare_export.finish".into(), "เสร็จสิ้นการส่งออก".into());

    m.insert(
        "igcare_export.ignored_dirs".into(),
        "ไดเรกทอรีที่ถูกละเว้น".into(),
    );

    m.insert(
        "igcare_export.ignored_extensions".into(),
        "นามสกุลที่ถูกละเว้น".into(),
    );

    m.insert("igcare_export.ignored_files".into(), "ไฟล์ที่ถูกละเว้น".into());

    m.insert("igcare_export.items".into(), "{} รายการ".into());

    m.insert("igcare_export.prompt".into(), "igcare-export>".into());

    m.insert(
        "igcare_export.select".into(),
        "เลือก/ยกเลิกโดยระบุหมายเลข".into(),
    );

    m.insert("igcare_export.selected".into(), "เลือกแล้ว".into());

    m.insert("igcare_export.selected_mark".into(), "(เลือกแล้ว)".into());

    m.insert(
        "igcare_export.title".into(),
        "igcare export — เลือกหมวดหมู่".into(),
    );
}
