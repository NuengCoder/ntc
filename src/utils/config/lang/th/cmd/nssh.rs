use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nssh.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.nssh.help_add".into(),
        "nssh add <ชื่อ> --host <โฮสต์> [--user ผู้ใช้] [--port พอร์ต] [--key คีย์] [--pass-env ตัวแปร]"
            .into(),
    );

    m.insert(
        "cmd.nssh.help_ssh".into(),
        "nssh ssh [ชื่อ] [--cmd \"คำสั่ง\"] [--trust] [--ask-pass] [--timeout วินาที]".into(),
    );

    m.insert(
        "cmd.nssh.help_cp".into(),
        "nssh cp <โลคอล> <ชื่อ>:<รีโมต> | <ชื่อ>:<รีโมต> <โลคอล>".into(),
    );

    m.insert(
        "cmd.nssh.help_scan".into(),
        "nssh scan <โฮสต์> [--ports พอร์ต | --top จำนวน] [--timeout มิลลิวินาที]".into(),
    );

    m.insert(
        "cmd.nssh.help_manage".into(),
        "nssh ls | nssh info <ชื่อ> | nssh rm <ชื่อ> | nssh edit <ชื่อ> [ตัวเลือก]".into(),
    );

    m.insert(
        "cmd.nssh.help_keygen".into(),
        "nssh keygen [อาร์กิวเมนต์...]   สร้างคีย์ผ่าน ssh-keygen".into(),
    );

    m.insert(
        "cmd.nssh.help_help".into(),
        "nssh help   แสดงความช่วยเหลือนี้ (เรียกเปล่าๆ จะให้เลือกโปรไฟล์)".into(),
    );

    m.insert("cmd.nssh.options_title".into(), "ตัวเลือก:".into());

    m.insert(
        "cmd.nssh.help_host".into(),
        "  --host <โฮสต์>         ชื่อหรือ IP เซิร์ฟเวอร์ (add ต้องระบุ)".into(),
    );

    m.insert(
        "cmd.nssh.help_user".into(),
        "  --user, -u <ผู้ใช้>     ผู้ใช้ล็อกอิน (ค่าเริ่มต้น: ผู้ใช้ปัจจุบัน)".into(),
    );

    m.insert(
        "cmd.nssh.help_port".into(),
        "  --port, -p <พอร์ต>     พอร์ต SSH (ค่าเริ่มต้น: 22)".into(),
    );

    m.insert(
        "cmd.nssh.help_key".into(),
        "  --key <พาธ>            ไฟล์คีย์ส่วนตัว (ค่าเริ่มต้น: ~/.ssh/id_ed25519, id_rsa)".into(),
    );

    m.insert(
        "cmd.nssh.help_pass_env".into(),
        "  --pass-env <ตัวแปร>    ชื่อ pass-env ใน deadlock (ไม่เก็บลงดิสก์); ไม่มีจะใช้ตัวแปรสภาพแวดล้อม"
            .into(),
    );

    m.insert(
        "cmd.nssh.help_cmd".into(),
        "  --cmd \"คำสั่ง\"          รันคำสั่งเดียวแทนเชลล์โต้ตอบ".into(),
    );

    m.insert(
        "cmd.nssh.help_trust".into(),
        "  --trust               ยอมรับ+บันทึกคีย์โฮสต์ที่ไม่รู้จัก (ค่าเริ่มต้น: ปฏิเสธ)".into(),
    );

    m.insert(
        "cmd.nssh.help_ask_pass".into(),
        "  --ask-pass            ถามรหัสผ่าน (ซ่อนการพิมพ์)".into(),
    );

    m.insert(
        "cmd.nssh.help_timeout".into(),
        "  --timeout <ตัวเลข>    หมดเวลาเชื่อมต่อวินาที (ssh/cp) หรือมิลลิวินาที (scan)".into(),
    );

    m.insert(
        "cmd.nssh.help_top".into(),
        "  --top <จำนวน>          สแกนพอร์ตยอดนิยม N อันดับแรก (ค่าเริ่มต้น: 20)".into(),
    );

    m.insert(
        "cmd.nssh.help_scan_ports".into(),
        "  --ports <สเปก>        เช่น 22,80,8000-8010 (ช่วงละไม่เกิน 2000)".into(),
    );

    m.insert(
        "cmd.nssh.help_yes".into(),
        "  --yes, -y             ข้ามการยืนยันลบ".into(),
    );

    m.insert("cmd.nssh.examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.nssh.example_add".into(),
        "nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa".into(),
    );

    m.insert(
        "cmd.nssh.example_ssh".into(),
        "nssh ssh prod --cmd \"uptime\"".into(),
    );

    m.insert(
        "cmd.nssh.example_cp".into(),
        "nssh cp ./app.tar.gz prod:/tmp/".into(),
    );

    m.insert(
        "cmd.nssh.example_scan".into(),
        "nssh scan example.com --top 20".into(),
    );

    m.insert("cmd.nssh.missing_host".into(), "ขาดค่าหลัง --host".into());

    m.insert("cmd.nssh.missing_user".into(), "ขาดค่าหลัง --user".into());

    m.insert("cmd.nssh.missing_port".into(), "ขาดค่าหลัง --port".into());

    m.insert("cmd.nssh.missing_key".into(), "ขาดค่าหลัง --key".into());

    m.insert(
        "cmd.nssh.missing_pass_env".into(),
        "ขาดค่าหลัง --pass-env".into(),
    );

    m.insert("cmd.nssh.missing_cmd".into(), "ขาดค่าหลัง --cmd".into());

    m.insert(
        "cmd.nssh.missing_timeout".into(),
        "ขาดค่าหลัง --timeout".into(),
    );

    m.insert("cmd.nssh.missing_ports".into(), "ขาดค่าหลัง --ports".into());

    m.insert("cmd.nssh.missing_top".into(), "ขาดค่าหลัง --top".into());

    m.insert("cmd.nssh.invalid_port".into(), "พอร์ตไม่ถูกต้อง: '{}'".into());

    m.insert(
        "cmd.nssh.invalid_timeout".into(),
        "หมดเวลาไม่ถูกต้อง: '{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_top".into(),
        "ค่า --top ไม่ถูกต้อง: '{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_name".into(),
        "ชื่อโปรไฟล์ไม่ถูกต้อง '{}' (ตัวอักษร ตัวเลข _ - . เท่านั้น)".into(),
    );

    m.insert("cmd.nssh.unknown_option".into(), "ไม่รู้จัก: {}".into());

    m.insert("cmd.nssh.add_usage".into(), "วิธีใช้: nssh add <ชื่อ> --host <โฮสต์> [--user ผู้ใช้] [--port พอร์ต] [--key คีย์] [--pass-env ตัวแปร]".into());

    m.insert(
        "cmd.nssh.profile_exists".into(),
        "มีโปรไฟล์ '{}' แล้ว (ใช้ nssh edit เพื่อแก้)".into(),
    );

    m.insert("cmd.nssh.profile_saved".into(), "บันทึกโปรไฟล์แล้ว: {}".into());

    m.insert(
        "cmd.nssh.profile_not_found".into(),
        "ไม่พบโปรไฟล์ '{}'".into(),
    );

    m.insert(
        "cmd.nssh.key_not_found".into(),
        "ไม่มีไฟล์คีย์ '{}' (บันทึกอยู่ดี)".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_missing".into(),
        "ไม่พบ pass-env '{}' ใน deadlock vault และไม่ได้ตั้งในตัวแปรสภาพแวดล้อม (บันทึกอยู่ดี)".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_registered".into(),
        "pass-env '{}' ลงทะเบียนใน deadlock vault แล้ว".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_pending".into(),
        "pass-env '{}' จะถูกแก้จาก deadlock vault เมื่อเชื่อมต่อ".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_from_vault".into(),
        "กำลังใช้ pass-env '{}' จาก deadlock vault".into(),
    );

    m.insert(
        "cmd.nssh.master_unlock_failed".into(),
        "ไม่สามารถปลดล็อก deadlock vault ได้ ({}); จะถามรหัสผ่าน SSH แทน".into(),
    );

    m.insert(
        "cmd.nssh.ls_empty".into(),
        "ยังไม่มีโปรไฟล์ เพิ่มเลย: nssh add <ชื่อ> --host <โฮสต์>".into(),
    );

    m.insert("cmd.nssh.ls_title".into(), "โปรไฟล์ SSH:".into());

    m.insert("cmd.nssh.info_title".into(), "โปรไฟล์ '{}':".into());

    m.insert("cmd.nssh.info_usage".into(), "วิธีใช้: nssh info <ชื่อ>".into());

    m.insert("cmd.nssh.info_host".into(), "โฮสต์:".into());

    m.insert("cmd.nssh.info_user".into(), "ผู้ใช้:".into());

    m.insert("cmd.nssh.info_port".into(), "พอร์ต:".into());

    m.insert("cmd.nssh.info_key".into(), "คีย์:".into());

    m.insert("cmd.nssh.info_pass".into(), "ตัวแปรรหัสผ่าน:".into());

    m.insert(
        "cmd.nssh.rm_usage".into(),
        "วิธีใช้: nssh rm <ชื่อ> [--yes]".into(),
    );

    m.insert("cmd.nssh.rm_confirm".into(), "ลบโปรไฟล์ '{}'?".into());

    m.insert("cmd.nssh.rm_deleted".into(), "ลบโปรไฟล์แล้ว: {}".into());

    m.insert("cmd.nssh.rm_cancelled".into(), "ยกเลิกแล้ว".into());

    m.insert("cmd.nssh.edit_usage".into(), "วิธีใช้: nssh edit <ชื่อ> [--host โฮสต์] [--user ผู้ใช้] [--port พอร์ต] [--key คีย์|--no-key] [--pass-env ตัวแปร|--no-pass-env]".into());

    m.insert("cmd.nssh.edit_saved".into(), "อัปเดตโปรไฟล์แล้ว: {}".into());

    m.insert("cmd.nssh.pick_profile".into(), "เลือกโปรไฟล์ SSH".into());

    m.insert("cmd.nssh.ask_pass".into(), "รหัสผ่านของ {}@{}: ".into());

    m.insert(
        "cmd.nssh.pass_env_unset".into(),
        "ตัวแปร '{}' ว่างหรือไม่ได้ตั้ง — จะถามถ้าใช้ --ask-pass".into(),
    );

    m.insert(
        "cmd.nssh.ssh_connecting".into(),
        "กำลังเชื่อม {}@{}:{}...".into(),
    );

    m.insert("cmd.nssh.cmd_exit".into(), "คำสั่งรีโมตจบด้วยรหัส {}".into());

    m.insert(
        "cmd.nssh.cp_usage".into(),
        "วิธีใช้: nssh cp <โลคอล> <ชื่อ>:<รีโมต> | nssh cp <ชื่อ>:<รีโมต> <โลคอล>".into(),
    );

    m.insert(
        "cmd.nssh.cp_no_remote".into(),
        "ต้องมีฝั่งหนึ่งเป็น <โปรไฟล์>:<พาธ>".into(),
    );

    m.insert(
        "cmd.nssh.cp_two_sides".into(),
        "รีโมตทั้งสองฝั่ง — คัดทีละฝั่ง".into(),
    );

    m.insert(
        "cmd.nssh.cp_local_missing".into(),
        "ไม่พบไฟล์โลคอล '{}'".into(),
    );

    m.insert("cmd.nssh.cp_read_fail".into(), "อ่านไฟล์โลคอลไม่ได้: {}".into());

    m.insert(
        "cmd.nssh.cp_too_large".into(),
        "ไฟล์เกินขีดจำกัดถ่ายโอน 256MB".into(),
    );

    m.insert(
        "cmd.nssh.cp_uploading".into(),
        "กำลังอัปโหลด {} ไป {}:{}...".into(),
    );

    m.insert(
        "cmd.nssh.cp_downloading".into(),
        "กำลังดาวน์โหลด {}:{} ไป {}...".into(),
    );

    m.insert("cmd.nssh.cp_done".into(), "เสร็จ ({} ไบต์)".into());

    m.insert(
        "cmd.nssh.scan_usage".into(),
        "วิธีใช้: nssh scan <โฮสต์> [--ports สเปก | --top จำนวน] [--timeout มิลลิวินาที]".into(),
    );

    m.insert(
        "cmd.nssh.scan_start".into(),
        "กำลังสแกน {} ({} พอร์ต)...".into(),
    );

    m.insert("cmd.nssh.scan_title".into(), "พอร์ตที่เปิดบน {}:".into());

    m.insert("cmd.nssh.scan_none".into(), "ไม่มีพอร์ตเปิดบน {}".into());

    m.insert("cmd.nssh.scan_done".into(), "เปิด {} จากที่สำรวจ {}".into());

    m.insert(
        "cmd.nssh.keygen_missing".into(),
        "ไม่พบ ssh-keygen: {}".into(),
    );

    m.insert(
        "cmd.nssh.keygen_exit".into(),
        "ssh-keygen จบด้วยรหัส {}".into(),
    );

    m.insert(
        "cmd.nssh.shell_hint".into(),
        "เชลล์โต้ตอบ — Enter ~ . เพื่อตัดการเชื่อมต่อ".into(),
    );

    m.insert("cmd.nssh.shell_back".into(), "กลับสู่ ntc shell แล้ว".into());

    m.insert("cmd.nssh.shell_forced".into(), "ตัดการเชื่อมต่อแล้ว (~.)".into());

    m.insert(
        "cmd.nssh.password_prompt".into(),
        "รหัสผ่านสำหรับ {}@{}: ".into(),
    );

    m.insert(
        "cmd.nssh.password_incorrect".into(),
        "รหัสผ่านไม่ถูกต้อง。รหัสผ่านสำหรับ {}@{}: ".into(),
    );

    m.insert(
        "cmd.nssh.auth_failed".into(),
        "การตรวจสอบสิทธิ์ SSH ล้มเหลวสำหรับ {}@{}.".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_not_in_vault".into(),
        "ไม่พบ pass-env '{}' ใน deadlock vault - กำลังลองใช้ตัวแปรสภาพแวดล้อม".into(),
    );
}
