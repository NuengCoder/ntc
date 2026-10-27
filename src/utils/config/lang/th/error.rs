use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "error.failed_write_completions".into(),
        "ไม่สามารถเขียนการเติมเต็ม: {}".into(),
    );

    m.insert(
        "error.unsupported_shell".into(),
        "เชลล์ไม่รองรับ: {}. ใช้: bash, zsh, fish, powershell".into(),
    );
}
