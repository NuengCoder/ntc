use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ntpkg.available".into(),
        "ntpkg available [lang] ## แสดงรายการภาษา หรือเวอร์ชันที่ดาวน์โหลดได้".into(),
    );

    m.insert(
        "help.ntpkg.help".into(),
        "ntpkg help ## แสดงการใช้งาน ntpkg".into(),
    );

    m.insert(
        "help.ntpkg.info".into(),
        "ntpkg info <lang> ## แสดงข้อมูลแพ็กเกจ (เวอร์ชัน, ตำแหน่ง, ไบนารี)".into(),
    );

    m.insert(
        "help.ntpkg.install".into(),
        "ntpkg install <lang> [ver] ## ดาวน์โหลดและติดตั้ง runtime".into(),
    );

    m.insert(
        "help.ntpkg.list".into(),
        "ntpkg list ## แสดง runtimes ที่ติดตั้งแล้ว".into(),
    );

    m.insert(
        "help.ntpkg.ntpx".into(),
        "ntpx <pkg> [bin] [args] ## เรียกไบนารีของแพ็กเกจที่ติดตั้ง".into(),
    );

    m.insert(
        "help.ntpkg.search".into(),
        "ntpkg search [keyword] ## ค้นหาแพ็กเกจที่มี".into(),
    );

    m.insert(
        "help.ntpkg.title".into(),
        "ตัวจัดการแพ็กเกจ (ntpkg) + ntpx:".into(),
    );

    m.insert(
        "help.ntpkg.uninstall".into(),
        "ntpkg uninstall <lang> [ver] ## ลบ runtime ที่ติดตั้ง".into(),
    );

    m.insert(
        "help.ntpkg.use".into(),
        "ntpkg use <lang> <ver> ## เปิดใช้งานเวอร์ชันเฉพาะ".into(),
    );

    m.insert(
        "help.ntpkg.which".into(),
        "ntpkg which <lang> ## แสดงเส้นทางไปยัง runtime ที่ใช้งานอยู่".into(),
    );

    m.insert(
        "help.ntpkg.example1".into(),
        "ntpkg install node 18 ## ติดตั้ง Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example2".into(),
        "ntpkg list ## แสดง runtimes ที่ติดตั้ง".into(),
    );

    m.insert(
        "help.ntpkg.example3".into(),
        "ntpkg use node 18 ## เปิดใช้งาน Node.js 18".into(),
    );

    m.insert(
        "help.ntpkg.example4".into(),
        "ntpkg which node ## แสดงเส้นทางไปยัง Node.js".into(),
    );

    m.insert(
        "help.ntpkg.example5".into(),
        "ntpkg search python ## ค้นหาแพ็กเกจ Python".into(),
    );

    m.insert(
        "help.ntpkg.example6".into(),
        "ntpkg info node ## แสดงข้อมูลแพ็กเกจ Node.js".into(),
    );

    m.insert(
        "help.ntpkg.example7".into(),
        "ntpkg available ## แสดงภาษาที่ดาวน์โหลดได้".into(),
    );

    m.insert(
        "help.ntpkg.example8".into(),
        "ntpx node npm init ## เรียก npm ผ่าน ntpx".into(),
    );

    m.insert(
        "help.ntpkg.example9".into(),
        "ntpkg help ## แสดงการใช้งาน ntpkg".into(),
    );
}
