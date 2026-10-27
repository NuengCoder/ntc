use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.alias.add".into(),
        "ral add <ชื่อ> \"<cmd>\" ## สร้าง alias".into(),
    );

    m.insert(
        "help.alias.add_array".into(),
        "ral add <ชื่อ>(x[],y) \"<cmd>\" ## Alias พร้อมพารามิเตอร์แบบ array".into(),
    );

    m.insert(
        "help.alias.add_rest".into(),
        "ral add <ชื่อ>(x) \"echo $*{x}\" ## รับหนึ่งอาร์กิวเมนต์ (คงช่องว่าง, ตัดคำพูด)".into(),
    );

    m.insert(
        "help.alias.add_defaults".into(),
        "ral add <ชื่อ>({x=a,e=txt}) \"<cmd>\" ## Alias พร้อมค่าเริ่มต้น".into(),
    );

    m.insert(
        "help.alias.add_param".into(),
        "ral add <ชื่อ>(x) \"<cmd>\" ## สร้าง alias แบบมีพารามิเตอร์ด้วย $x".into(),
    );

    m.insert(
        "help.alias.cls".into(),
        "ral cls ## ล้าง alias ทั้งหมด (ถามยืนยัน)".into(),
    );

    m.insert(
        "help.alias.edit".into(),
        "ral edit <ชื่อ> \"<cmd>\" ## อัปเดต alias".into(),
    );

    m.insert(
        "help.alias.export_all".into(),
        "ral export --all <ชื่อ> ## ส่งออก alias ทั้งหมดไปยัง <ชื่อ>.ntc.ral".into(),
    );

    m.insert(
        "help.alias.export_select".into(),
        "ral export --select <ชื่อ> ## เลือก alias เพื่อส่งออก".into(),
    );

    m.insert(
        "help.alias.import".into(),
        "ral import <ไฟล์> ## นำเข้า alias จากไฟล์ .ntc.ral".into(),
    );

    m.insert(
        "help.alias.info".into(),
        "ral info <ชื่อ> [--edit] ## แสดงรายละเอียด alias (--edit: คัดลอกคำสั่งแก้ไขไปยังคลิปบอร์ด)".into(),
    );

    m.insert(
        "help.alias.list".into(),
        "ral list ## แสดง alias ทั้งหมด".into(),
    );

    m.insert(
        "help.alias.make".into(),
        "ral make [ชื่อ] ## สร้างเทมเพลต alias ใน CWD".into(),
    );

    m.insert(
        "help.alias.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## จัดการรายการค่าที่ตั้งชื่อไว้".into(),
    );

    m.insert(
        "help.alias.rm".into(),
        "ral rm <ชื่อ>[, ...] ## ลบ alias".into(),
    );

    m.insert(
        "help.alias.rnm".into(),
        "ral rnm <เก่า> to <ใหม่> ## เปลี่ยนชื่อ alias".into(),
    );

    m.insert(
        "help.alias.run".into(),
        "run <alias> ## เรียก alias ด้วย 'run'".into(),
    );

    m.insert(
        "help.alias.run_direct".into(),
        "<alias> ## เรียก alias ตรง ๆ".into(),
    );

    m.insert(
        "help.alias.run_param".into(),
        "<alias>(<arg>) ## เรียก alias แบบมีพารามิเตอร์".into(),
    );

    m.insert("help.alias.title".into(), "คำสั่ง RUN ALIAS:".into());

    m.insert(
        "help.alias.example1".into(),
        "ral add build \"cargo build\" ## สร้าง alias ง่าย ๆ".into(),
    );

    m.insert(
        "help.alias.example2".into(),
        "ral add greet(x) \"echo Hello $x\" ## Alias แบบมีพารามิเตอร์".into(),
    );

    m.insert(
        "help.alias.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## Alias พร้อมค่าเริ่มต้น".into(),
    );

    m.insert(
        "help.alias.example4".into(),
        "ral list ## แสดง alias ทั้งหมด".into(),
    );

    m.insert(
        "help.alias.example5".into(),
        "ral export --all myaliases ## ส่งออก alias".into(),
    );

    m.insert(
        "help.alias.example6".into(),
        "build ## เรียก alias ตรง ๆ".into(),
    );

    m.insert(
        "help.alias.example7".into(),
        "ral hintn add port [8080,5050] ## รายการที่ตั้งชื่อไว้".into(),
    );
}
