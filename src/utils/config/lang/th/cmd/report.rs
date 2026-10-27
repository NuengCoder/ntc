use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.report.content_saved".into(),
        "บันทึกเนื้อหาไฟล์ไปที่: {}".into(),
    );

    m.insert(
        "cmd.report.copied_clipboard".into(),
        "คัดลอกไฟล์ '{}' ไปยังคลิปบอร์ดแล้ว!".into(),
    );

    m.insert("cmd.report.copy_failed".into(), "ไม่สามารถคัดลอก: {}".into());

    m.insert(
        "cmd.report.copying_clipboard".into(),
        "กำลังคัดลอก '{}' ไปยังคลิปบอร์ด...".into(),
    );

    m.insert(
        "cmd.report.dir_only".into(),
        "{} รายงานใช้งานได้เฉพาะกับไดเรกทอรี".into(),
    );

    m.insert(
        "cmd.report.example_json".into(),
        "  report json ..         รายงานไดเรกทอรีหลัก".into(),
    );

    m.insert(
        "cmd.report.example_txt".into(),
        "  report txt             รายงานไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert("cmd.report.examples".into(), "ตัวอย่าง:".into());

    m.insert("cmd.report.file_not_found".into(), "ไม่พบไฟล์: {}".into());

    m.insert(
        "cmd.report.no_files_in_dir".into(),
        "ไม่มีไฟล์ที่รองรับให้แสดงใน: {}".into(),
    );

    m.insert(
        "cmd.report.help_docx".into(),
        "  report docx [path]     สร้างรายงาน DOCX".into(),
    );

    m.insert(
        "cmd.report.help_html".into(),
        "  report html [path]     สร้างรายงาน HTML".into(),
    );

    m.insert(
        "cmd.report.help_json".into(),
        "  report json [path]     สร้างรายงาน JSON".into(),
    );

    m.insert(
        "cmd.report.help_md".into(),
        "  report md [path]       สร้างรายงาน Markdown".into(),
    );

    m.insert(
        "cmd.report.help_pdf".into(),
        "  report pdf [path]      สร้างรายงาน PDF".into(),
    );

    m.insert(
        "cmd.report.help_txt".into(),
        "  report txt [path]      สร้างรายงาน TXT".into(),
    );

    m.insert(
        "cmd.report.help_xlsx".into(),
        "  report xlsx [path]     สร้างรายงาน XLSX".into(),
    );

    m.insert(
        "cmd.report.invalid_depth".into(),
        "ความลึกไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "cmd.report.neovim_hint".into(),
        "คุณเปิดไฟล์นี้ใน Neovim ได้ด้วย: :edit {}".into(),
    );

    m.insert(
        "cmd.report.skipped_unsupported".into(),
        "ข้าม (ไม่รองรับรูปแบบ): {}".into(),
    );

    m.insert(
        "cmd.report.ext_usage".into(),
        "การใช้งาน: view -ext <นามสกุล...> (เช่น view -ext rs c)".into(),
    );

    m.insert("cmd.report.time_label".into(), "เวลา:".into());

    m.insert(
        "cmd.report.title".into(),
        "report — สร้างรายงานโปรเจกต์".into(),
    );

    m.insert(
        "cmd.report.unknown_option".into(),
        "ตัวเลือก view ไม่รู้จัก: {}".into(),
    );

    m.insert("cmd.report.view_usage".into(), "การใช้งาน: view [-s|--size] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>] [-p|--path <path|@tp>] [-ext|--ext <นามสกุล...>]".into());

    m.insert(
        "cmd.report.invalid_range".into(),
        "ช่วงไม่ถูกต้อง: '{}' (ต้องเป็น n:m เช่น --betw 10:20)".into(),
    );

    m.insert(
        "cmd.report.invalid_sort".into(),
        "คีย์เรียงไม่ถูกต้อง: '{}' (size|char|ext)".into(),
    );

    m.insert(
        "cmd.report.invalid_count".into(),
        "จำนวนบรรทัดไม่ถูกต้อง: '{}' (ต้องมากกว่า 0)".into(),
    );

    m.insert(
        "cmd.report.help_clip".into(),
        "  report clip <n>       แสดงเนื้อหาคลิปบอร์ดทั้งหมด".into(),
    );

    m.insert(
        "cmd.report.legacy_removed".into(),
        "วิธีการ report เดิมได้ถูกยกเลิก. กรุณาใช้ 'report project <fmt> ...' แทน.".into(),
    );

    m.insert(
        "cmd.report.example_project".into(),
        "  report project html . --name report.html".into(),
    );

    m.insert("cmd.report.example_clip".into(), "  report clip 1".into());

    m.insert("cmd.report.clip_header".into(), "คลิปบอร์ด {}: ".into());

    m.insert("cmd.report.clip_meta_date".into(), "วันที่".into());

    m.insert("cmd.report.clip_meta_lines".into(), "จำนวนสาย".into());

    m.insert("cmd.report.clip_meta_chars".into(), "จำนวนตัวอักษร".into());
}
