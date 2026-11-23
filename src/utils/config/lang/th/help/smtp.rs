use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.smtp.title".into(), "SMTP คำสั่ง:".into());

    m.insert(
        "help.smtp.usage".into(),
        "smtp ## ไคลเอนต์อีเมล SMTP (--id, -i, -u, -pwd, -s, -b, -f, --to)".into(),
    );

    m.insert(
        "help.smtp.example1".into(),
        "smtp --id 1 ## เปิดโปรไฟล์ไคลเอนต์ SMTP 1".into(),
    );
}
