use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.mode.admin_relaunch".into(),
        "กำลังเปิด ntc ใหม่ในฐานะ Administrator...".into(),
    );

    m.insert(
        "cmd.mode.elevate_fail".into(),
        "ไม่สามารถเปิดโพรเซสที่ยกระดับสิทธิ์: {}".into(),
    );

    m.insert(
        "cmd.mode.exe_fail".into(),
        "ไม่สามารถรับเส้นทางไฟล์ปฏิบัติการ: {}".into(),
    );

    m.insert(
        "cmd.mode.launch_fail".into(),
        "ไม่สามารถเปิดโพรเซส: {}".into(),
    );

    m.insert(
        "cmd.mode.normal_relaunch".into(),
        "กำลังเปิด ntc ใหม่ในโหมดปกติ...".into(),
    );

    m.insert(
        "cmd.mode.windows_only".into(),
        "mode เป็นคุณสมบัติเฉพาะของ Windows.".into(),
    );
}
