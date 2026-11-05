use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scan.depth".into(),
        "scan ext <นามสกุล...> -d, --depth <n> ## จำกัดความลึกการสแกน".into(),
    );

    m.insert(
        "help.scan.example1".into(),
        "scan ext rs ## แสดงไฟล์ .rs ทั้งหมด".into(),
    );

    m.insert(
        "help.scan.example2".into(),
        "scan ext rs c -d 2 ## แสดงไฟล์ .rs/.c ถึงความลึก 2".into(),
    );

    m.insert(
        "help.scan.example3".into(),
        "scan ext rs c -s ## แสดงพร้อมขนาดและผลรวม".into(),
    );

    m.insert(
        "help.scan.example4".into(),
        "scan port 192.168.1.1 --top 10 ## สแกน 10 พอร์ตแรก".into(),
    );

    m.insert(
        "help.scan.example5".into(),
        "scan ip 192.168.1.0/24 ## กวาดหาโฮสต์ที่ alive".into(),
    );

    m.insert(
        "help.scan.example6".into(),
        "scan files *.rs ## แสดงไฟล์ตามชื่อ".into(),
    );

    m.insert(
        "help.scan.example7".into(),
        "scan dir k* ## แสดงไดเรกทอรีตามชื่อ".into(),
    );

    m.insert(
        "help.scan.ext".into(),
        "scan ext <นามสกุล...> ## แสดงไฟล์ตามนามสกุล (ไม่นับบรรทัด)".into(),
    );

    m.insert(
        "help.scan.files".into(),
        "scan files [แบบ] ## แสดงไฟล์ตามชื่อ (* = ทั้งหมด)".into(),
    );

    m.insert(
        "help.scan.dir".into(),
        "scan dir [แบบ] ## แสดงไดเรกทอรีตามชื่อ (* = ทั้งหมด)".into(),
    );

    m.insert(
        "help.scan.port".into(),
        "scan port [host] [--top N | --ports <spec> | --all] ## สแกนพอร์ต TCP".into(),
    );

    m.insert(
        "help.scan.ip".into(),
        "scan ip [cidr] [--top N | --ports <spec>] ## กวาดหาโฮสต์ + ARP".into(),
    );

    m.insert(
        "help.scan.size".into(),
        "scan ext <นามสกุล...> -s, --size ## แสดงขนาดและผลรวม".into(),
    );

    m.insert(
        "help.scan.care".into(),
        "scan ext <นามสกุล...> --care ## รวมทุกไฟล์ (โหมด care)".into(),
    );

    m.insert("help.scan.title".into(), "คำสั่ง SCAN:".into());
    m.insert(
        "help.scan.waep".into(),
        "scan waep <โฮสต์|URL> ## ค้นหาเส้นทางบนเว็บ (/api, /admin, /upload, ...)".into(),
    );

    m.insert(
        "help.scan.waep_base".into(),
        "scan waep example.com/api -r -d 2 ## ค้นต่อใต้ base path".into(),
    );

    m.insert(
        "help.scan.waep_flags".into(),
        "scan waep <เป้าหมาย> [-w <ไฟล์>] [-x <นามสกุล,...>] [-m GET|HEAD] [-s <รหัส>] [--all] ## wordlist, นามสกุล, method, สถานะที่รับ".into(),
    );

    m.insert(
        "help.scan.waep_recursive".into(),
        "scan waep <เป้าหมาย> [-r] [-d 1-4] [-c 1-64] [--timeout <ms>] [-H 'ชื่อ: ค่า'] [-A <ua>] [-o <ไฟล์>] [-q] ## การไล่, ความเร็ว, header, เอาต์พุต".into(),
    );

    m.insert(
        "help.scan.example8".into(),
        "scan waep http://127.0.0.1:55555/api ## ดูว่า API ของ ntc serve เปิดอะไรไว้".into(),
    );

    m.insert(
        "help.scan.example9".into(),
        "scan waep example.com -x php,html --all ## แสดงทุกการตอบกลับ รวม .php/.html".into(),
    );

    m.insert(
        "help.scan.example10".into(),
        "scan waep example.com/api -r -d 2 -c 32 ## ไล่สองระดับด้วย 32 probe พร้อมกัน".into(),
    );
}
