use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "คำสั่ง NMS (NTC Music):".into());

    m.insert(
        "help.nms.play".into(),
        "nms play <ไฟล์> ## เครื่องเล่นเสียงเต็มจอ (space หยุด · ←/→ เลื่อน · ↑/↓ ระดับเสียง · a เล่นซ้ำ)"
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

    m.insert("help.nms.keys".into(), "หน้าจอเครื่องเล่น: คลื่นเสียงจริง + มาตรวัดระดับสูง/ต่ำ · วิดีโอเล่นเฉพาะเสียง · เมื่อจบจะหยุดนิ่ง กด a เพื่อเล่นซ้ำ".into());

    m.insert(
        "help.nms.example1".into(),
        "nms play song.mp3 ## เล่นไฟล์เสียง".into(),
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
