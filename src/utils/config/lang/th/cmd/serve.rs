use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.serve.db_title".into(), "ฐานข้อมูล:".into());

    m.insert(
        "cmd.serve.example_default".into(),
        "  serve files            HTTP บนพอร์ต 8080".into(),
    );

    m.insert(
        "cmd.serve.example_port".into(),
        "  serve files 3000       HTTP บนพอร์ต 3000".into(),
    );

    m.insert(
        "cmd.serve.example_spa".into(),
        "  serve files --spa      SPA fallback สำหรับ React Router".into(),
    );

    m.insert(
        "cmd.serve.example_tunnel".into(),
        "  serve files --tunnel   HTTP + อุโมงค์ cloudflared".into(),
    );

    m.insert("cmd.serve.examples_title".into(), "ตัวอย่าง:".into());

    m.insert("cmd.serve.files_stored_title".into(), "ไฟล์ที่เก็บใน:".into());

    m.insert(
        "cmd.serve.help_bind".into(),
        "  --bind, -b <addr>     ที่อยู่ bind (ค่าเริ่มต้น: 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.help_help".into(),
        "  --help, -h            แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "cmd.serve.help_port".into(),
        "  --port, -p <port>     พอร์ตที่รับฟัง (ค่าเริ่มต้น: 8080)".into(),
    );

    m.insert(
        "cmd.serve.help_spa".into(),
        "  --spa                 โหมด SPA (fallback ไปที่ index.html)".into(),
    );

    m.insert("cmd.serve.help_title".into(), "HTTP File Server".into());

    m.insert(
        "cmd.serve.help_tunnel".into(),
        "  --tunnel, -t          เปิดเผยผ่านอุโมงค์ cloudflared".into(),
    );

    m.insert(
        "cmd.serve.help_share".into(),
        "  --share [path]        ตรวจเน็ต แล้วแสดง LAN URL + QR สำหรับมือถือ".into(),
    );

    m.insert(
        "cmd.serve.help_qr".into(),
        "  --qr                  บังคับแสดง QR (รวม --share)".into(),
    );

    m.insert(
        "cmd.serve.help_no_qr".into(),
        "  --no-qr               ไม่แสดง QR".into(),
    );

    m.insert(
        "cmd.serve.example_share".into(),
        "  serve files 8080 --share  ตรวจเน็ต แล้วพิมพ์ LAN URL + QR".into(),
    );

    m.insert(
        "cmd.serve.share_checking".into(),
        "กำลังตรวจสอบการเชื่อมต่ออินเทอร์เน็ต...".into(),
    );

    m.insert(
        "cmd.serve.share_online".into(),
        "อินเทอร์เน็ตปกติ — โหมดแชร์".into(),
    );

    m.insert(
        "cmd.serve.share_offline".into(),
        "ไม่มีอินเทอร์เน็ต — แสดงเฉพาะ localhost".into(),
    );

    m.insert("cmd.serve.share_lan_title".into(), "LAN:".into());

    m.insert("cmd.serve.share_path_title".into(), "แชร์:".into());

    m.insert(
        "cmd.serve.share_no_lan".into(),
        "ไม่พบที่อยู่ LAN — แสดงเฉพาะ localhost".into(),
    );

    m.insert(
        "cmd.serve.share_bound_loopback".into(),
        "bind ไว้ที่ loopback: อุปกรณ์ LAN เข้าไม่ถึง (ใช้ค่าเริ่มต้น 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.share_not_private".into(),
        "ที่อยู่ {} ไม่ใช่ช่วง LAN ส่วนตัว".into(),
    );

    m.insert(
        "cmd.serve.share_not_private_hint".into(),
        "การแชร์มือถือต้องใช้ 10.x.x.x, 172.16-31.x.x หรือ 192.168.x.x".into(),
    );

    m.insert(
        "cmd.serve.share_qr_fail".into(),
        "เรนเดอร์ QR ล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.serve.help_usage".into(),
        "serve files [port] [options]".into(),
    );

    m.insert(
        "cmd.serve.help_web_dir".into(),
        "  --web-dir, -w <dir>   เสิร์ฟไดเรกทอรี frontend ที่กำหนดเอง".into(),
    );

    m.insert(
        "cmd.serve.help_auth".into(),
        "  --auth <user:pass>     ต้องล็อกอินสำหรับ /api/* (เช่น --auth admin:secret)".into(),
    );

    m.insert(
        "cmd.serve.example_auth".into(),
        "  serve files 8080 --auth admin:secret   HTTP พอร์ต 8080, API ต้องล็อกอิน".into(),
    );

    m.insert(
        "cmd.serve.invalid_auth".into(),
        "--auth ไม่ถูกต้อง (ใช้ --auth <user:pass>)".into(),
    );

    m.insert("cmd.serve.missing_auth".into(), "ขาดค่าของ --auth".into());

    m.insert(
        "cmd.serve.auth_on".into(),
        "เปิดการยืนยันตัวตน API สำหรับผู้ใช้ '{}' แล้ว".into(),
    );

    m.insert(
        "cmd.serve.help_lock".into(),
        "  --lock                อ่านอย่างเดียว: ดาวน์โหลด/ดูได้ ห้ามอัปโหลด/แก้ไข/สร้าง/ลบ".into(),
    );

    m.insert(
        "cmd.serve.lock_on".into(),
        "โหมดอ่านอย่างเดียว (--lock): บล็อกอัปโหลด แก้ไข สร้าง และลบแล้ว".into(),
    );

    m.insert("cmd.serve.invalid_port".into(), "พอร์ตไม่ถูกต้อง".into());

    m.insert("cmd.serve.missing_bind".into(), "ไม่พบ bind".into());

    m.insert("cmd.serve.missing_dir".into(), "ไม่พบไดเรกทอรี".into());

    m.insert("cmd.serve.missing_port".into(), "ไม่พบพอร์ต".into());

    m.insert("cmd.serve.options_title".into(), "ตัวเลือก:".into());

    m.insert("cmd.serve.stopped".into(), "หยุดเซิร์ฟเวอร์แล้ว.".into());

    m.insert("cmd.serve.unknown_option".into(), "ไม่รู้จัก: {}".into());

    m.insert("cmd.serve.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.serve.cloudflared_starting".into(),
        "กำลังเริ่มอุโมงค์ cloudflared (PID {})".into(),
    );

    m.insert(
        "cmd.serve.cloudflared_missing".into(),
        "ไม่พบ cloudflared: {}. ติดตั้ง: {}".into(),
    );

    m.insert(
        "cmd.serve.chess_soon".into(),
        "`serve chess` กำลังมา (หลายผู้เล่นออนไลน์)".into(),
    );

    m.insert(
        "cmd.serve.services_title".into(),
        "Serve — เลือกบริการ".into(),
    );

    m.insert(
        "cmd.serve.services_usage".into(),
        "serve <บริการ> [ตัวเลือก] (`serve` เปล่าๆ ไม่รันอะไร)".into(),
    );

    m.insert("cmd.serve.services_header".into(), "บริการ:".into());

    m.insert(
        "cmd.serve.services_row_files".into(),
        "  files         เว็บ file-drive (อัปโหลด/ดาวน์โหลดในเบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.services_row_http".into(),
        "  http-server   เซิร์ฟเวอร์ไฟล์สถิตของไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "cmd.serve.services_row_chess".into(),
        "  chess         หมากรุก 2 คน hotseat ในเบราว์เซอร์".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        ดู/ตั้งค่าเริ่มต้นรายบริการ (พอร์ต, bind)".into(),
    );

    m.insert(
        "cmd.serve.services_example_files".into(),
        "  serve files --port 9000".into(),
    );

    m.insert(
        "cmd.serve.services_example_http".into(),
        "  serve http-server ./dist".into(),
    );

    m.insert(
        "cmd.serve.services_example_config".into(),
        "  serve config files --port-default 9000".into(),
    );

    m.insert(
        "cmd.serve.http_title".into(),
        "เซิร์ฟเวอร์ไฟล์สถิต HTTP (ไดเรกทอรีปัจจุบัน)".into(),
    );

    m.insert(
        "cmd.serve.http_usage".into(),
        "serve http-server [dir] [--port 8000] [options]".into(),
    );

    m.insert(
        "cmd.serve.example_http_default".into(),
        "  serve http-server            เสิร์ฟ CWD พอร์ต 8080".into(),
    );

    m.insert(
        "cmd.serve.example_http_dir".into(),
        "  serve http-server ./dist --port 8001   เสิร์ฟ ./dist พอร์ต 8001".into(),
    );

    m.insert(
        "cmd.serve.example_http_auth".into(),
        "  serve http-server --auth admin:secret  ต้องล็อกอิน".into(),
    );

    m.insert(
        "cmd.serve.help_auth_http".into(),
        "  --auth <user:pass>     ทุกเส้นทางต้องล็อกอิน (เช่น --auth admin:secret)".into(),
    );

    m.insert("cmd.serve.http_serving".into(), "กำลังเสิร์ฟ".into());

    m.insert("cmd.serve.http_no_dir".into(), "ไม่พบไดเรกทอรี '{}'".into());

    m.insert(
        "cmd.serve.current_defaults".into(),
        "ค่าเริ่มต้นที่ตั้งไว้ (serve config): พอร์ต {} bind {}".into(),
    );

    m.insert(
        "cmd.serve.chess_no_auth".into(),
        "`serve chess` ไม่รับ --auth (hotseat ไม่ต้องล็อกอิน)".into(),
    );

    m.insert("cmd.serve.chess_serving".into(), "เซิร์ฟเวอร์หมากรุกที่".into());

    m.insert(
        "cmd.serve.chess_title".into(),
        "หมากรุก (2 คน hotseat)".into(),
    );

    m.insert(
        "cmd.serve.chess_usage".into(),
        "serve chess [--port 8000] [options]".into(),
    );

    m.insert(
        "cmd.serve.example_chess_default".into(),
        "  serve chess              หมากรุกพอร์ต 8080".into(),
    );

    m.insert(
        "cmd.serve.example_chess_port".into(),
        "  serve chess --port 9000  หมากรุกพอร์ต 9000".into(),
    );
}
