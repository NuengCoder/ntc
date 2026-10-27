use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.tl.dir_not_found".into(), "ไม่พบไดเรกทอรี: {}".into());

    m.insert(
        "cmd.tl.dir_usage".into(),
        "การใช้งาน: tl dir <เส้นทาง>".into(),
    );

    m.insert(
        "cmd.tl.ext_not_supported".into(),
        ".{} ไม่รองรับ — ถูกบล็อก".into(),
    );

    m.insert(
        "cmd.tl.ext_usage".into(),
        "การใช้งาน: tl ext <นามสกุล>".into(),
    );

    m.insert("cmd.tl.file_not_found".into(), "ไม่พบไฟล์: {}".into());

    m.insert(
        "cmd.tl.file_usage".into(),
        "การใช้งาน: tl file <เส้นทาง>".into(),
    );

    m.insert(
        "cmd.tl.help_dir".into(),
        "  tl dir <path>           แสดงนามสกุลที่รองรับและจำนวนบรรทัดรวมในไดเรกทอรี".into(),
    );

    m.insert(
        "cmd.tl.help_ext".into(),
        "  tl ext <extension>      แสดงไฟล์ทั้งหมดที่มีนามสกุลนั้นและจำนวนบรรทัดรวม".into(),
    );

    m.insert(
        "cmd.tl.help_file".into(),
        "  tl file <path>          นับบรรทัดในไฟล์เดียว".into(),
    );

    m.insert("cmd.tl.help_sort".into(), "  tl ext|dir <target> -s <-n|-l|-r>   เรียงลำดับผลลัพธ์ (-n ปกติ, -l เรียงตามบรรทัดจากน้อยไปมาก, -r จากมากไปน้อย)".into());

    m.insert("cmd.tl.highest".into(), "สูงสุด: {}  {} บรรทัด".into());

    m.insert("cmd.tl.no_ext_files".into(), "ไม่พบไฟล์ .{} ".into());

    m.insert(
        "cmd.tl.no_supported_files".into(),
        "ไม่พบไฟล์ที่รองรับใน {}".into(),
    );

    m.insert("cmd.tl.not_a_file".into(), "ไม่ใช่ไฟล์: {}".into());

    m.insert("cmd.tl.sort_usage".into(), "แฟลกเรียงลำดับไม่ถูกต้อง. ใช้: -s -n (ปกติ) | -s -l (ตามบรรทัด น้อยไปมาก) | -s -r (ย้อนกลับ มากไปน้อย).".into());

    m.insert(
        "cmd.tl.total_files_lines".into(),
        "รวม: {} ไฟล์, {} บรรทัด".into(),
    );

    m.insert("cmd.tl.unknown_sub".into(), "ไม่รู้จักคำสั่งย่อย '{}'.".into());

    m.insert("cmd.tl.usage_title".into(), "การใช้งาน:".into());
}
