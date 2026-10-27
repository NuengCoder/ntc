use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "image2doc.ocr_disabled".into(),
        "OCR ไม่เปิดใช้งานบนแพลตฟอร์มหรือ build นี้. คอมไพล์ใหม่ด้วยคุณสมบัติ 'ocr' หรือติดตั้ง tesseract.".into(),
    );

    m.insert(
        "image2doc.ocr_missing".into(),
        "ไม่พบ Tesseract. OCR ไม่พร้อมใช้งาน.".into(),
    );

    m.insert(
        "image2doc.ocr_install".into(),
        "ติดตั้งด้วย: ntpkg install tesseract".into(),
    );

    m.insert("image2doc.ocr_path_win".into(), "จากนั้นเพิ่มไปยัง PATH: usrpath add <dir> แล้ว 'mode admin' + 'syspath add <dir>' หรือเปิดเทอร์มินัลใหม่.".into());

    m.insert(
        "image2doc.ocr_path_unix".into(),
        "จากนั้นเพิ่มไปยัง PATH เช่น: export PATH=\"/<dir>:$PATH\"".into(),
    );

    m.insert(
        "image2doc.ocr_rerun".into(),
        "จากนั้นเรียกคำสั่งเดียวกันอีกครั้ง.".into(),
    );

    m.insert(
        "image2doc.ocr_failed".into(),
        "Tesseract ไม่สามารถประมวลผลภาพได้.".into(),
    );
}
