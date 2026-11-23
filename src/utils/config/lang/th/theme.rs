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

    m.insert("theme.info_defined".into(), "กำหนดแล้ว".into());

    m.insert("theme.info_description".into(), "คำอธิบาย".into());

    m.insert("theme.info_editor".into(), "สีตัวแก้ไข".into());

    m.insert("theme.info_shell".into(), "สีเชลล์".into());

    m.insert("theme.info_syntax".into(), "สีไวยากรณ์".into());

    m.insert("theme.info_title".into(), "ธีม: {}".into());

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

    m.insert("theme.ui.title".into(), "ตัวจัดการธีมของ ntc".into());

    m.insert("theme.unknown_cmd".into(), "คำสั่งไม่รู้จัก: {}".into());

    m.insert(
        "theme.usage_add".into(),
        "การใช้งาน: theme add <name>".into(),
    );

    m.insert(
        "theme.usage_update".into(),
        "การใช้งาน: theme update <name>".into(),
    );

    m.insert(
        "theme.updated".into(),
        "อัปเดตธีม '{}' แล้ว: เพิ่ม {} ฟิลด์ที่ขาด".into(),
    );

    m.insert("theme.up_to_date".into(), "ธีม '{}' มีครบทุกฟิลด์แล้ว".into());

    m.insert("theme.update_failed".into(), "อัปเดตไม่สำเร็จ: {}".into());

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

    m.insert(
        "theme.usage_rm".into(),
        "การใช้งาน: theme rm <ชื่อ> [<ชื่อ> ...] [-f|--force]".into(),
    );

    m.insert("theme.usage_use".into(), "การใช้งาน: theme use <ชื่อ>".into());

    m.insert("theme.remove_confirm".into(), "ลบ {} ธีม ({})? [y/N]".into());

    m.insert("theme.remove_cancelled".into(), "ยกเลิกแล้ว".into());

    m.insert(
        "theme.remove_current".into(),
        "ไม่สามารถลบธีมที่ใช้อยู่ '{}' ได้; เปลี่ยนไปใช้ธีมอื่นก่อน".into(),
    );

    m.insert(
        "theme.remove_builtin".into(),
        "ไม่สามารถลบธีมในตัว '{}' ได้".into(),
    );

    m.insert(
        "theme.usage_rnm".into(),
        "การใช้งาน: theme rnm <old_name> to <new_name>".into(),
    );

    m.insert(
        "theme.install_downloading".into(),
        "กำลังดาวน์โหลด {}...".into(),
    );

    m.insert("theme.install_failed".into(), "ติดตั้งธีมไม่สำเร็จ: {}".into());

    m.insert(
        "theme.install_fetching".into(),
        "กำลังดึงไฟล์ .ntc_theme จาก GitHub...".into(),
    );

    m.insert(
        "theme.install_none".into(),
        "ไม่พบไฟล์ .ntc_theme ที่ติดตั้งได้".into(),
    );

    m.insert("theme.install_prompt".into(), "เลือกธีม (0 เพื่อยกเลิก):".into());

    m.insert("theme.install_title".into(), "ติดตั้งธีม".into());

    m.insert("theme.installed".into(), "ติดตั้งธีม '{}' แล้ว".into());

    m.insert(
        "theme.installed_active".into(),
        "ติดตั้งและสลับไปยังธีม '{}' แล้ว".into(),
    );
}
