use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "msg.ignore.dir_added".into(),
        "กำลังละเว้นไดเรกทอรี: {} แล้ว".into(),
    );

    m.insert(
        "msg.ignore.dir_removed".into(),
        "เลิกละเว้นไดเรกทอรี: {} แล้ว".into(),
    );

    m.insert("msg.ignore.ext_added".into(), "กำลังละเว้นไฟล์ .{} แล้ว".into());

    m.insert("msg.ignore.ext_removed".into(), "กำลังดูแลไฟล์ .{} แล้ว".into());

    m.insert(
        "msg.ignore.file_added".into(),
        "กำลังละเว้นไฟล์: {} แล้ว".into(),
    );

    m.insert(
        "msg.ignore.file_removed".into(),
        "กำลังดูแลไฟล์: {} แล้ว".into(),
    );

    m.insert("msg.ignored.dirs".into(), "ไดเรกทอรีที่ถูกละเว้น: {}".into());

    m.insert("msg.ignored.extensions".into(), "นามสกุลที่ถูกละเว้น: {}".into());

    m.insert(
        "msg.ignored.extra_extensions".into(),
        "นามสกุลที่รองรับเพิ่มเติม: {}".into(),
    );

    m.insert(
        "msg.ignored.extra_files".into(),
        "ไฟล์ที่รองรับเพิ่มเติม: {}".into(),
    );

    m.insert("msg.ignored.files".into(), "ไฟล์ที่ถูกละเว้น: {}".into());

    m.insert(
        "msg.image.converted".into(),
        "แปลงภาพและบันทึกไปที่: {} แล้ว".into(),
    );

    m.insert("msg.image.not_found".into(), "ไม่พบภาพ: {}".into());

    m.insert(
        "msg.init.template_created".into(),
        "สร้างเทมเพลตแล้ว: {}".into(),
    );

    m.insert("msg.list.entry_format".into(), " {}".into());

    m.insert("msg.list.title".into(), "ntc {} - ฟังก์ชันบรรทัดคำสั่งที่มี:".into());

    m.insert(
        "msg.report.binary_clipboard_warning".into(),
        "รายงาน {} ไม่สามารถคัดลอกไปยังคลิปบอร์ดได้ (รูปแบบไบนารี)".into(),
    );

    m.insert(
        "msg.report.copied_to_clipboard".into(),
        "คัดลอกรายงาน {} ไปยังคลิปบอร์ดแล้ว!".into(),
    );

    m.insert("msg.report.file_saved".into(), "บันทึกไฟล์ไปที่: {} แล้ว".into());

    m.insert("msg.report.path_not_found".into(), "ไม่พบเส้นทาง: {}".into());

    m.insert(
        "msg.report.skipped_unsupported".into(),
        "ข้าม (ไม่รองรับรูปแบบ): {}".into(),
    );

    m.insert(
        "msg.setA.error".into(),
        "ค่า setA ไม่ถูกต้อง: {}. ใช้ ON หรือ OFF.".into(),
    );

    m.insert("msg.setA.off".into(), "ข้อความเลือน: OFF".into());

    m.insert("msg.setA.on".into(), "ข้อความเลือน: ON".into());

    m.insert(
        "msg.setA.status".into(),
        "ข้อความเลือน (ghost text): {}".into(),
    );

    m.insert(
        "msg.setC.error".into(),
        "ค่าสี setC ไม่ถูกต้อง: {}. ใช้ ON หรือ OFF.".into(),
    );

    m.insert("msg.setC.status".into(), "การแสดงสี: {}".into());

    m.insert("msg.setC.success".into(), "สี: {}".into());

    m.insert(
        "msg.setD.error_out_of_range".into(),
        "ค่าความลึกไม่ถูกต้อง: {}. ต้องอยู่ระหว่าง 1 ถึง 20.".into(),
    );

    m.insert(
        "msg.setD.error_parse".into(),
        "ค่าความลึกไม่ถูกต้อง: {}. ต้องเป็นจำนวนเต็มบวกระหว่าง 1 ถึง 20.".into(),
    );

    m.insert("msg.setD.status".into(), "ความลึกสูงสุดปัจจุบัน: {}".into());

    m.insert("msg.setD.success".into(), "ตั้งความลึกสูงสุดเป็น: {} แล้ว".into());

    m.insert("msg.setH.off".into(), "ประวัติ: OFF".into());

    m.insert("msg.setH.on".into(), "ประวัติ: ON".into());

    m.insert("msg.setH.path".into(), "เส้นทางประวัติ: {}".into());

    m.insert(
        "msg.setH.path_default".into(),
        "เส้นทางประวัติ: ค่าเริ่มต้น".into(),
    );

    m.insert(
        "msg.setH.path_reset".into(),
        "รีเซ็ตเส้นทางประวัติเป็นค่าเริ่มต้นแล้ว".into(),
    );

    m.insert(
        "msg.setH.path_set".into(),
        "ตั้งเส้นทางประวัติเป็น: {} แล้ว".into(),
    );

    m.insert("msg.setH.status".into(), "ประวัติ: {}".into());

    m.insert(
        "msg.setL.error".into(),
        "ค่า setL ไม่ถูกต้อง: {}. ใช้ ON หรือ OFF.".into(),
    );

    m.insert("msg.setL.status".into(), "หมายเลขบรรทัด: {}".into());

    m.insert("msg.setL.success".into(), "หมายเลขบรรทัด: {}".into());

    m.insert("msg.setO.status".into(), "เส้นทางผลลัพธ์ปัจจุบัน: {}".into());

    m.insert("msg.setO.success".into(), "ตั้งเส้นทางผลลัพธ์เป็น: {} แล้ว".into());

    m.insert(
        "msg.setT.error_out_of_range".into(),
        "จำนวนเธรดไม่ถูกต้อง: {}. ต้องอยู่ระหว่าง 1 ถึง 64.".into(),
    );

    m.insert(
        "msg.setT.error_parse".into(),
        "จำนวนเธรดไม่ถูกต้อง: {}. ต้องเป็นจำนวนเต็มบวกระหว่าง 1 ถึง 64.".into(),
    );

    m.insert("msg.setT.status".into(), "เธรดปัจจุบัน: {}".into());

    m.insert("msg.setT.success".into(), "ตั้งเธรดเป็น: {} แล้ว".into());

    m.insert("msg.showcg.box_bottom".into(), "\u{2514}{}\u{2518}".into());

    m.insert("msg.showcg.box_divider".into(), "\u{251C}{}\u{2524}".into());

    m.insert("msg.showcg.box_top".into(), "\u{250C}{}\u{2510}".into());

    m.insert("msg.showcg.label_color".into(), "สี:".into());

    m.insert("msg.showcg.label_history".into(), "ประวัติ:".into());

    m.insert(
        "msg.showcg.label_line_numbers".into(),
        "หมายเลขบรรทัด:".into(),
    );

    m.insert("msg.showcg.label_max_depth".into(), "ความลึกสูงสุด:".into());

    m.insert("msg.showcg.label_output_path".into(), "เส้นทางผลลัพธ์:".into());

    m.insert("msg.showcg.label_threads".into(), "เธรด:".into());

    m.insert("msg.showcg.label_watcher".into(), "ตัวเฝ้าดูไฟล์:".into());

    m.insert(
        "msg.showcg.label_watch_limit".into(),
        "ขีดจำกัด watch:".into(),
    );

    m.insert("msg.showcg.label_dir_view".into(), "Dir View:".into());

    m.insert(
        "msg.showcg.label_update_check".into(),
        "Update check:".into(),
    );

    m.insert("msg.showcg.label_serve_files".into(), "Serve files:".into());

    m.insert("msg.showcg.label_serve_http".into(), "Serve http:".into());

    m.insert("msg.showcg.label_serve_chess".into(), "Serve chess:".into());

    m.insert("msg.showcg.title".into(), "การกำหนดค่าปัจจุบัน".into());

    m.insert("msg.size.bytes_label".into(), "ไบต์:".into());

    m.insert("msg.size.human_label".into(), "มนุษย์:".into());

    m.insert("msg.size.path_label".into(), "เส้นทาง: {}".into());

    m.insert("msg.size.time_label".into(), "เวลา:".into());

    m.insert("msg.size.title".into(), "ขนาดไดเรกทอรีปัจจุบัน".into());

    m.insert(
        "msg.teleport.error".into(),
        "ไม่สามารถเทเลพอร์ตไปยัง '{}': {}".into(),
    );

    m.insert(
        "msg.teleport.hint_list".into(),
        "ใช้ 'ntc --tp-list' เพื่อดูจุดบันทึกทั้งหมด.".into(),
    );

    m.insert(
        "msg.teleport.not_found".into(),
        "ไม่พบจุดเทเลพอร์ต: '{}'".into(),
    );

    m.insert(
        "msg.teleport.success".into(),
        "\u{2713} เทเลพอร์ตไปยัง '{}' -> {} แล้ว".into(),
    );

    m.insert("msg.version.format".into(), "ntc {}".into());

    m.insert(
        "msg.watch.invalid".into(),
        "ใช้ --watch ON หรือ --watch OFF".into(),
    );

    m.insert(
        "msg.watch.off".into(),
        "ตัวเฝ้าดูไฟล์: OFF (รีสตาร์ท ntc เพื่อปิดใช้งาน)".into(),
    );

    m.insert(
        "msg.watch.on".into(),
        "ตัวเฝ้าดูไฟล์: ON (รีสตาร์ท ntc เพื่อเปิดใช้งาน)".into(),
    );

    m.insert("msg.watch.status".into(), "ตัวเฝ้าดูไฟล์: {}".into());

    m.insert(
        "msg.watch.limit_status".into(),
        "ขีดจำกัดการแสดงผล watch: {}".into(),
    );

    m.insert(
        "msg.watch.limit_success".into(),
        "ตั้งขีดจำกัดการแสดงผล watch เป็น {}".into(),
    );

    m.insert(
        "msg.watch.limit_error".into(),
        "ขีดจำกัด watch ไม่ถูกต้อง: '{}' ใช้ตัวเลขที่เป็นบวก".into(),
    );

    m.insert("msg.where.current_dir".into(), "ไดเรกทอรีปัจจุบัน: {}".into());

    m.insert(
        "msg.where.executable".into(),
        "ไฟล์ปฏิบัติการของ ntc: {}".into(),
    );

    m.insert("msg.showcg.label_file_view".into(), "มุมมองไฟล์:".into());

    m.insert("msg.showcg.label_auto_size".into(), "ขนาดอัตโนมัติ:".into());

    m.insert(
        "msg.showcg.auto_size_value".into(),
        "ไฟล์ {}, โฟลเดอร์ {}".into(),
    );

    m.insert("msg.setGit.status".into(), "Git integration: {}".into());

    m.insert("msg.setGit.on".into(), "เปิดใช้ Git integration แล้ว".into());

    m.insert("msg.setGit.off".into(), "ปิด Git integration แล้ว".into());

    m.insert(
        "msg.setGit.error".into(),
        "การใช้งาน: config git ON|OFF".into(),
    );

    m.insert("msg.setIV.status".into(), "Intro view: {}".into());

    m.insert(
        "msg.setIV.on".into(),
        "เปิดใช้ Intro view แล้ว (โลโก้, ข้อมูล ntpkg และ watcher)".into(),
    );

    m.insert(
        "msg.setIV.off".into(),
        "ปิด Intro view แล้ว (ไม่แสดงโลโก้, ข้อมูล ntpkg และ watcher)".into(),
    );

    m.insert(
        "msg.setIV.error".into(),
        "การใช้งาน: config intro_view ON|OFF".into(),
    );

    m.insert("msg.showcg.label_intro_view".into(), "Intro View:".into());

    m.insert("msg.setDV.status".into(), "Dir view: {}".into());

    m.insert("msg.setDV.on".into(), "เปิดใช้ Dir view แล้ว".into());

    m.insert("msg.setDV.off".into(), "ปิด Dir view แล้ว".into());

    m.insert(
        "msg.setDV.error".into(),
        "การใช้งาน: config dir_view ON|OFF".into(),
    );

    m.insert("msg.setFV.status".into(), "File view: {}".into());

    m.insert(
        "msg.setFV.on".into(),
        "เปิดใช้ File view แล้ว ไฟล์จะแสดงหลังจากเปลี่ยนไดเรกทอรี".into(),
    );

    m.insert("msg.setFV.off".into(), "ปิด File view แล้ว".into());

    m.insert(
        "msg.setFV.error".into(),
        "การใช้งาน: config file_view ON|OFF".into(),
    );

    m.insert(
        "msg.setSZ.status".into(),
        "Auto size: files {}, dirs {}".into(),
    );

    m.insert(
        "msg.setSZ.target_status".into(),
        "Auto size สำหรับ {}: {}".into(),
    );

    m.insert("msg.setSZ.on".into(), "เปิดใช้ Auto size สำหรับ {} แล้ว".into());

    m.insert("msg.setSZ.off".into(), "ปิด Auto size สำหรับ {} แล้ว".into());

    m.insert(
        "msg.setSZ.error".into(),
        "การตั้งค่าขนาดไม่ถูกต้อง การใช้งาน: config size all|file|dir ON|OFF".into(),
    );

    m.insert(
        "msg.setSZ.slow_warn".into(),
        "คำเตือน: Auto size ใช้ --care ทุกครั้ง cd/teleport อาจช้าลง".into(),
    );

    m.insert(
        "msg.setSZ.usage".into(),
        "การใช้งาน: config size all|file|dir ON|OFF".into(),
    );

    m.insert("msg.setUC.status".into(), "Update check: {}".into());

    m.insert("msg.setUC.on".into(), "เปิดแจ้งเตือนเวอร์ชันใหม่รายวันแล้ว".into());

    m.insert("msg.setUC.off".into(), "ปิดแจ้งเตือนเวอร์ชันใหม่แล้ว".into());

    m.insert(
        "msg.setUC.error".into(),
        "การใช้งาน: config update_check ON|OFF".into(),
    );

    m.insert(
        "msg.update.available".into(),
        "มี NTC {} ใหม่ (คุณใช้ {}) — รัน 'ntc --upgrade' ใน {} หรือดู {}".into(),
    );

    m.insert("msg.setTD.status".into(), "รูปแบบ Todo: {}".into());

    m.insert(
        "msg.setTD.set".into(),
        "ตั้งรูปแบบ Todo ให้เป็น {} (ไฟล์: future_todo.{})".into(),
    );

    m.insert(
        "msg.setTD.error".into(),
        "การใช้งาน: config todo format <txt|md|json|toml|yaml|html>".into(),
    );
}
