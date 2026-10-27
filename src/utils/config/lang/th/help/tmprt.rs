use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tmprt.title".into(), "คำสั่งอุณหภูมิและสภาพอากาศ:".into());

    m.insert(
        "help.tmprt.com".into(),
        "tmprt com [secs] ## ตรวจสอบอุณหภูมิฮาร์ดแวร์แบบสด (q=ออก s=แช่แข็ง c='oC/'oF)".into(),
    );

    m.insert(
        "help.tmprt.weather".into(),
        "tmprt weather [city] ## สภาพอากาศปัจจุบันผ่าน OpenWeather API".into(),
    );

    m.insert(
        "help.tmprt.key".into(),
        "tmprt key [API_KEY] ## บันทึก/แสดงคีย์ OpenWeather API".into(),
    );

    m.insert(
        "help.tmprt.city".into(),
        "tmprt city [name] ## บันทึก/แสดงเมืองเริ่มต้นสำหรับสภาพอากาศ".into(),
    );

    m.insert(
        "help.tmprt.sensors".into(),
        "tmprt sensors <status|install|uninstall|debug> ## จัดการแบ็กเอนด์เซ็นเซอร์ฮาร์ดแวร์ (LHM)".into(),
    );

    m.insert(
        "help.tmprt.example1".into(),
        "tmprt com 5 ## มุมมองแบบสดรีเฟรชทุก 5 วินาที".into(),
    );

    m.insert(
        "help.tmprt.example2".into(),
        "tmprt weather Bangkok ## สภาพอากาศสำหรับกรุงเทพฯ".into(),
    );

    m.insert(
        "help.tmprt.example3".into(),
        "tmprt key 1234567890abcdef ## บันทึกคีย์ API".into(),
    );

    m.insert(
        "help.tmprt.example4".into(),
        "tmprt city Bangkok ## ตั้งเมืองเริ่มต้น".into(),
    );
}
