use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trace.title".into(), "TRACE คำสั่ง:".into());

    m.insert(
        "help.trace.usage".into(),
        "trace <โฮสต์> [-m N] [-w MS] ## ติดตามเส้นทางทีละฮอป (tracert/traceroute หรือ TTL)".into(),
    );

    m.insert(
        "help.trace.example1".into(),
        "trace example.com ## ตามเส้นทางไปยัง example.com".into(),
    );
}
