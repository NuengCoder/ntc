use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.qur.usage".into(),
        "วิธีใช้: qur <glob> [-s \"pattern\"] [-rp \"replacement\"] [-dp] [-d <depth>] [-l <limit>]"
            .into(),
    );

    m.insert(
        "misc.qur.example1".into(),
        " qur *.* # แสดงไฟล์ที่ตรงกับ glob (แบบเรียกซ้ำ)".into(),
    );

    m.insert(
        "misc.qur.example2".into(),
        " qur *.rs -s \"TODO\" # สรุปรายไฟล์".into(),
    );

    m.insert(
        "misc.qur.example3".into(),
        " qur *.* -s \"old\" -dp # แสดงรายละเอียดที่ตรง (เหมือน gs)".into(),
    );

    m.insert(
        "misc.qur.example4".into(),
        " qur *.md -s \"old\" -rp \"new\" -d 3 # ดูตัวอย่าง ยืนยัน แทนที่".into(),
    );

    m.insert("misc.qur.searching".into(), "กำลังค้นหาไฟล์...".into());

    m.insert("misc.qur.no_files".into(), "qur: ไม่มีไฟล์ที่ตรงกับ '{}'".into());

    m.insert("misc.qur.no_matches".into(), "qur: ไม่พบ '{}'".into());

    m.insert("misc.qur.files_header".into(), "ไฟล์ที่ตรงกับคำค้น ({}):".into());

    m.insert(
        "misc.qur.summary_header".into(),
        "ผลที่ตรงกับ \"{}\" ({} บรรทัด ใน {} ไฟล์):".into(),
    );

    m.insert(
        "misc.qur.summary_hint".into(),
        "เพิ่ม -dp เพื่อดูบรรทัดละเอียด (เหมือน gs) หรือ -rp \"new\" เพื่อแทนที่".into(),
    );

    m.insert(
        "misc.qur.replace_preview".into(),
        "แทนที่ \"{}\" ด้วย \"{}\" — {} ตำแหน่ง ใน {} ไฟล์:".into(),
    );

    m.insert(
        "misc.qur.confirm".into(),
        "แทนที่ {} ตำแหน่ง ใน {} ไฟล์? [y/N]:".into(),
    );

    m.insert("misc.qur.cancelled".into(), "ยกเลิกการแทนที่แล้ว".into());

    m.insert(
        "misc.qur.replaced".into(),
        "แทนที่ {} ตำแหน่ง ใน {} ไฟล์แล้ว".into(),
    );

    m.insert(
        "misc.qur.skipped".into(),
        "ข้าม {} ไฟล์ (อ่าน/เขียนไม่สำเร็จ)".into(),
    );

    m.insert(
        "misc.qur.many_results".into(),
        "qur: พบ {} รายการ แสดงเฉพาะ 20,000 รายการแรก โปรดจำกัดการค้นหา".into(),
    );

    m.insert(
        "misc.qur.replace_needs_select".into(),
        "qur: -rp ต้องใช้ -s \"<pattern>\" (เช่น qur *.* -s \"old\" -rp \"new\")".into(),
    );

    m.insert(
        "misc.qur.display_needs_select".into(),
        "qur: -dp ต้องใช้ -s \"<pattern>\" (เช่น qur *.* -s \"foo\" -dp)".into(),
    );

    m.insert(
        "misc.qur.empty_select".into(),
        "qur: -s ต้องมี pattern (เช่น -s \"foo\")".into(),
    );

    m.insert(
        "misc.qur.empty_replace".into(),
        "qur: -rp ต้องมีข้อความแทนที่ (เช่น -rp \"bar\")".into(),
    );

    m.insert(
        "misc.qur.stdin_needs_select".into(),
        "qur: ข้อมูลที่ pipe เข้ามาต้องใช้ -s \"<pattern>\" (เช่น cmd | qur -s \"foo\")".into(),
    );

    m.insert(
        "misc.head.usage".into(),
        "วิธีใช้: head [N] [file] (N ค่าเริ่มต้น 10; ถ้าไม่ระบุไฟล์จะอ่านจาก pipe)".into(),
    );

    m.insert(
        "misc.tail.usage".into(),
        "วิธีใช้: tail [N] [file] (N ค่าเริ่มต้น 10; ถ้าไม่ระบุไฟล์จะอ่านจาก pipe)".into(),
    );

    m.insert(
        "misc.betw.usage".into(),
        "วิธีใช้: betw A:B [file] (A:B เริ่มที่ 1 รวมปลาย; ถ้าไม่ระบุไฟล์จะอ่านจาก pipe)".into(),
    );

    m.insert(
        "misc.window.bad_arg".into(),
        "{}: อาร์กิวเมนต์ที่ไม่คาดคิด '{}'".into(),
    );

    m.insert(
        "misc.window.no_input".into(),
        "{}: ไม่มีอินพุต (ให้ pipe เข้ามาหรือระบุไฟล์)".into(),
    );

    m.insert(
        "misc.sort.usage".into(),
        "วิธีใช้: sort [-r] [-n] [-u] [-f] [file]  (อ่านอินพุตที่ pipe เข้ามาเมื่อไม่ระบุไฟล์)".into(),
    );

    m.insert("misc.sort.bad_arg".into(), "sort: ตัวเลือกที่ไม่รู้จัก '{}'".into());

    m.insert(
        "misc.rev.usage".into(),
        "วิธีใช้: rev [file]  (กลับตัวอักษรของแต่ละบรรทัด; อ่านอินพุตที่ pipe เข้ามาเมื่อไม่ระบุไฟล์)".into(),
    );

    m.insert(
        "misc.rev.bad_arg".into(),
        "rev: อาร์กิวเมนต์ที่ไม่คาดคิด '{}' (วิธีใช้: rev [file])".into(),
    );

    m.insert(
        "misc.trim.usage".into(),
        "วิธีใช้: trim <pattern> [ข้อความ]   (หรือ: <คำสั่ง> | trim <pattern> [...])".into(),
    );

    m.insert(
        "misc.trim.empty_pattern".into(),
        "trim: แพตเทิร์นต้องไม่ว่าง".into(),
    );

    m.insert(
        "misc.clear.type_help".into(),
        "พิมพ์ 'help' สำหรับคำสั่งที่มี 'exit' เพื่อออก.".into(),
    );

    m.insert(
        "misc.clear.welcome".into(),
        "ยินดีต้อนรับสู่ ntc {} - Navigate, Toolkit, Center".into(),
    );

    m.insert(
        "misc.ds.example1".into(),
        "  ds src # ค้นหาโดยใช้ความลึกจาก config".into(),
    );

    m.insert(
        "misc.ds.example2".into(),
        "  ds src -d 3 # ค้นหาลึกสุด 3 ระดับ".into(),
    );

    m.insert(
        "misc.ds.example3".into(),
        "  ds test # ตรงกันบางส่วน: พบ test_utils/ เป็นต้น".into(),
    );

    m.insert(
        "misc.ds.usage".into(),
        "การใช้งาน: ds <รูปแบบ> [-d <ความลึก>]".into(),
    );

    m.insert(
        "misc.fs.example1".into(),
        "  fs main.c # ค้นหาโดยใช้ความลึกจาก config".into(),
    );

    m.insert(
        "misc.fs.example2".into(),
        "  fs main.c -d 5 # ค้นหาลึกสุด 5 ระดับ".into(),
    );

    m.insert(
        "misc.fs.example3".into(),
        "  fs main # ตรงกันบางส่วน: พบ main_helper.c เป็นต้น".into(),
    );

    m.insert(
        "misc.fs.usage".into(),
        "การใช้งาน: fs <รูปแบบ> [-d <ความลึก>]".into(),
    );

    m.insert(
        "misc.fsc.copied".into(),
        "คัดลอก '{}' ไปยังคลิปบอร์ดแล้ว!".into(),
    );

    m.insert(
        "misc.fsc.copy_prompt".into(),
        "คัดลอกไปยังคลิปบอร์ด? (y/N): ".into(),
    );

    m.insert("misc.fsc.done_opt".into(), "0. เสร็จสิ้น".into());

    m.insert("misc.fsc.done_str".into(), "เสร็จสิ้น.".into());

    m.insert(
        "misc.fsc.example1".into(),
        "  fsc main.rs # ค้นหาแล้วเลือกไฟล์เพื่อแสดง".into(),
    );

    m.insert(
        "misc.fsc.example2".into(),
        "  fsc main.rs -d 5 # ค้นหาลึกสุด 5 ระดับ".into(),
    );

    m.insert(
        "misc.fsc.found".into(),
        "\u{1F50D} พบ {} ไฟล์สำหรับ \"{}\":".into(),
    );

    m.insert("misc.fsc.invalid_input".into(), "ข้อมูลนำเข้าไม่ถูกต้อง.".into());

    m.insert(
        "misc.fsc.invalid_selection".into(),
        "การเลือกไม่ถูกต้อง.".into(),
    );

    m.insert("misc.fsc.not_file".into(), "เส้นทางที่เลือกไม่ใช่ไฟล์.".into());

    m.insert("misc.fsc.select_prompt".into(), "เลือกไฟล์เพื่อแสดง: ".into());

    m.insert("misc.fsc.skipped".into(), "ข้าม (ไม่รองรับรูปแบบ): {}".into());

    m.insert(
        "misc.fsc.usage".into(),
        "การใช้งาน: fsc <รูปแบบ> [-d <ความลึก>]".into(),
    );

    m.insert(
        "misc.gs.example1".into(),
        "  gs main # ค้นหาเนื้อหาไฟล์โดยใช้ความลึกจาก config".into(),
    );

    m.insert(
        "misc.gs.example2".into(),
        "  gs main -d 5 # ค้นหาลึกสุด 5 ระดับ".into(),
    );

    m.insert(
        "misc.gs.example3".into(),
        "  gs -i main # โหมดโต้ตอบ (เรียกดูผลลัพธ์ Enter เพื่อเปิด)".into(),
    );

    m.insert(
        "misc.gs.example4".into(),
        "  gs -i main -d 3 # โหมดโต้ตอบพร้อมจำกัดความลึก".into(),
    );

    m.insert(
        "misc.gs.example5".into(),
        "  gs main -d 5 -l 10 # แสดงเพียง 10 รายการแรก".into(),
    );

    m.insert(
        "misc.gs.example6".into(),
        "  gs main -p \"*.rs\" # ค้นเฉพาะไฟล์ที่ตรง glob".into(),
    );

    m.insert(
        "misc.gs.example7".into(),
        "  gs main -s size # เรียงตามขนาดไฟล์ (line|path|size, -s เปล่า = line)".into(),
    );

    m.insert(
        "misc.gs.example8".into(),
        "  gs error\\|warning # ค้นหาแบบ OR: ตรงกับคำใดคำหนึ่ง (a\\|b)".into(),
    );

    m.insert(
        "misc.gs.no_pipe_path".into(),
        "gs: -p ใช้กับ piped input ไม่ได้ (pipe ส่งข้อความ ไม่ใช่ไฟล์)".into(),
    );

    m.insert(
        "misc.gs.pipe_sort_only_line".into(),
        "gs: piped input ใช้ได้แค่ -s line (path/size ต้องใช้ไฟล์)".into(),
    );

    m.insert("misc.gs.searching".into(), "กำลังค้นหาเนื้อหา...".into());

    m.insert(
        "misc.gs.many_results".into(),
        "gs: พบ {} จากการตรงกัน แสดง 20,000 รายการแรก. แคบการค้นหาให้แคบลง.".into(),
    );

    m.insert(
        "misc.gs.usage".into(),
        "การใช้งาน: gs <รูปแบบ> [-d <ความลึก>] [-l <จำนวน>] [-i] [-p <glob>]... [-s [line|path|size]]"
            .into(),
    );

    m.insert("misc.wi.not_dir".into(), "wi: ไม่ใช่ไดเรกทอรี: '{}'".into());

    m.insert("misc.wi.start_error".into(), "wi: เฝ้าดูไม่ได้: {}".into());

    m.insert("misc.wi.usage".into(), "การใช้งาน: wi [path]".into());

    m.insert(
        "misc.locate.example1".into(),
        "  locate main.c # ค้นหาไฟล์และไดเรกทอรี (ความลึกจาก config)".into(),
    );

    m.insert(
        "misc.locate.example2".into(),
        "  locate main.c -d 5 # ค้นหาลึกสุด 5 ระดับ".into(),
    );

    m.insert(
        "misc.locate.example3".into(),
        "  locate main # ตรงกันบางส่วน: พบ main.c, main_helper/ เป็นต้น".into(),
    );

    m.insert(
        "misc.locate.header".into(),
        "\u{1F50D} {} สำหรับ \"{}\" ({} ผลลัพธ์):".into(),
    );

    m.insert("misc.locate.label_didyoumean".into(), "คุณหมายถึง?".into());

    m.insert("misc.locate.label_exact".into(), "ตรงกันทุกประการ".into());

    m.insert("misc.locate.label_partial".into(), "ตรงกันบางส่วน".into());

    m.insert(
        "misc.locate.no_results".into(),
        "\n\u{1F50D} ไม่มีผลลัพธ์สำหรับ \"{}\"\n".into(),
    );

    m.insert(
        "misc.locate.usage".into(),
        "การใช้งาน: locate <รูปแบบ> [-d <ความลึก>]".into(),
    );

    m.insert("misc.mkd.created".into(), "สร้างไดเรกทอรีแล้ว: {}".into());

    m.insert("misc.mkd.error".into(), "mkd: {}: {}".into());

    m.insert("misc.mkd.missing".into(), "mkd: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.mkf.created".into(), "สร้างไฟล์แล้ว: {}".into());

    m.insert("misc.mkf.error".into(), "mkf: {}: {}".into());

    m.insert("misc.mkf.exists".into(), "มีอยู่แล้ว: {}".into());

    m.insert("misc.mkf.missing".into(), "mkf: ขาดอาร์กิวเมนต์".into());

    m.insert(
        "misc.ne.create_failed".into(),
        "ไม่สามารถสร้างเทมเพลต: {}".into(),
    );

    m.insert(
        "misc.ne.created_hyper".into(),
        "สร้างโปรเจกต์ hyper แล้ว: {}".into(),
    );

    m.insert(
        "misc.ne.created_super".into(),
        "สร้างเทมเพลต super แล้ว: {}".into(),
    );

    m.insert(
        "misc.ne.created_template".into(),
        "สร้างเทมเพลตแล้ว: {}".into(),
    );

    m.insert("misc.ne.editor_error".into(), "ข้อผิดพลาดตัวแก้ไข: {}".into());

    m.insert(
        "misc.ne.file_missing".into(),
        "ไม่พบไฟล์: {} ใช้ --init <ไฟล์> หรือ --super <ไฟล์> เพื่อสร้าง".into(),
    );

    m.insert(
        "misc.ne.hyper_error".into(),
        "ข้อผิดพลาดโปรเจกต์ hyper: {}".into(),
    );

    m.insert("misc.ne.missing_file".into(), "ne: ไม่มีไฟล์ที่ระบุ".into());

    m.insert("misc.ne.one_file".into(), "สร้างได้ครั้งละหนึ่งไฟล์เท่านั้น".into());

    m.insert(
        "misc.ne.open_not_created".into(),
        "ไฟล์ -I ต้องเป็นหนึ่งในไฟล์ที่กำลังสร้าง: {}".into(),
    );

    m.insert(
        "misc.ne.opened_welcome".into(),
        "เปิดหน้าจอต้อนรับใน {} แล้ว".into(),
    );

    m.insert(
        "misc.ne.usage_open".into(),
        "การใช้งาน: -I <ไฟล์> ต้องเป็นอาร์กิวเมนต์สุดท้าย (หนึ่งไฟล์เท่านั้น)".into(),
    );

    m.insert(
        "misc.ne.usage_hyper".into(),
        "การใช้งาน: ne --hyper <ชื่อ> --lang <lang> (ชื่อต้องไม่มีนามสกุล)".into(),
    );

    m.insert(
        "misc.pathcp.copied".into(),
        "คัดลอกเส้นทางไปยังคลิปบอร์ดแล้ว: {}".into(),
    );

    m.insert("misc.pip.found".into(), "\u{2713} พบ pip ที่: {}".into());

    m.insert(
        "misc.pip.not_found".into(),
        "ไม่พบ pip ในไดเรกทอรี venv/Scripts".into(),
    );

    m.insert("misc.rmd.error".into(), "rmd: {}: {}".into());

    m.insert("misc.rmd.missing".into(), "rmd: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.rmd.not_dir".into(), "rmd: '{}' ไม่ใช่ไดเรกทอรี".into());

    m.insert("misc.rmd.not_exist".into(), "rmd: '{}' ไม่มีอยู่".into());

    m.insert("misc.rmd.removed".into(), "ลบไดเรกทอรีแล้ว: {}".into());

    m.insert(
        "misc.rmd.no_match".into(),
        "rmd: ไม่มีไดเรกทอรีตรงกับ '{}'".into(),
    );

    m.insert(
        "misc.rmd.pattern_matches".into(),
        "มี {} ไดเรกทอรีตรงกับ '{}':".into(),
    );

    m.insert(
        "misc.rmd.confirm_delete".into(),
        "ลบ {} ไดเรกทอรีหรือไม่?".into(),
    );

    m.insert("misc.rmd.cancelled".into(), "ยกเลิกแล้ว".into());

    m.insert(
        "misc.rmd.deadlock_vault_admin_only".into(),
        "rmd: '{}' มี deadlock vault ลบได้เฉพาะ root/ผู้ดูแลระบบ".into(),
    );

    m.insert("misc.rmf.error".into(), "rmf: {}: {}".into());

    m.insert(
        "misc.rmf.is_dir".into(),
        "rmf: '{}' เป็นไดเรกทอรี (ใช้ rmd แทน)".into(),
    );

    m.insert("misc.rmf.missing".into(), "rmf: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.rmf.not_exist".into(), "rmf: '{}' ไม่มีอยู่".into());

    m.insert("misc.rmf.no_match".into(), "rmf: ไม่มีไฟล์ตรงกับ '{}'".into());

    m.insert(
        "misc.rmf.pattern_matches".into(),
        "มี {} ไฟล์ตรงกับ '{}':".into(),
    );

    m.insert("misc.rmf.confirm_delete".into(), "ลบ {} ไฟล์หรือไม่?".into());

    m.insert("misc.rmf.cancelled".into(), "ยกเลิกแล้ว".into());

    m.insert("misc.rmf.removed".into(), "ลบแล้ว: {}".into());

    m.insert(
        "misc.rmf.deadlock_vault_admin_only".into(),
        "rmf: '{}' คือคลังล็อคตาย ลบได้เฉพาะ root/ผู้ดูแลระบบ".into(),
    );

    m.insert("misc.rnmd.error".into(), "rnmd: {}".into());

    m.insert("misc.rnmd.exists".into(), "rnmd: '{}' มีอยู่แล้ว".into());

    m.insert("misc.rnmd.missing".into(), "rnmd: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.rnmd.not_dir".into(), "rnmd: '{}' ไม่ใช่ไดเรกทอรี".into());

    m.insert("misc.rnmd.not_exist".into(), "rnmd: '{}' ไม่มีอยู่".into());

    m.insert(
        "misc.rnmd.renamed".into(),
        "เปลี่ยนชื่อไดเรกทอรี '{}' -> '{}' แล้ว".into(),
    );

    m.insert(
        "misc.rnmd.usage".into(),
        " การใช้งาน: rnmd <ชื่อเดิม> <ชื่อใหม่>".into(),
    );

    m.insert("misc.rnmf.error".into(), "rnmf: {}".into());

    m.insert("misc.rnmf.exists".into(), "rnmf: '{}' มีอยู่แล้ว".into());

    m.insert(
        "misc.rnmf.is_dir".into(),
        "rnmf: '{}' เป็นไดเรกทอรี (ใช้ rnmd แทน)".into(),
    );

    m.insert("misc.rnmf.missing".into(), "rnmf: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.rnmf.not_exist".into(), "rnmf: '{}' ไม่มีอยู่".into());

    m.insert(
        "misc.rnmf.renamed".into(),
        "เปลี่ยนชื่อ '{}' -> '{}' แล้ว".into(),
    );

    m.insert(
        "misc.rnmf.usage".into(),
        " การใช้งาน: rnmf <ชื่อเดิม> <ชื่อใหม่>".into(),
    );

    m.insert("misc.mv.error".into(), "mv: {}".into());

    m.insert(
        "misc.mv.exists".into(),
        "mv: เป้าหมายมีอยู่แล้ว: '{}' (ใช้ -f เพื่อเขียนทับ)".into(),
    );

    m.insert("misc.mv.missing".into(), "mv: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.mv.moved".into(), "ย้าย '{}' -> '{}' แล้ว".into());

    m.insert("misc.mv.not_exist".into(), "mv: ไม่พบต้นทาง: '{}'".into());

    m.insert("misc.mv.no_match".into(), "mv: ไม่มีไฟล์ที่ตรงกับ '{}'".into());

    m.insert(
        "misc.mv.dst_dir".into(),
        "mv: ปลายทางต้องเป็นไดเรกทอรี: '{}'".into(),
    );

    m.insert("misc.mv.wild_done".into(), "ย้าย {} ไฟล์ไปยัง '{}' แล้ว".into());

    m.insert(
        "misc.mv.usage".into(),
        "การใช้งาน: mv <src|@tp|pattern> <dst|@tp> [-f]".into(),
    );

    m.insert("misc.cp.error".into(), "cp: {}".into());

    m.insert(
        "misc.cp.exists".into(),
        "cp: เป้าหมายมีอยู่แล้ว: '{}' (ใช้ -f เพื่อเขียนทับ)".into(),
    );

    m.insert("misc.cp.missing".into(), "cp: ขาดอาร์กิวเมนต์".into());

    m.insert("misc.cp.copied".into(), "คัดลอก '{}' -> '{}' แล้ว".into());

    m.insert("misc.cp.not_exist".into(), "cp: ไม่พบต้นทาง: '{}'".into());

    m.insert("misc.cp.no_match".into(), "cp: ไม่มีไฟล์ที่ตรงกับ '{}'".into());

    m.insert(
        "misc.cp.dst_dir".into(),
        "cp: ปลายทางต้องเป็นไดเรกทอรี: '{}'".into(),
    );

    m.insert(
        "misc.cp.wild_done".into(),
        "คัดลอก {} ไฟล์ไปยัง '{}' แล้ว".into(),
    );

    m.insert(
        "misc.cp.same".into(),
        "cp: ต้นทางและปลายทางคือไฟล์เดียวกัน: '{}'".into(),
    );

    m.insert(
        "misc.cp.usage".into(),
        "การใช้งาน: cp <src|@tp|pattern> <dst|@tp> [-f]".into(),
    );

    m.insert(
        "misc.say.example1".into(),
        "  say \"hello world\" --loop 5 -n # 5 บรรทัด".into(),
    );

    m.insert(
        "misc.say.example2".into(),
        "  say \"hello world\" --loop 3 -t # 1 บรรทัด 3 ครั้ง".into(),
    );

    m.insert(
        "misc.say.loop_exceed".into(),
        "ค่า --loop เกินค่าสูงสุด ({}), กำลังจำกัดค่า".into(),
    );

    m.insert(
        "misc.say.loop_invalid".into(),
        "ค่า --loop '{}' ไม่ถูกต้อง ใช้ 1".into(),
    );

    m.insert(
        "misc.say.loop_requires".into(),
        "--loop ต้องเป็นตัวเลข ใช้ 1".into(),
    );

    m.insert(
        "misc.say.loop_warning".into(),
        "ค่า --loop ต้องไม่น้อยกว่า 1 ใช้ 1".into(),
    );

    m.insert(
        "misc.say.usage".into(),
        "การใช้งาน: say <ข้อความ> [--loop N] [-n|-t]".into(),
    );

    m.insert(
        "misc.show_env.activate_hint".into(),
        "เรียก 'python_activate' เพื่อเปิดใช้งานสภาพแวดล้อม virtual".into(),
    );

    m.insert(
        "misc.show_env.active".into(),
        "สภาพแวดล้อม virtual คือ".into(),
    );

    m.insert(
        "misc.show_env.and_more".into(),
        "... และอีก {} รายการ".into(),
    );

    m.insert("misc.show_env.cwd_label".into(), "ไดเรกทอรีปัจจุบัน:".into());

    m.insert(
        "misc.show_env.deactivate_hint".into(),
        "เรียก 'deactivate' เพื่อออกจากสภาพแวดล้อม virtual".into(),
    );

    m.insert(
        "misc.show_env.not_active".into(),
        "สภาพแวดล้อม virtual คือ".into(),
    );

    m.insert("misc.show_env.not_set".into(), "(ไม่ได้ตั้งค่า)".into());

    m.insert("misc.show_env.os_label".into(), "ระบบปฏิบัติการ:".into());

    m.insert(
        "misc.show_env.other_header".into(),
        "\u{1F527} ตัวแปร Python/Venv อื่น ๆ:".into(),
    );

    m.insert(
        "misc.show_env.path_full".into(),
        "\u{1F4C2} PATH (ทั้งหมด):".into(),
    );

    m.insert(
        "misc.show_env.path_header".into(),
        "\u{1F4C2} PATH (รายการแรก ๆ):".into(),
    );

    m.insert("misc.show_env.shell_label".into(), "เชลล์:".into());

    m.insert(
        "misc.show_env.summary_header".into(),
        "\u{1F4CA} สรุป:".into(),
    );

    m.insert(
        "misc.show_env.sys_header".into(),
        "\u{1F4BB} ข้อมูลระบบ:".into(),
    );

    m.insert(
        "misc.show_env.venv_header".into(),
        "\u{1F40D} สภาพแวดล้อม virtual ของ Python:".into(),
    );

    m.insert("misc.size.bytes_label".into(), "ไบต์:".into());

    m.insert(
        "misc.size.calculating".into(),
        "กำลังคำนวณขนาดไดเรกทอรี...".into(),
    );

    m.insert("misc.size.human_label".into(), "มนุษย์:".into());

    m.insert("misc.size.path_label".into(), "เส้นทาง:".into());

    m.insert("misc.size.time_label".into(), "เวลา:".into());

    m.insert("misc.size.title".into(), "ขนาดไดเรกทอรีปัจจุบัน".into());

    m.insert(
        "misc.tp.example_info".into(),
        "ตัวอย่าง: tp info my_project".into(),
    );

    m.insert("misc.tp.info_canonical".into(), "Canonical".into());

    m.insert("misc.tp.info_exists".into(), "Exists".into());

    m.insert("misc.tp.info_global".into(), "global".into());

    m.insert("misc.tp.info_index".into(), "ดัชนี".into());

    m.insert("misc.tp.info_name".into(), "ชื่อ".into());

    m.insert("misc.tp.info_path".into(), "เส้นทาง".into());

    m.insert("misc.tp.info_source".into(), "แหล่งที่มา".into());

    m.insert(
        "misc.tp.info_title".into(),
        "\u{1F4CC} ข้อมูลจุดบันทึกเทเลพอร์ต".into(),
    );

    m.insert(
        "misc.tp.invalid_format".into(),
        "รูปแบบไม่ถูกต้อง. ใช้: tp rnm <เก่า> to <ใหม่>".into(),
    );

    m.insert("misc.tp.not_found".into(), "ไม่พบจุดบันทึก '{}'".into());

    m.insert(
        "misc.tp.type_help".into(),
        "พิมพ์ 'tp help' สำหรับการใช้งาน.".into(),
    );

    m.insert(
        "misc.tp.unknown_subcmd".into(),
        "ไม่รู้จักคำสั่งย่อยของ tp: {}".into(),
    );

    m.insert(
        "misc.tp.usage_add".into(),
        "การใช้งาน: tp add <ชื่อ> [path]".into(),
    );

    m.insert("misc.tp.usage_info".into(), "การใช้งาน: tp info <ชื่อ>".into());

    m.insert(
        "misc.tp.usage_jump".into(),
        "การใช้งาน: tp jump <ชื่อ|หมายเลข>".into(),
    );

    m.insert(
        "misc.tp.usage_rm".into(),
        "การใช้งาน: tp rm <ชื่อ|หมายเลข>[, <ชื่อ|หมายเลข>, ...]".into(),
    );

    m.insert(
        "misc.tp.usage_rnm".into(),
        "การใช้งาน: tp rnm <ชื่อเดิม> to <ชื่อใหม่>".into(),
    );

    m.insert("misc.tpb.unknown".into(), "ตัวเลือก tpb ไม่รู้จัก: {}".into());

    m.insert("misc.tpb.usage".into(), "การใช้งาน:".into());

    m.insert(
        "misc.tpb.usage_basic".into(),
        "  tpb - เทเลพอร์ตกลับไปยังตำแหน่งก่อนหน้า".into(),
    );

    m.insert(
        "misc.tpb.usage_clear".into(),
        "  tpb clear - ล้างประวัติเทเลพอร์ต".into(),
    );

    m.insert(
        "misc.tpb.usage_history".into(),
        "  tpb history - แสดงประวัติเทเลพอร์ต".into(),
    );

    m.insert(
        "misc.ui.removed".into(),
        "UI สมัยใหม่ถูกลบแล้ว. มีเฉพาะเชลล์แบบคลาสสิก.".into(),
    );

    m.insert("misc.venva.activated".into(), "เปิดใช้งานแล้ว: {}".into());

    m.insert(
        "misc.venva.activating".into(),
        "กำลังเปิดใช้งานสภาพแวดล้อม virtual: {}".into(),
    );

    m.insert(
        "misc.venva.active_info".into(),
        "สภาพแวดล้อม virtual ทำงานอยู่แล้ว. คำสั่ง pip และ python จะใช้สภาพแวดล้อมนี้.".into(),
    );

    m.insert(
        "misc.venva.already_in".into(),
        "อยู่ในสภาพแวดล้อม virtual แล้ว. ใช้ 'deactivate' เพื่อออกก่อน".into(),
    );

    m.insert("misc.venva.failed".into(), "ไม่สามารถเปิดใช้งาน: {}".into());

    m.insert(
        "misc.venva.no_script".into(),
        "ไม่พบสคริปต์ activate ใน: {}".into(),
    );

    m.insert(
        "misc.venva.not_found".into(),
        "ไม่พบสภาพแวดล้อม virtual. การใช้งาน: python_activate <ชื่อ_venv>".into(),
    );

    m.insert(
        "misc.venvd.deactivated".into(),
        "ปิดใช้งานสภาพแวดล้อม virtual แล้ว".into(),
    );

    m.insert(
        "misc.venvd.not_in_venv".into(),
        "ไม่ได้อยู่ในสภาพแวดล้อม virtual".into(),
    );
}
