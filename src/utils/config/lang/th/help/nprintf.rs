use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nprintf.title".into(), r#"คำสั่ง NPRINTF:"#.into());
    m.insert(
        "help.nprintf.usage".into(),
        r#"nprintf <รูปแบบ> [อาร์กิวเมนต์...] ## เอาต์พุตแบบจัดรูปแบบสไตล์ Bash"#.into(),
    );
    m.insert("help.nprintf.conversions".into(), r#"  ตัวแปลง"#.into());
    m.insert("help.nprintf.specs".into(), r#"  %s ข้อความ · %d/%i จำนวนเต็มมีเครื่องหมาย · %u ไม่มีเครื่องหมาย · %o ฐานแปด · %x/%X ฐานสิบหก · %c ตัวอักษรแรก"#.into());
    m.insert("help.nprintf.specs2".into(), r#"  %b แปลง escape ในอาร์กิวเมนต์ · %q ใส่ quote แบบ shell · %f/%e/%g/%a ทศนิยม · %% เครื่องหมายเปอร์เซ็นต์"#.into());
    m.insert(
        "help.nprintf.flags".into(),
        r#"  แฟล็ก ความกว้าง ความแม่นยำ"#.into(),
    );
    m.insert("help.nprintf.flags_detail".into(), r#"  %-8s ชิดซ้าย · %05d เติมศูนย์ · %+d แสดงเครื่องหมาย · %.2f ความแม่นยำ · %*s ความกว้างจากอาร์กิวเมนต์"#.into());
    m.insert(
        "help.nprintf.escapes".into(),
        r#"  ESCAPE (แต่ละตัวเริ่มด้วยแบ็กสแลช)"#.into(),
    );
    m.insert("help.nprintf.escapes_detail".into(), r#"  \n ขึ้นบรรทัดใหม่ · \t แท็บ · \r carriage return · \ แบ็กสแลช · \" เครื่องหมายคำพูด · \xHH ฐานสิบหก · \0NNN ฐานแปด · \c หยุดเอาต์พุต"#.into());
    m.insert("help.nprintf.newline".into(), r#"  ทำไมต้องมี \n?"#.into());
    m.insert("help.nprintf.newline_detail".into(), r#"  nprintf ไม่เพิ่มอะไรต่อท้ายรูปแบบ. ใส่ \n ตอนท้ายเมื่อต้องการขึ้นบรรทัดใหม่; ถ้าไม่ใส่ เอาต์พุตจะต่อในบรรทัดเดิม."#.into());
    m.insert("help.nprintf.quoting".into(), r#"  การใส่ quote"#.into());
    m.insert(
        "help.nprintf.quoting_detail".into(),
        r#"  ใส่รูปแบบในเครื่องหมายคำพูดคู่ เพื่อให้ช่องว่างและ escape อยู่ในอาร์กิวเมนต์เดียว."#.into(),
    );
    m.insert(
        "help.nprintf.reuse".into(),
        r#"  รูปแบบจะถูกใช้ซ้ำเมื่อยังมีอาร์กิวเมนต์; %s ที่ขาดจะเป็นค่าว่าง และตัวเลขที่ขาดจะเป็น 0."#.into(),
    );
    m.insert(
        "help.nprintf.example1".into(),
        r#"nprintf "hello %s\n" world ## หนึ่งบรรทัด; \n จบบรรทัด"#.into(),
    );
    m.insert(
        "help.nprintf.example2".into(),
        r#"nprintf "%s has %d items\n" cart 3 ## ข้อความและจำนวนเต็ม"#.into(),
    );
    m.insert(
        "help.nprintf.example3".into(),
        r#"nprintf "%-8s|%05d|%.2f\n" left 42 3.14159 ## ชิดซ้าย เติมศูนย์ ความแม่นยำ"#.into(),
    );
    m.insert(
        "help.nprintf.example4".into(),
        r#"nprintf "%b\n" "a\tb" ## %b แปลง escape จากอาร์กิวเมนต์"#.into(),
    );
    m.insert(
        "help.nprintf.example5".into(),
        r#"nprintf "no newline" ## อยู่ในบรรทัดเดิม"#.into(),
    );
}
