use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.report.cp".into(),
        "report <fmt> --cp ## คัดลอกรายงานไปยังคลิปบอร์ดแทนการบันทึก".into(),
    );

    m.insert(
        "help.report.docx".into(),
        "report docx [path] ## สร้างรายงาน DOCX".into(),
    );

    m.insert(
        "help.report.html".into(),
        "report html [path] ## สร้างรายงาน HTML".into(),
    );

    m.insert(
        "help.report.json".into(),
        "report json [path] ## สร้างรายงาน JSON".into(),
    );

    m.insert(
        "help.report.md".into(),
        "report md [path] ## สร้างรายงาน Markdown".into(),
    );

    m.insert(
        "help.report.pdf".into(),
        "report pdf [path] ## สร้างรายงาน PDF".into(),
    );

    m.insert("help.report.title".into(), "คำสั่งรายงาน (REPORT):".into());

    m.insert(
        "help.report.txt".into(),
        "report txt [path] ## สร้างรายงาน TXT".into(),
    );

    m.insert(
        "help.report.txtc".into(),
        "txtc <file> ## คัดลอกเนื้อหาไฟล์ไปยังคลิปบอร์ด".into(),
    );

    m.insert("help.report.txtf".into(), "txtf [--head N|--tail N|--betw A:B] [file|dir|@tp/file|@tp/dir] ## แสดงเนื้อหาไฟล์หรือเลือกไฟล์จากไดเรกทอรี (เลขบรรทัดเดิม)".into());

    m.insert(
        "help.report.txtf_example1".into(),
        "txtf main.rs --head 30 ## 30 บรรทัดแรกของ main.rs".into(),
    );

    m.insert(
        "help.report.txtf_example2".into(),
        "txtf --betw 10:20 main.rs ## บรรทัด 10-20 ของ main.rs".into(),
    );

    m.insert(
        "help.report.txtf_example3".into(),
        "txtf @docs/notes.txt ## แสดงไฟล์ในจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "help.report.txtf_example4".into(),
        "txtf src ## เลือกไฟล์ที่รองรับจากไดเรกทอรี src เพื่อแสดง".into(),
    );

    m.insert(
        "help.report.xlsx".into(),
        "report xlsx [path] ## สร้างรายงาน XLSX (พร้อมการวิเคราะห์การอ้างอิง)".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report txt ## สร้างรายงาน TXT".into(),
    );

    m.insert(
        "help.report.example2".into(),
        "report html --cp ## คัดลอกรายงาน HTML ไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.report.example3".into(),
        "report pdf ## สร้างรายงาน PDF".into(),
    );

    m.insert(
        "help.report.example4".into(),
        "txtc file.txt ## คัดลอกเนื้อหาไฟล์ไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## แสดงเนื้อหาคลิปบอร์ดทั้งหมด".into(),
    );

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## บันทึกรายงานโปรเจกต์ไว้ที่ <file>".into(),
    );

    m.insert("help.report.project_example".into(), "report project html . --name docs/project ## รายงาน HTML พร้อมต้นไม้ที่พับ/ขยายได้ บันทึกเป็น docs/project.html".into());

    m.insert("help.report.clip_example".into(), "report clip 1".into());
}
