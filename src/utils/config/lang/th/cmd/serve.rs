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
    m.insert(
        "cmd.serve.url_web".into(),
        "หน้าเว็บ:      {}  (เรียกดู + ลากไฟล์มาอัปโหลด)".into(),
    );

    m.insert(
        "cmd.serve.url_upload".into(),
        "อัปโหลด:      {}  (POST multipart, ฟิลด์ 'file')".into(),
    );

    m.insert(
        "cmd.serve.url_download".into(),
        "ดาวน์โหลด:    {}  (ต่อด้วยเส้นทางไฟล์)".into(),
    );

    m.insert(
        "cmd.serve.url_info".into(),
        "ข้อมูล:       {}  (คืออะไร + ตัวอย่าง curl/wget/ntc)".into(),
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
        "cmd.serve.services_row_git".into(),
        "  git           Git GUI: สถานะ, diff, stage, commit, branch, tag, stash, push/pull".into(),
    );

    m.insert(
        "cmd.serve.services_row_arc".into(),
        "  arc           Archive GUI: สร้าง, ดู/แตกไฟล์บีบอัด".into(),
    );

    m.insert(
        "cmd.serve.services_row_theme".into(),
        "  theme         Theme GUI: แก้ไข ดูตัวอย่าง บันทึก และใช้งานธีม".into(),
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

    m.insert(
        "cmd.serve.help_online".into(),
        "  --online              เปิดล็อบบี้และห้องออนไลน์ (ผู้เยี่ยมชม, จับคู่เร็ว, ห้องโค้ช)".into(),
    );

    m.insert(
        "cmd.serve.example_chess_online".into(),
        "  serve chess --online -b 0.0.0.0 --share  หมากรุกออนไลน์ผ่าน LAN พร้อม QR".into(),
    );

    m.insert(
        "cmd.serve.git_serving".into(),
        "เซิร์ฟเวอร์ Git ทำงานที่ {} ({})".into(),
    );

    m.insert(
        "cmd.serve.git_title".into(),
        "Git (อินเทอร์เฟซเบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.git_usage".into(),
        "serve git [ไดเรกทอรี] [--port 8000] [--bind ที่อยู่]".into(),
    );

    m.insert(
        "cmd.serve.git_help_port".into(),
        "  --port, -p <พอร์ต>    พอร์ตที่รับฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.git_help_bind".into(),
        "  --bind, -b <ที่อยู่>  ที่อยู่ที่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.git_example".into(),
        "  serve git --port 9000  ให้บริการคลังปัจจุบันที่พอร์ต 9000".into(),
    );

    m.insert("cmd.serve.git_no_repo".into(), "ไม่ใช่คลัง Git: {}".into());

    m.insert(
        "cmd.serve.git_no_git".into(),
        "ไม่พบ git หรือไม่อยู่ใน PATH".into(),
    );

    m.insert(
        "cmd.serve.git_local_warning".into(),
        "ผูกกับ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.arc_serving".into(),
        "เซิร์ฟเวอร์ไฟล์บีบอัดทำงานที่ {} ({})".into(),
    );

    m.insert(
        "cmd.serve.arc_title".into(),
        "ไฟล์บีบอัด (อินเทอร์เฟซเบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.arc_usage".into(),
        "serve arc [ไดเรกทอรี] [--port 8000] [--bind ที่อยู่]".into(),
    );

    m.insert(
        "cmd.serve.arc_help_port".into(),
        "  --port, -p <พอร์ต>    พอร์ตที่รับฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.arc_help_bind".into(),
        "  --bind, -b <ที่อยู่>  ที่อยู่ที่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.arc_example".into(),
        "  serve arc --port 9000  ให้บริการไดเรกทอรีปัจจุบันที่พอร์ต 9000".into(),
    );

    m.insert("cmd.serve.arc_no_dir".into(), "ไม่พบไดเรกทอรี '{}'".into());

    m.insert(
        "cmd.serve.arc_local_warning".into(),
        "ผูกกับ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.theme_title".into(),
        "ธีม (อินเทอร์เฟซเบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.theme_usage".into(),
        "serve theme [--port 8000] [--bind ที่อยู่]".into(),
    );

    m.insert(
        "cmd.serve.theme_help_port".into(),
        "  --port, -p <พอร์ต>    พอร์ตที่รับฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.theme_help_bind".into(),
        "  --bind, -b <ที่อยู่>  ที่อยู่ที่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.theme_example".into(),
        "  serve theme --port 9000  แก้ไขธีมที่พอร์ต 9000".into(),
    );

    m.insert(
        "cmd.serve.theme_serving".into(),
        "เซิร์ฟเวอร์ธีมที่ {} ({})".into(),
    );

    m.insert(
        "cmd.serve.theme_local_warning".into(),
        "ผูกกับ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.services_row_ndb".into(),
        "  ndb           เรียกดู SQLite: เลือกฐานข้อมูลในโฟลเดอร์, ดูโครงสร้าง, เลือกใช้ --write".into(),
    );

    m.insert("cmd.serve.ndb_title".into(), "เบราว์เซอร์ฐานข้อมูล".into());

    m.insert(
        "cmd.serve.ndb_usage".into(),
        "serve ndb [--port 8000] [--bind ที่อยู่] [--auth user:pass] [--write]".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_port".into(),
        "  --port, -p <พอร์ต>    พอร์ตที่รับฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_bind".into(),
        "  --bind, -b <ที่อยู่>  ที่อยู่ที่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_write".into(),
        "  --write              อนุญาต INSERT/UPDATE/DELETE/REPLACE; จะสร้างสำเนา <ชื่อ>.bak.<เวลา> ก่อนการเขียนครั้งแรก".into(),
    );

    m.insert(
        "cmd.serve.ndb_serving".into(),
        "ให้บริการฐานข้อมูลใน {} ที่ {}".into(),
    );

    m.insert(
        "cmd.serve.ndb_write_on".into(),
        "เปิดการเขียนแล้ว: ทีละหนึ่งคำสั่ง INSERT, UPDATE, DELETE หรือ REPLACE; สำเนา <ชื่อ>.bak.<เวลา> จะอยู่ข้างฐานข้อมูลก่อนการเขียนครั้งแรก".into(),
    );

    m.insert(
        "cmd.serve.ndb_readonly".into(),
        "อ่านเท่านั้น: อนุญาต SELECT, PRAGMA, EXPLAIN, WITH, VALUES และ TABLE; ใช้ --write เพื่อเปิดการเขียนระดับแถว".into(),
    );

    m.insert(
        "cmd.serve.ndb_local_warning".into(),
        "เฉพาะ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0 พร้อม --auth เพื่อแชร์)".into(),
    );

    m.insert(
        "cmd.serve.ndb_no_db".into(),
        "ไม่พบฐานข้อมูล SQLite ที่อ่านได้ใน {} - คัดลอกไฟล์ .db, .sqlite หรือ .sqlite3 ไปไว้ที่นั่น".into(),
    );

    m.insert(
        "cmd.serve.ndb_flags_only".into(),
        "'{}' ไม่ใช่ตัวเลือก: `serve ndb` รับเฉพาะแฟล็ก - โปรดเลือกฐานข้อมูลในหน้าเว็บ".into(),
    );

    m.insert(
        "cmd.serve.ndb_requires_auth".into(),
        "ปฏิเสธการเปิดฐานข้อมูลบน {} โดยไม่มี --auth <user:pass>".into(),
    );

    m.insert(
        "cmd.serve.ndb_example".into(),
        "  serve ndb --port 9000  แสดงฐานข้อมูลในโฟลเดอร์นี้ที่พอร์ต 9000".into(),
    );

    m.insert(
        "cmd.serve.services_row_fake".into(),
        "  fake          UI ข้อมูลจำลอง: เลือกชนิดหรือกลุ่ม แล้วดูตัวอย่างแถว".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        UI การตั้งค่า: แก้ทุกค่าและค่าเริ่มต้นของ serve ในเบราว์เซอร์".into(),
    );

    m.insert(
        "cmd.serve.services_example_fake".into(),
        "  serve fake --port 9000".into(),
    );

    m.insert(
        "cmd.serve.fake_title".into(),
        "ข้อมูลจำลอง (เบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.fake_usage".into(),
        "serve fake [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.fake_help_port".into(),
        "  --port, -p <port>     พอร์ตที่ฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.fake_help_bind".into(),
        "  --bind, -b <addr>     ที่อยู่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.fake_example".into(),
        "  serve fake --port 9000  เปิด UI ข้อมูลจำลองที่พอร์ต 9000".into(),
    );

    m.insert(
        "cmd.serve.fake_serving".into(),
        "เซิร์ฟเวอร์ข้อมูลจำลองที่ {}".into(),
    );

    m.insert(
        "cmd.serve.fake_local_warning".into(),
        "ผูกกับ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.config_title".into(),
        "การตั้งค่า NTC (เบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.config_usage".into(),
        "serve config [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.config_help_port".into(),
        "  --port, -p <port>     พอร์ตที่ฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.config_help_bind".into(),
        "  --bind, -b <addr>     ที่อยู่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.config_example".into(),
        "  serve config --port 9000  เปิด UI การตั้งค่าที่พอร์ต 9000".into(),
    );

    m.insert(
        "cmd.serve.config_serving".into(),
        "เซิร์ฟเวอร์การตั้งค่าที่ {}".into(),
    );

    m.insert(
        "cmd.serve.config_local_warning".into(),
        "ผูกกับ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.services_row_backup".into(),
        "  backup        UI สำรองข้อมูล: สร้าง, diff, กู้คืน, ย้อนกลับ, ล้างของโฟลเดอร์นี้".into(),
    );

    m.insert(
        "cmd.serve.services_example_backup".into(),
        "  serve backup --port 9000".into(),
    );

    m.insert(
        "cmd.serve.backup_title".into(),
        "สำรองข้อมูล (เบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.backup_usage".into(),
        "serve backup [path] [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.backup_help_port".into(),
        "  --port, -p <port>     พอร์ตที่ฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.backup_help_bind".into(),
        "  --bind, -b <addr>     ที่อยู่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.backup_example".into(),
        "  serve backup --port 9000  จัดการสำรองข้อมูลของโฟลเดอร์นี้ที่พอร์ต 9000".into(),
    );

    m.insert(
        "cmd.serve.backup_serving".into(),
        "เซิร์ฟเวอร์สำรองข้อมูลที่ {} ({})".into(),
    );

    m.insert(
        "cmd.serve.backup_local_warning".into(),
        "ผูกกับ loopback: อุปกรณ์ใน LAN เข้าถึงไม่ได้ (ใช้ --bind 0.0.0.0)".into(),
    );

    m.insert("cmd.serve.backup_no_dir".into(), "ไม่พบโฟลเดอร์: {}".into());

    m.insert(
        "cmd.serve.services_row_mdview".into(),
        "  mdview        โปรแกรมแก้ไข Markdown พร้อมตัวอย่างสด (เขียนไฟล์ .md)".into(),
    );

    m.insert(
        "cmd.serve.mdview_title".into(),
        "โปรแกรมแก้ไข Markdown (เบราว์เซอร์)".into(),
    );

    m.insert(
        "cmd.serve.mdview_usage".into(),
        "serve mdview [โฟลเดอร์] [--port 8000] [--bind ที่อยู่]".into(),
    );

    m.insert(
        "cmd.serve.mdview_help_port".into(),
        "  --port, -p <พอร์ต>     พอร์ตที่รับฟัง (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.mdview_help_bind".into(),
        "  --bind, -b <ที่อยู่>     ที่อยู่ที่ผูก (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "cmd.serve.mdview_example".into(),
        "  serve mdview docs --port 9000  แก้ไข markdown ใน docs/ ที่พอร์ต 9000".into(),
    );

    m.insert(
        "cmd.serve.mdview_serving".into(),
        "โปรแกรมแก้ไข Markdown สำหรับ {} ที่ {}".into(),
    );

    m.insert(
        "cmd.serve.mdview_no_dir".into(),
        "ไม่ใช่โฟลเดอร์: {}".into(),
    );

    m.insert(
        "cmd.serve.mdview_local_warning".into(),
        "ผูกกับ loopback: เฉพาะเครื่องนี้เท่านั้นที่แก้ไขไฟล์ได้ (ใช้ --bind 0.0.0.0 เพื่อแชร์)".into(),
    );
}
