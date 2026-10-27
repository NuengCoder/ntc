use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ranfile.available_title".into(), "target ที่มี:".into());

    m.insert("ranfile.depends_on".into(), " ─▶ ขึ้นอยู่กับ: ".into());

    m.insert(
        "ranfile.runner.hint".into(),
        "เรียก {} เพื่อดู target ที่มี.".into(),
    );
}
