use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qr.decode".into(),
        "qr decode <image> ## ถอดรหัส QR code จากภาพ".into(),
    );

    m.insert(
        "help.qr.encode".into(),
        "qr <text> ## เข้ารหัสข้อความและแสดง QR ในเทอร์มินัล".into(),
    );

    m.insert(
        "help.qr.save".into(),
        "qr encode <text> -o <file> ## บันทึก QR เป็นภาพ PNG".into(),
    );

    m.insert(
        "help.qr.image".into(),
        "qr encode <image> -o <file> ## เข้ารหัสภาพเป็น QR (สแกนเพื่อดูภาพ)".into(),
    );

    m.insert("help.qr.title".into(), "คำสั่ง QR CODE:".into());

    m.insert(
        "help.qr.example1".into(),
        "qr \"Hello World\" ## แสดง QR code ในเทอร์มินัล".into(),
    );

    m.insert(
        "help.qr.example2".into(),
        "qr encode \"Hello\" -o hello.png ## บันทึก QR เป็น PNG".into(),
    );

    m.insert(
        "help.qr.example3".into(),
        "qr decode qr.png ## ถอดรหัส QR จากภาพ".into(),
    );

    m.insert(
        "help.qr.example4".into(),
        "qr encode image.png -o imageqrcode.png ## เข้ารหัสภาพเพื่อให้สแกนแล้วเห็นภาพ".into(),
    );
}
