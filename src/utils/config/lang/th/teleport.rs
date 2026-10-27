use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("teleport.added".into(), "สร้างจุดบันทึก '{}' แล้ว -> {}".into());

    m.insert("teleport.all_cleared".into(), "ล้างจุดบันทึกทั้งหมดแล้ว.".into());

    m.insert(
        "teleport.back_hint".into(),
        "ใช้ 'tpb' เพื่อกลับไปยังตำแหน่งก่อนหน้า.".into(),
    );

    m.insert("teleport.back_opt".into(), "b. เทเลพอร์ตกลับ".into());

    m.insert("teleport.cancel_opt".into(), "0. ยกเลิก".into());

    m.insert(
        "teleport.choose_prompt".into(),
        "ป้อนหมายเลขเพื่อเทเลพอร์ต (1-{}) หรือ b/h/0: ".into(),
    );

    m.insert("teleport.clear_cancelled".into(), "ยกเลิกการล้าง.".into());

    m.insert(
        "teleport.clear_confirm".into(),
        "\u{26A0} แน่ใจหรือว่าต้องการลบจุดบันทึกทั้งหมด? (y/N): ".into(),
    );

    m.insert(
        "teleport.clear_hint".into(),
        "ใช้ 'tp clear' เพื่อล้างประวัติ.".into(),
    );

    m.insert(
        "teleport.fs_failed".into(),
        "ไม่สามารถเข้าถึงระบบไฟล์: {}".into(),
    );

    m.insert(
        "teleport.history_cleared".into(),
        "ล้างประวัติเทเลพอร์ตแล้ว.".into(),
    );

    m.insert("teleport.history_opt".into(), "h. แสดงประวัติ".into());

    m.insert(
        "teleport.history_title".into(),
        "\u{1F4DC} ประวัติเทเลพอร์ต (ล่าสุดอยู่ท้ายสุด)".into(),
    );

    m.insert(
        "teleport.invalid_char".into(),
        "อักขระไม่ถูกต้องในชื่อ: '{}'".into(),
    );

    m.insert(
        "teleport.invalid_index".into(),
        "ดัชนีไม่ถูกต้อง: {}. ใช้ 1-{}".into(),
    );

    m.insert(
        "teleport.invalid_input_interactive".into(),
        "ข้อมูลนำเข้าไม่ถูกต้อง: {}. ป้อนตัวเลข 'b' หรือ 'h'.".into(),
    );

    m.insert(
        "teleport.invalid_name".into(),
        "ชื่อไม่ถูกต้อง: '{}' ถูกสงวนหรือว่างเปล่า".into(),
    );

    m.insert(
        "teleport.jump_failed".into(),
        "ไม่สามารถเทเลพอร์ตได้: '{}'".into(),
    );

    m.insert(
        "teleport.jump_hint".into(),
        "ใช้ 'tp jump <name>' เพื่อเทเลพอร์ต.".into(),
    );

    m.insert(
        "teleport.jumped".into(),
        "เทเลพอร์ตไปยัง '{}' -> {} แล้ว".into(),
    );

    m.insert(
        "teleport.list_title".into(),
        "\u{1F4CC} จุดบันทึกเทเลพอร์ตของคุณ (global)".into(),
    );

    m.insert(
        "teleport.menu_title".into(),
        "\u{1F4CC} เทเลพอร์ต - จุดบันทึกของคุณ (global)".into(),
    );

    m.insert(
        "teleport.name_exists".into(),
        "ไม่สามารถเปลี่ยนชื่อได้: '{}' มีอยู่แล้วเป็นจุดบันทึก".into(),
    );

    m.insert(
        "teleport.name_taken".into(),
        "จุดบันทึก '{}' มีอยู่แล้ว. ใช้ 'tp edit' เพื่ออัปเดต.".into(),
    );

    m.insert(
        "teleport.name_taken_edit".into(),
        "จุดบันทึก '{}' มีอยู่แล้ว".into(),
    );

    m.insert(
        "teleport.no_history_first".into(),
        "ไม่มีประวัติเทเลพอร์ต. ใช้ 'tp jump <name>' ก่อนเพื่อสร้างประวัติ.".into(),
    );

    m.insert(
        "teleport.no_history_start".into(),
        "ไม่มีประวัติเทเลพอร์ต. ใช้ 'tp jump <name>' เพื่อเริ่มสร้างประวัติ.".into(),
    );

    m.insert(
        "teleport.no_savepoints_to_clear".into(),
        "ไม่มีจุดบันทึกต้องล้าง.".into(),
    );

    m.insert(
        "teleport.no_savepoints_yet".into(),
        "ยังไม่มีจุดบันทึก. ใช้ 'tp add <name>' เพื่อสร้าง.".into(),
    );

    m.insert("teleport.not_found".into(), "ไม่พบจุดบันทึก: '{}'".into());

    m.insert(
        "teleport.overwrite_cancelled".into(),
        "ยกเลิกการเขียนทับ.".into(),
    );

    m.insert(
        "teleport.overwrite_prompt".into(),
        "จุดบันทึก '{}' มีอยู่แล้วในตำแหน่งอื่น. เขียนทับ? (y/N): ".into(),
    );

    m.insert(
        "teleport.prev_gone".into(),
        "ตำแหน่งก่อนหน้าไม่มีอยู่แล้ว: {}".into(),
    );

    m.insert("teleport.removed".into(), "ลบจุดบันทึกแล้ว: '{}'".into());

    m.insert(
        "teleport.removing_from_history".into(),
        "กำลังลบออกจากประวัติ. ใช้ 'tp history' เพื่อดูรายการที่เหลือ.".into(),
    );

    m.insert(
        "teleport.rename_failed".into(),
        "ไม่สามารถเปลี่ยนชื่อจุดบันทึก: {}".into(),
    );

    m.insert("teleport.rename_same".into(), "ชื่อใหม่เหมือนชื่อเดิม".into());

    m.insert(
        "teleport.renamed".into(),
        "เปลี่ยนชื่อจุดบันทึก '{}' \u{2192} '{}' แล้ว".into(),
    );

    m.insert(
        "teleport.renamed_with_path".into(),
        "เปลี่ยนชื่อจุดบันทึก '{}' -> '{}' (-> {}) แล้ว".into(),
    );

    m.insert(
        "teleport.teleport_cancelled".into(),
        "ยกเลิกการเทเลพอร์ต.".into(),
    );

    m.insert(
        "teleport.teleported_back".into(),
        "เทเลพอร์ตกลับไปที่: {}".into(),
    );

    m.insert(
        "teleport.tpb_hint".into(),
        "ใช้ 'tpb' เพื่อกลับไปยังตำแหน่งก่อนหน้า.".into(),
    );

    m.insert(
        "teleport.updated".into(),
        "อัปเดตจุดบันทึก '{}': {} -> {} แล้ว".into(),
    );
}
