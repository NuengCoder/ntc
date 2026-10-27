use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ral_export.all".into(), "เลือกทั้งหมด".into());

    m.insert("ral_export.cancel".into(), "ยกเลิก".into());

    m.insert(
        "ral_export.export_cancelled".into(),
        "ยกเลิกการส่งออก.".into(),
    );

    m.insert("ral_export.finish".into(), "เสร็จสิ้นการส่งออก".into());

    m.insert("ral_export.prompt".into(), "ral-export>".into());

    m.insert("ral_export.select".into(), "เลือก/ยกเลิกโดยระบุหมายเลข".into());

    m.insert("ral_export.selected".into(), "เลือกแล้ว".into());

    m.insert("ral_export.selected_mark".into(), "(เลือกแล้ว)".into());

    m.insert(
        "ral_export.title".into(),
        "ral export — เลือก alias เพื่อส่งออก".into(),
    );
}
