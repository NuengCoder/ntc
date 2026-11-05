use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "dl.cancel_hint".into(),
        "ใช้ 'dl --cancel <number|url>' เพื่อลบระเบียน.".into(),
    );

    m.insert("dl.completed".into(), "ดาวน์โหลด {} ({} ใน {}, {})".into());

    m.insert(
        "dl.dns_host_required".into(),
        "ต้องระบุโฮสต์. การใช้งาน: dns <host>".into(),
    );

    m.insert(
        "dl.dns_looking_up".into(),
        "กำลังค้นหา {} {} ระเบียนด้วยเครื่องมือระบบ...".into(),
    );

    m.insert(
        "dl.dns_lookup_failed".into(),
        "DNS lookup ล้มเหลว (รหัสออก: {})".into(),
    );

    m.insert(
        "dl.dns_missing_type".into(),
        "ไม่พบประเภทระเบียนหลัง -t".into(),
    );

    m.insert("dl.dns_no_records".into(), "  ไม่พบ {} {} ระเบียน".into());

    m.insert(
        "dl.dns_tool_failed".into(),
        "ไม่สามารถเรียกเครื่องมือ DNS: {}. ติดตั้ง dig หรือ nslookup เพื่อรองรับการค้นหา {}".into(),
    );

    m.insert("dl.downloading".into(), "กำลังดาวน์โหลด {}".into());

    m.insert(
        "dl.failed_append".into(),
        "ไม่สามารถเปิด {} เพื่อผนวกข้อมูล".into(),
    );

    m.insert("dl.failed_create".into(), "ไม่สามารถสร้าง {}".into());

    m.insert("dl.failed_download".into(), "ไม่สามารถดาวน์โหลด {}".into());

    m.insert("dl.failed_read".into(), "ไม่สามารถอ่าน {}".into());

    m.insert(
        "dl.file_still_exists".into(),
        "ไฟล์ '{}' ยังคงอยู่. ลบด้วยตนเองหากต้องการ.".into(),
    );

    m.insert("dl.help.examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "dl.help.opt_cancel".into(),
        "   --cancel <number|url> ลบระเบียนดาวน์โหลด".into(),
    );

    m.insert(
        "dl.help.opt_help".into(),
        "   -h, --help แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "dl.help.opt_o".into(),
        "   -o, --output <file> บันทึกเป็นชื่อไฟล์".into(),
    );

    m.insert(
        "dl.help.opt_history".into(),
        "   --history แสดงประวัติการดาวน์โหลด".into(),
    );

    m.insert(
        "dl.help.opt_resume".into(),
        "   --resume ดำเนินการดาวน์โหลดต่อรายการที่ยังไม่เสร็จ (เลือกจากรายการ)".into(),
    );

    m.insert(
        "dl.help.opt_stop".into(),
        "   --stop หยุดการดาวน์โหลดที่กำลังทำงาน (เลือกจากรายการ)".into(),
    );

    m.insert("dl.help.options".into(), "ตัวเลือก:".into());

    m.insert("dl.help.title".into(), "ดาวน์โหลด (dl)".into());

    m.insert("dl.help.usage".into(), "การใช้งาน:".into());

    m.insert("dl.help.usage_line".into(), "  dl <url> [options]".into());

    m.insert("dl.history_title".into(), "ประวัติการดาวน์โหลด:".into());

    m.insert("dl.http_error".into(), "HTTP {}: {}".into());

    m.insert(
        "dl.invalid_header".into(),
        "รูปแบบส่วนหัวไม่ถูกต้อง คาดว่า \"Key: Value\" แต่ได้: {}".into(),
    );

    m.insert("dl.invalid_index".into(), "ดัชนีไม่ถูกต้อง".into());

    m.insert("dl.invalid_request".into(), "คำขอไม่ถูกต้อง: {}".into());

    m.insert("dl.missing_cancel".into(), "ไม่พบ URL/ชื่อหลัง --cancel".into());

    m.insert("dl.missing_data".into(), "ไม่พบข้อมูลหลัง -d".into());

    m.insert("dl.missing_header".into(), "ไม่พบค่าส่วนหัวหลัง -H".into());

    m.insert("dl.missing_o".into(), "ไม่พบชื่อไฟล์หลัง -o".into());

    m.insert("dl.no_records".into(), "ยังไม่มีระเบียนดาวน์โหลด.".into());

    m.insert("dl.not_found".into(), "ไม่พบระเบียนดาวน์โหลดสำหรับ: {}".into());

    m.insert("dl.not_found_404".into(), "ไม่พบ URL (404): {}".into());

    m.insert("dl.ping_exit_code".into(), "ping ออกด้วยรหัส: {}".into());

    m.insert("dl.ping_failed".into(), "ไม่สามารถเรียก ping: {}".into());

    m.insert(
        "dl.ping_host_required".into(),
        "ต้องระบุโฮสต์. การใช้งาน: ping <host>".into(),
    );

    m.insert("dl.ping_invalid_count".into(), "จำนวนไม่ถูกต้อง".into());

    m.insert("dl.ping_missing_count".into(), "ไม่พบจำนวนหลัง -c".into());

    m.insert("dl.removed_record".into(), "ลบระเบียนดาวน์โหลดแล้ว: {}".into());

    m.insert("dl.request_failed".into(), "คำขอไปยัง {} ล้มเหลว".into());

    m.insert(
        "dl.up_url_required".into(),
        "ต้องระบุ URL. การใช้งาน: up <file> <url>".into(),
    );

    m.insert("dl.upload_failed".into(), "อัปโหลดไปยัง {} ล้มเหลว".into());

    m.insert("dl.uploading".into(), "กำลังอัปโหลด {} ({})".into());

    m.insert(
        "dl.url_required".into(),
        "ต้องระบุ URL. การใช้งาน: dl <url>".into(),
    );

    m.insert("dl.active_title".into(), "รายการที่กำลังดาวน์โหลด:".into());

    m.insert("dl.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert(
        "dl.incomplete_title".into(),
        "รายการที่ยังดาวน์โหลดไม่เสร็จ:".into(),
    );

    m.insert("dl.invalid_choice".into(), "ตัวเลือกไม่ถูกต้อง.".into());

    m.insert("dl.no_active".into(), "ไม่มีการดาวน์โหลดที่กำลังทำงาน.".into());

    m.insert(
        "dl.nothing_incomplete".into(),
        "ไม่มีรายการที่ยังดาวน์โหลดไม่เสร็จให้ทำต่อ.".into(),
    );

    m.insert("dl.resuming".into(), "กำลังทำต่อ {} จาก {}".into());

    m.insert(
        "dl.select_resume".into(),
        "เลือกไฟล์ที่จะทำต่อ [1-N] (เว้นว่างหรือ q เพื่อยกเลิก): ".into(),
    );

    m.insert(
        "dl.select_stop".into(),
        "เลือกไฟล์ที่จะหยุด [1-N] (เว้นว่างหรือ q เพื่อยกเลิก): ".into(),
    );

    m.insert(
        "dl.stopped_ok".into(),
        "หยุด {} แล้ว ทำต่อได้ด้วย 'dl --resume'.".into(),
    );

    m.insert(
        "dl.stopped_stale".into(),
        "ทำเครื่องหมาย {} เป็นหยุดแล้ว (ไม่ได้ทำงานอยู่แล้ว).".into(),
    );

    m.insert(
        "dl.dns_unsupported_type".into(),
        "ไม่รองรับประเภทระเบียน: {} (รองรับ: {})".into(),
    );

    m.insert("dl.dns_rcode".into(), "เซิร์ฟเวอร์ตอบกลับ {} ({})".into());

    m.insert(
        "dl.dns_timeout".into(),
        "ไม่มีการตอบกลับจากเซิร์ฟเวอร์ DNS (หมดเวลา)".into(),
    );

    m.insert("dl.dns_malformed".into(), "การตอบกลับ DNS ผิดรูปแบบ".into());

    m.insert("dl.dns_io".into(), "ข้อผิดพลาดเครือข่าย DNS: {}".into());

    m.insert(
        "dl.dns_system_fallback".into(),
        "เปลี่ยนไปใช้เครื่องมือ DNS ของระบบ...".into(),
    );
}
