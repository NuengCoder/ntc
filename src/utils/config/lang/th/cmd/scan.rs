use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.scan.depth_usage".into(),
        "การใช้งาน: scan ext <นามสกุล...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.scan.ext_usage".into(),
        "การใช้งาน: scan ext <นามสกุล...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.help_ext".into(), "  scan ext <นามสกุล...> [-d|--depth <n>] [-s|--size] [--care]  แสดงไฟล์ตามนามสกุล (ไม่นับบรรทัด)".into());

    m.insert(
        "cmd.scan.help_files".into(),
        "  scan files [แบบ] [-d|--depth <n>] [-s|--size] [--care]  แสดงไฟล์ตามชื่อ (* = ทั้งหมด)".into(),
    );

    m.insert(
        "cmd.scan.help_dir".into(),
        "  scan dir [แบบ] [-d|--depth <n>] [-s|--size] [--care]  แสดงไดเรกทอรีตามชื่อ (* = ทั้งหมด)"
            .into(),
    );

    m.insert("cmd.scan.help_port".into(), "  scan port [host] [--top N | --ports <spec> | --all] [--timeout MS]  สแกนพอร์ต TCP (ค่าเริ่มต้น localhost, 20 พอร์ตแรก)".into());

    m.insert("cmd.scan.help_ip".into(), "  scan ip [cidr] [--top N | --ports <spec>] [--timeout MS]  กวาดหาโฮสต์ที่ alive (ค่าเริ่มต้น /24 อัตโนมัติ)".into());

    m.insert("cmd.scan.ip_arp_only".into(), "(arp)".into());

    m.insert("cmd.scan.ip_done".into(), "alive: {} จาก {} ที่ sweep".into());

    m.insert("cmd.scan.ip_none".into(), "ไม่พบโฮสต์ที่ alive ใน {}".into());

    m.insert(
        "cmd.scan.ip_start".into(),
        "กำลัง sweep {} ({} ports/host)...".into(),
    );

    m.insert("cmd.scan.ip_title".into(), "โฮสต์ที่ alive ใน {}:".into());

    m.insert(
        "cmd.scan.missing_ports".into(),
        "ไม่พบค่าหลัง '--ports'.".into(),
    );

    m.insert(
        "cmd.scan.missing_timeout".into(),
        "ไม่พบค่าหลัง '--timeout'.".into(),
    );

    m.insert("cmd.scan.missing_top".into(), "ไม่พบค่าหลัง '--top'.".into());

    m.insert("cmd.scan.invalid_depth".into(), "ความลึกไม่ถูกต้อง: {}".into());

    m.insert(
        "cmd.scan.invalid_timeout".into(),
        "ค่า --timeout ไม่ถูกต้อง: '{}'".into(),
    );

    m.insert(
        "cmd.scan.invalid_top".into(),
        "ค่า --top ไม่ถูกต้อง: '{}'".into(),
    );

    m.insert("cmd.scan.list_title".into(), "ผลสแกน: {}".into());

    m.insert("cmd.scan.no_files".into(), "ไม่พบไฟล์ {}".into());

    m.insert("cmd.scan.port_done".into(), "เปิด: {} จาก {} ที่สแกน".into());

    m.insert(
        "cmd.scan.files_usage".into(),
        "การใช้งาน: scan files [แบบ] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert(
        "cmd.scan.dir_usage".into(),
        "การใช้งาน: scan dir [แบบ] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.no_dirs".into(), "ไม่พบไดเรกทอรี {}".into());

    m.insert("cmd.scan.total_dirs".into(), "ทั้งหมด: {} ไดเรกทอรี".into());

    m.insert(
        "cmd.scan.total_dirs_size".into(),
        "ทั้งหมด: {} ไดเรกทอรี ({})".into(),
    );

    m.insert("cmd.scan.port_none".into(), "ไม่มีพอร์ตเปิดบน {}".into());

    m.insert(
        "cmd.scan.port_start".into(),
        "กำลังสแกน {} ({} ports)...".into(),
    );

    m.insert("cmd.scan.port_title".into(), "พอร์ตที่เปิดบน {}:".into());

    m.insert(
        "cmd.scan.progress".into(),
        "กำลังสแกน {} … {}/{} ({}%) · เปิด {}".into(),
    );

    m.insert("cmd.scan.total_files".into(), "ทั้งหมด: {} ไฟล์".into());

    m.insert(
        "cmd.scan.total_files_size".into(),
        "ทั้งหมด: {} ไฟล์ ({})".into(),
    );

    m.insert(
        "cmd.scan.unknown_option".into(),
        "ตัวเลือก scan ไม่รู้จัก: {}".into(),
    );

    m.insert(
        "cmd.scan.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อยของ scan: '{}'".into(),
    );

    m.insert(
        "cmd.scan.unexpected_arg".into(),
        "อาร์กิวเมนต์ไม่คาดคิด: '{}'".into(),
    );

    m.insert("cmd.scan.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.scan.help_waep".into(),
        "  scan waep <โฮสต์|URL>   ค้นหาเส้นทางบนเว็บ (endpoint discovery)".into(),
    );

    m.insert(
        "cmd.scan.waep_usage".into(),
        "วิธีใช้: scan waep <โฮสต์|โฮสต์:พอร์ต|URL[/base]> [ตัวเลือก] (ดู: scan help)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_target".into(),
        "เป้าหมายไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_method".into(),
        "เมธอดไม่ถูกต้อง: {} (ใช้ GET หรือ HEAD)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_status".into(),
        "รายการสถานะไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_depth".into(),
        "ความลึกไม่ถูกต้อง: {} (1-4)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_concurrency".into(),
        "จำนวนพร้อมกันไม่ถูกต้อง: {} (1-64)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_timeout".into(),
        "เวลาหมดไม่ถูกต้อง: {} (>=50 ms)".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_header".into(),
        "เฮดเดอร์ไม่ถูกต้อง: {} (รูปแบบ 'Name: value')".into(),
    );

    m.insert(
        "cmd.scan.waep_start".into(),
        "กำลังตรวจสอบ {} ด้วย {} เส้นทาง...".into(),
    );

    m.insert(
        "cmd.scan.waep_none".into(),
        "ไม่มีเส้นทางใดตอบสนองบน {}".into(),
    );

    m.insert("cmd.scan.waep_summary".into(), "พบ {} รายการ".into());

    m.insert("cmd.scan.waep_elapsed".into(), "ใช้เวลา: {} ms".into());

    m.insert("cmd.scan.waep_saved".into(), "บันทึกแล้ว: {}".into());

    m.insert(
        "cmd.scan.waep_save_failed".into(),
        "บันทึก {} ไม่ได้: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_wordlist_failed".into(),
        "อ่าน wordlist {} ไม่ได้: {}".into(),
    );

    m.insert(
        "cmd.scan.waep_empty_wordlist".into(),
        "wordlist {} ไม่มีรายการที่ใช้ได้".into(),
    );

    m.insert("cmd.scan.waep_flags_title".into(), "แฟล็ก:".into());

    m.insert(
        "cmd.scan.waep_flags".into(),
        "-w, --wordlist <ไฟล์>      ใช้ wordlist ของตัวเอง (หนึ่งพาธต่อบรรทัด)\n  -x, --ext <นามสกุล,...>    ทดสอบ word.ext สำหรับแต่ละนามสกุลด้วย\n  -m, --method GET|HEAD      HTTP method (ค่าเริ่มต้น GET)\n  -s, --status <รหัส,...>    รหัสสถานะที่ถือว่าพบ คั่นด้วยจุลภาค\n      --all                  แสดงทุกสถานะ รวม 404\n  -r, --recursive            ไล่ลงไปในพาธที่พบ\n  -d, --depth <1-4>          ความลึกของการไล่ (ค่าเริ่มต้น 1)\n  -c, --concurrency <1-64>   จำนวน probe พร้อมกัน (ค่าเริ่มต้น 16)\n      --timeout <ms>         เวลารอต่อ probe (ค่าเริ่มต้น 4000)\n  -H, --header ชื่อ:ค่า      เพิ่ม header (ใช้ซ้ำได้)\n  -A, --user-agent <ua>      User-Agent header\n  -o, --output <ไฟล์>        บันทึกผลลงไฟล์\n  -q, --quiet                แสดงเฉพาะสรุป\n  -h, --help                 แสดงความช่วยเหลือนี้".into(),
    );

    m.insert("cmd.scan.waep_options_title".into(), "ตัวเลือก:".into());

    m.insert(
        "cmd.scan.waep_options".into(),
        "target        host, host:port, url หรือ url/base (scheme เริ่มต้น http)\n  paths         แสดงเทียบกับ base: .../api จะแสดง /files ไม่ใช่ /api/files\n  สถานะ 405    ถือว่าพบ (พาธมีอยู่ แต่ method นี้ไม่รองรับ)\n  recursion     ไล่ลงเฉพาะพาธที่ดูเป็นโฟลเดอร์ (ไม่มีจุด)".into(),
    );

    m.insert(
        "cmd.scan.waep_unreachable".into(),
        "ไม่มีการตอบกลับจาก {} - ตรวจสอบโฮสต์ พอร์ต และ scheme (DNS/การเชื่อมต่อ/TLS ล้มเหลว)".into(),
    );

    m.insert(
        "cmd.scan.waep_some_failed".into(),
        "{} จาก {} probe ไม่มีการตอบกลับ (หมดเวลาหรือถูกปฏิเสธการเชื่อมต่อ)".into(),
    );
}
