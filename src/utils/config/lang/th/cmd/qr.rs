use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qr.decode_fail".into(),
        "ไม่สามารถถอดรหัส QR #{}: {}".into(),
    );

    m.insert(
        "cmd.qr.decode_requires".into(),
        "การถอดรหัส QR ต้องใช้คุณสมบัติ 'image2doc' (เปิดใช้งานเป็นค่าเริ่มต้น). เรียกด้วย --features image2doc."
            .into(),
    );

    m.insert(
        "cmd.qr.decode_usage".into(),
        "การใช้งาน: qr decode <ไฟล์ภาพ>".into(),
    );

    m.insert(
        "cmd.qr.encode_usage".into(),
        "การใช้งาน: qr encode <ข้อความ|ภาพ> [-o <ไฟล์>]".into(),
    );

    m.insert("cmd.qr.example1".into(), "  qr \"Hello, World!\"".into());

    m.insert(
        "cmd.qr.example2".into(),
        "  qr encode \\\"hello world\\\" -o qr.png".into(),
    );

    m.insert("cmd.qr.example3".into(), "  qr decode qr.png".into());

    m.insert(
        "cmd.qr.example_image".into(),
        "  qr encode image.png -o imageqrcode.png".into(),
    );

    m.insert("cmd.qr.examples_title".into(), "ตัวอย่าง:".into());

    m.insert("cmd.qr.file_not_found".into(), "ไม่พบไฟล์: {}".into());

    m.insert("cmd.qr.gen_fail".into(), "ไม่สามารถสร้าง QR code: {}".into());

    m.insert(
        "cmd.qr.help_decode".into(),
        "  qr decode <ไฟล์ภาพ>           ถอดรหัส QR code จากภาพ".into(),
    );

    m.insert(
        "cmd.qr.help_encode".into(),
        "  qr <ข้อความ>                        เข้ารหัสและแสดง QR code ในเทอร์มินัล".into(),
    );

    m.insert(
        "cmd.qr.help_same_as_above".into(),
        "  qr encode <ข้อความ>                 เหมือนกับด้านบน".into(),
    );

    m.insert(
        "cmd.qr.help_save_png".into(),
        "  qr encode <ข้อความ> -o <ไฟล์>       บันทึก QR เป็นภาพ PNG".into(),
    );

    m.insert(
        "cmd.qr.help_encode_image".into(),
        "  qr encode <ภาพ> -o <ไฟล์>       เข้ารหัสภาพเป็น QR code (ย่อขนาดอัตโนมัติ)".into(),
    );

    m.insert(
        "cmd.qr.image_save_requires".into(),
        "การบันทึกภาพต้องใช้คุณสมบัติ 'image2doc' (เปิดใช้งานเป็นค่าเริ่มต้น)".into(),
    );

    m.insert(
        "cmd.qr.image_requires_out".into(),
        "การเข้ารหัสภาพต้องระบุไฟล์ผลลัพธ์: qr encode <ภาพ> -o <ไฟล์>.png".into(),
    );

    m.insert(
        "cmd.qr.image_too_large".into(),
        "ภาพใหญ่เกินไปที่จะใส่ใน QR code แม้ขนาดต่ำสุดแล้ว.".into(),
    );

    m.insert("cmd.qr.no_qr_detected".into(), "ไม่พบ QR code ในภาพ.".into());

    m.insert("cmd.qr.no_text".into(), "ไม่มีข้อความสำหรับเข้ารหัส".into());

    m.insert("cmd.qr.open_image_fail".into(), "ไม่สามารถเปิดภาพ: {}".into());

    m.insert("cmd.qr.qr_label".into(), "{} {}".into());

    m.insert("cmd.qr.saved".into(), "บันทึก QR code ไปที่: {}".into());

    m.insert("cmd.qr.usage_title".into(), "QR Code:".into());
}
