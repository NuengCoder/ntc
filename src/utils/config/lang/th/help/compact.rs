use std::collections::HashMap;

/// คำอธิบายบรรทัดเดียวสำหรับ help แบบย่อ (`help --all` ยังแสดงแบบเต็ม)
pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "ภาพรวมคำสั่ง".into());
    m.insert(
        "help.compact.footer".into(),
        "พิมพ์ 'help <คำสั่ง>' เพื่อดูรายละเอียด, 'help --all' เพื่อดูทั้งหมด.".into(),
    );

    m.insert(
        "help.compact.nav".into(),
        "เปลี่ยนไดเรกทอรี ย้อนกลับ กระโดด".into(),
    );
    m.insert("help.compact.config".into(), "ดูหรือแก้ไขการตั้งค่า ntc".into());
    m.insert("help.compact.editor".into(), "เปิดตัวแก้ไขในตัว".into());
    m.insert("help.compact.theme".into(), "สลับ เพิ่ม ส่งออกธีม".into());
    m.insert(
        "help.compact.teleport".into(),
        "เส้นทางที่บันทึกไว้ (tp jump <name>)".into(),
    );
    m.insert(
        "help.compact.ran".into(),
        "เรียกงานจาก NTCRANFILE.toml".into(),
    );
    m.insert("help.compact.scr".into(), "เรียกไฟล์สคริปต์ที่บันทึกไว้".into());
    m.insert(
        "help.compact.alias".into(),
        "เรียก alias (ral add <name> ...)".into(),
    );
    m.insert("help.compact.view".into(), "ต้นไม้ไดเรกทอรีและไฟล์".into());
    m.insert("help.compact.size".into(), "ขนาดไดเรกทอรี".into());
    m.insert(
        "help.compact.report".into(),
        "รายงานโปรเจกต์ (report project)".into(),
    );
    m.insert("help.compact.backup".into(), "สำรองและกู้คืนโปรเจกต์".into());
    m.insert(
        "help.compact.search".into(),
        "ค้นหาข้อความ ไฟล์ และโฟลเดอร์".into(),
    );
    m.insert(
        "help.compact.igcare".into(),
        "กฎ ignore/care สำหรับการสแกน".into(),
    );
    m.insert(
        "help.compact.network".into(),
        "dl, up, http, serve, ping, dns, smtp".into(),
    );
    m.insert("help.compact.tmprt".into(), "อุณหภูมิและอากาศ".into());
    m.insert("help.compact.time".into(), "จับเวลาและ benchmark".into());
    m.insert(
        "help.compact.git".into(),
        "คำสั่ง Git ลัด (status, branch, log)".into(),
    );
    m.insert(
        "help.compact.clipboard".into(),
        "ประวัติคลิปบอร์ด (clip list)".into(),
    );
    m.insert("help.compact.qsc".into(), "ตัวฟังคำสั่งด่วน".into());
    m.insert("help.compact.todo".into(), "รายการงานของโปรเจกต์".into());
    m.insert("help.compact.arc".into(), "สร้างและแตกไฟล์บีบอัด".into());
    m.insert("help.compact.scan".into(), "สแกนนามสกุล พอร์ต IP".into());
    m.insert("help.compact.tl".into(), "นับบรรทัดในไฟล์".into());
    m.insert(
        "help.compact.ntpkg".into(),
        "ติดตั้งและจัดการเครื่องมือ (ntpx)".into(),
    );
    m.insert("help.compact.qr".into(), "สร้างและอ่าน QR code".into());
    m.insert(
        "help.compact.pathman".into(),
        "จัดการรายการ PATH (usrpath, syspath)".into(),
    );
    m.insert(
        "help.compact.autoup".into(),
        "จัดการโปรแกรมเริ่มต้นอัตโนมัติ".into(),
    );
    m.insert("help.compact.mode".into(), "โหมดผู้ดูแล / ปกติ".into());
    m.insert("help.compact.explorer".into(), "ตัวเรียกดูไฟล์ (nfe)".into());
    m.insert("help.compact.math".into(), "คำนวณนิพจน์คณิตศาสตร์".into());
    m.insert("help.compact.python".into(), "เปิดใช้งาน Python venv".into());
    m.insert(
        "help.compact.encdec".into(),
        "encode, decode, cvcode".into(),
    );
    m.insert("help.compact.fmt".into(), "จัดรูปแบบไฟล์".into());
    m.insert("help.compact.image".into(), "แสดงภาพในเทอร์มินัล".into());
    m.insert("help.compact.ndb".into(), "เรียกดูฐานข้อมูล SQLite".into());
    m.insert("help.compact.nsheet".into(), "ดูสเปรดชีต".into());
    m.insert(
        "help.compact.nssh".into(),
        "โปรไฟล์ SSH และการคัดลอกไฟล์".into(),
    );
    m.insert("help.compact.nms".into(), "เครื่องเล่นเพลงในเทอร์มินัล".into());
    m.insert("help.compact.fake".into(), "สร้างข้อมูลจำลอง".into());
    m.insert("help.compact.deadlock".into(), "ตู้รหัสผ่านเข้ารหัส".into());
    m.insert("help.compact.syscmd".into(), "เรียกคำสั่งระบบใดก็ได้".into());
    m.insert(
        "help.compact.pipe".into(),
        "ไปป์ การเปลี่ยนทาง และตัวกรอง".into(),
    );
    m.insert(
        "help.compact.fileops".into(),
        "mkf, mkd, rmf, rmd, mv, cp".into(),
    );
    m.insert(
        "help.compact.other".into(),
        "clear, version, say, ui, exit, help".into(),
    );
}
