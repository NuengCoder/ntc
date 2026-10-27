use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.tl.dir".into(),
        "tl dir <path> [-s <-n|-l|-r>] ## แสดงนามสกุลที่รองรับและจำนวนบรรทัดรวมในไดเรกทอรี (เรียงได้)"
            .into(),
    );

    m.insert(
        "help.tl.ext".into(),
        "tl ext <ext> [-s <-n|-l|-r>] ## แสดงไฟล์ <ext> ทั้งหมดและจำนวนบรรทัดรวม (เคารพ igcare เรียงได้)"
            .into(),
    );

    m.insert(
        "help.tl.file".into(),
        "tl file <path> ## นับบรรทัดในไฟล์เดียว".into(),
    );

    m.insert("help.tl.sort".into(), "tl ext|dir <target> -s <-n|-l|-r> ## เรียง: -n ปกติ -l ตามบรรทัด (น้อยไปมาก) -r ย้อนกลับ (มากไปน้อย)".into());

    m.insert(
        "help.tl.title".into(),
        "คำสั่งนับบรรทัดรวม (TOTAL LINE):".into(),
    );

    m.insert(
        "help.tl.example1".into(),
        "tl file main.rs ## นับบรรทัดใน main.rs".into(),
    );

    m.insert(
        "help.tl.example2".into(),
        "tl ext .rs ## นับบรรทัดในไฟล์ .rs ทั้งหมด".into(),
    );

    m.insert(
        "help.tl.example3".into(),
        "tl dir . ## นับบรรทัดในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.tl.example4".into(),
        "tl ext .rs -s -r ## แสดงไฟล์ .rs เรียงตามบรรทัดจากมากไปน้อย".into(),
    );
}
