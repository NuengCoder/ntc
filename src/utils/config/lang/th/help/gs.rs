use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gs.title".into(), "GS คำสั่ง:".into());

    m.insert(
        "help.gs.usage".into(),
        "gs <รูปแบบ> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-A <n>] [-B <n>] [-I] [-x] [-h|-g|-w] [-z] ## ค้นหาเนื้อหาไฟล์ (grep ง่าย ๆ)".into(),
    );

    m.insert(
        "help.gs.interactive".into(),
        "gs -I [รูปแบบ] [-d <n>] ## grep แบบโต้ตอบ (10/หน้า, f=รูปแบบ, e/Enter=เปิด, 0-9=แถว, ?=ปุ่ม, x/h/g/w/z=สวิตช์)".into(),
    );

    m.insert(
        "help.gs.pipe".into(),
        "cmd | gs <รูปแบบ> ## ค้นหาเนื้อหาที่ pipe เข้ามา (qur -s <รูปแบบ> ก็ได้)".into(),
    );

    m.insert(
        "help.gs.context".into(),
        "gs <รูปแบบ> -A <n> -B <n> ## แสดง n บรรทัดหลัง/ก่อนแต่ละที่ตรง (10 รายการ เว้นแต่ใช้ -l <n>)".into(),
    );

    m.insert(
        "help.gs.filters".into(),
        "gs -x <ข้อความตรงตัว> -h <ขึ้นต้น> -g <ลงท้าย> -w <ทั้งบรรทัด> -z <ตัวพิมพ์> ## -x ค้นตามข้อความจริง (a\\|b คือ 'a|b' ไม่ใช่ OR); -h/-g/-w เทียบบรรทัดที่ตัดช่องว่างหัวท้ายและใช้ร่วมกันไม่ได้; -z ไม่สนตัวพิมพ์เล็กใหญ่".into(),
    );

    m.insert(
        "help.gs.example1".into(),
        "gs fn main ## ค้นหา 'fn main' ในไฟล์".into(),
    );

    m.insert(
        "help.gs.example2".into(),
        "gs -I config ## ผลการค้นหาแบบโต้ตอบ".into(),
    );

    m.insert(
        "help.gs.example3".into(),
        "gs TODO < app.log ## ค้นหาเนื้อหาที่ pipe/redirect เข้ามา".into(),
    );

    m.insert(
        "help.gs.example4".into(),
        "gs error\\|warning ## ค้นหาแบบ OR: ตรงกับคำใดคำหนึ่ง (a\\|b)".into(),
    );

    m.insert(
        "help.gs.example5".into(),
        "gs main -A 3 -B 2 ## บรรทัดที่ตรง พร้อม 3 บรรทัดถัดไปและ 2 บรรทัดก่อนหน้า".into(),
    );

    m.insert(
        "help.gs.example6".into(),
        "gs -w main ## เฉพาะบรรทัดที่เป็น 'main' ทั้งบรรทัด".into(),
    );

    m.insert(
        "help.gs.example7".into(),
        "gs -z Main -h ## ตัวพิมพ์ตรง: บรรทัดที่ขึ้นต้นด้วย 'Main'".into(),
    );
}
