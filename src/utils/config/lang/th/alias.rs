use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "alias.ral.add_cmd".into(),
        " ral add <ชื่อ> <คำสั่ง> สร้าง run alias ใหม่".into(),
    );

    m.insert("alias.ral.alias_count".into(), "คุณมี alias {} ตัว:".into());

    m.insert("alias.ral.and_more".into(), "... และอีก {} ตัว".into());

    m.insert(
        "alias.ral.array_cmd".into(),
        " ral add <ชื่อ>(x[],y) <cmd> สร้าง alias ที่มีพารามิเตอร์แบบ array (ใช้ $x[].ext หรือ ${{x}}[].ext)"
            .into(),
    );

    m.insert("alias.ral.cls_cancelled".into(), "ยกเลิกการล้าง.".into());

    m.insert(
        "alias.ral.cls_cmd".into(),
        " ral cls ล้าง alias ทั้งหมด (ถามยืนยัน)".into(),
    );

    m.insert(
        "alias.ral.cls_prompt".into(),
        "แน่ใจหรือ? พิมพ์ 'yes' เพื่อยืนยัน: ".into(),
    );

    m.insert(
        "alias.ral.cls_warning".into(),
        "\u{26A0}\u{FE0F} คำเตือน: การดำเนินการนี้จะลบ alias ทั้งหมด!".into(),
    );

    m.insert("alias.ral.config_path".into(), " กำหนดค่า: {}".into());

    m.insert(
        "alias.ral.create_failed".into(),
        "ไม่สามารถสร้างไฟล์: {}".into(),
    );

    m.insert(
        "alias.ral.default_cmd".into(),
        " ral add <ชื่อ>({{x=a,e=txt}}) <cmd> สร้าง alias ที่มีค่าเริ่มต้น".into(),
    );

    m.insert(
        "alias.ral.edit_cmd".into(),
        " ral edit <ชื่อ> <คำสั่ง> อัปเดต alias ที่มีอยู่".into(),
    );

    m.insert(
        "alias.ral.empty".into(),
        "ยังไม่มี run alias ที่กำหนด ใช้ 'ral add <ชื่อ> <คำสั่ง>'".into(),
    );

    m.insert("alias.ral.examples_header".into(), "ตัวอย่าง:".into());

    m.insert(
        "alias.ral.export_all_cmd".into(),
        " ral export --all <ชื่อ> ส่งออก alias ทั้งหมดไปยัง <ชื่อ>.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_import_header".into(),
        "ส่งออก/นำเข้า:".into(),
    );

    m.insert(
        "alias.ral.export_name_empty".into(),
        "ชื่อสำหรับส่งออกต้องไม่ว่างเปล่า.".into(),
    );

    m.insert(
        "alias.ral.export_select_cmd".into(),
        " ral export --select <ชื่อ> เลือก alias เพื่อส่งออกไปยัง <ชื่อ>.ntc.ral".into(),
    );

    m.insert("alias.ral.file_exists".into(), "ไฟล์มีอยู่แล้ว: {}".into());

    m.insert(
        "alias.ral.import_cmd".into(),
        " ral import <ไฟล์> นำเข้า alias จากไฟล์ .ntc.ral".into(),
    );

    m.insert(
        "alias.ral.info_cmd".into(),
        " ral info <ชื่อ> [--edit] แสดงรายละเอียด alias (--edit คัดลอกคำสั่งแก้ไขไปยังคลิปบอร์ด)".into(),
    );

    m.insert("alias.ral.info_command".into(), "คำสั่ง".into());

    m.insert("alias.ral.info_name".into(), "ชื่อ".into());

    m.insert("alias.ral.info_none".into(), "ไม่มี".into());

    m.insert("alias.ral.info_params".into(), "พารามิเตอร์".into());

    m.insert("alias.ral.info_hints".into(), "คำใบ้การเติม".into());

    m.insert("alias.ral.hints_saved".into(), "คำใบ้การเติม: {}".into());

    m.insert(
        "alias.ral.hint_error".into(),
        "คำใบ้ผิด ({}): ไม่ได้บันทึก alias".into(),
    );

    m.insert("alias.ral.hint_usage".into(), "คำใบ้หน้าตาแบบ: #x --hint-dir  #y --hint-tp --hint-dir  เลือกแบบ: #x --select-dir  #x --select-ext rs,txt  ค่าตรง ๆ: #x --hint [8080,5000]  รายการที่ตั้งชื่อ: #x --hint-n port (ดู ral hintn)  แฟล็กใหม่: --hint-ext-name [list] | --hint-file-name | --select-ext-name [list] | --select-file-name".into());

    m.insert(
        "alias.ral.hint_cmd".into(),
        " ral add <ชื่อ> <คำสั่ง> #x --hint-dir เติมพารามิเตอร์ x เป็นไดเรกทอรี".into(),
    );

    m.insert(
        "alias.ral.select_cmd".into(),
        " ral add <ชื่อ> <คำสั่ง> #x --select-dir เลือก x แบบโต้ตอบเมื่อขาดอาร์กิวเมนต์".into(),
    );

    m.insert(
        "alias.ral.hintn_cmd".into(),
        " ral hintn add|rm|rnm|info|ls จัดการรายการค่าที่ตั้งชื่อไว้ ใช้กับ #x --hint-n <รายการ>".into(),
    );

    m.insert("alias.ral.usage_hintn".into(), "วิธีใช้: ral hintn add <ชื่อ> [ค่า] | ral hintn rm <ชื่อ>[, ...] [--force] | ral hintn rnm <เก่า> to <ใหม่> | ral hintn info <ชื่อ> | ral hintn ls".into());

    m.insert(
        "alias.ral.usage_hintn_add".into(),
        "วิธีใช้: ral hintn add <ชื่อ> [ค่า, ...] (ค่ามีช่องว่างให้ใส่เครื่องหมายคำพูด: [\"what is this\", ok])"
            .into(),
    );

    m.insert(
        "alias.ral.usage_hintn_rm".into(),
        "วิธีใช้: ral hintn rm <ชื่อ>[, ...] [--force]".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_rnm".into(),
        "วิธีใช้: ral hintn rnm <ชื่อเก่า> to <ชื่อใหม่>".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_info".into(),
        "วิธีใช้: ral hintn info <ชื่อ>".into(),
    );

    m.insert(
        "alias.ral.hintn_added".into(),
        "บันทึกรายการ '{}' แล้ว ({} ค่า): {}".into(),
    );

    m.insert("alias.ral.hintn_removed".into(), "ลบรายการ '{}' แล้ว".into());

    m.insert("alias.ral.hintn_not_found".into(), "ไม่พบรายการ '{}'".into());

    m.insert(
        "alias.ral.hintn_renamed".into(),
        "เปลี่ยนชื่อรายการ '{}' เป็น '{}' แล้ว".into(),
    );

    m.insert(
        "alias.ral.hintn_empty".into(),
        "ยังไม่มีรายการ ใช้ 'ral hintn add <ชื่อ> [ค่า]'".into(),
    );

    m.insert(
        "alias.ral.hintn_referenced".into(),
        "รายการ '{}' ถูกใช้โดย: {} ใช้ --force เพื่อลบทิ้ง".into(),
    );

    m.insert("alias.ral.hintn_info_title".into(), "📌 ข้อมูลรายการ".into());

    m.insert("alias.ral.hintn_info_name".into(), "ชื่อ".into());

    m.insert("alias.ral.hintn_info_values".into(), "ค่า".into());

    m.insert("alias.ral.hintn_info_used_by".into(), "ถูกใช้โดย".into());

    m.insert("alias.ral.hintn_value_count".into(), "{} ค่า".into());

    m.insert(
        "alias.ral.unknown_hint_list".into(),
        "ไม่รู้จักรายการ '{}'".into(),
    );

    m.insert(
        "alias.ral.hintn_create_hint".into(),
        "สร้างก่อน: ral hintn add {} [ค่า]".into(),
    );

    m.insert(
        "alias.ral.arr_sep_error".into(),
        "error: {} มี {} ตัวอักษรใน array {} ลอง $x[,] หรือ $x[c]".into(),
    );

    m.insert(
        "alias.ral.example_hint_ext".into(),
        "ral add opimg \"image display ${x}\" #x --hint-ext png,jpeg,jpg,ico,bmp".into(),
    );

    m.insert(
        "alias.ral.example_rest".into(),
        "ral add hi \"echo $*{x}\"    # hi \"wow nice\" -> echo wow nice".into(),
    );

    m.insert(
        "alias.ral.example_hint_array".into(),
        "ral add runc \"gcc -o ${o}.exe $x[] && .\\${o}.exe && rmf ${o}.exe\" #x[] --hint-ext c"
            .into(),
    );

    m.insert(
        "alias.ral.example_select_dir".into(),
        "ral add tldsel \"tl dir ${x:-.}\" #x --select-dir".into(),
    );

    m.insert(
        "alias.ral.example_hint_values".into(),
        "ral add srv \"ssh $p\" #p --hint [8080,5050,5000]".into(),
    );

    m.insert(
        "alias.ral.example_hintn".into(),
        "ral hintn add port [8080,5050,5000] + ral add srvn \"ssh $p\" #p --hint-n port".into(),
    );

    m.insert(
        "alias.ral.example_hint_sep".into(),
        "array join: $x[,] บน [a,b] ได้ \"a , b\"".into(),
    );

    m.insert("alias.ral.info_source".into(), "แหล่งที่มา".into());

    m.insert("alias.ral.info_source_global".into(), "global".into());

    m.insert(
        "alias.ral.info_source_local".into(),
        "local (ntconfig.toml)".into(),
    );

    m.insert("alias.ral.info_title".into(), "\u{1F4CC} ข้อมูล Alias".into());

    m.insert(
        "alias.ral.info_edit_copied".into(),
        "คัดลอกคำสั่งแก้ไขไปยังคลิปบอร์ดแล้ว:".into(),
    );

    m.insert(
        "alias.ral.info_edit_hint".into(),
        "วางมัน ปรับแก้คำสั่ง แล้วกด Enter.".into(),
    );

    m.insert(
        "alias.ral.invalid_name".into(),
        "ชื่อ alias ไม่ถูกต้อง: '{}'".into(),
    );

    m.insert("alias.ral.invalid_suffix".into(), "{} (ไม่ถูกต้อง)".into());

    m.insert(
        "alias.ral.list_cmd".into(),
        " ral list แสดง alias ทั้งหมด".into(),
    );

    m.insert(
        "alias.ral.make_cmd".into(),
        " ral make [ชื่อ] สร้างเทมเพลต alias ในโฟลเดอร์ปัจจุบัน".into(),
    );

    m.insert("alias.ral.name_rule_at".into(), " - ขึ้นต้นด้วย @ หรือ #".into());

    m.insert(
        "alias.ral.name_rule_reserved".into(),
        " - เป็นคำสั่งที่สงวนไว้ (go, view, txt ฯลฯ)".into(),
    );

    m.insert("alias.ral.name_rules".into(), "ชื่อ alias ไม่สามารถ:".into());

    m.insert("alias.ral.no_aliases".into(), "ไม่มี run alias ให้ล้าง.".into());

    m.insert("alias.ral.not_found".into(), "ไม่พบ alias '{}'".into());

    m.insert("alias.ral.now_run".into(), "ตอนนี้คุณสามารถรัน: {}".into());

    m.insert(
        "alias.ral.now_run_arg".into(),
        "ตอนนี้คุณสามารถรัน: {}({})".into(),
    );

    m.insert(
        "alias.ral.now_run_defaults".into(),
        "ตอนนี้คุณสามารถรัน: {}(<arg>) (ค่าเริ่มต้น: {})".into(),
    );

    m.insert(
        "alias.ral.param_cmd".into(),
        " ral add <ชื่อ>(x) <คำสั่ง> สร้าง alias แบบมีพารามิเตอร์ (ใช้ $x หรือ ${{x}} ในคำสั่ง)".into(),
    );

    m.insert(
        "alias.ral.rest_cmd".into(),
        " ral add <ชื่อ>(x) <คำสั่ง> ใช้ $*{x}  รับหนึ่งอาร์กิวเมนต์ (คงช่องว่าง, ตัดเครื่องหมายคำพูด)".into(),
    );

    m.insert(
        "alias.ral.renamed".into(),
        "เปลี่ยนชื่อ alias '{}' เป็น '{}'".into(),
    );

    m.insert(
        "alias.ral.rm_cmd".into(),
        " ral rm <ชื่อ>[, <ชื่อ>, ...] ลบ alias".into(),
    );

    m.insert(
        "alias.ral.rnm_cmd".into(),
        " ral rnm <เก่า> to <ใหม่> เปลี่ยนชื่อ alias".into(),
    );

    m.insert(
        "alias.ral.template_created".into(),
        "สร้างเทมเพลต ral แล้ว: {}".into(),
    );

    m.insert(
        "alias.ral.template_info".into(),
        "นี่คือไฟล์เทมเพลตท้องถิ่น ใช้ 'ral import' เพื่อเพิ่ม alias ไปยัง config ของคุณ".into(),
    );

    m.insert("alias.ral.title".into(), "คำสั่ง Run Alias (ral):".into());

    m.insert(
        "alias.ral.type_ral_help".into(),
        "พิมพ์ 'ral' เพื่อดูความช่วยเหลือ.".into(),
    );

    m.insert(
        "alias.ral.unknown_flag".into(),
        "flag การส่งออกไม่รู้จัก: {}. ใช้ --all หรือ --select".into(),
    );

    m.insert(
        "alias.ral.unknown_subcmd".into(),
        "คำสั่งย่อย ral ไม่รู้จัก: {}".into(),
    );

    m.insert(
        "alias.ral.usage_add".into(),
        "การใช้งาน: ral add <ชื่อ> <คำสั่ง>".into(),
    );

    m.insert(
        "alias.ral.usage_edit".into(),
        "การใช้งาน: ral edit <ชื่อ> <คำสั่งใหม่>".into(),
    );

    m.insert(
        "alias.ral.usage_export".into(),
        "การใช้งาน: ral export --all <ชื่อ> | ral export --select <ชื่อ>".into(),
    );

    m.insert(
        "alias.ral.usage_hint".into(),
        "การใช้งาน: <alias> หรือ <alias>(<arg>) สำหรับ alias แบบมีพารามิเตอร์".into(),
    );

    m.insert(
        "alias.ral.usage_import".into(),
        "การใช้งาน: ral import <ไฟล์>".into(),
    );

    m.insert(
        "alias.ral.usage_info".into(),
        "การใช้งาน: ral info <alias> [--edit]".into(),
    );

    m.insert(
        "alias.ral.usage_rm".into(),
        "การใช้งาน: ral rm <ชื่อ>[, <ชื่อ>, ...]".into(),
    );

    m.insert(
        "alias.ral.usage_rnm".into(),
        "การใช้งาน: ral rnm <ชื่อเก่า> to <ชื่อใหม่>".into(),
    );

    m.insert("alias.ral.usage_with_run".into(), "การใช้งานกับ run:".into());

    m.insert(
        "alias.ral.run_example1".into(),
        "  run btr              # รัน: cargo build --release".into(),
    );

    m.insert(
        "alias.ral.run_example2".into(),
        "  run py               # รัน: python test.py".into(),
    );

    m.insert(
        "alias.ral.run_example3".into(),
        "  run_file(hello)      # รัน: python hello.py".into(),
    );

    m.insert(
        "alias.ral.run_example4".into(),
        "  runc [add , minus , mul] math  # รัน: cls && gcc -o math.exe add.c minus.c mul.c && ..."
            .into(),
    );

    m.insert(
        "alias.ral.export_example1".into(),
        "  ral export --all myaliases       # สร้าง myaliases.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_example2".into(),
        "  ral export --select myaliases    # เลือกและส่งออกแบบโต้ตอบ".into(),
    );

    m.insert(
        "alias.ral.export_example3".into(),
        "  ral import myaliases.ntc.ral     # นำเข้าไปยัง config ปัจจุบัน".into(),
    );

    m.insert("alias.run.examples_header".into(), "ตัวอย่าง:".into());

    m.insert(
        "alias.run.example_alias".into(),
        "  run btr                     # รัน alias".into(),
    );

    m.insert(
        "alias.run.example_alias_args".into(),
        "  run py test.py              # รัน alias พร้อมอาร์กิวเมนต์".into(),
    );

    m.insert(
        "alias.run.example_param".into(),
        "  run_file(hello)             # รัน alias แบบมีพารามิเตอร์".into(),
    );

    m.insert(
        "alias.run.example_real".into(),
        "  run python --version        # รันคำสั่งจริง".into(),
    );

    m.insert(
        "alias.run.usage".into(),
        "การใช้งาน: run <command|alias> [args...]".into(),
    );

    m.insert("alias.ral.section_global".into(), "[global]".into());

    m.insert("alias.ral.section_local".into(), "[local]".into());

    m.insert(
        "alias.ral.no_local".into(),
        "(ไม่มี config local ในไดเรกทอรีนี้)".into(),
    );

    m.insert("alias.ral.none".into(), "(ไม่มี)".into());

    m.insert("alias.ral.shadowed".into(), "(ถูก local บัง)".into());

    m.insert(
        "alias.ral.shadowed_global".into(),
        "รายการ Global (ถูก local บัง):".into(),
    );

    m.insert(
        "alias.ral.global_active_again".into(),
        "alias global '{}' ยังคงอยู่ และทำงานอีกครั้งแล้ว".into(),
    );

    m.insert(
        "alias.ral.global_only_hint".into(),
        "alias '{}' ถูกกำหนดไว้เฉพาะใน config global ใช้ 'ral rm -g {}' เพื่อลบออกจาก config global"
            .into(),
    );

    m.insert(
        "alias.ral.local_priority_note".into(),
        "หมายเหตุ: alias local '{}' ใน ntconfig.toml มีความสำคัญกว่า alias global นี้".into(),
    );

    m.insert("alias.ral.scope_local".into(), "config local".into());

    m.insert("alias.ral.scope_global".into(), "config global".into());

    m.insert("alias.ral.cls_target".into(), "เป้าหมาย: {}".into());
}
