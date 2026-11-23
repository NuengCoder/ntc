use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ral.title".into(), "คำสั่ง RUN ALIAS:".into());

    m.insert(
        "help.ral.add".into(),
        "ral add <ชื่อ> \"<cmd>\" ## สร้าง alias".into(),
    );

    m.insert(
        "help.ral.add_param".into(),
        "ral add <ชื่อ>(x) \"<cmd>\" ## สร้าง alias แบบมีพารามิเตอร์ด้วย ${x}".into(),
    );

    m.insert(
        "help.ral.env_ref".into(),
        "ral add <ชื่อ> \"<cmd>\" ## $NAME เดี่ยวคือ environment variable ใช้ ${x} สำหรับพารามิเตอร์".into(),
    );

    m.insert(
        "help.ral.add_defaults".into(),
        "ral add <ชื่อ>({x=a,e=txt}) \"<cmd>\" ## Alias พร้อมค่าเริ่มต้น".into(),
    );

    m.insert(
        "help.ral.add_array".into(),
        "ral add <ชื่อ>(x[],y) \"<cmd>\" ## Alias พร้อมพารามิเตอร์แบบ array".into(),
    );

    m.insert(
        "help.ral.add_rest".into(),
        "ral add <ชื่อ>(x) \"echo $*{x}\" ## รับหนึ่งอาร์กิวเมนต์ (คงช่องว่าง, ตัดคำพูด)".into(),
    );

    m.insert(
        "help.ral.edit".into(),
        "ral edit <ชื่อ> \"<cmd>\" ## อัปเดต alias".into(),
    );

    m.insert(
        "help.ral.rnm".into(),
        "ral rnm <เก่า> to <ใหม่> ## เปลี่ยนชื่อ alias".into(),
    );

    m.insert(
        "help.ral.rm".into(),
        "ral rm <ชื่อ>[, ...] ## ลบ alias".into(),
    );

    m.insert(
        "help.ral.list".into(),
        "ral list ## แสดง alias ทั้งหมด".into(),
    );

    m.insert(
        "help.ral.info".into(),
        "ral info <ชื่อ> [--edit] ## แสดงรายละเอียด alias (--edit: คัดลอกคำสั่งแก้ไขไปยังคลิปบอร์ด)".into(),
    );

    m.insert(
        "help.ral.cls".into(),
        "ral cls ## ล้าง alias ทั้งหมด (ถามยืนยัน)".into(),
    );

    m.insert(
        "help.ral.export_all".into(),
        "ral export --all <ชื่อ> ## ส่งออก alias ทั้งหมดไปยัง <ชื่อ>.ntc.ral".into(),
    );

    m.insert(
        "help.ral.export_select".into(),
        "ral export --select <ชื่อ> ## เลือก alias เพื่อส่งออก".into(),
    );

    m.insert(
        "help.ral.import".into(),
        "ral import <ไฟล์> ## นำเข้า alias จากไฟล์ .ntc.ral".into(),
    );

    m.insert(
        "help.ral.make".into(),
        "ral make [ชื่อ] ## สร้างเทมเพลต alias ใน CWD".into(),
    );

    m.insert(
        "help.ral.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## จัดการรายการค่าที่ตั้งชื่อไว้".into(),
    );

    m.insert(
        "help.ral.example1".into(),
        "ral add build \"cargo build\" ## สร้าง alias ง่าย ๆ".into(),
    );

    m.insert(
        "help.ral.example2".into(),
        "ral add greet(x) \"echo Hello ${x}\" ## Alias แบบมีพารามิเตอร์".into(),
    );

    m.insert(
        "help.ral.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## Alias พร้อมค่าเริ่มต้น".into(),
    );

    m.insert(
        "help.ral.example4".into(),
        "ral list ## แสดง alias ทั้งหมด".into(),
    );

    m.insert(
        "help.ral.example5".into(),
        "ral export --all myaliases ## ส่งออก alias".into(),
    );

    m.insert(
        "help.ral.example6".into(),
        "ral hintn add port [8080,5050] ## รายการที่ตั้งชื่อไว้".into(),
    );

    m.insert(
        "help.ral.install".into(),
        "ral install ## ติดตั้ง run aliases จากแคตตาล็อก GitHub ของ ntc".into(),
    );
}
