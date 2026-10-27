use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.image.convert".into(),
        "image <path> to <fmt> ## แปลงและแสดงผลลัพธ์ (json|md|html|txt|desc)".into(),
    );

    m.insert(
        "help.image.display".into(),
        "image display <เส้นทาง> ## แสดงภาพในเทอร์มินัลโดยตรง".into(),
    );

    m.insert(
        "help.image.show".into(),
        "image show <เส้นทาง> ## แสดงภาพในเทอร์มินัลโดยตรง".into(),
    );

    m.insert(
        "help.image.sixel".into(),
        "image display <เส้นทาง> --sixel ## บังคับเรนเดอร์แบบ sixel".into(),
    );

    m.insert(
        "help.image.interactive".into(),
        "image ## ตัวแปลงภาพแบบโต้ตอบ (เลือกจากไดเรกทอรีปัจจุบัน)".into(),
    );

    m.insert(
        "help.image.layout".into(),
        "image <path> --layout ## เปิดการตรวจจับเลย์เอาต์ UI".into(),
    );

    m.insert(
        "help.image.ocr".into(),
        "image <path> --ocr ## เปิดการแยกข้อความ OCR".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <path> ## เปิดภาพในโปรแกรมดูเริ่มต้น".into(),
    );

    m.insert(
        "help.image.save".into(),
        "image <path> out <fmt> ## แปลงและบันทึกไปยังเส้นทาง setO".into(),
    );

    m.insert("help.image.title".into(), "คำสั่งภาพ (IMAGE):".into());

    m.insert(
        "help.image.example1".into(),
        "image screenshot.png to json ## แปลงภาพเป็น JSON สำหรับ AI".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image photo.jpg --ocr ## แยกข้อความจากภาพด้วย OCR".into(),
    );
}
