use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmd.title".into(), "RMD คำสั่ง:".into());

    m.insert(
        "help.rmd.usage".into(),
        "rmd <dir>[, ...] ## ลบไดเรกทอรี; wildcard ลบทุกที่ตรงกันหลังยืนยัน".into(),
    );

    m.insert(
        "help.rmd.example1".into(),
        "rmd build ## ลบไดเรกทอรี build".into(),
    );
}
