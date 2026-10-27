use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.tmprt.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อยของ tmprt: {}".into(),
    );

    m.insert("cmd.tmprt.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.tmprt.usage_com".into(),
        "  tmprt com [secs]        ตรวจสอบอุณหภูมิฮาร์ดแวร์แบบสด".into(),
    );

    m.insert(
        "cmd.tmprt.usage_weather".into(),
        "  tmprt weather [city]    สภาพอากาศปัจจุบันผ่าน OpenWeather API".into(),
    );

    m.insert(
        "cmd.tmprt.usage_key".into(),
        "  tmprt key [API_KEY]     บันทึก/แสดงคีย์ OpenWeather API".into(),
    );

    m.insert(
        "cmd.tmprt.usage_city".into(),
        "  tmprt city [name]       บันทึก/แสดงเมืองสภาพอากาศเริ่มต้น".into(),
    );

    m.insert(
        "cmd.tmprt.usage_sensors".into(),
        "  tmprt sensors <sub>     จัดการแบ็กเอนด์เซ็นเซอร์ฮาร์ดแวร์".into(),
    );

    m.insert("cmd.tmprt.examples_title".into(), "ตัวอย่าง:".into());

    m.insert("cmd.tmprt.example1".into(), "tmprt com".into());

    m.insert("cmd.tmprt.example2".into(), "tmprt com 5".into());

    m.insert("cmd.tmprt.example3".into(), "tmprt weather Bangkok".into());

    m.insert(
        "cmd.tmprt.example4".into(),
        "tmprt key 1234567890abcdef".into(),
    );

    m.insert(
        "cmd.tmprt.com.no_sensors".into(),
        "ไม่พบเซ็นเซอร์อุณหภูมิ.".into(),
    );

    m.insert(
        "cmd.tmprt.com.no_sensors_hint".into(),
        "บน Windows อาจต้องใช้สิทธิ์ผู้ดูแลระบบ; บน Linux ต้องมีเซ็นเซอร์ hwmon/thermal.".into(),
    );

    m.insert("cmd.tmprt.com.footer".into(), " กด q เพื่อออก".into());

    m.insert(
        "cmd.tmprt.com.lhm_missing".into(),
        "ไม่พบแบ็กเอนด์อุณหภูมิ CPU (ACPI ไม่เปิดเผยเซ็นเซอร์บนเครื่องนี้).".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_prompt".into(),
        "ติดตั้ง LibreHardwareMonitorLib (ดาวน์โหลดอัตโนมัติ ~7 MB) สำหรับอุณหภูมิ CPU/GPU/บอร์ด/ไดรฟ์?".into(),
    );

    m.insert("cmd.tmprt.com.lhm_confirm".into(), "ติดตั้งตอนนี้? [Y/n]".into());

    m.insert(
        "cmd.tmprt.com.lhm_installing".into(),
        "กำลังดาวน์โหลดและติดตั้ง LibreHardwareMonitorLib...".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_installed".into(),
        "ติดตั้ง LibreHardwareMonitorLib แล้ว.".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_install_failed".into(),
        "การติดตั้ง LibreHardwareMonitorLib ล้มเหลว: {}".into(),
    );

    m.insert("cmd.tmprt.com.admin_hint".into(), "อุณหภูมิ CPU ต้องใช้ไดรเวอร์ PawnIO และโพรเซสที่ยกระดับสิทธิ์. เรียก 'mode admin' เพื่อเปิด ntc แบบยกระดับสิทธิ์.".into());

    m.insert(
        "cmd.tmprt.com.pawnio_hint".into(),
        "อุณหภูมิ CPU ต้องใช้ไดรเวอร์ PawnIO. เรียก 'tmprt sensors install' (ต้องใช้สิทธิ์ผู้ดูแลระบบ).".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key".into(),
        "ไม่พบคีย์ OpenWeather API.".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key_hint".into(),
        "ตั้งค่าด้วย: tmprt key <API_KEY>  (หรือ export OPENWEATHER_API_KEY)".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city".into(),
        "ไม่มีการระบุเมืองและไม่มีเมืองเริ่มต้นที่ตั้งค่าไว้.".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city_hint".into(),
        "การใช้งาน: tmprt weather <city>   หรือตั้งค่าเริ่มต้น: tmprt city <name>".into(),
    );

    m.insert(
        "cmd.tmprt.weather.request_failed".into(),
        "คำขอสภาพอากาศล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.read_failed".into(),
        "ไม่สามารถอ่านการตอบสนองสภาพอากาศ: {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.bad_key".into(),
        "OpenWeather ปฏิเสธคีย์ API (HTTP 401).".into(),
    );

    m.insert(
        "cmd.tmprt.weather.city_not_found".into(),
        "ไม่พบเมือง: {}  (HTTP 404)".into(),
    );

    m.insert(
        "cmd.tmprt.weather.http_error".into(),
        "OpenWeather error (HTTP {}): {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.parse_failed".into(),
        "ไม่สามารถแยกวิเคราะห์ข้อมูลสภาพอากาศ: {}".into(),
    );

    m.insert("cmd.tmprt.weather.title".into(), "สภาพอากาศ:".into());

    m.insert("cmd.tmprt.weather.condition".into(), "สภาพอากาศ".into());

    m.insert("cmd.tmprt.weather.temp".into(), "อุณหภูมิ".into());

    m.insert("cmd.tmprt.weather.feels".into(), "รู้สึกเหมือน".into());

    m.insert("cmd.tmprt.weather.range".into(), "ต่ำสุด / สูงสุด".into());

    m.insert("cmd.tmprt.weather.humidity".into(), "ความชื้น".into());

    m.insert("cmd.tmprt.weather.pressure".into(), "ความดัน".into());

    m.insert("cmd.tmprt.weather.wind".into(), "ลม".into());

    m.insert("cmd.tmprt.weather.visibility".into(), "ทัศนวิสัย".into());

    m.insert("cmd.tmprt.weather.clouds".into(), "เมฆ".into());

    m.insert("cmd.tmprt.weather.sunrise".into(), "พระอาทิตย์ขึ้น".into());

    m.insert("cmd.tmprt.weather.sunset".into(), "พระอาทิตย์ตก".into());

    m.insert("cmd.tmprt.weather.coords".into(), "พิกัด".into());

    m.insert(
        "cmd.tmprt.key.env_source".into(),
        "คีย์ OpenWeather API (จากตัวแปร OPENWEATHER_API_KEY): {}".into(),
    );

    m.insert(
        "cmd.tmprt.key.status".into(),
        "คีย์ OpenWeather API: {}".into(),
    );

    m.insert(
        "cmd.tmprt.key.not_set".into(),
        "ยังไม่ได้กำหนดคีย์ OpenWeather API.".into(),
    );

    m.insert(
        "cmd.tmprt.key.saved".into(),
        "บันทึกคีย์ OpenWeather API แล้ว: {}".into(),
    );

    m.insert(
        "cmd.tmprt.city.status".into(),
        "เมืองสภาพอากาศเริ่มต้น: {}".into(),
    );

    m.insert(
        "cmd.tmprt.city.not_set".into(),
        "ยังไม่ได้กำหนดเมืองสภาพอากาศเริ่มต้น.".into(),
    );

    m.insert(
        "cmd.tmprt.city.saved".into(),
        "บันทึกเมืองสภาพอากาศเริ่มต้นแล้ว: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_title".into(),
        "คำสั่งย่อยของ tmprt sensors:".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_status".into(),
        "  tmprt sensors status        แสดงแบ็กเอนด์เซ็นเซอร์ที่พร้อมใช้งาน".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_install".into(),
        "  tmprt sensors install       ดาวน์โหลด LibreHardwareMonitorLib (Windows)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_uninstall".into(),
        "  tmprt sensors uninstall     ลบไฟล์ LibreHardwareMonitorLib ที่ดาวน์โหลด".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_title".into(),
        "แบ็กเอนด์เซ็นเซอร์ฮาร์ดแวร์:".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_acpi".into(),
        "ACPI/hwmon (sysinfo)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_nvidia".into(),
        "NVIDIA (nvidia-smi)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_lhm".into(),
        "LibreHardwareMonitor".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_unavailable".into(),
        "ไม่พร้อมใช้งาน".into(),
    );

    m.insert("cmd.tmprt.sensors.src_installed".into(), "ติดตั้งแล้ว".into());

    m.insert(
        "cmd.tmprt.sensors.src_not_installed".into(),
        "ยังไม่ติดตั้ง".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_hint".into(),
        "เรียก 'tmprt sensors install' เพื่อเปิดใช้เซ็นเซอร์ที่ใช้ LibreHardwareMonitor (Windows).".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.installed".into(),
        "ติดตั้ง LibreHardwareMonitorLib แล้ว.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.install_failed".into(),
        "การติดตั้ง LibreHardwareMonitorLib ล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstalled".into(),
        "ลบไฟล์ LibreHardwareMonitorLib แล้ว.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstall_failed".into(),
        "ไม่สามารถลบไฟล์ LibreHardwareMonitorLib: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.not_installed".into(),
        "LibreHardwareMonitorLib ยังไม่ติดตั้ง.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.windows_only".into(),
        "LibreHardwareMonitor รองรับเฉพาะบน Windows.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.downloading".into(),
        "กำลังดาวน์โหลด LibreHardwareMonitorLib".into(),
    );

    m.insert("cmd.tmprt.sensors.elevation".into(), "ระดับสิทธิ์".into());

    m.insert("cmd.tmprt.sensors.admin_yes".into(), "ผู้ดูแลระบบ".into());

    m.insert("cmd.tmprt.sensors.admin_no".into(), "ผู้ใช้มาตรฐาน".into());

    m.insert("cmd.tmprt.sensors.need_admin".into(), "อุณหภูมิ CPU ผ่าน LHM ต้องใช้โพรเซสที่ยกระดับสิทธิ์ (ไดรเวอร์ PawnIO). เรียก 'mode admin' เพื่อเปิด ntc แบบยกระดับสิทธิ์.".into());

    m.insert(
        "cmd.tmprt.sensors.src_pawnio".into(),
        "ไดรเวอร์ PawnIO".into(),
    );

    m.insert("cmd.tmprt.sensors.pawnio_missing".into(), "ไดรเวอร์ PawnIO ยังไม่ติดตั้ง - อุณหภูมิ CPU จะแสดง '--'. เรียก 'tmprt sensors install' เพื่อติดตั้ง (ต้องใช้สิทธิ์ผู้ดูแลระบบ).".into());

    m.insert(
        "cmd.tmprt.sensors.pawnio_present".into(),
        "ไดรเวอร์ PawnIO ติดตั้งแล้ว.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_elevating".into(),
        "กำลังยกระดับสิทธิ์เพื่อติดตั้งไดรเวอร์ PawnIO (ยอมรับ UAC prompt)...".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_downloading".into(),
        "กำลังดาวน์โหลดไดรเวอร์ PawnIO".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_installed".into(),
        "ติดตั้งไดรเวอร์ PawnIO แล้ว.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_reboot".into(),
        "ติดตั้งไดรเวอร์ PawnIO แล้ว - รีสตาร์ทเครื่องเพื่อเปิดใช้งาน.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_failed".into(),
        "การติดตั้งไดรเวอร์ PawnIO ล้มเหลว: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_debug".into(),
        "  tmprt sensors debug         ทิ้งฮาร์ดแวร์และค่าเซ็นเซอร์ที่ตรวจจับได้ (การวินิจฉัย)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_title".into(),
        "การวินิจฉัยเซ็นเซอร์ (LibreHardwareMonitor):".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_none".into(),
        "LibreHardwareMonitor ยังไม่ติดตั้ง - เรียก 'tmprt sensors install' ก่อน.".into(),
    );
}
