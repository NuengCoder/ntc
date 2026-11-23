use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.qur.title".into(), "QUR คำสั่ง:".into());

    m.insert(
        "help.qur.usage".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] ## ค้นหาไฟล์ เลือกเนื้อหา แทนที่".into(),
    );

    m.insert(
        "help.qur.example1".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + แสดงผลแบบ grep".into(),
    );
}
