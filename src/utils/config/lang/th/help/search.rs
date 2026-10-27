use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.search.ds".into(),
        "ds <pattern> [-d <n>] ## ค้นหาไดเรกทอรีตามชื่อ".into(),
    );

    m.insert(
        "help.search.fs".into(),
        "fs <pattern> [-d <n>] ## ค้นหาไฟล์ตามชื่อ (สำรองแบบ fuzzy)".into(),
    );

    m.insert(
        "help.search.fsc".into(),
        "fsc <pattern> [-d <n>] ## ค้นหาไฟล์ เลือกหนึ่งเพื่อแสดงและคัดลอกได้".into(),
    );

    m.insert("help.search.gs".into(), "gs <pattern> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-i] ## ค้นหาเนื้อหาไฟล์ (grep ง่าย ๆ)".into());

    m.insert(
        "help.search.gs_interactive".into(),
        "gs -i <pattern> [-d <n>] ## grep แบบโต้ตอบ (10 รายการ/หน้า Enter เพื่อเปิด)".into(),
    );

    m.insert(
        "help.search.locate".into(),
        "locate <pattern> [-d <n>] ## ค้นหาทั้งไฟล์และไดเรกทอรี".into(),
    );

    m.insert(
        "help.search.pipe".into(),
        "cmd | gs <pattern> ## ค้นหาเนื้อหาที่ pipe เข้ามา (qur -s <pattern> ก็ได้)".into(),
    );

    m.insert(
        "help.search.qur".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] ## ค้นหาไฟล์ เลือกเนื้อหา แทนที่".into(),
    );

    m.insert("help.search.title".into(), "คำสั่งค้นหา (SEARCH):".into());

    m.insert(
        "help.search.example1".into(),
        "gs fn main ## ค้นหา 'fn main' ในไฟล์".into(),
    );

    m.insert(
        "help.search.example2".into(),
        "gs -i config ## ผลการค้นหาแบบโต้ตอบ".into(),
    );

    m.insert(
        "help.search.example3".into(),
        "fs *.rs ## ค้นหาไฟล์ .rs".into(),
    );

    m.insert(
        "help.search.example4".into(),
        "ds src ## ค้นหาไดเรกทอรีชื่อ 'src'".into(),
    );

    m.insert(
        "help.search.example5".into(),
        "locate README ## ค้นหาไฟล์และไดเรกทอรีสำหรับ 'README'".into(),
    );

    m.insert(
        "help.search.example6".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + แสดงผลแบบ grep".into(),
    );

    m.insert(
        "help.search.example7".into(),
        "gs TODO < app.log ## ค้นหาเนื้อหาที่ pipe/redirect เข้ามา".into(),
    );

    m.insert(
        "help.search.example8".into(),
        "gs error\\|warning ## ค้นหาแบบ OR: ตรงกับคำใดคำหนึ่ง (a\\|b)".into(),
    );
}
