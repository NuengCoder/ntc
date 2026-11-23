use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.http.title".into(), "HTTP คำสั่ง:".into());

    m.insert(
        "help.http.usage".into(),
        "http <method> <url> [options] ## ส่งคำขอ (method เริ่มต้นคือ GET)".into(),
    );

    m.insert(
        "help.http.usage2".into(),
        "http GET|POST|PUT|PATCH|DELETE|HEAD|QUERY <url> ## วิธีที่รองรับ".into(),
    );

    m.insert(
        "help.http.note".into(),
        "ตัวเลือก: -H \"Key: Value\" header, -d ข้อความ body, --json body แบบ JSON (ตั้ง Content-Type ให้), -q key=value query, -a user:pass Basic auth, --bearer token, -o ไฟล์ บันทึก body, -s แสดงเฉพาะ body, -v แสดงคำขอ, -t วินาที timeout (ค่าเริ่มต้น 30) ผลลัพธ์ JSON จะจัดรูปแบบให้อัตโนมัติ เว้นแต่ใช้ --no-pretty".into(),
    );

    m.insert(
        "help.http.example1".into(),
        "http GET https://api.example.com/users ## คำขอ GET ง่าย ๆ".into(),
    );

    m.insert(
        "help.http.example2".into(),
        "http POST https://api.example.com/users --json '{\"name\":\"ada\"}' ## ส่ง body แบบ JSON".into(),
    );

    m.insert(
        "help.http.example3".into(),
        "http GET https://api.github.com/repos/rust-lang/rust -H \"Accept: application/vnd.github+json\" -q per_page=1 ## header + query".into(),
    );

    m.insert(
        "help.http.example4".into(),
        "http GET https://api.example.com/me -a user:pass -o me.json ## Basic auth และบันทึก body ลงไฟล์".into(),
    );

    m.insert(
        "help.http.example5".into(),
        "http GET https://api.example.com/users -s | qur -s name ## ส่ง body ดิบต่อไปยังคำสั่งอื่น".into(),
    );
}
