use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("theme.create_failed".into(), "ไม่สามารถสร้างธีม: {}".into());

    m.insert(
        "theme.create_file_failed".into(),
        "ไม่สามารถสร้างไฟล์: {}".into(),
    );

    m.insert(
        "theme.create_first".into(),
        "เรียก 'theme add <name>' เพื่อสร้างธีมแรกของคุณ".into(),
    );

    m.insert(
        "theme.created".into(),
        "สร้างธีม '{}' แล้ว (เปิดในตัวแก้ไข)".into(),
    );

    m.insert(
        "theme.created_hint".into(),
        "แก้ไขไฟล์ บันทึก แล้วเรียก 'theme reload' เพื่อนำการเปลี่ยนแปลงไปใช้".into(),
    );

    m.insert("theme.current_title".into(), "ธีมปัจจุบัน: {}".into());

    m.insert("theme.dir_title".into(), "ไดเรกทอรีธีม:".into());

    m.insert(
        "theme.edit_hint".into(),
        "แก้ไข '{}' บันทึก แล้วเรียก 'theme reload'".into(),
    );

    m.insert("theme.export_failed".into(), "ไม่สามารถส่งออกธีม: {}".into());

    m.insert(
        "theme.export_failed_simple".into(),
        "ไม่สามารถส่งออก: {}".into(),
    );

    m.insert("theme.exported_simple".into(), "ส่งออกไปที่: {} แล้ว".into());

    m.insert(
        "theme.exported_to".into(),
        "ส่งออกธีม '{}' ไปที่: {} แล้ว".into(),
    );

    m.insert("theme.file_exists".into(), "ไฟล์มีอยู่แล้ว: {}".into());

    m.insert("theme.file_not_found".into(), "ไม่พบไฟล์: {}".into());

    m.insert("theme.files_title".into(), "ไฟล์ธีม:".into());

    m.insert("theme.import_failed".into(), "ไม่สามารถนำเข้าธีม: {}".into());

    m.insert(
        "theme.import_failed_simple".into(),
        "ไม่สามารถนำเข้า: {}".into(),
    );

    m.insert(
        "theme.import_hint".into(),
        "เรียก 'theme list' เพื่อดูธีมที่มี".into(),
    );

    m.insert("theme.imported".into(), "นำเข้าธีมจาก: {} แล้ว".into());

    m.insert("theme.info_author".into(), "ผู้เขียน".into());

    m.insert("theme.info_defined".into(), "\u{2713} กำหนดแล้ว".into());

    m.insert("theme.info_description".into(), "คำอธิบาย".into());

    m.insert("theme.info_editor".into(), "สีตัวแก้ไข".into());

    m.insert("theme.info_shell".into(), "สีเชลล์".into());

    m.insert("theme.info_syntax".into(), "สีไวยากรณ์".into());

    m.insert("theme.info_title".into(), "\u{1F3A8} ธีม: {}".into());

    m.insert(
        "theme.invalid_number".into(),
        "ตัวเลขไม่ถูกต้อง: {}. ใช้ 1-{}".into(),
    );

    m.insert("theme.name_empty".into(), "ชื่อธีมต้องไม่ว่างเปล่า".into());

    m.insert("theme.no_dir".into(), "ไม่สามารถระบุไดเรกทอรีธีมได้".into());

    m.insert("theme.no_dir_yet".into(), "(ไดเรกทอรียังไม่มี)".into());

    m.insert("theme.no_files".into(), "(ไม่มีไฟล์ธีม)".into());

    m.insert("theme.not_found".into(), "ไม่พบธีม '{}'".into());

    m.insert(
        "theme.not_found_hint".into(),
        "ไม่พบธีม: {}. ใช้ 'theme list' เพื่อดูธีมที่มี.".into(),
    );

    m.insert("theme.opened_dir".into(), "เปิดไดเรกทอรีธีม: {} แล้ว".into());

    m.insert("theme.reloaded".into(), "โหลดธีมจากดิสก์ใหม่แล้ว".into());

    m.insert("theme.remove_failed".into(), "ไม่สามารถลบธีม: {}".into());

    m.insert("theme.removed".into(), "ลบธีมแล้ว: {}".into());

    m.insert("theme.rename_failed".into(), "ไม่สามารถเปลี่ยนชื่อธีม: {}".into());

    m.insert(
        "theme.rename_failed_simple".into(),
        "ไม่สามารถเปลี่ยนชื่อ: {}".into(),
    );

    m.insert("theme.renamed".into(), "เปลี่ยนชื่อธีม '{}' -> '{}' แล้ว".into());

    m.insert(
        "theme.renamed_simple".into(),
        "เปลี่ยนชื่อ '{}' -> '{}' แล้ว".into(),
    );

    m.insert("theme.switch_failed".into(), "ไม่สามารถสลับไปใช้ธีม: {}".into());

    m.insert(
        "theme.switch_failed_simple".into(),
        "ไม่สามารถสลับไปใช้ธีม: {}".into(),
    );

    m.insert("theme.switched".into(), "สลับไปใช้ธีม: {} แล้ว".into());

    m.insert(
        "theme.template_created".into(),
        "สร้างเทมเพลตธีมแล้ว: {}".into(),
    );

    m.insert(
        "theme.template_info".into(),
        "นี่คือไฟล์เทมเพลตในเครื่อง. ใช้ 'theme import' เพื่อเพิ่มลงในรายการธีมของคุณ.".into(),
    );

    m.insert(
        "theme.type_interactive".into(),
        "พิมพ์ 'theme' สำหรับเมนูแบบโต้ตอบ".into(),
    );

    m.insert("theme.ui.current".into(), "ธีมปัจจุบัน".into());

    m.insert("theme.ui.exit".into(), "กำลังออกจากตัวจัดการธีม.".into());

    m.insert("theme.ui.name_prompt".into(), "ชื่อธีม: ".into());

    m.insert("theme.ui.theme_prompt".into(), "theme>".into());

    m.insert("theme.ui.title".into(), "\u{1F3A8} ตัวจัดการธีมของ ntc".into());

    m.insert("theme.unknown_cmd".into(), "คำสั่งไม่รู้จัก: {}".into());

    m.insert(
        "theme.usage_add".into(),
        "การใช้งาน: theme add <name>".into(),
    );

    m.insert(
        "theme.usage_export".into(),
        "การใช้งาน: theme export <name> [output_file]".into(),
    );

    m.insert(
        "theme.usage_import".into(),
        "การใช้งาน: theme import <file.ntc_theme>".into(),
    );

    m.insert(
        "theme.usage_info".into(),
        "การใช้งาน: theme info <name>".into(),
    );

    m.insert(
        "theme.usage_rename".into(),
        "การใช้งาน: theme rename <old_name> to <new_name>".into(),
    );

    m.insert("theme.usage_rm".into(), "การใช้งาน: theme rm <name>".into());

    m.insert(
        "theme.usage_rnm".into(),
        "การใช้งาน: theme rnm <old_name> to <new_name>".into(),
    );
}
