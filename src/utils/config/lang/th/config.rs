use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "config.general.autosuggest".into(),
        "  config autosuggest ON|OFF แสดง/ตั้งค่าข้อความเลือน".into(),
    );

    m.insert(
        "config.general.color".into(),
        "  config color ON|OFF แสดง/ตั้งค่าสี".into(),
    );

    m.insert(
        "config.general.depth".into(),
        "  config depth [n] แสดง/ตั้งค่าความลึกสูงสุด (1-20)".into(),
    );

    m.insert("config.general.examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "config.general.history".into(),
        "  config history [on|off|path] แสดง/ตั้งค่าประวัติ".into(),
    );

    m.insert(
        "config.general.lang".into(),
        "  config lang [language] แสดง/ตั้งค่าภาษาอินเทอร์เฟซ".into(),
    );

    m.insert(
        "config.general.linenumbers".into(),
        "  config linenumbers ON|OFF แสดง/ตั้งค่าหมายเลขบรรทัด".into(),
    );

    m.insert(
        "config.general.output".into(),
        "  config output [path] แสดง/ตั้งค่าเส้นทางผลลัพธ์".into(),
    );

    m.insert(
        "config.general.qsc".into(),
        "  config qsc \"#c 0\" แสดง/ตั้งค่าปุ่มลัด QSC".into(),
    );

    m.insert("config.general.settings".into(), "การตั้งค่า:".into());

    m.insert(
        "config.general.threads".into(),
        "  config threads [n] แสดง/ตั้งค่าจำนวนเธรด".into(),
    );

    m.insert(
        "config.general.title".into(),
        "config \u{2014} ดูหรือเปลี่ยนการตั้งค่าการกำหนดค่า".into(),
    );

    m.insert(
        "config.general.watch".into(),
        "  config watch ON|OFF แสดง/ตั้งค่าตัวเฝ้าดูไฟล์".into(),
    );

    m.insert("config.lang.set".into(), "ตั้งภาษาเป็น: {}".into());
}
