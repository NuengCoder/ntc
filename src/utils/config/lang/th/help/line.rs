use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.line.title".into(), "LINE คำสั่ง:".into());

    m.insert(
        "help.line.usage".into(),
        "line [-A <คำนำหน้า>] [-B <ตัวคั่น>] [ไฟล์] ## ใส่เลขทุกบรรทัด (ค่าเริ่มต้น '1: บรรทัด')"
            .into(),
    );

    m.insert(
        "help.line.example1".into(),
        "gs main | line -A 'line' -B ':' ## เติม line1: ... หน้าทุกบรรทัด".into(),
    );
}
