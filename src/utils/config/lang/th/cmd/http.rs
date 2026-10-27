use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.http.authorization_label".into(), "การอนุญาต:".into());

    m.insert(
        "cmd.http.basic_placeholder".into(),
        "Basic <encoded>".into(),
    );

    m.insert(
        "cmd.http.bearer_placeholder".into(),
        "Bearer <โทเคน>".into(),
    );

    m.insert("cmd.http.byte_count".into(), "{} ไบต์".into());

    m.insert(
        "cmd.http.help_auth".into(),
        "  -a, --auth <ผู้ใช้:รหัสผ่าน>         การรับรองความถูกต้องพื้นฐาน".into(),
    );

    m.insert(
        "cmd.http.help_bearer".into(),
        "  --bearer <โทเคน>               การรับรองความถูกต้องด้วย Bearer token".into(),
    );

    m.insert(
        "cmd.http.help_body".into(),
        "  -d, --data <body>              ส่วนเนื้อหาคำขอ (request body)".into(),
    );

    m.insert(
        "cmd.http.help_example1".into(),
        "  http GET https://api.github.com/users/octocat".into(),
    );

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert(
        "cmd.http.help_example3".into(),
        "  http GET https://api.example.com/search -q q=hello -q page=1 -s > results.json".into(),
    );

    m.insert(
        "cmd.http.help_example4".into(),
        "  http GET https://api.example.com/data --bearer mytoken -o response.json".into(),
    );

    m.insert(
        "cmd.http.help_example5".into(),
        "  http GET https://api.example.com/health -s -t 5".into(),
    );

    m.insert(
        "cmd.http.help_example6".into(),
        "  http GET https://api.example.com/users -a admin:secret123".into(),
    );

    m.insert("cmd.http.help_examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.http.help_header".into(),
        "  -H, --header <\\\"Key: Value\\\">     เพิ่มส่วนหัวของคำขอ (ทำซ้ำได้)".into(),
    );

    m.insert(
        "cmd.http.help_help".into(),
        "  -h, --help                     แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "cmd.http.help_json".into(),
        "  --json <ข้อมูล>                  ส่งเป็น JSON (ตั้งค่า Content-Type อัตโนมัติ)".into(),
    );

    m.insert(
        "cmd.http.help_methods".into(),
        "  GET, POST, PUT, DELETE, PATCH, HEAD".into(),
    );

    m.insert("cmd.http.help_methods_title".into(), "เมธอด:".into());

    m.insert(
        "cmd.http.help_no_pretty".into(),
        "  --no-pretty                    ปิดการจัดรูปแบบ JSON ให้สวยงาม".into(),
    );

    m.insert("cmd.http.help_options_title".into(), "ตัวเลือก:".into());

    m.insert(
        "cmd.http.help_output".into(),
        "  -o, --output <ไฟล์>            บันทึกเนื้อหาไปยังไฟล์ (แบบ raw)".into(),
    );

    m.insert(
        "cmd.http.help_query".into(),
        "  -q, --query <key=value>        พารามิเตอร์ query ของ URL (ทำซ้ำได้)".into(),
    );

    m.insert(
        "cmd.http.help_silent".into(),
        "  -s, --silent                   แสดงเฉพาะเนื้อหา (ไม่มีสถานะ/ส่วนหัว)".into(),
    );

    m.insert(
        "cmd.http.help_timeout".into(),
        "  -t, --timeout <วินาที>           หมดเวลาคำขอ (ค่าเริ่มต้น 30)".into(),
    );

    m.insert("cmd.http.help_title".into(), "HTTP API Client".into());

    m.insert(
        "cmd.http.help_usage_default".into(),
        "  <url>                          (GET โดยค่าเริ่มต้นหาก URL เป็นอาร์กิวเมนต์แรก)".into(),
    );

    m.insert(
        "cmd.http.help_usage_line".into(),
        "  http <method> <url> [options]".into(),
    );

    m.insert("cmd.http.help_usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.http.help_verbose".into(),
        "  -v, --verbose                  แสดงรายละเอียดคำขอที่ส่งออก".into(),
    );

    m.insert(
        "cmd.http.invalid_body".into(),
        "เนื้อหาคำขอไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_header".into(),
        "รูปแบบส่วนหัวไม่ถูกต้อง คาดว่า \"Key: Value\" แต่ได้: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_query".into(),
        "รูปแบบ query ไม่ถูกต้อง คาดว่า key=value แต่ได้: {}".into(),
    );

    m.insert("cmd.http.invalid_request".into(), "คำขอไม่ถูกต้อง: {}".into());

    m.insert(
        "cmd.http.invalid_timeout".into(),
        "หมดเวลาไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "cmd.http.missing_auth".into(),
        "ไม่พบ user:pass หลัง -a".into(),
    );

    m.insert(
        "cmd.http.missing_bearer".into(),
        "ไม่พบโทเคนหลัง --bearer".into(),
    );

    m.insert(
        "cmd.http.missing_body_data".into(),
        "ไม่พบข้อมูลเนื้อหาหลัง -d".into(),
    );

    m.insert("cmd.http.missing_filename".into(), "ไม่พบชื่อไฟล์หลัง -o".into());

    m.insert("cmd.http.missing_header".into(), "ไม่พบค่าส่วนหัวหลัง -H".into());

    m.insert(
        "cmd.http.missing_json".into(),
        "ไม่พบข้อมูล JSON หลัง --json".into(),
    );

    m.insert(
        "cmd.http.missing_query".into(),
        "ไม่พบ key=value หลัง -q".into(),
    );

    m.insert(
        "cmd.http.missing_timeout".into(),
        "ไม่พบจำนวนวินาทีหลัง -t".into(),
    );

    m.insert("cmd.http.request_failed".into(), "{} {} ล้มเหลว".into());

    m.insert("cmd.http.saved_bytes".into(), "บันทึก {} ไบต์ไปยัง {}".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}".into());

    m.insert("cmd.http.unknown_status".into(), "ไม่รู้จัก".into());

    m.insert(
        "cmd.http.url_required".into(),
        "ต้องระบุ URL. การใช้งาน: http <method> <url>".into(),
    );

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.write_fail".into(), "ไม่สามารถเขียนไปยัง {}".into());
}
