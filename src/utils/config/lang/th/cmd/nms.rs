use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nms.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.nms.help_play".into(),
        "nms play [-sf <dur>] <ไฟล์>                     เล่นเสียงในเทอร์มินัล (Ctrl+C หยุด; วิดีโอเล่นเฉพาะเสียง)"
            .into(),
    );

    m.insert(
        "cmd.nms.help_scan".into(),
        "nms scan [-d|--depth <n>] [-s|--size] [-l|--long <dur>] [--care]  แสดงไฟล์เสียง + วิดีโอ"
            .into(),
    );

    m.insert(
        "cmd.nms.help_help".into(),
        "nms help                                         แสดงความช่วยเหลือนี้".into(),
    );

    m.insert("cmd.nms.options_title".into(), "ตัวเลือก:".into());

    m.insert(
        "cmd.nms.help_depth".into(),
        "  -d, --depth <n>   ความลึกสูงสุด (ค่าเริ่มต้น: จาก config)".into(),
    );

    m.insert(
        "cmd.nms.help_size".into(),
        "  -s, --size        แสดงขนาดไฟล์และรวมทั้งหมด".into(),
    );

    m.insert(
        "cmd.nms.help_long".into(),
        "  -l, --long <dur>  แสดงเฉพาะมีเดียที่ยาวกว่า <dur> (เช่น 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "cmd.nms.help_care".into(),
        "  --care            ไม่สนใจกฎ ignore (โหมด care)".into(),
    );

    m.insert(
        "cmd.nms.help_start".into(),
        "  -sf, --start-from <dur>  เริ่มเล่นที่ <dur> (เฉพาะ play เช่น -sf 30)".into(),
    );

    m.insert(
        "cmd.nms.start_usage".into(),
        "วิธีใช้: -sf <dur> (เช่น -sf 30, -sf 1mn)".into(),
    );

    m.insert(
        "cmd.nms.invalid_start".into(),
        "เวลาเริ่มไม่ถูกต้อง: '{}' (เช่น 30, 1mn)".into(),
    );

    m.insert("cmd.nms.playing".into(), "♪ {}  [{}]".into());

    m.insert("cmd.nms.stop_hint".into(), "กด Ctrl+C เพื่อหยุด".into());

    m.insert("cmd.nms.keys_title".into(), "การเล่น:".into());

    m.insert(
        "cmd.nms.help_keys".into(),
        "  นับเวลาถอยหลังตำแหน่ง / เวลาที่เหลือ · กด Ctrl+C เพื่อหยุด · -sf <dur> เริ่มจากตำแหน่งที่กำหนด".into(),
    );

    m.insert("cmd.nms.examples_title".into(), "ตัวอย่าง:".into());

    m.insert("cmd.nms.example_play".into(), "nms play -sf 30 song.mp3".into());

    m.insert("cmd.nms.example_video".into(), "nms play movie.mp4".into());

    m.insert("cmd.nms.example_scan".into(), "nms scan -d 2 -s".into());

    m.insert("cmd.nms.example_long".into(), "nms scan -l 1mn".into());

    m.insert(
        "cmd.nms.play_missing_file".into(),
        "ขาดไฟล์ วิธีใช้: nms play <ไฟล์>".into(),
    );

    m.insert("cmd.nms.file_not_found".into(), "ไม่พบไฟล์ '{}'".into());

    m.insert("cmd.nms.not_media".into(), "ไม่ใช่ไฟล์เสียง/วิดีโอ: '{}'".into());

    m.insert(
        "cmd.nms.no_sound_device".into(),
        "ไม่พบอุปกรณ์เสียงบนเครื่องนี้ — ไม่สามารถเล่นเสียงได้".into(),
    );

    m.insert(
        "cmd.nms.need_ffmpeg".into(),
        "เล่น '{}' ไม่ได้: ฟอร์แมตหรือตัวถอดรหัสนี้ต้องมี ffmpeg".into(),
    );

    m.insert(
        "cmd.nms.install_ffmpeg".into(),
        "ติดตั้งได้ด้วย: ntpkg install ffmpeg".into(),
    );

    m.insert(
        "cmd.nms.no_audio_track".into(),
        "'{}' ไม่มีแทร็กเสียง".into(),
    );

    m.insert("cmd.nms.play_failed".into(), "เล่น '{}' ไม่ได้: {}".into());

    m.insert("cmd.nms.unknown_sub".into(), "ไม่รู้จักคำสั่งย่อย nms: {}".into());

    m.insert("cmd.nms.unknown_option".into(), "ไม่รู้จักตัวเลือก: {}".into());

    m.insert("cmd.nms.depth_usage".into(), "วิธีใช้: --depth <n>".into());

    m.insert("cmd.nms.invalid_depth".into(), "ความลึกไม่ถูกต้อง: '{}'".into());

    m.insert(
        "cmd.nms.long_usage".into(),
        "วิธีใช้: --long <dur> (เช่น 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "cmd.nms.invalid_long".into(),
        "ระยะเวลาไม่ถูกต้อง: '{}' (เช่น 25s, 1mn, 1h)".into(),
    );

    m.insert("cmd.nms.long_none".into(), "ไม่พบมีเดียที่ยาวกว่า {}".into());

    m.insert("cmd.nms.no_files".into(), "ไม่พบไฟล์เสียง/วิดีโอ".into());

    m.insert("cmd.nms.list_title".into(), "ไฟล์มีเดีย (เสียง + วิดีโอ):".into());

    m.insert("cmd.nms.total_files".into(), "{} ไฟล์".into());

    m.insert("cmd.nms.total_files_size".into(), "{} ไฟล์ รวม {}".into());
}
