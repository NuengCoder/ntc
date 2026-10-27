use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup.and_more".into(), " ... และอีก {} รายการ".into());

    m.insert(
        "backup.available_title".into(),
        " แบ็กอัปที่พร้อมใช้งาน (ใหม่ที่สุดก่อน)".into(),
    );

    m.insert("backup.cancel_opt".into(), " 0. ยกเลิก".into());

    m.insert("backup.cancelled".into(), "ยกเลิกการล้าง.".into());

    m.insert(
        "backup.clear_history_prompt".into(),
        "\u{26A0} ล้างประวัติการเลิกทำ? ไม่สามารถยกเลิกได้ (y/N):".into(),
    );

    m.insert("backup.cleared".into(), "ลบแบ็กอัปทั้งหมดแล้ว.".into());

    m.insert(
        "backup.cleared_count".into(),
        "ล้างแบ็กอัป {} รายการสำหรับโปรเจกต์นี้.".into(),
    );

    m.insert("backup.confirm".into(), "พิมพ์ 'yes' เพื่อยืนยัน: ".into());

    m.insert(
        "backup.create_hint".into(),
        "ใช้ 'backup create' เพื่อสร้างแบ็กอัปแรกของคุณ.".into(),
    );

    m.insert(
        "backup.created".into(),
        "สร้างแบ็กอัปแล้ว: #{} \u{2014} {} \u{2014} {} ไฟล์".into(),
    );

    m.insert(
        "backup.created_detail".into(),
        "สร้างแบ็กอัป #{} แล้ว: {} ({} ไฟล์, {})".into(),
    );

    m.insert("backup.creating".into(), "กำลังสร้างแบ็กอัป...".into());

    m.insert(
        "backup.delete_extra_failed".into(),
        "ไม่สามารถลบไฟล์พิเศษ {}: {}".into(),
    );

    m.insert(
        "backup.deleted_since".into(),
        "{} ถูกลบไปหลังจากการแบ็กอัป.".into(),
    );

    m.insert(
        "backup.entry_file_count".into(),
        " แบ็กอัป #{} \u{2014} {} \u{2014} {} \u{2014} {} ไฟล์".into(),
    );

    m.insert("backup.exists".into(), "มีแบ็กอัปอยู่: {} แบ็กอัปที่ {}".into());

    m.insert(
        "backup.failed_to_backup".into(),
        "ไม่สามารถแบ็กอัป {}: {}".into(),
    );

    m.insert("backup.file_deleted".into(), "{} ถูกลบไปหลังการแบ็กอัป.".into());

    m.insert(
        "backup.file_modified".into(),
        "{} เปลี่ยนไปตั้งแต่การบันทึกล่าสุด.".into(),
    );

    m.insert(
        "backup.help_clear".into(),
        " backup clear ลบแบ็กอัปทั้งหมด (ต้องยืนยัน)".into(),
    );

    m.insert(
        "backup.help_clear_force".into(),
        " backup clear --force ลบแบ็กอัปทั้งหมด (ไม่ต้องยืนยัน)".into(),
    );

    m.insert(
        "backup.help_create".into(),
        " backup create สร้างแบ็กอัปใหม่".into(),
    );

    m.insert(
        "backup.help_diff".into(),
        " backup diff [num] เปรียบเทียบโปรเจกต์ปัจจุบันกับแบ็กอัป".into(),
    );

    m.insert("backup.help_examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "backup.help_list".into(),
        " backup list แสดงแบ็กอัปทั้งหมด".into(),
    );

    m.insert(
        "backup.help_recreate".into(),
        " backup recreate ล้างทั้งหมด + ประวัติเลิกทำ + สร้างใหม่".into(),
    );

    m.insert(
        "backup.help_recreate_force".into(),
        " backup recreate --force เหมือนเดิม ข้ามการยืนยันทั้งหมด".into(),
    );

    m.insert(
        "backup.help_restore".into(),
        " backup restore [num] กู้คืนแบ็กอัป (โต้ตอบหากไม่มีตัวเลข)".into(),
    );

    m.insert(
        "backup.help_restore_choose".into(),
        " backup restore --choose เลือกแบ็กอัปจากเมนูโต้ตอบ".into(),
    );

    m.insert(
        "backup.help_restore_choose_deleted".into(),
        " backup restore --choose --deleted เลือก + กู้คืนไฟล์ที่ถูกลบ".into(),
    );

    m.insert(
        "backup.help_restore_deleted".into(),
        " backup restore --deleted กู้คืนเฉพาะไฟล์ที่ถูกลบหลังแบ็กอัป".into(),
    );

    m.insert(
        "backup.help_title".into(),
        "backup \u{2014} จัดการแบ็กอัปโปรเจกต์".into(),
    );

    m.insert(
        "backup.help_undo".into(),
        " backup undo เลิกทำการกู้คืนครั้งล่าสุด".into(),
    );

    m.insert(
        "backup.help_undo_cls".into(),
        " backup undo --cls ล้างประวัติการเลิกทำ".into(),
    );

    m.insert(
        "backup.help_undo_force".into(),
        " backup undo --force ล้างประวัติการเลิกทำ (ไม่ต้องยืนยัน)".into(),
    );

    m.insert("backup.invalid_arg".into(), "อาร์กิวเมนต์ไม่ถูกต้อง: {}".into());

    m.insert(
        "backup.invalid_diff_arg".into(),
        "อาร์กิวเมนต์ไม่ถูกต้อง: {}. การใช้งาน: backup diff <number>".into(),
    );

    m.insert(
        "backup.invalid_selection".into(),
        "ตัวเลือกไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "backup.list_entry".into(),
        " #{} \u{2014} {} \u{2014} {} \u{2014} {} ไฟล์".into(),
    );

    m.insert("backup.list_location".into(), "ตำแหน่ง: {}".into());

    m.insert(
        "backup.list_title".into(),
        "แบ็กอัปสำหรับโปรเจกต์นี้: พบ {} รายการ".into(),
    );

    m.insert("backup.menu_item".into(), "แบ็กอัป #{}".into());

    m.insert(
        "backup.modified_since".into(),
        "{} ถูกแก้ไขตั้งแต่การแบ็กอัป.".into(),
    );

    m.insert(
        "backup.no_deleted_files".into(),
        "ไม่มีไฟล์ที่ถูกลบให้กู้คืน \u{2014} โปรเจกต์ตรงกับสแนปชอตแบ็กอัปแล้ว.".into(),
    );

    m.insert("backup.none".into(), "ไม่พบแบ็กอัปสำหรับโปรเจกต์นี้.".into());

    m.insert("backup.not_found_num".into(), "ไม่พบแบ็กอัป #{}.".into());

    m.insert(
        "backup.pick_detail".into(),
        "{} \u{2014} {} \u{2014} {} ไฟล์".into(),
    );

    m.insert(
        "backup.pick_prompt".into(),
        "ป้อนตัวเลข (1-{}) หรือ 0: ".into(),
    );

    m.insert("backup.pick_title".into(), "เลือกแบ็กอัปเพื่อกู้คืน".into());

    m.insert(
        "backup.previous_state_saved".into(),
        "บันทึกสถานะก่อนหน้าแล้ว ใช้ 'unpd' เพื่อเลิกทำการกู้คืนนี้.".into(),
    );

    m.insert(
        "backup.recreate_proceed".into(),
        "ดำเนินการต่อ? (y/N): ".into(),
    );

    m.insert(
        "backup.recreate_prompt".into(),
        "ไม่พบแบ็กอัปที่มีอยู่. สร้างแบ็กอัปใหม่เลยหรือไม่? (y/N): ".into(),
    );

    m.insert(
        "backup.recreate_step1".into(),
        " 1. ลบแบ็กอัปที่มีอยู่ทั้งหมด".into(),
    );

    m.insert(
        "backup.recreate_step2".into(),
        " 2. ล้างประวัติการเลิกทำ".into(),
    );

    m.insert(
        "backup.recreate_step3".into(),
        " 3. สร้างแบ็กอัปใหม่ของสถานะปัจจุบัน".into(),
    );

    m.insert(
        "backup.recreate_will_title".into(),
        "\u{26A0}\u{FE0F} RECREATE จะ:".into(),
    );

    m.insert(
        "backup.recreated".into(),
        "สร้างโปรเจกต์ใหม่แล้ว: ล้างแบ็กอัปเก่า, สร้างแบ็กอัปใหม่.".into(),
    );

    m.insert("backup.restore_add_count".into(), "จะกู้คืน {} ไฟล์.".into());

    m.insert("backup.restore_cancelled".into(), "ยกเลิกการกู้คืน.".into());

    m.insert(
        "backup.restore_confirm".into(),
        "ดำเนินการกู้คืน? (y/N): ".into(),
    );

    m.insert(
        "backup.restore_create_title".into(),
        "\u{26A0} ไฟล์ต่อไปนี้จะถูกสร้าง ({}):".into(),
    );

    m.insert(
        "backup.restore_delete_title".into(),
        "\u{26A0} ไฟล์ต่อไปนี้จะถูกลบ (ไม่อยู่ในแบ็กอัป) ({}):".into(),
    );

    m.insert(
        "backup.restore_deleted_confirm".into(),
        "ดำเนินการกู้คืนไฟล์ที่ถูกลบต่อ? (y/N): ".into(),
    );

    m.insert(
        "backup.restore_deleted_title".into(),
        " จะกู้คืน {} ไฟล์ (อยู่ในแบ็กอัปแต่หายไปแล้ว):".into(),
    );

    m.insert("backup.restore_failed".into(), "กู้คืน {} ไม่สำเร็จ: {}".into());

    m.insert(
        "backup.restore_overwrite_count".into(),
        "จะเขียนทับ {} ไฟล์.".into(),
    );

    m.insert(
        "backup.restore_overwrite_title".into(),
        "\u{26A0} ไฟล์ต่อไปนี้จะถูกเขียนทับ ({}):".into(),
    );

    m.insert("backup.restore_preview_title".into(), "ตัวอย่างการกู้คืน".into());

    m.insert(
        "backup.restored_backup".into(),
        "กู้คืนเสร็จสมบูรณ์: แบ็กอัป #{} \u{2014} กู้คืน {} ไฟล์".into(),
    );

    m.insert("backup.restored_date".into(), "กู้คืนเมื่อ: {}".into());

    m.insert(
        "backup.restored_deleted".into(),
        "กู้คืนไฟล์ที่ถูกลบจากแบ็กอัป #{}: กู้คืน {} ไฟล์".into(),
    );

    m.insert("backup.restored_result".into(), "กู้คืนแบ็กอัป #{} ({})".into());

    m.insert(
        "backup.select_entry".into(),
        " {}. แบ็กอัป #{} \u{2014} {} \u{2014} {} \u{2014} {} ไฟล์".into(),
    );

    m.insert(
        "backup.select_prompt".into(),
        "เลือกแบ็กอัปเพื่อกู้คืน (1-{}): ".into(),
    );

    m.insert(
        "backup.skip_large".into(),
        "ข้าม {} ({} MB เกินขีดจำกัด 50 MB)".into(),
    );

    m.insert(
        "backup.skip_metadata".into(),
        "ข้าม {} (ข้อผิดพลาด metadata: {})".into(),
    );

    m.insert(
        "backup.skip_rel_path".into(),
        "ข้าม {} (ไม่สามารถแก้เส้นทางสัมพัทธ์ได้)".into(),
    );

    m.insert(
        "backup.skip_symlink".into(),
        "ข้าม {} (symlink \u{2014} ไม่ติดตาม)".into(),
    );

    m.insert(
        "backup.skipped_count".into(),
        "ข้าม {} ไฟล์ \u{2014} ดู ignore.txt สำหรับรายละเอียด".into(),
    );

    m.insert("backup.skipped_error".into(), "ข้าม {} ({})".into());

    m.insert(
        "backup.skipped_large".into(),
        "ข้าม {} ({} \u{2014} ใหญ่เกินไป)".into(),
    );

    m.insert(
        "backup.skipped_unsupported".into(),
        "ข้าม {} (ประเภทไฟล์ที่ไม่รองรับ)".into(),
    );

    m.insert(
        "backup.snapshot_failed".into(),
        "ไม่สามารถสแนปชอต {} สำหรับการเลิกทำ ({}), ดำเนินการต่อ".into(),
    );

    m.insert(
        "backup.some_restore_failed".into(),
        "บางไฟล์ไม่สามารถกู้คืนได้. เก็บพื้นที่จัดเก็บการเลิกทำไว้.".into(),
    );

    m.insert("backup.stored_in".into(), "แบ็กอัปถูกเก็บใน: {}".into());

    m.insert(
        "backup.undo_cleared".into(),
        "ล้างประวัติการเลิกทำแล้ว (คืนพื้นที่ {} ฟรี)".into(),
    );

    m.insert(
        "backup.undo_complete".into(),
        "เลิกทำเสร็จสิ้น: กู้คืน {} ไฟล์จากก่อนการกู้คืนครั้งล่าสุด".into(),
    );

    m.insert(
        "backup.undo_complete_detail".into(),
        "เลิกทำการกู้คืนแบ็กอัป #{} (กู้คืน {}, ลบ {}, กู้คืนใหม่ {})".into(),
    );

    m.insert(
        "backup.undo_delete_failed".into(),
        "ไม่สามารถลบไฟล์ที่กู้คืน {}: {}".into(),
    );

    m.insert(
        "backup.undo_missing_original".into(),
        "ไม่สามารถกู้คืน {} (ไฟล์ต้นฉบับหายไปจากพื้นที่เก็บเลิกทำ)".into(),
    );

    m.insert(
        "backup.undo_missing_snapshot".into(),
        "ไม่สามารถกู้คืนใหม่ {} (สแนปชอตหายไปจากพื้นที่เก็บเลิกทำ)".into(),
    );

    m.insert("backup.undo_no_history".into(), "ไม่พบประวัติการเลิกทำ.".into());

    m.insert("backup.undo_none".into(), "ไม่มีการกู้คืนให้เลิกทำ.".into());

    m.insert(
        "backup.undo_rerestore_failed".into(),
        "ไม่สามารถกู้คืนไฟล์ที่ถูกลบใหม่ {}: {}".into(),
    );

    m.insert(
        "backup.unknown_clear_option".into(),
        "ตัวเลือกการล้างแบ็กอัปไม่รู้จัก: {}".into(),
    );

    m.insert(
        "backup.unknown_create_option".into(),
        "ตัวเลือกการสร้างแบ็กอัปไม่รู้จัก: {}".into(),
    );

    m.insert(
        "backup.unknown_recreate_option".into(),
        "ตัวเลือกการสร้างใหม่ไม่รู้จัก: {}".into(),
    );

    m.insert(
        "backup.unknown_undo_option".into(),
        "ตัวเลือกการเลิกทำไม่รู้จัก: {}".into(),
    );

    m.insert(
        "backup.usage_clear".into(),
        "การใช้งาน: backup clear [--force]".into(),
    );

    m.insert(
        "backup.usage_recreate".into(),
        "การใช้งาน: backup recreate [--force]".into(),
    );

    m.insert(
        "backup.usage_restore".into(),
        "การใช้งาน: backup restore [--choose] [--deleted] [num]".into(),
    );

    m.insert("backup.warning_count".into(), "คุณมี {} แบ็กอัป:".into());

    m.insert(
        "backup.warning_title".into(),
        "\u{26A0}\u{FE0F} คำเตือน: การดำเนินการนี้จะลบแบ็กอัปทั้งหมดของโปรเจกต์นี้!".into(),
    );

    m.insert(
        "backup.would_overwrite".into(),
        "{} จะถูกเขียนทับ (ใช้ --force เพื่อข้าม)".into(),
    );

    m.insert(
        "backup.would_overwrite_deleted".into(),
        "กำลังกู้คืนไฟล์ที่ถูกลบ: {}".into(),
    );
}
