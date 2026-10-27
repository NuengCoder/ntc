use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "helpers.building_tree".into(),
        "กำลังสร้างต้นไม้ไดเรกทอรี...".into(),
    );

    m.insert(
        "helpers.calculating_sizes".into(),
        "กำลังคำนวณขนาดไดเรกทอรี...".into(),
    );

    m.insert("helpers.export_cancelled".into(), "ยกเลิกการส่งออก.".into());

    m.insert(
        "helpers.exported_all".into(),
        "ส่งออก alias {} ทั้งหมดไปยัง: {}".into(),
    );

    m.insert(
        "helpers.exported_categories".into(),
        "ส่งออก {} หมวดหมู่ไปยัง: {}".into(),
    );

    m.insert(
        "helpers.exported_count".into(),
        "ส่งออก {} alias ไปยัง: {}".into(),
    );

    m.insert(
        "helpers.exported_igcare_all".into(),
        "ส่งออกการตั้งค่า ignore/care ทั้งหมดไปยัง: {}".into(),
    );

    m.insert("helpers.file_not_found".into(), "ไม่พบไฟล์: {}".into());

    m.insert(
        "helpers.imported_igcare".into(),
        "นำเข้าการตั้งค่า ignore/care จาก: {}".into(),
    );

    m.insert(
        "helpers.imported_ral".into(),
        "นำเข้า run aliases จาก: {}".into(),
    );

    m.insert(
        "helpers.invalid_back_count".into(),
        "จำนวน back ไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "helpers.invalid_input".into(),
        "ข้อมูลนำเข้าไม่ถูกต้อง: {}".into(),
    );

    m.insert("helpers.invalid_num".into(), "ตัวเลขไม่ถูกต้อง: {}".into());

    m.insert(
        "helpers.invalid_num_range".into(),
        "ตัวเลขไม่ถูกต้อง: {}. ใช้ 1-{} หรือ 0 เพื่อเสร็จสิ้น.".into(),
    );

    m.insert(
        "helpers.no_editor".into(),
        "ไม่พบตัวแก้ไขภายนอก. เปิดตัวแก้ไขในตัว...".into(),
    );

    m.insert(
        "helpers.no_ral_to_export".into(),
        "ไม่มี run aliases ให้ส่งออก.".into(),
    );
}
