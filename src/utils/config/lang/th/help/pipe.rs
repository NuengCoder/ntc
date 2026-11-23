use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.pipe.title".into(),
        "ไพป์และการเปลี่ยนเส้นทาง (PIPES & REDIRECTION) [NON-COMMAND]:".into(),
    );

    m.insert(
        "help.pipe.pipe".into(),
        "a | b ## ส่งเอาต์พุตของ a ไปเป็นอินพุตของ b".into(),
    );

    m.insert(
        "help.pipe.and".into(),
        "a && b ## รัน b เฉพาะเมื่อ a สำเร็จ".into(),
    );

    m.insert(
        "help.pipe.or".into(),
        "a || b ## รัน b เฉพาะเมื่อ a ล้มเหลว".into(),
    );

    m.insert(
        "help.pipe.seq".into(),
        "a ; b ## รัน b แม้ a จะล้มเหลว (ต้องมีช่องว่างสองข้าง)".into(),
    );

    m.insert("help.pipe.in".into(), "cmd < file ## อ่านอินพุตจากไฟล์".into());

    m.insert(
        "help.pipe.out".into(),
        "cmd > file ## เขียนเอาต์พุตลงไฟล์ (เขียนทับ)".into(),
    );

    m.insert(
        "help.pipe.append".into(),
        "cmd >> file ## ต่อท้ายเอาต์พุตลงไฟล์".into(),
    );

    m.insert(
        "help.pipe.heredoc".into(),
        "cmd <<EOF ... EOF ## อ่านอินพุตจนถึงบรรทัด EOF".into(),
    );

    m.insert(
        "help.pipe.herestring".into(),
        "cmd <<< text ## อ่านอินพุตจากข้อความโดยตรง".into(),
    );

    m.insert(
        "help.pipe.filters".into(),
        "cmd | <filter> ## ตัวกรองไปป์: head, tail, betw, skip, gs, sort, rev, count, trim, encode, qur, clip (เติมแฟล็กได้ด้วย)"
            .into(),
    );

    m.insert(
        "help.pipe.note".into(),
        "ตัวดำเนินการต้องมีช่องว่างสองข้าง ใช้ - เพื่ออ่าน stdin".into(),
    );

    m.insert(
        "help.pipe.hint".into(),
        "คำใบ้: | แสดงตัวกรองไปป์; >> และ < แสดงเฉพาะไฟล์ข้อความ; > ไม่แสดง; << แสดงเฉพาะ EOF".into(),
    );

    m.insert(
        "help.pipe.example1".into(),
        "gs main > results.txt ## บันทึกผลการค้นหาลงไฟล์".into(),
    );

    m.insert(
        "help.pipe.example2".into(),
        "view readme.md | encode b64 ## เข้ารหัสเนื้อหาไฟล์".into(),
    );

    m.insert(
        "help.pipe.example3".into(),
        "gs main | say ## ดูผลการค้นหาผ่าน say".into(),
    );

    m.insert(
        "help.pipe.example4".into(),
        "head 20 app.log ## 20 บรรทัดแรกของไฟล์ (หรืออินพุตที่ pipe เข้ามา)".into(),
    );

    m.insert(
        "help.pipe.example5".into(),
        "gs error | sort -u | head 10 ## เรียงผลลัพธ์ที่ไม่ซ้ำ แล้วเอา 10 บรรทัดแรก".into(),
    );

    m.insert(
        "help.pipe.example6".into(),
        "echo nice ... bro ... | trim \" ...\" ## ตัดแพตเทิร์นออกจากข้อความที่ส่งผ่านไปป์ (แบบอินไลน์: trim \" ...\" \"nice ... bro ...\")".into(),
    );
}
