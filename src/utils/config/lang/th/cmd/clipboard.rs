use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.clipboard.access_fail".into(),
        "ไม่สามารถเข้าถึงคลิปบอร์ด: {}".into(),
    );

    m.insert(
        "cmd.clipboard.already_empty".into(),
        "ประวัติคลิปบอร์ดว่างเปล่าอยู่แล้ว.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_cancelled".into(),
        "ยกเลิกการล้าง.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_error".into(),
        "ไม่สามารถล้างคลิปบอร์ดได้: {}.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_prompt".into(),
        "ล้างประวัติคลิปบอร์ด ({} บริการ)? [y/N] ".into(),
    );

    m.insert("cmd.clipboard.cleared".into(), "ล้างประวัติคลิปบอร์ดแล้ว.".into());

    m.insert(
        "cmd.clipboard.copied_entry".into(),
        "คัดลอกรายการ #{} ({}) ไปยังคลิปบอร์ดแล้ว.".into(),
    );

    m.insert("cmd.clipboard.empty".into(), "ประวัติคลิปบอร์ดว่างเปล่า.".into());

    m.insert(
        "cmd.clipboard.entry_not_found".into(),
        "ไม่พบรายการ #{} (มี {} รายการ).".into(),
    );

    m.insert(
        "cmd.clipboard.help_clear".into(),
        "  clip clear         ล้างประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "cmd.clipboard.help_list".into(),
        "  clip               แสดงประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "cmd.clipboard.help_select".into(),
        "  clip select <N>    คัดลอกรายการ N กลับไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "cmd.clipboard.history_header".into(),
        "  ประวัติคลิปบอร์ด (ล่าสุด {}):".into(),
    );

    m.insert(
        "cmd.clipboard.select_usage".into(),
        "การใช้งาน: clip select <N>".into(),
    );

    m.insert(
        "cmd.clipboard.set_fail".into(),
        "ไม่สามารถตั้งค่าคลิปบอร์ด: {}".into(),
    );

    m.insert(
        "cmd.clipboard.termux_unavailable".into(),
        "termux-clipboard-set ไม่พร้อมใช้งาน: {}".into(),
    );

    m.insert(
        "cmd.clipboard.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อยของ clip: {}".into(),
    );

    m.insert(
        "cmd.clipboard.copied_piped".into(),
        "คัดลอกข้อมูลจาก pipe ไปยังคลิปบอร์ดแล้ว.".into(),
    );
}
