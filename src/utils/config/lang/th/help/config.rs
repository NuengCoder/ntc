use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.config.autosuggest".into(),
        "config autosuggest ON|OFF ## แสดง/ตั้งค่าข้อความเลือน".into(),
    );

    m.insert(
        "help.config.color".into(),
        "config color ON|OFF ## แสดง/ตั้งค่าสีผลลัพธ์".into(),
    );

    m.insert(
        "help.config.depth".into(),
        "config depth [n] ## แสดง/ตั้งค่าความลึกสูงสุด (1-20)".into(),
    );

    m.insert(
        "help.config.history".into(),
        "config history [on|off|path] ## แสดง/ตั้งค่าประวัติ".into(),
    );

    m.insert(
        "help.config.lang".into(),
        "config lang [language] ## แสดง/ตั้งค่าภาษาอินเทอร์เฟซ".into(),
    );

    m.insert(
        "help.config.linenumbers".into(),
        "config linenumbers ON|OFF ## แสดง/ตั้งค่าหมายเลขบรรทัด".into(),
    );

    m.insert(
        "help.config.local_deinit".into(),
        "local deinit ## ลบ ntconfig.toml จากไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.config.local_help".into(),
        "local help ## แสดงการใช้งานคำสั่ง local".into(),
    );

    m.insert(
        "help.config.local_init".into(),
        "local init ## สร้างเทมเพลต ntconfig.toml (แบบ comment)".into(),
    );

    m.insert(
        "help.config.opencg".into(),
        "opencg ## เปิด config.toml ในตัวแก้ไข".into(),
    );

    m.insert(
        "help.config.output".into(),
        "config output [path] ## แสดง/ตั้งค่าเส้นทางผลลัพธ์".into(),
    );

    m.insert(
        "help.config.qsc".into(),
        "config qsc \"#c 0\" ## ตั้งค่าปุ่มลัด QSC".into(),
    );

    m.insert(
        "help.config.qsc_off".into(),
        "config qsc off ## ปิดการใช้งานปุ่มลัด QSC".into(),
    );

    m.insert(
        "help.config.resetcg".into(),
        "resetcg ## รีเซ็ตการกำหนดค่าเป็นค่าเริ่มต้น (มีตัวเลือกสำรอง)".into(),
    );

    m.insert(
        "help.config.restorecg".into(),
        "restorecg ## คืนค่าการกำหนดค่าจากข้อมูลสำรอง".into(),
    );

    m.insert(
        "help.config.showcg".into(),
        "showcg ## แสดงภาพรวมการกำหนดค่าปัจจุบัน".into(),
    );

    m.insert(
        "help.config.showenv".into(),
        "showenv ## แสดงตัวแปรสภาพแวดล้อม (PATH 5 รายการแรก)".into(),
    );

    m.insert(
        "help.config.showenv_all".into(),
        "showenv --all ## แสดง PATH ทั้งหมด".into(),
    );

    m.insert(
        "help.config.showenv_d".into(),
        "showenv -d <n> ## แสดง n รายการแรกของ PATH".into(),
    );

    m.insert(
        "help.config.threads".into(),
        "config threads [n] ## แสดง/ตั้งค่าจำนวนเธรด".into(),
    );

    m.insert(
        "help.config.title".into(),
        "คำสั่งการกำหนดค่า (CONFIGURATION):".into(),
    );

    m.insert(
        "help.config.watch".into(),
        "config watch ON|OFF ## เปิด/ปิดตัวเฝ้าดูไฟล์".into(),
    );

    m.insert(
        "help.config.watch_trigger".into(),
        "config watch trigger <a> ## เรียก alias อัตโนมัติเมื่อ watch ตรวจจับ".into(),
    );

    m.insert(
        "help.config.watch_trigger_off".into(),
        "config watch trigger off ## ปิดการทำงานอัตโนมัติ".into(),
    );

    m.insert(
        "help.config.watch_limit".into(),
        "config watch limit <n> ## จำนวนแถวที่แสดงในกล่อง watch".into(),
    );

    m.insert(
        "help.config.wi".into(),
        "wi [path] ## ฟีดการเปลี่ยนแปลงไฟล์แบบสด (q=ออก)".into(),
    );

    m.insert(
        "help.config.example1".into(),
        "config output ~/Desktop ## ตั้งเส้นทางผลลัพธ์เป็น Desktop".into(),
    );

    m.insert(
        "help.config.example2".into(),
        "config depth 3 ## ตั้งความลึกสูงสุดเป็น 3".into(),
    );

    m.insert(
        "help.config.example3".into(),
        "config color ON ## เปิดการแสดงสี".into(),
    );

    m.insert(
        "help.config.example4".into(),
        "config threads 8 ## ตั้งจำนวนเธรดเป็น 8".into(),
    );

    m.insert(
        "help.config.example5".into(),
        "showcg ## แสดงภาพรวมการกำหนดค่าปัจจุบัน".into(),
    );

    m.insert(
        "help.config.git".into(),
        "config git ON|OFF ## เปิด/ปิด git integration (prompt + สัญลักษณ์)".into(),
    );

    m.insert(
        "help.config.dir_view".into(),
        "config dir_view ON|OFF ## แสดง/ซ่อนแผนผังไดเรกทอรีใน shell".into(),
    );

    m.insert(
        "help.config.file_view".into(),
        "config file_view ON|OFF ## แสดงไฟล์ในแผนผังนำทางอัตโนมัติ (alias: fv)".into(),
    );

    m.insert(
        "help.config.size".into(),
        "config size all|file|dir ON|OFF ## คำนวณขนาดอัตโนมัติในแผนผังนำทาง; ใช้ --care (alias: s)"
            .into(),
    );

    m.insert(
        "help.config.update_check".into(),
        "config update_check ON|OFF ## แสดง/ซ่อนแจ้งเตือนเวอร์ชันใหม่รายวัน (alias: uc)".into(),
    );

    m.insert("help.config.serve".into(), "config serve [บริการ] [--port-default พอร์ต] [--bind-default ที่อยู่] ## ค่าเริ่มต้น serve รายบริการ (alias: serve config)".into());

    m.insert(
        "help.config.intro_view".into(),
        "config intro_view ON|OFF ## แสดง/ซ่อนโลโก้ตอนเริ่ม, ข้อมูล ntpkg และ watcher (alias: iv)".into(),
    );

    m.insert(
        "help.config.todo".into(),
        "config todo format <txt|md|json|toml|yaml|html> ## ตั้งรูปแบบไฟล์ todo".into(),
    );
}
