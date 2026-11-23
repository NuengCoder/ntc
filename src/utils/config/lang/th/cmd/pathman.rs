use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.pathman.access_denied".into(),
        "การเข้าถึงถูกปฏิเสธ — ต้องใช้สิทธิ์ผู้ดูแลระบบ.".into(),
    );

    m.insert(
        "cmd.pathman.added_to_path".into(),
        "เพิ่มไปที่ {} แล้ว: {}".into(),
    );

    m.insert(
        "cmd.pathman.already_in_path".into(),
        "อยู่ใน PATH แล้ว: {}".into(),
    );

    m.insert("cmd.pathman.empty_path".into(), "(ว่าง)".into());

    m.insert(
        "cmd.pathman.example_add".into(),
        "  {} add ~/bin".into()
    );

    m.insert(
        "cmd.pathman.example_add_cwd".into(),
        "  {} add --cwd".into(),
    );

    m.insert(
        "cmd.pathman.example_move".into(),
        "  {} move 3 1               ย้ายรายการ #3 ขึ้นไปด้านบน".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_idx".into(),
        "  {} rm 3                    ลบรายการ #3 ออกจากรายการ".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_path".into(),
        "  {} rm ~/old-tools".into()
    );

    m.insert("cmd.pathman.example_show".into(), "  {} show".into());

    m.insert("cmd.pathman.examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.pathman.help_add".into(),
        "  {} add <เส้นทาง>               เพิ่มไดเรกทอรีไปยัง PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_add_cwd".into(),
        "  {} add --cwd | --here       เพิ่มไดเรกทอรีปัจจุบันไปยัง PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_move".into(),
        "  {} move <n> <m>             ย้ายรายการ #n ไปยังตำแหน่ง #m".into(),
    );

    m.insert(
        "cmd.pathman.help_rm".into(),
        "  {} rm <เส้นทางหรือหมายเลข>      ลบออกจาก PATH (ตามเส้นทางหรือดัชนี)".into(),
    );

    m.insert(
        "cmd.pathman.help_show".into(),
        "  {} show                     แสดงรายการ PATH พร้อมดัชนี".into(),
    );

    m.insert(
        "cmd.pathman.idx_out_of_range".into(),
        "ดัชนีอยู่นอกช่วง: {} (1-{})".into(),
    );

    m.insert(
        "cmd.pathman.indices_numbers".into(),
        "ดัชนีต้องเป็นตัวเลข".into(),
    );

    m.insert(
        "cmd.pathman.mode_admin_hint".into(),
        "จากนั้นเรียกคำสั่งอีกครั้ง. หรือใช้: mode admin".into(),
    );

    m.insert(
        "cmd.pathman.move_usage".into(),
        "การใช้งาน: {} move <n> <m>".into(),
    );

    m.insert(
        "cmd.pathman.moved".into(),
        "ย้ายไปยังตำแหน่ง #{} ใน {} แล้ว: {}".into(),
    );

    m.insert(
        "cmd.pathman.new_terminal".into(),
        "การเปลี่ยนแปลงจะมีผลกับหน้าต่างเทอร์มินัลใหม่.".into(),
    );

    m.insert(
        "cmd.pathman.not_found_in_path".into(),
        "ไม่พบใน PATH: {}".into(),
    );

    m.insert(
        "cmd.pathman.path_update_fail".into(),
        "ไม่สามารถอัปเดต PATH: {}".into(),
    );

    m.insert(
        "cmd.pathman.reg_query_fail".into(),
        "ไม่สามารถสอบถามรีจิสทรี: {}".into(),
    );

    m.insert(
        "cmd.pathman.reg_write_fail".into(),
        "ไม่สามารถเขียนรีจิสทรี: {}".into(),
    );

    m.insert(
        "cmd.pathman.removed_from_path".into(),
        "ลบออกจาก {} แล้ว: {}".into(),
    );

    m.insert(
        "cmd.pathman.rm_usage".into(),
        "การใช้งาน: usrpath rm <เส้นทางหรือหมายเลข>".into(),
    );

    m.insert(
        "cmd.pathman.run_as_admin".into(),
        "เรียกเทอร์มินัลของคุณในฐานะ Administrator (คลิกขวา → \\\"Run as administrator\\\"),".into(),
    );

    m.insert(
        "cmd.pathman.same_source_target".into(),
        "แหล่งต้นทางและเป้าหมายเป็นชื่อเดียวกัน \u{2014} ไม่มีอะไรต้องย้าย".into(),
    );

    m.insert("cmd.pathman.system_level".into(), "System".into());

    m.insert(
        "cmd.pathman.total_entries".into(),
        "  รวม: {} รายการ".into(),
    );

    m.insert("cmd.pathman.usage_title".into(), "การใช้งาน:".into());

    m.insert("cmd.pathman.user_level".into(), "User".into());

    m.insert(
        "cmd.pathman.new_login_shell".into(),
        "การเปลี่ยนแปลงจะมีผลในเซสชันใหม่.".into(),
    );

    m.insert(
        "cmd.pathman.next_login".into(),
        "บันทึกแล้ว PATH ใหม่จะมีผลเมื่อเข้าสู่ระบบครั้งถัดไป (ออกแล้วเข้าใหม่ หรือเปิด login shell).".into(),
    );

    m.insert(
        "cmd.pathman.platform_unsupported".into(),
        "usrpath/syspath ไม่รองรับบนแพลตฟอร์มนี้.".into(),
    );

    m.insert(
        "cmd.pathman.shell_profile_hint".into(),
        "เพิ่มบรรทัดนี้ในโปรไฟล์ shell ของคุณ: {}".into(),
    );

    m.insert("cmd.pathman.store_file".into(), "  ไฟล์: {}".into());

    m.insert("cmd.pathman.sudo_fail".into(), "เรียก sudo ไม่ได้: {}".into());

    m.insert(
        "cmd.pathman.sudo_write".into(),
        "PATH ของระบบเป็นของ root — กำลังเขียน {} ด้วย sudo …".into(),
    );
}
