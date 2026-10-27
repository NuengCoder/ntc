use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.time.count_usage".into(),
        "วิธีใช้: time count <ระยะเวลา>".into(),
    );

    m.insert("cmd.time.run_usage".into(), "วิธีใช้: time run <คำสั่ง>".into());

    m.insert("cmd.time.executing".into(), "กำลังเรียก: {}".into());

    m.insert("cmd.time.elapsed".into(), "ใช้เวลา: {}".into());

    m.insert("cmd.time.exit_status".into(), "สถานะออก: {}".into());

    m.insert("cmd.time.terminated".into(), "คำสั่งถูกยกเลิก".into());

    m.insert(
        "cmd.time.first_command".into(),
        "กำลังเรียกคำสั่งแรก: {}".into(),
    );

    m.insert(
        "cmd.time.second_command".into(),
        "กำลังเรียกคำสั่งที่สอง: {}".into(),
    );

    m.insert("cmd.time.first_result".into(), "แรก:  {} (ออก {})".into());

    m.insert("cmd.time.second_result".into(), "ที่สอง: {} (ออก {})".into());

    m.insert("cmd.time.outputs_match".into(), "ผลลัพธ์ของคำสั่งตรงกัน".into());

    m.insert(
        "cmd.time.outputs_differ".into(),
        "ผลลัพธ์ของคำสั่งแตกต่างกัน:".into(),
    );

    m.insert(
        "cmd.time.large_diff".into(),
        "ผลต่างมีขนาดใหญ่ กำลังแสดงบรรทัดที่ลบและเพิ่มโดยไม่จัดแนว:".into(),
    );

    m.insert(
        "cmd.time.invalid_duration".into(),
        "ระยะเวลา '{}' ไม่ถูกต้อง ใช้จำนวนวินาทีหรือรูปแบบเช่น 90s, 5m หรือ 1h30m".into(),
    );

    m.insert("cmd.time.cancelled".into(), "ยกเลิกตัวจับเวลาแล้ว".into());

    m.insert("cmd.time.finished".into(), "หมดเวลาแล้ว!".into());

    m.insert("cmd.time.remaining".into(), "เหลืออีก {} วินาที ".into());

    m.insert(
        "cmd.time.compare_usage".into(),
        "วิธีใช้: time compare <คำสั่ง A> --with <คำสั่ง B> [-w]".into(),
    );

    m.insert(
        "cmd.time.bench_usage".into(),
        "วิธีใช้: time bench [--runs N] <คำสั่ง>".into(),
    );

    m.insert(
        "cmd.time.invalid_runs".into(),
        "time bench: จำนวนรอบไม่ถูกต้อง '{}' กรุณาใช้จำนวนเต็มบวก".into(),
    );

    m.insert(
        "cmd.time.bench_header".into(),
        "ทดสอบประสิทธิภาพ {} รอบ: {}".into(),
    );

    m.insert("cmd.time.bench_run".into(), "  รอบที่ {}: {} ({})".into());

    m.insert("cmd.time.bench_summary".into(), "สรุป ({} รอบ):".into());

    m.insert("cmd.time.bench_min".into(), "  ต่ำสุด:  {}".into());

    m.insert("cmd.time.bench_max".into(), "  สูงสุด:  {}".into());

    m.insert("cmd.time.bench_mean".into(), "  เฉลี่ย:  {}".into());

    m.insert("cmd.time.bench_median".into(), "  มัธยฐาน: {}".into());

    m.insert(
        "cmd.time.bench_note".into(),
        "การเรียกซ้ำอาจเปลี่ยนไฟล์หรือสภาพแวดล้อม; ผลข้างเคียงจะไม่ถูกย้อนกลับ".into(),
    );

    m.insert("cmd.time.status_ok".into(), "สำเร็จ".into());

    m.insert("cmd.time.status_failed".into(), "ล้มเหลว".into());

    m.insert(
        "cmd.time.ignoring_whitespace".into(),
        "ไม่สนใจความต่างที่เป็นเพียงช่องว่าง".into(),
    );
}
