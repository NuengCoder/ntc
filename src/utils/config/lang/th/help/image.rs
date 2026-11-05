use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.image.title".into(), "คำสั่งภาพ (IMAGE):".into());

    m.insert(
        "help.image.interactive".into(),
        "image ## เลือกภาพจากไดเรกทอรีปัจจุบันแล้วแสดงในเทอร์มินัล".into(),
    );

    m.insert(
        "help.image.open".into(),
        "image <เส้นทาง> ## แสดงภาพในเทอร์มินัล (เหมือน display)".into(),
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
        "help.image.example1".into(),
        "image screenshot.png ## แสดงภาพในเทอร์มินัล".into(),
    );

    m.insert(
        "help.image.example2".into(),
        "image display photo.jpg --sixel ## บังคับเรนเดอร์แบบ sixel".into(),
    );
}
