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
}
