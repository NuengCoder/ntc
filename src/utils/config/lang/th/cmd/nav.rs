use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.nav.back.invalid_number".into(),
        "ตัวเลขไม่ถูกต้อง: {}. การใช้งาน: back [n]".into(),
    );

    m.insert(
        "cmd.nav.back.invalid_step".into(),
        "ไม่มีไดเรกทอรีหลักที่ขั้นตอน {} - ไม่มีที่ไปต่อ".into(),
    );

    m.insert("cmd.nav.cd.example1".into(), "ตัวอย่าง: cd C:\\Users".into());

    m.insert("cmd.nav.cd.example2".into(), "  cd subdir".into());

    m.insert(
        "cmd.nav.cd.example3".into(),
        "  cd to work # เทเลพอร์ตไปยังจุดบันทึก 'work'".into(),
    );

    m.insert("cmd.nav.cd.navigated".into(), "นำทางไปที่: {}".into());

    m.insert(
        "cmd.nav.cd.tp_list".into(),
        "ใช้ 'tp list' เพื่อดูจุดบันทึกทั้งหมด.".into(),
    );

    m.insert(
        "cmd.nav.cd.tp_not_found".into(),
        "ไม่พบจุดเทเลพอร์ต: '{}'".into(),
    );

    m.insert(
        "cmd.nav.cd.usage1".into(),
        "การใช้งาน: cd <เส้นทางไดเรกทอรี>".into(),
    );

    m.insert(
        "cmd.nav.cd.usage2".into(),
        "  cd to <ชื่อ_tp> เทเลพอร์ตไปยังจุดบันทึก".into(),
    );

    m.insert(
        "cmd.nav.drive_windows_only".into(),
        "การนำทางไดรฟ์รองรับเฉพาะบน Windows.".into(),
    );

    m.insert("cmd.nav.fgo.cancel".into(), "0. ยกเลิก".into());

    m.insert("cmd.nav.fgo.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert(
        "cmd.nav.fgo.example1".into(),
        "  fgo main.rs # ค้นหาแล้วเลือกไฟล์เพื่อไปยังไดเรกทอรีหลัก".into(),
    );

    m.insert(
        "cmd.nav.fgo.example2".into(),
        "  fgo main.rs -d 5 # ค้นหาลึกสุด 5 ระดับ".into(),
    );

    m.insert(
        "cmd.nav.fgo.found".into(),
        "\u{1f50d} พบ {} ไฟล์สำหรับ \"{}\":".into(),
    );

    m.insert(
        "cmd.nav.fgo.select".into(),
        "เลือกไฟล์เพื่อไปยังไดเรกทอรีหลัก: ".into(),
    );

    m.insert(
        "cmd.nav.fgo_usage".into(),
        "การใช้งาน: fgo <รูปแบบ> [-d <ความลึก>]".into(),
    );

    m.insert("cmd.nav.gos.empty".into(), "(ไม่มีไดเรกทอรีย่อย)".into());

    m.insert("cmd.nav.gos.exit".into(), "exit".into());

    m.insert("cmd.nav.gos.staying".into(), "อยู่ที่นี่ต่อไป.".into());

    m.insert("cmd.nav.invalid_drive".into(), "หมายเลขไดรฟ์ไม่ถูกต้อง.".into());

    m.insert("cmd.nav.invalid_input".into(), "ข้อมูลนำเข้าไม่ถูกต้อง.".into());

    m.insert("cmd.nav.invalid_number".into(), "ตัวเลขไม่ถูกต้อง.".into());

    m.insert("cmd.nav.invalid_selection".into(), "การเลือกไม่ถูกต้อง.".into());

    m.insert(
        "cmd.nav.where.config_file".into(),
        "\u{2699}\u{fe0f} ไฟล์การกำหนดค่า:".into(),
    );

    m.insert(
        "cmd.nav.where.config_missing".into(),
        "\u{26a0} ไม่พบไฟล์การกำหนดค่า (จะสร้างขึ้นเมื่อบันทึกครั้งแรก)".into(),
    );

    m.insert(
        "cmd.nav.where.config_size".into(),
        "\u{1f4cf} ขนาดการกำหนดค่า:".into(),
    );

    m.insert(
        "cmd.nav.where.current_dir".into(),
        "\u{1f4c2} ไดเรกทอรีปัจจุบัน:".into(),
    );

    m.insert(
        "cmd.nav.where.executable".into(),
        "\u{1f4c1} ไฟล์ปฏิบัติการ:".into(),
    );

    m.insert(
        "cmd.nav.where.not_found".into(),
        "\u{2717} ไม่พบ '{}' ใน PATH".into(),
    );

    m.insert(
        "cmd.nav.where.path_title".into(),
        "\u{1f50d} ตำแหน่งของ '{}' ใน PATH:".into(),
    );

    m.insert("cmd.nav.where.title".into(), "ข้อมูลตำแหน่งของ ntc".into());
}
