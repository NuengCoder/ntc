use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fake.title".into(), "fake - สร้างข้อมูลจำลอง/สุ่ม".into());

    m.insert(
        "help.fake.gen".into(),
        "  fake gen <ชนิด> [-n N] [-f รูปแบบ] [-s ซีด] [-o ไฟล์]   สร้างแถว".into(),
    );

    m.insert(
        "help.fake.gen_options".into(),
        "  รูปแบบ: json | txt | sql | csv | xlsx (ค่าเริ่มต้น: ตาราง txt)".into(),
    );

    m.insert(
        "help.fake.gen_group".into(),
        "  fake gen -g <กลุ่ม> ...                          สร้างจากกลุ่มที่บันทึกไว้".into(),
    );

    m.insert(
        "help.fake.gen_unique".into(),
        "  -u, --unique [คอลัมน์]                          แถวหรือคอลัมน์ที่ไม่ซ้ำ".into(),
    );

    m.insert("help.fake.gen_locale".into(), "  -l, --locale <รหัส>                            ข้อมูลตามภาษา (en_US en_GB zh_CN th_TH lo_LA)".into());

    m.insert(
        "help.fake.types".into(),
        "  fake types                                     แสดงชนิดทั้งหมด".into(),
    );

    m.insert(
        "help.fake.types_search".into(),
        "  fake types -s <รูปแบบ>                          ค้นหาชนิดแบบคลุมเครือ".into(),
    );

    m.insert(
        "help.fake.locales".into(),
        "  fake locales                                    แสดงรหัส --locale".into(),
    );

    m.insert(
        "help.fake.group_add".into(),
        "  fake group add <ชื่อ> [นามแฝง as ชนิด, ...]     สร้างกลุ่ม".into(),
    );

    m.insert(
        "help.fake.group_edit".into(),
        "  fake group edit <ชื่อ> [นามแฝง as ชนิด, ...]    แทนที่คอลัมน์ของกลุ่ม".into(),
    );

    m.insert(
        "help.fake.group_rm".into(),
        "  fake group rm <ชื่อ>                            ลบกลุ่ม".into(),
    );

    m.insert(
        "help.fake.group_rnm".into(),
        "  fake group rnm <เก่า> <ใหม่>                    เปลี่ยนชื่อกลุ่ม".into(),
    );

    m.insert(
        "help.fake.group_ls".into(),
        "  fake group ls [รูปแบบ]                          แสดงกลุ่ม (ไวลด์การ์ด เช่น u*)".into(),
    );

    m.insert(
        "help.fake.group_info".into(),
        "  fake group info <ชื่อ>                          แสดงคอลัมน์ของกลุ่ม".into(),
    );

    m.insert(
        "help.fake.group_clear".into(),
        "  fake group clear [-f]                           ลบทุกกลุ่ม".into(),
    );

    m.insert(
        "help.fake.group_export".into(),
        "  fake group export <ชื่อ>|--all [-o ไฟล์]        ส่งออกกลุ่มเป็น .ntc_fake".into(),
    );

    m.insert(
        "help.fake.group_import".into(),
        "  fake group import <ไฟล์> [--force] [--prefix p]  โหลดกลุ่มจากไฟล์".into(),
    );

    m.insert(
        "help.fake.group_ref".into(),
        "  คอลัมน์: 'นามแฝง as ชนิด', 'ชนิด !' = ไม่ซ้ำ, 'นามแฝง as ref:กลุ่ม.คอลัมน์'".into(),
    );

    m.insert(
        "help.fake.help".into(),
        "  fake help                                      แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "help.fake.example1".into(),
        "fake gen name,email,city -n 5 ## ตาราง txt".into(),
    );

    m.insert(
        "help.fake.example2".into(),
        "fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "help.fake.example3".into(),
        "fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "help.fake.example4".into(),
        "fake gen -g users -f sql -n 10 ## INSERT INTO users ...".into(),
    );
}
