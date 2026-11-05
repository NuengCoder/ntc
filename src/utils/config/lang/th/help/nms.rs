use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "คำสั่ง NMS (NTC Music):".into());

    m.insert(
        "help.nms.play".into(),
        "nms play [-sf <dur>] <ไฟล์> ## เล่นเสียงในเทอร์มินัล (Ctrl+C หยุด; -sf เริ่มจากตำแหน่งที่กำหนด)"
            .into(),
    );

    m.insert(
        "help.nms.scan".into(),
        "nms scan [-d <n>] [-s] [-l <dur>] [--care] ## แสดงไฟล์เสียง + วิดีโอ (dur: 25s, 1mn, 1h)"
            .into(),
    );

    m.insert(
        "help.nms.long".into(),
        "nms scan -l <dur> ## แสดงเฉพาะมีเดียที่ยาวกว่า <dur> (เช่น 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "help.nms.keys".into(),
        "เล่นในเทอร์มินัลพร้อมนับตำแหน่ง / เวลาที่เหลือถอยหลัง · วิดีโอเล่นเฉพาะเสียง (ใช้ ffmpeg) · กด Ctrl+C เพื่อหยุด".into(),
    );

    m.insert(
        "help.nms.example1".into(),
        "nms play -sf 30 song.mp3 ## เริ่มเล่นไฟล์เสียงที่ 0:30".into(),
    );

    m.insert(
        "help.nms.example2".into(),
        "nms play movie.mp4 ## เล่นเสียงจากวิดีโอ".into(),
    );

    m.insert(
        "help.nms.example3".into(),
        "nms scan -d 2 -s ## แสดงไฟล์มีเดียพร้อมขนาด".into(),
    );

    m.insert(
        "help.nms.example4".into(),
        "nms scan -l 1mn ## แสดงมีเดียที่ยาวกว่า 1 นาที".into(),
    );
}
