use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.config.local.init_all_removed".into(), "'local init --all' ถูกลบแล้ว - local config ไม่สามารถคัดลอกการตั้งค่า global ได้อีกต่อไป ใช้ 'local init' เพื่อสร้างเทมเพลต".into());

    m.insert(
        "cmd.config.local.precedence_hint".into(),
        "รายการ local มีความสำคัญกว่า รายการ global ยังทำงานต่อ และ config global จะไม่ถูกแก้ไข".into(),
    );

    m.insert("cmd.config.deinit.confirm".into(), "ลบ {}? (y/N): ".into());

    m.insert(
        "cmd.config.deinit.local_not_found".into(),
        "ไม่พบ ntconfig.toml ข้ามไป.".into(),
    );

    m.insert(
        "cmd.config.deinit.local_remove_failed".into(),
        "ไม่สามารถลบ ntconfig.toml: {}".into(),
    );

    m.insert(
        "cmd.config.deinit.local_removed".into(),
        "ลบ ntconfig.toml แล้ว.".into(),
    );

    m.insert(
        "cmd.config.deinit.not_found".into(),
        "ไม่พบ {} ข้ามไป.".into(),
    );

    m.insert(
        "cmd.config.deinit.remove_failed".into(),
        "ไม่สามารถลบ {}: {}".into(),
    );

    m.insert("cmd.config.deinit.removed".into(), "ลบ {} แล้ว".into());

    m.insert("cmd.config.deinit.skipped".into(), "ข้ามไป.".into());

    m.insert(
        "cmd.config.deinit.usage".into(),
        "การใช้งาน: deinit [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.deinit.opencode_not_found".into(),
        "ไม่พบ scaffold .opencode ข้ามไป.".into(),
    );

    m.insert(
        "cmd.config.example_depth".into(),
        "  config depth 4  # ตั้งค่าความลึกของต้นไม้สูงสุดเป็น 4".into(),
    );

    m.insert(
        "cmd.config.example_history".into(),
        "  config history off  # ปิดใช้งานประวัติ".into(),
    );

    m.insert(
        "cmd.config.example_linenumbers".into(),
        "  config linenumbers ON  # เปิดหมายเลขบรรทัดในการแสดงไฟล์".into(),
    );

    m.insert(
        "cmd.config.example_output".into(),
        "  config output C:\\reports  # ตั้งค่าเส้นทางผลลัพธ์".into(),
    );

    m.insert(
        "cmd.config.example_watch".into(),
        "  config watch ON  # เปิดใช้งานตัวเฝ้าดูไฟล์".into(),
    );

    m.insert("cmd.config.examples_header".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.config.init.create_failed".into(),
        "ไม่สามารถสร้าง {}: {}".into(),
    );

    m.insert("cmd.config.init.created".into(), "สร้าง {} แล้ว".into());

    m.insert(
        "cmd.config.init.exists_warn".into(),
        "{} มีอยู่แล้ว ข้ามไป.".into(),
    );

    m.insert(
        "cmd.config.init.local_exists".into(),
        "ntconfig.toml มีอยู่แล้ว ข้ามไป.".into(),
    );

    m.insert(
        "cmd.config.init.usage".into(),
        "การใช้งาน: init [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.init.opencode_ready".into(),
        "OpenCode scaffold พร้อมแล้ว: {}".into(),
    );

    m.insert("cmd.config.lang.active".into(), "{} (ใช้งานอยู่)".into());

    m.insert("cmd.config.lang.available".into(), "ภาษาที่มี:".into());

    m.insert(
        "cmd.config.lang.coming_soon".into(),
        "{} (ตั้งค่าแล้ว จะมีเร็วๆ นี้)".into(),
    );

    m.insert("cmd.config.lang.set".into(), "ตั้งภาษเป็น: {}".into());

    m.insert(
        "cmd.config.lang.set_coming_soon".into(),
        "ตั้งภาษาเป็น: {} (จะมีเร็วๆ นี้)".into(),
    );

    m.insert(
        "cmd.config.lang.unsupported".into(),
        "ไม่รองรับภาษา '{}'. ภาษาที่มี: {}".into(),
    );

    m.insert(
        "cmd.config.local.create_failed".into(),
        "ไม่สามารถสร้าง ntconfig.toml: {}".into(),
    );

    m.insert(
        "cmd.config.local.created_template".into(),
        "สร้างเทมเพลต ntconfig.toml ใน: {}".into(),
    );

    m.insert(
        "cmd.config.local.exists_warning".into(),
        "\u{26a0}\u{fe0f} ntconfig.toml มีอยู่แล้ว!".into(),
    );

    m.insert(
        "cmd.config.local.init_cancelled".into(),
        "ยกเลิกการเริ่มต้น.".into(),
    );

    m.insert(
        "cmd.config.local.overwrite_prompt".into(),
        "เขียนทับ? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.local.template_edit_hint".into(),
        " แก้ไขไฟล์และลบ '#' ออกจากบรรทัดที่ต้องการเปิดใช้".into(),
    );

    m.insert(
        "cmd.config.local.template_hint".into(),
        "\u{1f4dd} นี่คือ COMMENTED TEMPLATE - ยังไม่มีการตั้งค่าที่ใช้งานอยู่".into(),
    );

    m.insert(
        "cmd.config.local.tip_global".into(),
        "\u{1f4a1} เคล็ดลับ: การตั้งค่าระดับ global (teleports, เส้นทางผลลัพธ์, ความลึก) ยังคงเป็น global"
            .into(),
    );

    m.insert(
        "cmd.config.local_deinit.cancelled".into(),
        "ยกเลิกการลบการกำหนดค่า.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.confirm".into(),
        "ลบ ntconfig.toml? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.local_deinit.done".into(),
        "ลบ ntconfig.toml แล้ว. ตอนนี้การกำหนดค่า global เป็นการกำหนดค่าเดียวที่ใช้งานอยู่.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.global_safe".into(),
        "การกำหนดค่า global จะไม่ได้รับผลกระทบ.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.lose_settings".into(),
        "การตั้งค่า ignore/care ในเครื่องและ run alias จะหายไป.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.not_found".into(),
        "ไม่พบ ntconfig.toml ในไดเรกทอรีปัจจุบัน.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.remove_failed".into(),
        "ไม่สามารถลบ ntconfig.toml: {}".into(),
    );

    m.insert(
        "cmd.config.local_deinit.title".into(),
        "\u{26A0}\u{FE0F} การลบการกำหนดค่า LOCAL".into(),
    );

    m.insert(
        "cmd.config.local_deinit.will_delete".into(),
        "การดำเนินการนี้จะลบ: {}".into(),
    );

    m.insert(
        "cmd.config.opencg.created".into(),
        "สร้างไฟล์การกำหนดค่าที่: {}".into(),
    );

    m.insert(
        "cmd.config.opencg.opening".into(),
        "กำลังเปิดการกำหนดค่า: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.disabled".into(),
        "ปิดการใช้งานปุ่มลัด QSC แล้ว.".into(),
    );

    m.insert(
        "cmd.config.qsc.hotkey_set".into(),
        "ตั้งปุ่มลัด QSC เป็น: {}".into(),
    );

    m.insert("cmd.config.qsc.hotkey_status".into(), "ปุ่มลัด QSC: {}".into());

    m.insert(
        "cmd.config.qsc.invalid_format".into(),
        "รูปแบบ QSC ไม่ถูกต้อง: '{}'. ใช้รูปแบบ '#c #s 0'.".into(),
    );

    m.insert("cmd.config.qsc.not_configured".into(), "ยังไม่ได้ตั้งค่า".into());

    m.insert(
        "cmd.config.qsc.usage1".into(),
        "การใช้งาน: config qsc \"#c 0\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage2".into(),
        "  config qsc \"#c #s a\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage3".into(),
        "  config qsc \"#c #s #a F1\"".into(),
    );

    m.insert("cmd.config.qsc.usage4".into(), "  config qsc off".into());

    m.insert(
        "cmd.config.resetcg.backup_created".into(),
        "สร้างข้อมูลสำรองแล้ว: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_failed".into(),
        "ไม่สามารถสร้างข้อมูลสำรอง: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_info".into(),
        "บันทึกข้อมูลสำรองแล้ว. ใช้ 'restorecg' เพื่อคืนค่าหากจำเป็น.".into(),
    );

    m.insert(
        "cmd.config.resetcg.cancelled".into(),
        "ยกเลิกการรีเซ็ต.".into(),
    );

    m.insert(
        "cmd.config.resetcg.config_file_label".into(),
        "ไฟล์การกำหนดค่า:".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_aliases".into(),
        "   \u{2022} Run aliases \u{2192} ล้างแล้ว".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_custom_ignores".into(),
        "   \u{2022} การละเว้นแบบกำหนดเอง \u{2192} ลบแล้ว".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_depth".into(),
        "   \u{2022} ความลึกสูงสุด \u{2192} 2".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_history".into(),
        "   \u{2022} ประวัติ \u{2192} OFF".into(),
    );

    m.insert("cmd.config.resetcg.default_ignored_dirs".into(), "   \u{2022} ไดเรกทอรีที่ถูกละเว้น \u{2192} target, build, venv, node_modules, installer, logs, .git".into());

    m.insert(
        "cmd.config.resetcg.default_linenumbers".into(),
        "   \u{2022} หมายเลขบรรทัด \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_output".into(),
        "   \u{2022} เส้นทางผลลัพธ์ \u{2192} Desktop".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_teleports".into(),
        "   \u{2022} Teleports \u{2192} ล้างแล้ว".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_threads".into(),
        "   \u{2022} Threads \u{2192} 4".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_watcher".into(),
        "   \u{2022} ตัวเฝ้าดูไฟล์ \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.not_found".into(),
        "ไม่พบไฟล์การกำหนดค่า. ไม่มีอะไรต้องรีเซ็ต.".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_backup".into(),
        "   \u{2022} พิมพ์ 'backup' - สร้างข้อมูลสำรองก่อนรีเซ็ต".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_no".into(),
        "   \u{2022} พิมพ์ 'no' - ยกเลิก".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_yes".into(),
        "   \u{2022} พิมพ์ 'yes' - รีเซ็ตการกำหนดค่า (ไม่สำรองข้อมูล)".into(),
    );

    m.insert("cmd.config.resetcg.options_label".into(), "ตัวเลือก:".into());

    m.insert(
        "cmd.config.resetcg.reset_done".into(),
        "รีเซ็ตการกำหนดค่าเป็นค่าเริ่มต้นแล้ว!".into(),
    );

    m.insert(
        "cmd.config.resetcg.serialize_failed".into(),
        "ไม่สามารถแปลงการกำหนดค่าเริ่มต้นเป็นอนุกรมได้".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_desc".into(),
        "การดำเนินการนี้จะรีเซ็ตการกำหนดค่าของคุณให้เป็นค่าเริ่มต้น:".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_title".into(),
        "\u{26A0}\u{FE0F} คำเตือนการรีเซ็ตการกำหนดค่า".into(),
    );

    m.insert(
        "cmd.config.resetcg.write_failed".into(),
        "ไม่สามารถเขียนการกำหนดค่า: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.your_choice".into(),
        "ตัวเลือกของคุณ: ".into(),
    );

    m.insert(
        "cmd.config.restorecg.backed_up".into(),
        "สำรองการกำหนดค่าปัจจุบันไปที่: {}".into(),
    );

    m.insert("cmd.config.restorecg.cancel_opt".into(), "0. ยกเลิก".into());

    m.insert(
        "cmd.config.restorecg.cancelled".into(),
        "ยกเลิกการคืนค่า.".into(),
    );

    m.insert(
        "cmd.config.restorecg.failed".into(),
        "ไม่สามารถคืนค่า: {}".into(),
    );

    m.insert(
        "cmd.config.restorecg.invalid".into(),
        "การเลือกไม่ถูกต้อง.".into(),
    );

    m.insert("cmd.config.restorecg.none".into(), "ไม่พบไฟล์สำรอง.".into());

    m.insert(
        "cmd.config.restorecg.reloaded".into(),
        "โหลดการกำหนดค่าใหม่แล้ว! เรียก 'showcg' เพื่อดูการตั้งค่าที่คืนค่า.".into(),
    );

    m.insert(
        "cmd.config.restorecg.restored".into(),
        "คืนค่าการกำหนดค่าจาก: {}".into(),
    );

    m.insert(
        "cmd.config.restorecg.title".into(),
        "\u{1F4CB} ไฟล์สำรองที่พร้อมใช้งาน:".into(),
    );

    m.insert("cmd.config.mgmt_header".into(), "การจัดการ:".into());

    m.insert("cmd.config.settings_header".into(), "การตั้งค่า:".into());

    m.insert(
        "cmd.config.showcg.lang_coming_soon".into(),
        "{} (จะมีเร็วๆ นี้)".into(),
    );

    m.insert("cmd.config.showcg.lang_label".into(), "ภาษา:".into());

    m.insert("cmd.config.showcg.qsc_label".into(), "ปุ่มลัด QSC:".into());

    m.insert(
        "cmd.config.title".into(),
        "config \u{2014} ดูหรือเปลี่ยนการตั้งค่าการกำหนดค่า".into(),
    );

    m.insert(
        "cmd.config.watch.alias_not_found".into(),
        "ไม่พบ alias '{}'. ใช้ 'ral list' เพื่อดู alias.".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_cleared".into(),
        "ล้าง alias ตัวเรียกใช้ของ watch แล้ว.".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_label".into(),
        "ตัวเรียกใช้ alias:".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_off_usage".into(),
        "config watch trigger off  ปิดการทำงานอัตโนมัติ".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_set".into(),
        "ตั้งตัวเรียกใช้ watch เป็น alias: '{}'".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_usage".into(),
        "config watch trigger <alias> เรียก alias อัตโนมัติเมื่อมีการเปลี่ยนแปลง".into(),
    );

    m.insert(
        "cmd.config.watch.limit_usage".into(),
        "config watch limit <n> จำนวนแถวที่แสดงในกล่อง watch".into(),
    );

    m.insert("cmd.config.watch.usage".into(), "ใช้: config watch ON | config watch OFF | config watch trigger <alias> | config watch trigger off | config watch limit <n>".into());

    m.insert(
        "cmd.config.watch.usage_line".into(),
        "การใช้งาน: config watch ON|OFF".into(),
    );

    m.insert(
        "cmd.config.serve.title".into(),
        "การตั้งค่า Serve (ค่าเริ่มต้นรายบริการ)".into(),
    );

    m.insert("cmd.config.serve.status".into(), "  {}: {}:{}".into());

    m.insert("cmd.config.serve.usage".into(), "วิธีใช้: serve config [files|http-server|chess] [--port-default <พอร์ต>] [--bind-default <ที่อยู่>] (alias: config serve …)".into());

    m.insert(
        "cmd.config.serve.example_show".into(),
        "  serve config                  แสดงทุกบริการ".into(),
    );

    m.insert(
        "cmd.config.serve.example_show_one".into(),
        "  serve config files            แสดงค่าเริ่มต้น files".into(),
    );

    m.insert(
        "cmd.config.serve.example_set".into(),
        "  serve config files --port-default 9000   ตั้งพอร์ตเริ่มต้น files".into(),
    );

    m.insert(
        "cmd.config.serve.saved".into(),
        "บันทึกค่าเริ่มต้น Serve {} แล้ว: {}:{}".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_service".into(),
        "ไม่รู้จักบริการ '{}' (ใช้ files|http-server|chess)".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_port".into(),
        "พอร์ต '{}' ไม่ถูกต้อง (ใช้ 1-65535)".into(),
    );

    m.insert(
        "cmd.config.serve.empty_bind".into(),
        "--bind-default ขาดค่า".into(),
    );

    m.insert("cmd.config.serve.unknown_option".into(), "ไม่รู้จัก: {}".into());
}
