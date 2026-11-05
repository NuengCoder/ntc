use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.network.dl".into(),
        "dl <url> ## ดาวน์โหลดไฟล์ (--resume, --stop, --history, --cancel)".into(),
    );

    m.insert(
        "help.network.dns".into(),
        "dns <host> ## DNS lookup (-t A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL)".into(),
    );

    m.insert(
        "help.network.http".into(),
        "http <method> <url> ## HTTP API client (-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t)"
            .into(),
    );

    m.insert(
        "help.network.ping".into(),
        "ping <host> ## ICMP ping (-c count)".into(),
    );

    m.insert(
        "help.network.ipcfg".into(),
        "ipcfg [ชื่อ] ## แสดงอะแดปเตอร์เครือข่ายแบบ ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.network.serve".into(),
        "serve files|http-server|config ## บริการ: file-drive, เซิร์ฟเวอร์ CWD, ค่าเริ่มต้นรายบริการ".into(),
    );

    m.insert(
        "help.network.smtp".into(),
        "smtp ## SMTP email client (--id, -i, -u, -pwd, -s, -b, -f, --to)".into(),
    );

    m.insert(
        "help.network.spec".into(),
        "spec ## แสดงสเปกคอมพิวเตอร์/ระบบ (CPU, GPU, RAM, พื้นที่เก็บ, OS, เลขซีเรียล)".into(),
    );

    m.insert(
        "help.network.sys".into(),
        "sys, top ## ตัวตรวจสอบระบบแบบสด (CPU, หน่วยความจำ, ดิสก์, โพรเซส)".into(),
    );

    m.insert(
        "help.network.title".into(),
        "คำสั่งระบบและเครือข่าย (SYSTEM & NETWORK):".into(),
    );

    m.insert(
        "help.network.up".into(),
        "up <file> <url> ## อัปโหลดไฟล์ (multipart POST เป็นค่าเริ่มต้น; --put, --raw, -a, --bearer, -F, -H, -d)".into(),
    );

    m.insert(
        "help.network.example1".into(),
        "sys ## แสดงตัวตรวจสอบระบบ".into(),
    );

    m.insert(
        "help.network.example2".into(),
        "http GET https://api.example.com ## คำขอ GET ง่าย ๆ".into(),
    );

    m.insert(
        "help.network.example3".into(),
        "dl https://example.com/file.zip ## ดาวน์โหลดพร้อมความคืบหน้า".into(),
    );

    m.insert(
        "help.network.example4".into(),
        "ping google.com ## ping google.com".into(),
    );

    m.insert(
        "help.network.example5".into(),
        "dns example.com -t MX ## ค้นหา DNS MX record".into(),
    );

    m.insert(
        "help.network.example6".into(),
        "serve files --port 8080 ## เริ่มเซิร์ฟเวอร์ file-drive บนพอร์ต 8080".into(),
    );
    m.insert(
        "help.network.trace".into(),
        "trace <โฮสต์> [-m N] [-w MS] ## ติดตามเส้นทางทีละฮอป (tracert/traceroute หรือ TTL)".into(),
    );

    m.insert(
        "help.network.ping_flags".into(),
        "ping <โฮสต์> [-c N] [--forever] [-w MS] [-s ไบต์] [-t TTL] [-4|-6] ## ping พร้อมตัวเลือก"
            .into(),
    );
}
