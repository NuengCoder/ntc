use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.smtp.build_fail".into(),
        "ไม่สามารถสร้างข้อความอีเมลได้".into(),
    );

    m.insert(
        "cmd.smtp.create_one_hint".into(),
        "  สร้างเลย: {} -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert(
        "cmd.smtp.create_syntax".into(),
        "smtp -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert("cmd.smtp.create_usage".into(), "ต้องระบุเซิร์ฟเวอร์ ชื่อผู้ใช้ และรหัสผ่าน. การใช้งาน: smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.created_label".into(), "สร้างแล้ว:".into());

    m.insert(
        "cmd.smtp.decode_failed".into(),
        "ไม่สามารถถอดรหัสรหัสผ่านที่เก็บไว้ได้".into(),
    );

    m.insert(
        "cmd.smtp.body_file_missing".into(),
        "ไม่พบไฟล์ body: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_html_from_file".into(),
        "body แบบ HTML จากไฟล์: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_md_from_file".into(),
        "body แบบ Markdown จากไฟล์: {} (แปลงเป็น HTML แล้ว)".into(),
    );

    m.insert(
        "cmd.smtp.body_read_fail".into(),
        "อ่านไฟล์ body {} ไม่สำเร็จ: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_stdin_empty".into(),
        "ไม่ได้รับ body จาก stdin".into(),
    );

    m.insert(
        "cmd.smtp.body_text_from_file".into(),
        "body แบบข้อความล้วนจากไฟล์: {}".into(),
    );

    m.insert(
        "cmd.smtp.help_body_file".into(),
        "  -b letter.html | letter.md      อ่าน body จากไฟล์ (Markdown ถูกแปลงเป็น HTML)".into(),
    );

    m.insert(
        "cmd.smtp.help_body_format".into(),
        "  --body-format auto|text|html|md กำหนดวิธีตีความ -b".into(),
    );

    m.insert(
        "cmd.smtp.help_body_inline".into(),
        "  -b '<p style=text-align:center>สวัสดี</p>'   ตรวจพบ HTML ในบรรทัดโดยอัตโนมัติ".into(),
    );

    m.insert(
        "cmd.smtp.help_body_stdin".into(),
        "  -b -                           อ่าน body จาก stdin".into(),
    );

    m.insert("cmd.smtp.help_body_title".into(), "เนื้อหาอีเมล:".into());

    m.insert(
        "cmd.smtp.invalid_body_format".into(),
        "รูปแบบ body ไม่รู้จัก '{}' (ใช้ auto, text, html หรือ md)".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_file".into(),
        "ไม่พบ path หลัง --body-file".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_format".into(),
        "ไม่พบค่าหลัง --body-format".into(),
    );

    m.insert(
        "cmd.smtp.help_attachments_desc".into(),
        "  -f รับเส้นทางไฟล์หรือไดเรกทอรีคั่นด้วยเครื่องหมายจุลภาค (ขยายไฟล์ในไดเรกทอรีแบบเรียกซ้ำ)".into(),
    );

    m.insert("cmd.smtp.help_attachments_example".into(), "  ตัวอย่าง: smtp --id a1b2 --to user@test.com -s \\\"Report\\\" -b \\\"See attached\\\" -f \\\"./report.pdf,./images/\\\"".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "ไฟล์แนบ:".into());

    m.insert(
        "cmd.smtp.help_create_example".into(),
        "  ตัวอย่าง: smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123".into(),
    );

    m.insert(
        "cmd.smtp.help_create_syntax".into(),
        "  smtp -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert("cmd.smtp.help_create_title".into(), "สร้างโปรไฟล์:".into());

    m.insert(
        "cmd.smtp.help_info_syntax".into(),
        "  smtp --id-info <id>".into(),
    );

    m.insert("cmd.smtp.help_info_title".into(), "แสดงข้อมูลโปรไฟล์:".into());

    m.insert("cmd.smtp.help_list_syntax".into(), "  smtp --id-ls".into());

    m.insert("cmd.smtp.help_list_title".into(), "แสดงรายชื่อโปรไฟล์:".into());

    m.insert(
        "cmd.smtp.help_port_25".into(),
        "  25   Plain (ถ้าเซิร์ฟเวอร์รองรับ)".into(),
    );

    m.insert("cmd.smtp.help_port_465".into(), "  465  TLS (SMTPS)".into());

    m.insert(
        "cmd.smtp.help_port_587".into(),
        "  587  STARTTLS (ค่าเริ่มต้น)".into(),
    );

    m.insert("cmd.smtp.help_ports_title".into(), "หมายเลขพอร์ต:".into());

    m.insert(
        "cmd.smtp.help_rm_syntax".into(),
        "  smtp --id-rm <id>".into(),
    );

    m.insert("cmd.smtp.help_rm_title".into(), "ลบโปรไฟล์:".into());

    m.insert("cmd.smtp.help_send_syntax".into(), "  smtp --id <id> --to <email> [--cc <email>] [--bcc <email>] -s <subject> -b <body> [-f <files>]".into());

    m.insert("cmd.smtp.help_send_title".into(), "ส่งอีเมล:".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP Email Client".into());

    m.insert("cmd.smtp.id_label".into(), "ID:".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "BCC ไม่ถูกต้อง: {}".into());

    m.insert("cmd.smtp.invalid_cc".into(), "CC ไม่ถูกต้อง: {}".into());

    m.insert("cmd.smtp.invalid_port".into(), "หมายเลขพอร์ตไม่ถูกต้อง".into());

    m.insert(
        "cmd.smtp.invalid_pwd_encoding".into(),
        "การเข้ารหัสรหัสผ่านไม่ถูกต้อง: {}".into(),
    );

    m.insert("cmd.smtp.invalid_recipient".into(), "ผู้รับไม่ถูกต้อง: {}".into());

    m.insert("cmd.smtp.invalid_sender".into(), "ที่อยู่อีเมลผู้ส่งไม่ถูกต้อง".into());

    m.insert("cmd.smtp.missing_body".into(), "ไม่พบเนื้อหาหลัง -b".into());

    m.insert("cmd.smtp.missing_id".into(), "ไม่พบ ID หลัง --id".into());

    m.insert(
        "cmd.smtp.missing_id_info".into(),
        "ไม่พบ ID หลัง --id-info".into(),
    );

    m.insert(
        "cmd.smtp.missing_id_rm".into(),
        "ไม่พบ ID หลัง --id-rm".into(),
    );

    m.insert(
        "cmd.smtp.missing_password".into(),
        "ไม่พบรหัสผ่านหลัง -pwd".into(),
    );

    m.insert("cmd.smtp.missing_path".into(), "ไม่พบเส้นทางหลัง -f".into());

    m.insert("cmd.smtp.missing_port".into(), "ไม่พบพอร์ตหลัง -p".into());

    m.insert(
        "cmd.smtp.missing_recipient_bcc".into(),
        "ไม่พบผู้รับหลัง --bcc".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_cc".into(),
        "ไม่พบผู้รับหลัง --cc".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_to".into(),
        "ไม่พบผู้รับหลัง --to".into(),
    );

    m.insert(
        "cmd.smtp.missing_server".into(),
        "ไม่พบเซิร์ฟเวอร์หลัง -i".into(),
    );

    m.insert("cmd.smtp.missing_subject".into(), "ไม่พบหัวข้อหลัง -s".into());

    m.insert("cmd.smtp.missing_user".into(), "ไม่พบชื่อผู้ใช้หลัง -u".into());

    m.insert(
        "cmd.smtp.need_recipient".into(),
        "ต้องระบุผู้รับ. ใช้ --to <email>".into(),
    );

    m.insert(
        "cmd.smtp.need_subject".into(),
        "ต้องระบุหัวข้อ. ใช้ -s <subject>".into(),
    );

    m.insert(
        "cmd.smtp.no_files_in_dir".into(),
        "ไม่พบไฟล์ในไดเรกทอรี: {}".into(),
    );

    m.insert(
        "cmd.smtp.no_profiles".into(),
        "  {}  ไม่พบโปรไฟล์ SMTP.".into(),
    );

    m.insert("cmd.smtp.password_label".into(), "รหัสผ่าน:".into());

    m.insert("cmd.smtp.path_not_found".into(), "ไม่พบเส้นทาง: {}".into());

    m.insert("cmd.smtp.port_label".into(), "พอร์ต:".into());

    m.insert("cmd.smtp.profile_label".into(), "โปรไฟล์ SMTP:".into());

    m.insert("cmd.smtp.profile_not_found".into(), "ไม่พบโปรไฟล์: {}".into());

    m.insert(
        "cmd.smtp.profile_saved".into(),
        "บันทึกโปรไฟล์ SMTP แล้ว: {}".into(),
    );

    m.insert("cmd.smtp.profiles_title".into(), "โปรไฟล์ SMTP:".into());

    m.insert(
        "cmd.smtp.read_attach_fail".into(),
        "ไม่สามารถอ่านไฟล์แนบ: {}".into(),
    );

    m.insert(
        "cmd.smtp.removed_profile".into(),
        "ลบโปรไฟล์ SMTP แล้ว: {}".into(),
    );

    m.insert("cmd.smtp.send_fail".into(), "ไม่สามารถส่งอีเมล: {}".into());

    m.insert("cmd.smtp.send_ok".into(), "ส่งอีเมลสำเร็จแล้ว{}".into());

    m.insert("cmd.smtp.sending".into(), "กำลังส่งอีเมลไปยัง {}...".into());

    m.insert("cmd.smtp.server_label".into(), "เซิร์ฟเวอร์:".into());

    m.insert("cmd.smtp.skipping_exec".into(), "ข้ามไฟล์ปฏิบัติการ: {}".into());

    m.insert(
        "cmd.smtp.skipping_not_file".into(),
        "ข้าม (ไม่ใช่ไฟล์หรือไดเรกทอรี): {}".into(),
    );

    m.insert(
        "cmd.smtp.transport_fail".into(),
        "ไม่สามารถกำหนดค่า SMTP transport: {}".into(),
    );

    m.insert("cmd.smtp.unknown_option".into(), "ตัวเลือกไม่รู้จัก: {}".into());

    m.insert("cmd.smtp.user_label".into(), "ผู้ใช้:".into());

    m.insert("cmd.smtp.with_attachments".into(), " ({} ไฟล์แนบ{})".into());

    m.insert(
        "cmd.smtp.help_info_deadlock".into(),
        "  smtp --id-info <id> --deadlock   แสดงรหัสผ่านแอปด้วย (ถามรหัสผ่านหลักก่อน)".into(),
    );

    m.insert(
        "cmd.smtp.password_hidden".into(),
        "******** (ซ่อนอยู่ - ใช้ --deadlock)".into(),
    );
}
