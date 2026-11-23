use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ping.title".into(), "PING คำสั่ง:".into());

    m.insert(
        "help.ping.usage".into(),
        "ping <host> ## ICMP ping (-c จำนวนครั้ง)".into(),
    );

    m.insert(
        "help.ping.flags".into(),
        "ping <โฮสต์> [-c N] [--forever] [-w MS] [-s ไบต์] [-t TTL] [-4|-6] ## ping พร้อมตัวเลือก"
            .into(),
    );

    m.insert(
        "help.ping.example1".into(),
        "ping google.com ## ping ไปยัง google.com".into(),
    );
}
