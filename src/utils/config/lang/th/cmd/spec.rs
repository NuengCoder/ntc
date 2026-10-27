use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.spec.other".into(), "อื่นๆ".into());

    m.insert("cmd.spec.title".into(), "สเปกระบบ".into());

    m.insert("cmd.spec.cpu".into(), "CPU".into());

    m.insert("cmd.spec.vendor".into(), "ผู้ผลิต".into());

    m.insert("cmd.spec.generation".into(), "เจนเนอเรชัน".into());

    m.insert("cmd.spec.model".into(), "รุ่น".into());

    m.insert("cmd.spec.cores".into(), "คอร์ / เธรด".into());

    m.insert("cmd.spec.gpu".into(), "การ์ดจอ".into());

    m.insert("cmd.spec.ram".into(), "แรม".into());

    m.insert("cmd.spec.storage".into(), "พื้นที่เก็บข้อมูล".into());

    m.insert("cmd.spec.total".into(), "รวม".into());

    m.insert("cmd.spec.os".into(), "ระบบปฏิบัติการ".into());

    m.insert("cmd.spec.manufacturer".into(), "ผู้ผลิต".into());

    m.insert("cmd.spec.computer_model".into(), "รุ่นเครื่อง".into());

    m.insert("cmd.spec.serial".into(), "เลขซีเรียล".into());

    m.insert("cmd.spec.hostname".into(), "ชื่อโฮสต์".into());

    m.insert("cmd.spec.kernel".into(), "เคอร์เนล".into());

    m.insert("cmd.spec.uptime".into(), "เวลาทำงาน".into());

    m.insert("cmd.spec.unknown".into(), "ไม่ทราบ".into());

    m.insert("cmd.spec.custom".into(), "ประกอบเอง".into());
}
