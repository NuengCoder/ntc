use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.todo.add".into(),
        "todo add \"name\" ## ผนวกงานใหม่".into(),
    );

    m.insert(
        "help.todo.add_at".into(),
        "todo add \"name\" -> N ## แทรกงานที่ตำแหน่ง N".into(),
    );

    m.insert(
        "help.todo.clear".into(),
        "todo clear ## ล้างงานทั้งหมด (ถามยืนยัน)".into(),
    );

    m.insert(
        "help.todo.info".into(),
        "todo info [N] ## แสดงงานทั้งหมดหรือรายละเอียดงาน N".into(),
    );

    m.insert("help.todo.rm".into(), "todo rm <N> ## ลบงาน N".into());

    m.insert(
        "help.todo.show".into(),
        "todo ## แสดงงานทั้งหมด (จาก future_todo.txt ใน CWD)".into(),
    );

    m.insert("help.todo.title".into(), "คำสั่ง TODO:".into());

    m.insert(
        "help.todo.done".into(),
        "todo done <N> ## ทำเครื่องหมายงาน N ว่าเสร็จ".into(),
    );

    m.insert(
        "help.todo.undo".into(),
        "todo undo <N> ## เปิดงาน N ใหม่".into(),
    );

    m.insert(
        "help.todo.edit".into(),
        "todo edit <N> \"ชื่อ\" ## เปลี่ยนชื่อ งาน N".into(),
    );

    m.insert("help.todo.example1".into(), "todo ## แสดงงานทั้งหมด".into());

    m.insert(
        "help.todo.example2".into(),
        "todo add \"Fix bug\" ## เพิ่มงานใหม่".into(),
    );

    m.insert("help.todo.example3".into(), "todo rm 1 ## ลบงาน #1".into());
}
