use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.upgrade.after_restart".into(),
        "เวอร์ชันใหม่จะพร้อมใช้งานหลังรีสตาร์ท.".into(),
    );

    m.insert(
        "cmd.upgrade.ahead".into(),
        "เวอร์ชันในเครื่อง ({}) ใหม่กว่ารุ่นล่าสุด ({}).".into(),
    );

    m.insert(
        "cmd.upgrade.already_latest".into(),
        "ntc {} เป็นเวอร์ชันล่าสุดแล้ว.".into(),
    );

    m.insert(
        "cmd.upgrade.api_parse_fail".into(),
        "ไม่สามารถแยกวิเคราะห์การตอบสนองของ GitHub API".into(),
    );

    m.insert(
        "cmd.upgrade.api_read_fail".into(),
        "ไม่สามารถอ่านการตอบสนองของ GitHub API".into(),
    );

    m.insert(
        "cmd.upgrade.api_request_fail".into(),
        "คำขอ GitHub API ล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.upgrade.check_fail".into(),
        "ไม่สามารถตรวจสอบอัปเดต: {}".into(),
    );

    m.insert(
        "cmd.upgrade.checking".into(),
        "กำลังตรวจสอบอัปเดต (เวอร์ชันปัจจุบัน: {})...".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail".into(),
        "ไม่สามารถคัดลอกไฟล์ปฏิบัติการ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail_android".into(),
        "ไม่สามารถคัดลอกไฟล์ปฏิบัติการ. ตรวจสอบพื้นที่ดิสก์หรือสิทธิ์.".into(),
    );

    m.insert(
        "cmd.upgrade.dev_build".into(),
        "คุณอาจกำลังใช้ build สำหรับพัฒนา.".into(),
    );

    m.insert(
        "cmd.upgrade.download_complete".into(),
        "ดาวน์โหลดเสร็จสิ้น. กำลังติดตั้งอัปเดต...".into(),
    );

    m.insert(
        "cmd.upgrade.download_fail".into(),
        "ดาวน์โหลดล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.upgrade.downloading".into(),
        "กำลังดาวน์โหลดอัปเดต...".into(),
    );

    m.insert(
        "cmd.upgrade.downloading_url".into(),
        "กำลังดาวน์โหลด: {}".into(),
    );

    m.insert(
        "cmd.upgrade.exe_path_fail".into(),
        "ไม่สามารถรับเส้นทางไฟล์ปฏิบัติการ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.install_fail".into(),
        "การติดตั้งล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.upgrade.missing_tag".into(),
        "ไม่พบ 'tag_name' ในการตอบสนองของ GitHub release".into(),
    );

    m.insert(
        "cmd.upgrade.new_version".into(),
        "มีเวอร์ชันใหม่: ntc {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_assets_release".into(),
        "ไม่พบแอสเซตใน release v{}".into(),
    );

    m.insert(
        "cmd.upgrade.perm_fail".into(),
        "ไม่สามารถตั้งค่าสิทธิ์ไฟล์ปฏิบัติการ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.progress_bytes".into(),
        "ดาวน์โหลดแล้ว: {} bytes".into(),
    );

    m.insert("cmd.upgrade.progress_pct".into(), "ความคืบหน้า: {}%".into());

    m.insert(
        "cmd.upgrade.restart".into(),
        "รีสตาร์ท ntc เพื่อใช้เวอร์ชันใหม่.".into(),
    );

    m.insert(
        "cmd.upgrade.stream_fail".into(),
        "ไม่สามารถอ่านสตรีมดาวน์โหลดได้".into(),
    );

    m.insert(
        "cmd.upgrade.sudo_hint".into(),
        "เรียกด้วยสิทธิ์ sudo/administrator".into(),
    );

    m.insert(
        "cmd.upgrade.time_remaining".into(),
        "เวลาที่เหลือ: {:>3} วินาที".into(),
    );

    m.insert(
        "cmd.upgrade.tmpdir_fail".into(),
        "ไม่สามารถสร้างไดเรกทอรีชั่วคราว: {}".into(),
    );

    m.insert(
        "cmd.upgrade.unsupported_platform".into(),
        "แพลตฟอร์มไม่รองรับ: {}".into(),
    );

    m.insert("cmd.upgrade.updated".into(), "ntc อัปเดตเป็น v{} แล้ว!".into());

    m.insert(
        "cmd.upgrade.write_fail".into(),
        "ไม่สามารถเขียนไฟล์ที่ดาวน์โหลด".into(),
    );

    m.insert(
        "cmd.upgrade.extracting".into(),
        "กำลังแยกไฟล์ binary...".into(),
    );

    m.insert(
        "cmd.upgrade.extract_fail".into(),
        "ไม่สามารถอ่าน/แยกไฟล์ที่ดาวน์โหลด: {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_file_in_archive".into(),
        "ไม่พบ '{}' ในไฟล์ที่ดาวน์โหลด".into(),
    );

    m.insert(
        "cmd.upgrade.no_installer_asset".into(),
        "ไม่พบ Windows installer สำหรับอัปเดต ntc".into(),
    );

    m.insert(
        "cmd.upgrade.installer_launch".into(),
        "กำลังเริ่มตัวติดตั้ง ntc (แบบเงียบ)...".into(),
    );

    m.insert(
        "cmd.upgrade.installer_launch_fail".into(),
        "ไม่สามารถเริ่มตัวติดตั้ง: {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_arch_asset".into(),
        "ไม่มี binary ntc สำหรับสถาปัตยกรรมนี้ ({}) สร้างจาก source: cargo build --release".into(),
    );

    m.insert(
        "cmd.upgrade.no_asset_match".into(),
        "ไม่พบ asset ที่ตรงกับ platform key '{}'".into(),
    );

    m.insert(
        "cmd.upgrade.installed_user_dir".into(),
        "ติดตั้ง ntc ไปที่ {} (ติดตั้งสำหรับผู้ใช้ ไม่ต้องใช้ admin/sudo).".into(),
    );

    m.insert(
        "cmd.upgrade.home_fail".into(),
        "ไม่สามารถกำหนด home directory ของคุณ".into(),
    );

    m.insert(
        "cmd.upgrade.path_exported".into(),
        "อัปเดต PATH ใน {}".into(),
    );
}
