use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nssh.title".into(), "คำสั่ง NSSH:".into());

    m.insert(
        "help.nssh.add".into(),
        "nssh add <ชื่อ> --host <โฮสต์> ## บันทึกโปรไฟล์ SSH".into(),
    );

    m.insert(
        "help.nssh.ssh".into(),
        "nssh ssh [ชื่อ] [--cmd \"คำสั่ง\"] ## เชลล์หรือคำสั่งเดียว".into(),
    );

    m.insert(
        "help.nssh.cp".into(),
        "nssh cp <โลคอล> <ชื่อ>:<รีโมต> ## อัปโหลด/ดาวน์โหลด SFTP".into(),
    );

    m.insert(
        "help.nssh.scan".into(),
        "nssh scan <โฮสต์> [--top N] ## สแกนพอร์ต TCP".into(),
    );

    m.insert(
        "help.nssh.manage".into(),
        "nssh ls|info|rm|edit ## จัดการโปรไฟล์".into(),
    );

    m.insert(
        "help.nssh.keygen".into(),
        "nssh keygen ## สร้างคีย์ผ่าน ssh-keygen".into(),
    );

    m.insert(
        "help.nssh.example1".into(),
        "nssh add prod --host 203.0.113.10 --user root ## บันทึกโปรไฟล์".into(),
    );

    m.insert(
        "help.nssh.example2".into(),
        "nssh ssh prod --cmd \"uptime\" ## รันคำสั่งรีโมต".into(),
    );

    m.insert(
        "help.nssh.example3".into(),
        "nssh cp ./app.tar.gz prod:/tmp/ ## อัปโหลดไฟล์".into(),
    );
}
