use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.todo.add_usage".into(),
        "การใช้งาน: todo add \"ชื่องาน\" [-> N]".into(),
    );

    m.insert("cmd.todo.all_cleared".into(), "ล้างงานทั้งหมดแล้ว.".into());

    m.insert("cmd.todo.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert(
        "cmd.todo.clear_prompt".into(),
        "ล้างงานทั้งหมด {} รายการ? [y/N] ".into(),
    );

    m.insert("cmd.todo.empty_name".into(), "ชื่องานต้องไม่ว่างเปล่า".into());

    m.insert(
        "cmd.todo.help_add".into(),
        "  todo add \\\"ชื่อ\\\" [-> N]  เพิ่มงาน (ไม่บังคับระบุตำแหน่ง N)".into(),
    );

    m.insert(
        "cmd.todo.help_clear".into(),
        "  todo clear              ล้างงานทั้งหมด (ถามยืนยัน)".into(),
    );

    m.insert(
        "cmd.todo.help_info".into(),
        "  todo info [N]           แสดงงานทั้งหมดหรืองาน N".into(),
    );

    m.insert(
        "cmd.todo.help_rm".into(),
        "  todo rm <N>             ลบงาน N".into(),
    );

    m.insert(
        "cmd.todo.help_show".into(),
        "  todo                    แสดงงานทั้งหมด".into(),
    );

    m.insert(
        "cmd.todo.info_usage".into(),
        "การใช้งาน: todo info <N>  — N ต้องเป็นตัวเลข".into(),
    );

    m.insert(
        "cmd.todo.invalid_position".into(),
        "ตำแหน่งไม่ถูกต้อง: '{}' ต้องเป็นตัวเลข".into(),
    );

    m.insert("cmd.todo.list_title".into(), "รายการงาน (TODO):".into());

    m.insert(
        "cmd.todo.missing_position".into(),
        "ไม่พบหมายเลขตำแหน่งหลัง '->'".into(),
    );

    m.insert(
        "cmd.todo.missing_quote".into(),
        "ไม่พบเครื่องหมายคำพูดปิด".into(),
    );

    m.insert(
        "cmd.todo.must_be_quoted".into(),
        "ชื่องานต้องอยู่ในเครื่องหมายคำพูด".into(),
    );

    m.insert("cmd.todo.no_tasks".into(), "  {}".into());

    m.insert("cmd.todo.none_to_clear".into(), "ไม่มีงานต้องล้าง.".into());

    m.insert(
        "cmd.todo.one_based".into(),
        "หมายเลขงานเริ่มจาก 1 (ขั้นต่ำ 1)".into(),
    );

    m.insert(
        "cmd.todo.position_one_based".into(),
        "ตำแหน่งต้องเริ่มจาก 1 (ขั้นต่ำ 1)".into(),
    );

    m.insert("cmd.todo.rm_usage".into(), "การใช้งาน: todo rm <N>".into());

    m.insert(
        "cmd.todo.rm_usage_num".into(),
        "การใช้งาน: todo rm <N>  — N ต้องเป็นตัวเลข".into(),
    );

    m.insert("cmd.todo.task_added".into(), "เพิ่มงานแล้ว: {}".into());

    m.insert(
        "cmd.todo.task_not_found".into(),
        "ไม่พบงาน #{} (มี {} งาน)".into(),
    );

    m.insert("cmd.todo.task_removed".into(), "ลบงานแล้ว: {}".into());

    m.insert(
        "cmd.todo.unexpected_text".into(),
        "ข้อความที่ไม่คาดคิดหลังชื่องาน: '{}'".into(),
    );

    m.insert(
        "cmd.todo.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อยของ todo: '{}'".into(),
    );

    m.insert("cmd.todo.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.todo.help_done".into(),
        "  todo done <N>           ทำเครื่องหมายงาน N ว่าเสร็จ".into(),
    );

    m.insert(
        "cmd.todo.help_undo".into(),
        "  todo undo <N>           เปิดงาน N ใหม่".into(),
    );

    m.insert(
        "cmd.todo.help_edit".into(),
        "  todo edit <N> \"ชื่อ\"    เปลี่ยนชื่อ งาน N".into(),
    );

    m.insert(
        "cmd.todo.done_usage".into(),
        "การใช้งาน: todo done <N>".into(),
    );

    m.insert(
        "cmd.todo.undo_usage".into(),
        "การใช้งาน: todo undo <N>".into(),
    );

    m.insert(
        "cmd.todo.edit_usage".into(),
        "การใช้งาน: todo edit <N> \"ชื่อใหม่\"".into(),
    );

    m.insert("cmd.todo.task_done".into(), "งานเสร็จแล้ว: {}".into());

    m.insert("cmd.todo.task_undone".into(), "เปิดงานใหม่แล้ว: {}".into());

    m.insert("cmd.todo.task_edited".into(), "อัปเดตงานแล้ว: {}".into());

    m.insert("cmd.todo.already_done".into(), "งาน #{} เสร็จแล้ว".into());

    m.insert("cmd.todo.already_open".into(), "งาน #{} ยังเปิดอยู่".into());

    m.insert(
        "cmd.todo.help_format".into(),
        "  config todo format <fmt>  ตั้งรูปแบบไฟล์ todo (txt|md|json|toml|yaml|html)".into(),
    );

    m.insert(
        "cmd.todo.unknown_format".into(),
        "รูปแบบ todo ไม่รู้จัก: '{}' (ใช้ txt|md|json|toml|yaml|html)".into(),
    );

    m.insert(
        "cmd.todo.parse_failed".into(),
        "ไม่สามารถวิเคราะห์ไฟล์ todo {} ได้".into(),
    );
}
