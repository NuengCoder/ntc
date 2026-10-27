use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ndb.title".into(), "คำสั่ง NDB (ฐานข้อมูล):".into());

    m.insert(
        "help.ndb.launch".into(),
        "ndb ## เปิด ndb database shell (.db browser) ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.ndb.root_help".into(),
        "help ## แสดงทุกคำสั่ง ndb (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.root_view".into(),
        "view | ls ## แสดงไฟล์ .db ที่ใช้ได้ (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.root_interface".into(),
        "interface <file.db> ## เปิดฐานข้อมูล: พรอมป์กลายเป็น ndb [file.db]>".into(),
    );

    m.insert(
        "help.ndb.root_sql".into(),
        "sql <query> ## ทำคิวรีแบบอ่านอย่างเดียวหลังเปิดฐานข้อมูล".into(),
    );

    m.insert(
        "help.ndb.root_exit".into(),
        "exit ## ปิดฐานข้อมูล แล้วออกจาก ndb".into(),
    );

    m.insert(
        "help.ndb.db_viewf".into(),
        "viewf | v ## แสดงโครงสร้างตารางและคอลัมน์".into(),
    );

    m.insert(
        "help.ndb.db_tables".into(),
        "tables | t ## แสดงชื่อตารางและวิว".into(),
    );

    m.insert(
        "help.ndb.db_tablef".into(),
        "tablef <table> ## แสดงแถวของตาราง (เหมือน txtf)".into(),
    );

    m.insert(
        "help.ndb.db_schema".into(),
        "schema <table> ## แสดง CREATE TABLE SQL และรายละเอียดคอลัมน์".into(),
    );

    m.insert(
        "help.ndb.db_sql".into(),
        "sql <query> ## ทำคิวรีแบบอ่านอย่างเดียว (-l N แถว, --all)".into(),
    );

    m.insert(
        "help.ndb.db_write".into(),
        "write YES <stmt> ## ทำการเขียนหนึ่งคำสั่ง ต้อง edit on ก่อน (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.db_edit".into(),
        "edit on|off ## อนุญาตหรือบล็อกการเขียน (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.db_backup".into(),
        "backup ## คัดลอกไฟล์ .db ไว้ข้างๆ (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.more".into(),
        "more ## แสดงหน้าถัดไปของคิวรีล่าสุด (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.indexes".into(),
        "indexes [table] ## แสดงดัชนี (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.info".into(),
        "info ## แสดงไฟล์ เอนจิน และจำนวนออบเจกต์ (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.dbal".into(),
        "dbal ## จัดการคิวรีอ่านอย่างเดียวที่บันทึกไว้: add|run|list|rm (ภายใน ndb)".into(),
    );

    m.insert(
        "help.ndb.db_exit".into(),
        "exit ## ปิดฐานข้อมูลและกลับไป ndb [cwd]>".into(),
    );

    m.insert(
        "help.ndb.readonly".into(),
        "อ่านอย่างเดียว ## บล็อกการเขียน".into(),
    );

    m.insert(
        "help.ndb.example1".into(),
        "ndb ## จากนั้น: interface users.db".into(),
    );

    m.insert(
        "help.ndb.example2".into(),
        "ndb ## จากนั้น: tablef users --all".into(),
    );
}
