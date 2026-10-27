use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.context.cr".into(), "<cr> กด Enter เพื่อรันคำสั่ง".into());

    m.insert(
        "help.context.qur.needs_glob".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.select_value".into(),
        "qur -s <pattern>  (pattern เนื้อหา, ไม่สนตัวพิมพ์เล็กใหญ่)".into(),
    );

    m.insert(
        "help.context.qur.replace_value".into(),
        "qur -rp <replacement>  (ต้องใช้ -s \"<pattern>\")".into(),
    );

    m.insert(
        "help.context.qur.depth_value".into(),
        "qur -d <n>  (ความลึกสูงสุด, 0 = เฉพาะโฟลเดอร์ปัจจุบัน)".into(),
    );

    m.insert(
        "help.context.qur.limit_value".into(),
        "qur -l <n>  (แสดงเฉพาะ n รายการ/ไฟล์แรก)".into(),
    );

    m.insert(
        "help.context.qur.next_select".into(),
        "qur <glob> -s \"<pattern>\" | -d <n> | -l <n>".into(),
    );

    m.insert(
        "help.context.qur.next_display_replace".into(),
        "qur <glob> -s \"<pattern>\" -dp | -rp \"<new>\" [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.next_depth".into(),
        "qur <glob> -s \"<pattern>\" [-dp | -rp \"<new>\"] [-d <n>] [-l <n>]  (<cr> เพื่อรัน)".into(),
    );
}
