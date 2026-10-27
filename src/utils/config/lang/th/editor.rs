use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("editor.already_root".into(), "อยู่ที่รากอยู่แล้ว".into());

    m.insert(
        "editor.auto_save_off".into(),
        "บันทึกอัตโนมัติเมื่อออก: OFF".into(),
    );

    m.insert("editor.auto_save_on".into(), "บันทึกอัตโนมัติเมื่อออก: ON".into());

    m.insert("editor.cleared_cursors".into(), "ล้างเคอร์เซอร์พิเศษแล้ว".into());

    m.insert(
        "editor.clip_copied_chars".into(),
        "คัดลอก {} อักขระแล้ว".into(),
    );

    m.insert("editor.clip_copied_line".into(), "คัดลอกบรรทัดแล้ว".into());

    m.insert("editor.clip_cut_chars".into(), "ตัด {} อักขระแล้ว".into());

    m.insert("editor.clip_cut_line".into(), "ตัดบรรทัดแล้ว".into());

    m.insert("editor.clip_empty".into(), "คลิปบอร์ดว่างเปล่า".into());

    m.insert("editor.clip_yanked".into(), "yank บรรทัดแล้ว".into());

    m.insert("editor.clipboard_empty".into(), "คลิปบอร์ดว่างเปล่า".into());

    m.insert("editor.close_btn".into(), "\u{2715}".into());

    m.insert("editor.cmd_already_at_root".into(), "อยู่ที่รากอยู่แล้ว".into());

    m.insert("editor.cmd_auto_off".into(), "บันทึกอัตโนมัติเมื่อออก: OFF".into());

    m.insert("editor.cmd_auto_on".into(), "บันทึกอัตโนมัติเมื่อออก: ON".into());

    m.insert("editor.cmd_cd_prefix".into(), "cd:".into());

    m.insert("editor.cmd_color_off".into(), "สี: OFF".into());

    m.insert("editor.cmd_color_on".into(), "สี: ON".into());

    m.insert("editor.cmd_created_dir".into(), "สร้างไดเรกทอรีแล้ว:".into());

    m.insert("editor.cmd_current_theme".into(), "ธีมปัจจุบัน:".into());

    m.insert(
        "editor.cmd_exited_hex".into(),
        "ออกจากโหมด hex แล้ว (ละทิ้งการเปลี่ยนแปลง)".into(),
    );

    m.insert(
        "editor.cmd_gos_only".into(),
        "gos: มีเพียง {} ไดเรกทอรีย่อย".into(),
    );

    m.insert("editor.cmd_gos_usage".into(), "gos: ใช้งาน gos <N>".into());

    m.insert(
        "editor.cmd_goto_usage".into(),
        "goto: ใช้งาน goto <addr> หรือ goto 0x<hex>".into(),
    );

    m.insert(
        "editor.cmd_hex_save_failed".into(),
        "บันทึก hex ล้มเหลว:".into(),
    );

    m.insert("editor.cmd_hex_saved".into(), "บันทึก hex แล้ว".into());

    m.insert("editor.cmd_jumped_to_line".into(), "ข้ามไปยังบรรทัดแล้ว".into());

    m.insert(
        "editor.cmd_jumped_to_offset".into(),
        "ข้ามไปยังออฟเซ็ตแล้ว".into(),
    );

    m.insert("editor.cmd_mkd_failed".into(), "mkd:".into());

    m.insert("editor.cmd_mkd_missing".into(), "mkd: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.cmd_mkdir_failed".into(), "mkdir:".into());

    m.insert(
        "editor.cmd_mkdir_missing".into(),
        "mkdir: ขาดอาร์กิวเมนต์".into(),
    );

    m.insert("editor.cmd_mkf_failed".into(), "mkf:".into());

    m.insert("editor.cmd_mkf_missing".into(), "mkf: ขาดอาร์กิวเมนต์".into());

    m.insert(
        "editor.cmd_no_cwd".into(),
        "ไม่สามารถรับไดเรกทอรีปัจจุบันได้".into(),
    );

    m.insert("editor.cmd_no_subdirs".into(), "ไม่มีไดเรกทอรีย่อย".into());

    m.insert("editor.cmd_no_tp".into(), "ไม่มีจุดบันทึกเทเลพอร์ต".into());

    m.insert("editor.cmd_not_dir".into(), "ไม่ใช่ไดเรกทอรี".into());

    m.insert("editor.cmd_not_exist".into(), "ไม่มีอยู่".into());

    m.insert("editor.cmd_removed".into(), "ลบแล้ว:".into());

    m.insert("editor.cmd_removed_dir".into(), "ลบไดเรกทอรีแล้ว:".into());

    m.insert("editor.cmd_rmd".into(), "rmd:".into());

    m.insert("editor.cmd_rmd_missing".into(), "rmd: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.cmd_rmf".into(), "rmf:".into());

    m.insert("editor.cmd_rmf_failed".into(), "rmf:".into());

    m.insert("editor.cmd_rmf_missing".into(), "rmf: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.cmd_run_missing".into(), "run: ขาดคำสั่ง".into());

    m.insert("editor.cmd_save_failed".into(), "บันทึกล้มเหลว:".into());

    m.insert("editor.cmd_saved".into(), "บันทึกแล้ว".into());

    m.insert("editor.cmd_syntax_off".into(), "ไฮไลต์ไวยากรณ์: OFF".into());

    m.insert("editor.cmd_syntax_on".into(), "ไฮไลต์ไวยากรณ์: ON".into());

    m.insert(
        "editor.cmd_theme_list_hint".into(),
        "ใช้ 'theme list' เพื่อดูธีมที่มี".into(),
    );

    m.insert("editor.cmd_theme_not_found".into(), "ไม่พบธีม".into());

    m.insert("editor.cmd_theme_switched".into(), "สลับไปใช้ธีมแล้ว".into());

    m.insert("editor.cmd_themes".into(), "ธีม:".into());

    m.insert("editor.cmd_touch_failed".into(), "touch:".into());

    m.insert(
        "editor.cmd_touch_missing".into(),
        "touch: ขาดอาร์กิวเมนต์".into(),
    );

    m.insert("editor.cmd_tp_add_error".into(), "tp-add:".into());

    m.insert("editor.cmd_tp_add_missing".into(), "tp-add: ขาดชื่อ".into());

    m.insert("editor.cmd_tp_added".into(), "สร้างจุดบันทึกแล้ว".into());

    m.insert("editor.cmd_tp_error".into(), "tp:".into());

    m.insert("editor.cmd_tp_jumped".into(), "เทเลพอร์ตไปยังแล้ว".into());

    m.insert("editor.cmd_tp_list".into(), "จุดเทเลพอร์ต:".into());

    m.insert("editor.cmd_tp_not_found".into(), "ไม่พบจุดบันทึก:".into());

    m.insert("editor.cmd_unknown".into(), "คำสั่งไม่รู้จัก:".into());

    m.insert(
        "editor.cmd_visual_hint".into(),
        "โหมด Visual ใช้ปุ่มลูกศรเพื่อเลือก Esc เพื่อออก".into(),
    );

    m.insert("editor.color_off".into(), "สี: OFF".into());

    m.insert("editor.color_on".into(), "สี: ON".into());

    m.insert("editor.compl_builtin".into(), "[built-in]".into());

    m.insert("editor.compl_math_abs".into(), "ค่าสัมบูรณ์".into());

    m.insert("editor.compl_math_acos".into(), "arccosine (เรเดียน)".into());

    m.insert(
        "editor.compl_math_acot".into(),
        "arccotangent (เรเดียน)".into(),
    );

    m.insert(
        "editor.compl_math_acsc".into(),
        "arccosecant (เรเดียน)".into(),
    );

    m.insert("editor.compl_math_asec".into(), "arcsecant (เรเดียน)".into());

    m.insert("editor.compl_math_asin".into(), "arcsine (เรเดียน)".into());

    m.insert(
        "editor.compl_math_atan".into(),
        "arctangent (เรเดียน)".into(),
    );

    m.insert("editor.compl_math_avg".into(), "ค่าเฉลี่ยของค่า".into());

    m.insert("editor.compl_math_ceil".into(), "เพดาน (ปัดขึ้น)".into());

    m.insert("editor.compl_math_cos".into(), "cosine (เรเดียน)".into());

    m.insert("editor.compl_math_cot".into(), "cotangent (เรเดียน)".into());

    m.insert("editor.compl_math_csc".into(), "cosecant (เรเดียน)".into());

    m.insert("editor.compl_math_floor".into(), "พื้น (ปัดลง)".into());

    m.insert("editor.compl_math_ln".into(), "ลอการิทึมธรรมชาติ".into());

    m.insert("editor.compl_math_log10".into(), "ลอการิทึมฐาน 10".into());

    m.insert("editor.compl_math_log2".into(), "ลอการิทึมฐาน 2".into());

    m.insert("editor.compl_math_max".into(), "ค่าสูงสุดของค่า".into());

    m.insert("editor.compl_math_min".into(), "ค่าต่ำสุดของค่า".into());

    m.insert("editor.compl_math_pow".into(), "ยกกำลัง (x^y)".into());

    m.insert("editor.compl_math_rand".into(), "ตัวเลขสุ่ม".into());

    m.insert(
        "editor.compl_math_round".into(),
        "ปัดเศษเป็นจำนวนเต็มที่ใกล้ที่สุด".into(),
    );

    m.insert("editor.compl_math_sec".into(), "secant (เรเดียน)".into());

    m.insert("editor.compl_math_sin".into(), "sine (เรเดียน)".into());

    m.insert("editor.compl_math_sqrt".into(), "รากที่สอง".into());

    m.insert("editor.compl_math_sum".into(), "ผลรวมของค่า".into());

    m.insert("editor.compl_math_tan".into(), "tangent (เรเดียน)".into());

    m.insert("editor.compl_math_tobinary".into(), "แปลงเป็นไบนารี".into());

    m.insert("editor.compl_math_todecimal".into(), "แปลงเป็นฐานสิบ".into());

    m.insert("editor.compl_math_tohb".into(), "แปลงเป็นไบต์ที่อ่านง่าย".into());

    m.insert("editor.compl_math_tohex".into(), "แปลงเป็น hex".into());

    m.insert("editor.compl_math_tooctal".into(), "แปลงเป็นฐานแปด".into());

    m.insert("editor.compl_other".into(), "[other]".into());

    m.insert("editor.compl_print_values".into(), "พิมพ์ค่า".into());

    m.insert("editor.compl_return_value".into(), "ส่งคืนค่า".into());

    m.insert("editor.completion_title".into(), "การเติมเต็ม".into());

    m.insert("editor.copied_line".into(), "คัดลอกบรรทัดแล้ว".into());

    m.insert("editor.cut_line".into(), "ตัดบรรทัดแล้ว".into());

    m.insert(
        "editor.edit_added_cursor".into(),
        "เพิ่มเคอร์เซอร์แล้ว (รวม {})".into(),
    );

    m.insert("editor.edit_already_selected".into(), "เลือกไว้แล้ว".into());

    m.insert(
        "editor.edit_no_more_occurrences".into(),
        "ไม่มีการเกิดเพิ่มเติม".into(),
    );

    m.insert("editor.edit_wrap".into(), "wrap".into());

    m.insert(
        "editor.exited_hex".into(),
        "ออกจากโหมด hex แล้ว (ละทิ้งการเปลี่ยนแปลง)".into(),
    );

    m.insert(
        "editor.exited_multicursor".into(),
        "ออกจากโหมดเคอร์เซอร์หลายตัวแล้ว".into(),
    );

    m.insert(
        "editor.ff_prompt_find_empty".into(),
        "Find: พิมพ์เพื่อค้นหา...".into(),
    );

    m.insert("editor.ff_prompt_find_prefix".into(), "Find:".into());

    m.insert("editor.ff_prompt_gs_prefix".into(), "gs:".into());

    m.insert(
        "editor.file_changed_reload".into(),
        "ไฟล์เปลี่ยนจากภายนอก \u{2014} โหลดใหม่แล้ว".into(),
    );

    m.insert("editor.gosc_exit_hint".into(), "0=ออก ".into());

    m.insert(
        "editor.gosc_hint".into(),
        " Enter=ไป -N=ย้อนกลับ N Esc=ปิด ".into(),
    );

    m.insert(
        "editor.gosc_title".into(),
        " gosc \u{2014} นำทางต่อเนื่อง ".into(),
    );

    m.insert("editor.gs_loading".into(), " กำลังค้นหา... {} โปรดรอ".into());

    m.insert("editor.help_screen".into(), " โหมดปกติ (verbs + motion)\n i=insert v=visual q=query x=บันทึก n=ไฟล์ m=dir c=copy e=nfe f=ค้นหา b=files t=term u=encoding g=gmode :=cmd ?=ช่วยเหลือ 0=ออก(ถามถ้าไม่เซฟ)\n hjkl/awsd/ลูกศร เลื่อน \u{B7} Home/End ต้น/ท้ายบรรทัด \u{B7} PgUp/PgDn เลื่อนหน้า \u{B7} ^J=ข้าม\n\n GMODE (กด g พิมพ์ Enter)\n t/T=แท็บถัดไป/ก่อนหน้า qt=ปิดแท็บ aqt=ปิดทั้งหมด ตัวเลข=ข้าม q=ออก i=insert v=visual e=nfe Esc=ยกเลิก\n\n NFE SIDEBAR (กด e)\n hjkl/ลูกศร/wasd เลื่อน \u{B7} Enter=เข้าdir/เปิดไฟล์(อยู่ต่อ) \u{B7} i=insert v=visual \u{B7} Esc=กลับ\n\n โหมด INSERT (แก้ไข)\n พิมพ์เพื่อแทรก \u{B7} ลูกศรเลื่อน \u{B7} Esc=ปกติ ^S=บันทึก ^Z=เลิกทำ ^R=ทำซ้ำ ^D=หลายเคอร์เซอร์ ^F=ค้นหา ^N=ไฟล์ ^P=วาง ^T=เทอร์มินัล ^G=gmode ^E=nfe ^W=visual ^:=cmd \u{B7} Home/End ต้น/ท้าย \u{B7} PgUp/PgDn เลื่อนหน้า \u{B7} F3/Sh+F3 ค้นหาถัดไป/ก่อนหน้า\n\n โหมด VISUAL (เลือก)\n hjkl/ลูกศร เลื่อน (Shift เลือก) \u{B7} Alt+hjkl/ลูกศร ย้าย selection (j/k) หรือ indent (h/l) \u{B7} a=เลือกทั้งหมดสลับ c=copy x=cut y=yank p=paste :=คำสั่ง e=nfe f=ค้นหา g=gmode \u{B7} Esc=ออก\n\n โหมด COMMAND\n :<number> ไปที่หมายเลขบรรทัด\n :l N หรือ :line N ไปที่บรรทัด N\n :q หรือ :quit ออกจากตัวแก้ไข (พร้อมถามบันทึก)\n :wq บันทึกและออก\n :w หรือ :write บันทึกไฟล์\n :auto on เปิดบันทึกอัตโนมัติ\n :auto off ปิดบันทึกอัตโนมัติ\n :syntax on เปิดไฮไลต์ไวยากรณ์\n :syntax off ปิดไฮไลต์ไวยากรณ์\n :color on|off เปิด/ปิดสี\n :cd <path> เปลี่ยนไดเรกทอรี (Navigator)\n :back [N] ไปยังไดเรกทอรีหลัก (N ระดับ)\n :gos <N> ไปยังไดเรกทอรีย่อยที่ N\n :gosc แสดงรายการไดเรกทอรีย่อยแบบมีเลข\n :mkdir <dir> สร้างไดเรกทอรี\n :mkd <dir> สร้างไดเรกทอรี (ชื่อย่อของ :mkdir)\n :e เปิด file explorer (nfe)\n :git diff หรือ :gd แสดง git diff ของไฟล์ปัจจุบัน\n :git blame หรือ :blame แสดง blame ของบรรทัดปัจจุบัน\n\n โหมด QUERY (q ใน Normal, Ctrl+Alt+Q ใน Insert, :x ใน Command — ไม่ต้องใส่ / นำหน้าก็ได้)\n /s 'ข้อความ' เลือกการเกิดถัดไปหลังเคอร์เซอร์ (วนรอบ); /s เปล่า เลือกคำใต้เคอร์เซอร์\n /l N ไปที่บรรทัด N (เคอร์เซอร์ไปต้นบรรทัด)\n /sa 'ข้อความ' เลือกทุกการเกิดในบัฟเฟอร์\n /sal เลือกทุกบรรทัด\n /sl N เลือกบรรทัด N; /sl A B ช่วง A..B (สลับกันได้); /sl A , B , … เลือกบรรทัดแบบแยก\n /rp <ค้นหา> <แทนที่> แทนที่ในการเลือก (ต้องเลือกก่อน; หลายคำใส่ '...' : /rp 'a b' 'c')\n /add <ข้อความ> to #end|#start|#hm เพิ่มข้อความในบรรทัดที่เลือก (ต้องเลือกก่อน)\n /rm <ข้อความ> from #end|#start|#hm ลบ prefix/suffix ในบรรทัดที่เลือก (ต้องเลือกก่อน)\n /set <รูปแบบ> to #end|#start|#hm จัดแนวการเกิดครั้งแรก (ต้องเลือกก่อน)\n /c | /cs ล้างการเลือก\n /up แปลงส่วนที่เลือกเป็นตัวพิมพ์ใหญ่ (/upper alias ต้องเลือกก่อน)\n /low แปลงส่วนที่เลือกเป็นตัวพิมพ์เล็ก (/lower alias ต้องเลือกก่อน)\n /qal add <n> <q> เพิ่ม query alias (ต.ย. /qal sw /rp ${1} ${2}; s/l/sa/sal/sl/rp/c เป็นชื่อสงวน)\n /qal rm <n> ลบ alias\n /qal info <n> แสดง alias\n /qal edit <n> <q> อัปเดต alias\n /qal rnm <a> to <b> เปลี่ยนชื่อ alias\n /qal list แสดงรายการ alias\n <name> [args] เรียก alias โดยตรง (ไม่ต้องมี /)\n /undo เลิกทำ\n /redo ทำซ้ำ\n /exit /quit /Esc ออกจาก Query Mode\n ; เชื่อมหลายคำสั่ง\n ${1} ${2} พารามิเตอร์ alias ตามตำแหน่ง\n ${1:-default} พารามิเตอร์ alias พร้อมค่าเริ่มต้น\n ${1[]} พารามิเตอร์ alias แบบอาร์เรย์ (อาร์กิวเมนต์ที่เหลือทั้งหมด)".into());

    m.insert(
        "editor.hex_cannot_read".into(),
        "hex: ไม่สามารถอ่านไฟล์: {}".into(),
    );

    m.insert(
        "editor.hex_mode_hint".into(),
        "โหมด Hex — 0-9/a-f แก้ไขไบต์  u=เลิกทำ  Ctrl+R=ทำซ้ำ  :=คำสั่ง  Esc=ออก".into(),
    );

    m.insert("editor.hex_saved".into(), "บันทึก hex แล้ว".into());

    m.insert(
        "editor.hex_unsaved".into(),
        "มีการเปลี่ยนแปลง hex ที่ยังไม่บันทึก. ใช้ :w เพื่อบันทึก, :q! เพื่อละทิ้ง.".into(),
    );

    m.insert(
        "editor.hex_unsaved_warning".into(),
        "มีการเปลี่ยนแปลง hex ที่ยังไม่บันทึก. ใช้ :w เพื่อบันทึก, :q! เพื่อละทิ้ง.".into(),
    );

    m.insert(
        "editor.hint_command_actions".into(),
        "Enter=เรียก Esc=ยกเลิก".into(),
    );

    m.insert(
        "editor.hint_command_nav".into(),
        "cd|back|gos|gs|mkdir|touch|ne|tp|tp-add".into(),
    );

    m.insert(
        "editor.hint_file_finder_results".into(),
        "ผลลัพธ์ \u{2191}\u{2193}=นำทาง Enter=เปิด Esc=ปิด".into(),
    );

    m.insert("editor.hint_find".into(), "Find: {}".into());

    m.insert("editor.hint_find_prefix".into(), "Find \u{2014}".into());

    m.insert("editor.hint_gosc".into(), "Go to: {}".into());

    m.insert("editor.hint_gosc_close".into(), "Esc=ปิด".into());

    m.insert(
        "editor.hint_gosc_nav".into(),
        "dirs พิมพ์หมายเลข+Enter=นำทาง 0=ออก".into(),
    );

    m.insert("editor.hint_gosc_prefix".into(), "gosc \u{2014}".into());

    m.insert("editor.hint_gs_close".into(), "Esc=ปิด".into());

    m.insert("editor.hint_gs_done".into(), "เสร็จสิ้น: {} การตรงกัน".into());

    m.insert("editor.hint_gs_done_prefix".into(), "gs \u{2014}".into());

    m.insert(
        "editor.hint_gs_loading".into(),
        "gs \u{2014} กำลังค้นหา...".into(),
    );

    m.insert(
        "editor.hint_help".into(),
        " Help \u{2014} Esc/q=ปิด \u{2191}\u{2193}=เลื่อน".into(),
    );

    m.insert(
        "editor.hint_hex_actions".into(),
        "0-9/a-f=แก้ไข u=เลิกทำ ^R=ทำซ้ำ :=คำสั่ง :w=บันทึก Esc=ออก".into(),
    );

    m.insert(
        "editor.hint_hex_edit".into(),
        "แก้ไข Hex — พิมพ์ไบต์ hex".into(),
    );

    m.insert("editor.hint_insert".into(), " โหมด Insert — Esc=ปกติ ^S=บันทึก ^Z=เลิกทำ ^R=ทำซ้ำ ^D=หลายเคอร์เซอร์ ^F=ค้นหา ^N=ไฟล์ ^P=วาง ^T=เทอร์มินัล ^G=gmode ^E=nfe ^W=visual ^:=cmd".into());

    m.insert("editor.hint_insert_msg".into(), " Esc=ปกติ".into());

    m.insert(
        "editor.hint_insert_multi".into(),
        " Insert* \u{2014} Esc=ออกจากเคอร์เซอร์หลายตัว ^D=การเกิดถัดไป".into(),
    );

    m.insert(
        "editor.hint_insert_multi_msg".into(),
        " Esc=ออกจากเคอร์เซอร์หลายตัว".into(),
    );

    m.insert("editor.hint_jump".into(), "ไปที่บรรทัด \u{2014}".into());

    m.insert(
        "editor.hint_jump_actions".into(),
        "Enter=ข้าม Esc=ยกเลิก".into(),
    );

    m.insert("editor.hint_gmode".into(), "g".into());

    m.insert("editor.hint_gmode_actions".into(), "t/T=แท็บ qt/aqt=ปิด ตัวเลข=ข้าม q=ออก u=undo r=redo n=ไฟล์ m=dir gi=insert gv=visual ge=nfe Enter=รัน Esc=ยกเลิก".into());

    m.insert(
        "editor.gmode_unknown".into(),
        "คำสั่ง gmode ไม่รู้จัก: '{}'".into(),
    );

    m.insert("editor.hint_normal".into(), " i=insert v=visual f=ค้นหา b=files e=nfe t=term g=gmode x=บันทึก n=ไฟล์ m=dir c=copy Sh=เลือก :cmd ?=ช่วยเหลือ 0=ออก".into());

    m.insert(
        "editor.hint_create_actions".into(),
        "Enter=สร้าง Esc=ยกเลิก".into(),
    );

    m.insert(
        "editor.term_run_usage".into(),
        "วิธีใช้: ! <คำสั่ง> (รันในเทอร์มินัล)".into(),
    );

    m.insert(
        "editor.hint_normal_msg".into(),
        " i=insert v=visual :cmd ?=ช่วยเหลือ 0=ออก".into(),
    );

    m.insert("editor.hint_query".into(), "Query: {}".into());

    m.insert("editor.hint_query_actions".into(), "การกระทำ: /s ถัดไป, /l ไปที่บรรทัด, /sa ทั้งหมด, /sal ทุกบรรทัด, /sl บรรทัด, /rp แทนที่, /add, /rm, /set, /z เคอร์เซอร์, /up, /low, /qal, /c ล้าง".into());

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_results".into(), "ผลลัพธ์: {}".into());

    m.insert(
        "editor.hint_results_so_far".into(),
        "ผลลัพธ์: {} จนถึงตอนนี้".into(),
    );

    m.insert(
        "editor.hint_run_empty".into(),
        "ผลลัพธ์การเรียก \u{2014} (ว่าง)".into(),
    );

    m.insert(
        "editor.hint_run_entry".into(),
        "Enter=เรียก Esc=ยกเลิก".into(),
    );

    m.insert(
        "editor.hint_run_output".into(),
        "ผลลัพธ์การเรียก \u{2014} บรรทัด".into(),
    );

    m.insert(
        "editor.hint_run_scroll".into(),
        ":=คำสั่ง \u{2191}\u{2193}/j/k=เลื่อน Esc=ปิด".into(),
    );

    m.insert("editor.hint_search".into(), "ค้นหา:".into());

    m.insert(
        "editor.hint_search_actions".into(),
        "Enter=ถัดไป Shift+Enter=ก่อนหน้า Esc=ปิด".into(),
    );

    m.insert(
        "editor.hint_term_actions".into(),
        "บรรทัด PgUp/PgDn=เลื่อน Ctrl+Shift+C=คัดลอก Ctrl+C=ขัดจังหวะ".into(),
    );

    m.insert("editor.hint_term_close".into(), "Esc/q=ปิด".into());

    m.insert("editor.hint_term_exited".into(), "เทอร์มินัลสิ้นสุดแล้ว".into());

    m.insert("editor.hint_term_lines".into(), "เทอร์มินัล \u{2014}".into());

    m.insert("editor.hint_terminal".into(), "เทอร์มินัล — {} — {}".into());

    m.insert(
        "editor.hint_visual".into(),
        " hjkl=เลื่อน Sh=เลือก i=insert u=undo r=redo a=all c=copy x=cut p=paste Esc=ออก".into(),
    );

    m.insert(
        "editor.query_z_usage".into(),
        "/z [#hm|#end|#start] (ค่าเริ่มต้น #hm)".into(),
    );

    m.insert(
        "editor.query_cursors_added".into(),
        "เพิ่มเคอร์เซอร์ {} ตัวแล้ว".into(),
    );

    m.insert(
        "editor.hint_visual_no_msg".into(),
        " โหมด Visual — hjkl/ลูกศร เลื่อน · Shift เลือก · a=all c=copy x=cut y=yank p=paste".into(),
    );

    m.insert("editor.hover_title".into(), " info ".into());

    m.insert(
        "editor.hyper_created".into(),
        "สร้างโปรเจกต์ hyper '{}' สำเร็จแล้ว!".into(),
    );

    m.insert(
        "editor.hyper_creating".into(),
        "กำลังสร้างโปรเจกต์ hyper '{}' (C)...".into(),
    );

    m.insert(
        "editor.hyper_dir_exists".into(),
        "ไดเรกทอรี '{}' มีอยู่แล้ว.".into(),
    );

    m.insert(
        "editor.hyper_name_with_ext".into(),
        "ชื่อโปรเจกต์ '{}' มีนามสกุลไฟล์. ใช้ชื่อที่ไม่มีจุด.".into(),
    );

    m.insert(
        "editor.hyper_unsupported_lang".into(),
        "ไม่รองรับภาษา '{}'. รองรับเฉพาะ 'c'.".into(),
    );

    m.insert("editor.line_deleted".into(), "ลบบรรทัดแล้ว".into());

    m.insert("editor.line_selected".into(), "เลือกบรรทัดแล้ว".into());

    m.insert("editor.line_yanked".into(), "yank บรรทัดแล้ว".into());

    m.insert("editor.max_tabs".into(), "ถึงจำนวนแท็บสูงสุด ({}) แล้ว".into());

    m.insert("editor.mkd_missing".into(), "mkd: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.mkdir_missing".into(), "mkdir: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.mkf_missing".into(), "mkf: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.mode.command".into(), "[Command]".into());

    m.insert("editor.mode.finder".into(), "[Finder]".into());

    m.insert("editor.mode.gosc".into(), "[Gosc]".into());

    m.insert("editor.mode.help".into(), "[Help]".into());

    m.insert("editor.mode.hex".into(), "[HEX]".into());

    m.insert("editor.mode.insert".into(), "[Insert]".into());

    m.insert("editor.mode.insert_star".into(), "[Insert *]".into());

    m.insert("editor.mode.jump".into(), "[Jump]".into());

    m.insert("editor.mode.normal".into(), "[Normal]".into());

    m.insert("editor.mode.query".into(), "[Query]".into());

    m.insert("editor.mode.run".into(), "[Run]".into());

    m.insert("editor.mode.search".into(), "[Search]".into());

    m.insert("editor.mode.term".into(), "[Term]".into());

    m.insert("editor.mode.visual".into(), "[Visual]".into());

    m.insert(
        "editor.mode_cleared_cursors".into(),
        "ล้างเคอร์เซอร์พิเศษแล้ว".into(),
    );

    m.insert("editor.mode_command".into(), "[Command]".into());

    m.insert(
        "editor.mode_exited_multi".into(),
        "ออกจากโหมดเคอร์เซอร์หลายตัวแล้ว".into(),
    );

    m.insert("editor.mode_finder".into(), "[Finder]".into());

    m.insert("editor.mode_gosc".into(), "[Gosc]".into());

    m.insert("editor.mode_git_diff".into(), "[GitDiff]".into());

    m.insert("editor.git_no_diff".into(), "git: ไม่มีความต่างของไฟล์นี้".into());

    m.insert(
        "editor.git_no_blame".into(),
        "git: ไม่มีข้อมูล blame สำหรับบรรทัดนี้".into(),
    );

    m.insert(
        "editor.git_disabled".into(),
        "ปิดใช้งาน git integration แล้ว (config git ON เพื่อเปิด)".into(),
    );

    m.insert("editor.git_quit_hint".into(), "Esc/q เพื่อปิด".into());

    m.insert("editor.mode_help".into(), "[Help]".into());

    m.insert("editor.mode_hex".into(), "[HEX]".into());

    m.insert("editor.mode_insert".into(), "[Insert]".into());

    m.insert("editor.mode_insert_multi".into(), "[Insert *]".into());

    m.insert("editor.mode_jump".into(), "[Jump]".into());

    m.insert(
        "editor.mode_jump_range".into(),
        "บรรทัด {} อยู่นอกช่วง (1-{})".into(),
    );

    m.insert("editor.mode_normal".into(), "[Normal]".into());

    m.insert("editor.mode_gmode".into(), "[Gmode]".into());

    m.insert("editor.mode_nfe".into(), "[Nfe]".into());

    m.insert("editor.hint_nfe".into(), " hjkl/wasd/ลูกศร เลื่อน · Enter เปิด · n สร้างไฟล์ · m สร้างไดเรกทอรี · r เปลี่ยนชื่อ · d ล้าง · i/v แก้ไข · :=ขนาด/Tabs · Esc กลับ".into());

    m.insert(
        "editor.hint_nfe_cmd_actions".into(),
        "Enter=ใช้ Esc=ยกเลิก · :i/:v แก้ไข".into(),
    );

    m.insert("editor.nfe_unknown".into(), "คำสั่ง nfe ไม่รู้จัก: '{}' (ลอง size full|def|N% | n <ไฟล์> | m <ไดเรกทอรี> | rmd <ไดเรกทอรี> | rmf <ไฟล์> | i | v)".into());

    m.insert(
        "editor.nfe_rm_usage".into(),
        "วิธีใช้: rmd <ไดเรกทอรี> | rmf <ไฟล์> (เทียบกับราก sidebar)".into(),
    );

    m.insert("editor.nfe_rmd_not_dir".into(), "ไม่ใช่ไดเรกทอรี: {}".into());

    m.insert(
        "editor.nfe_rmf_is_dir".into(),
        "เป็นไดเรกทอรี (ใช้ rmd): {}".into(),
    );

    m.insert("editor.nfe_rm_not_exist".into(), "ไม่พบ: {}".into());

    m.insert("editor.nfe_removed".into(), "ลบแล้ว: {}".into());

    m.insert("editor.nfe_rm_failed".into(), "ลบไม่สำเร็จ: {}".into());

    m.insert(
        "editor.nfe_rm_has_unsaved".into(),
        "'{}' มีงานที่ยังไม่บันทึก — บันทึกก่อน".into(),
    );

    m.insert(
        "editor.nfe_new_usage".into(),
        "วิธีใช้: n <ไฟล์> | m <ไดเรกทอรี> (เทียบกับราก sidebar)".into(),
    );

    m.insert("editor.nfe_created_file".into(), "สร้างไฟล์แล้ว: {}".into());

    m.insert(
        "editor.nfe_created_dir".into(),
        "สร้างไดเรกทอรีแล้ว: {}".into(),
    );

    m.insert("editor.nfe_create_exists".into(), "มีอยู่แล้ว: {}".into());

    m.insert("editor.nfe_create_failed".into(), "สร้างล้มเหลว: {}".into());

    m.insert(
        "editor.nfe_size_usage".into(),
        "วิธีใช้: size full|def|N% (ได้ '{}')".into(),
    );

    m.insert("editor.nfe_size_set".into(), "ความกว้างแถบด้านข้าง: {}".into());

    m.insert(
        "editor.nfe_size_default".into(),
        "ค่าเริ่มต้น (30 คอลัมน์)".into(),
    );

    m.insert("editor.nfe_size_full".into(), "เต็มจอ".into());

    m.insert("editor.mode_query".into(), "[Query]".into());

    m.insert("editor.mode_run".into(), "[Run]".into());

    m.insert("editor.mode_run_err".into(), "run: {}".into());

    m.insert("editor.mode_search".into(), "[Search]".into());

    m.insert("editor.mode_term_copied".into(), "คัดลอก {} อักขระแล้ว".into());

    m.insert("editor.mode_terminal".into(), "[Term]".into());

    m.insert(
        "editor.mode_terminal_err".into(),
        "ข้อผิดพลาดเทอร์มินัล: {}".into(),
    );

    m.insert("editor.mode_visual".into(), "[Visual]".into());

    m.insert(
        "editor.mode_visual_start".into(),
        "โหมด Visual — ใช้ปุ่มลูกศรเพื่อเลือก Esc เพื่อออก".into(),
    );

    m.insert(
        "editor.mode_zoom_hint".into(),
        "ซูม: Ctrl++ / Ctrl+- ในโปรแกรมจำลองเทอร์มินัล".into(),
    );

    m.insert("editor.modified_marker".into(), " \u{25CF}".into());

    m.insert("editor.mouse_closed_all".into(), "ปิดแท็บทั้งหมดแล้ว".into());

    m.insert("editor.nav_line_selected".into(), "เลือกบรรทัดแล้ว".into());

    m.insert("editor.nav_selected_all".into(), "เลือกทั้งหมดแล้ว".into());

    m.insert(
        "editor.nav_split_cursor".into(),
        "แยกเคอร์เซอร์ (รวม {})".into(),
    );

    m.insert("editor.no_subdirs".into(), "ไม่มีไดเรกทอรีย่อย".into());

    m.insert("editor.no_teleports".into(), "ไม่มีจุดบันทึกเทเลพอร์ต".into());

    m.insert("editor.nothing_redo".into(), "ไม่มีอะไรให้ทำซ้ำ".into());

    m.insert("editor.nothing_undo".into(), "ไม่มีอะไรให้เลิกทำ".into());

    m.insert("editor.opened_file".into(), "เปิด {} แล้ว".into());

    m.insert("editor.panel_executing".into(), "กำลังเรียก:".into());

    m.insert("editor.panel_lines".into(), "บรรทัด".into());

    m.insert("editor.panel_run".into(), "RUN:".into());

    m.insert("editor.panel_run_hint".into(), "j/k เลื่อน, Esc/q ปิด".into());

    m.insert("editor.panel_run_output".into(), "ผลลัพธ์การเรียก".into());

    m.insert("editor.panel_terminal".into(), "เทอร์มินัล".into());

    m.insert("editor.pattern_not_found".into(), "ไม่พบรูปแบบ: {}".into());

    m.insert(
        "editor.query_add_format".into(),
        "รูปแบบ: /add <ข้อความ> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_add_position".into(),
        "ตำแหน่งต้องเป็น #end, #start หรือ #hm".into(),
    );

    m.insert(
        "editor.query_add_usage".into(),
        "/add <ข้อความ> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_added_text".into(),
        "เพิ่มข้อความไปยัง {} บรรทัดแล้ว".into(),
    );

    m.insert(
        "editor.query_rm_usage".into(),
        "/rm <ข้อความ> from #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_rm_format".into(),
        "รูปแบบ: /rm <ข้อความ> from #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_removed_text".into(),
        "ลบออกจาก {} บรรทัดแล้ว ข้าม {}".into(),
    );

    m.insert(
        "editor.visual_dedent_blocked".into(),
        "Alt+h ต้องการ indent ทุกบรรทัดที่เลือก".into(),
    );

    m.insert(
        "editor.query_alias_not_found".into(),
        "ไม่พบ alias: {}".into(),
    );

    m.insert(
        "editor.query_empty_pattern".into(),
        "รูปแบบการค้นหาว่างเปล่า".into(),
    );

    m.insert(
        "editor.query_l_usage".into(),
        "/l <บรรทัด> — ไปที่บรรทัด".into(),
    );

    m.insert(
        "editor.query_lowercased".into(),
        "แปลงส่วนที่เลือกเป็นตัวพิมพ์เล็กแล้ว".into(),
    );

    m.insert(
        "editor.query_no_matches".into(),
        "ไม่มีการตรงกันสำหรับ '{}'".into(),
    );

    m.insert(
        "editor.query_no_pattern_matches".into(),
        "ไม่มีบรรทัดใดมี '{}'".into(),
    );

    m.insert(
        "editor.query_qal_add_usage".into(),
        "/qal add <ชื่อ> <query>".into(),
    );

    m.insert("editor.query_qal_added".into(), "เพิ่ม alias '{}' แล้ว".into());

    m.insert(
        "editor.query_qal_edit_usage".into(),
        "/qal edit <ชื่อ> <query_ใหม่>".into(),
    );

    m.insert(
        "editor.query_qal_exists".into(),
        "มี alias '{}' แล้ว (ใช้ /qal edit)".into(),
    );

    m.insert(
        "editor.query_qal_help".into(),
        "/qal: add|rm|info|edit|rnm|list|run".into(),
    );

    m.insert(
        "editor.query_qal_info_usage".into(),
        "/qal info <ชื่อ>".into(),
    );

    m.insert("editor.query_qal_list".into(), "aliases: {}".into());

    m.insert(
        "editor.query_qal_no_aliases".into(),
        "ไม่มี query aliases".into(),
    );

    m.insert(
        "editor.query_qal_not_found".into(),
        "ไม่พบ alias '{}'".into(),
    );

    m.insert(
        "editor.query_qal_removed".into(),
        "ลบ alias '{}' แล้ว".into(),
    );

    m.insert("editor.query_qal_reserved".into(), "ชื่อสงวน: '{}'".into());

    m.insert(
        "editor.query_qal_renamed".into(),
        "เปลี่ยนชื่อ '{}' เป็น '{}'".into(),
    );

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <ชื่อ>".into());

    m.insert(
        "editor.query_qal_rnm_usage".into(),
        "/qal rnm <เก่า> to <ใหม่>".into(),
    );

    m.insert(
        "editor.query_qal_updated".into(),
        "อัปเดต alias '{}' แล้ว".into(),
    );

    m.insert(
        "editor.query_replace_format".into(),
        "รูปแบบ: /p <ค้นหา> to <แทนที่>".into(),
    );

    m.insert(
        "editor.query_replace_usage".into(),
        "/p <ค้นหา> to <แทนที่>".into(),
    );

    m.insert(
        "editor.query_replaced_count".into(),
        "แทนที่ {} การเกิดแล้ว".into(),
    );

    m.insert(
        "editor.query_rp_usage".into(),
        "/rp <ค้นหา> <แทนที่> — แทนที่ในการเลือก".into(),
    );

    m.insert(
        "editor.query_s_usage".into(),
        "/s \"ข้อความ\" — เลือกการเกิดถัดไป (/s เปล่า = คำใต้เคอร์เซอร์)".into(),
    );

    m.insert(
        "editor.query_sa_usage".into(),
        "/sa \"ข้อความ\" — เลือกทุกการเกิด".into(),
    );

    m.insert(
        "editor.query_selected_all_lines".into(),
        "เลือกทุกบรรทัดแล้ว".into(),
    );

    m.insert(
        "editor.query_selected_line".into(),
        "เลือกบรรทัด {} แล้ว".into(),
    );

    m.insert(
        "editor.query_selected_lines".into(),
        "เลือก {} บรรทัดแล้ว".into(),
    );

    m.insert(
        "editor.query_selected_lines_range".into(),
        "เลือกบรรทัด {}-{} แล้ว".into(),
    );

    m.insert(
        "editor.query_selected_next".into(),
        "เลือก '{}' ถัดไปแล้ว".into(),
    );

    m.insert(
        "editor.query_selected_occurrences".into(),
        "เลือก {} การเกิดของ '{}' แล้ว".into(),
    );

    m.insert("editor.query_selected_word".into(), "เลือกคำแล้ว".into());

    m.insert(
        "editor.query_selection_cleared".into(),
        "ล้างการเลือกแล้ว".into(),
    );

    m.insert(
        "editor.query_selection_required".into(),
        "ต้องมีการเลือกก่อน (เลือกด้วย /s, /sa, /sl หรือ /sal)".into(),
    );

    m.insert(
        "editor.query_set_alignment".into(),
        "ตั้งการจัดตำแหน่งบน {} บรรทัดแล้ว".into(),
    );

    m.insert(
        "editor.query_set_format".into(),
        "รูปแบบ: /set <รูปแบบ> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_set_usage".into(),
        "/set <รูปแบบ> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_sl_usage".into(),
        "/sl <บรรทัด> [บรรทัด] | <บรรทัด> , ... — เลือกบรรทัด".into(),
    );

    m.insert("editor.query_unknown".into(), "query ไม่รู้จัก: {}".into());

    m.insert(
        "editor.query_uppercased".into(),
        "แปลงส่วนที่เลือกเป็นตัวพิมพ์ใหญ่แล้ว".into(),
    );

    m.insert("editor.redo".into(), "ทำซ้ำ".into());

    m.insert("editor.rmd_missing".into(), "rmd: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.rmf_missing".into(), "rmf: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.run_exited".into(), "โพรเซสสิ้นสุดด้วยรหัส {}".into());

    m.insert("editor.run_missing".into(), "run: ขาดคำสั่ง".into());

    m.insert("editor.run_terminated".into(), "โพรเซสสิ้นสุดแล้ว".into());

    m.insert(
        "editor.save_auto_error".into(),
        "ข้อผิดพลาดการบันทึกอัตโนมัติ: {}".into(),
    );

    m.insert("editor.save_failed".into(), "บันทึกล้มเหลว: {}".into());

    m.insert("editor.encoding_changed".into(), "เปลี่ยนการเข้ารหัส".into());

    m.insert("editor.enc_title".into(), "การเข้ารหัส".into());

    m.insert(
        "editor.enc_hint".into(),
        "↑↓ เลือก · Enter ยืนยัน · Esc ยกเลิก".into(),
    );

    m.insert("editor.hint_encoding".into(), "การเข้ารหัส:".into());

    m.insert(
        "editor.encoding_reload_error".into(),
        "โหลดซ้ำล้มเหลว: {}".into(),
    );

    m.insert(
        "editor.encoding_applied_on_save".into(),
        "ใช้ในการบันทึกครั้งถัดไป".into(),
    );

    m.insert("editor.save_prompt".into(), "บันทึกไฟล์ [Y|N]?".into());

    m.insert("editor.saved".into(), "บันทึกแล้ว \u{2713}".into());

    m.insert("editor.saved_full".into(), " บันทึกแล้ว ✓".into());

    m.insert("editor.saved_marker".into(), " \u{2713}".into());

    m.insert("editor.scratch".into(), "[scratch]".into());

    m.insert("editor.scratch_label".into(), "[scratch]".into());

    m.insert(
        "editor.search_pattern_not_found".into(),
        "ไม่พบรูปแบบ: {}".into(),
    );

    m.insert("editor.selected_all".into(), "เลือกทั้งหมดแล้ว".into());

    m.insert("editor.status_modified".into(), "แก้ไขแล้ว".into());

    m.insert("editor.status_modified_tag".into(), " \u{25cf}".into());

    m.insert("editor.status_saved".into(), "บันทึกแล้ว".into());

    m.insert("editor.status_saved_tag".into(), " \u{2713}".into());

    m.insert("editor.syntax_off".into(), "ไฮไลต์ไวยากรณ์: OFF".into());

    m.insert("editor.syntax_on".into(), "ไฮไลต์ไวยากรณ์: ON".into());

    m.insert("editor.tab_cannot_open".into(), "ไม่สามารถเปิด {}".into());

    m.insert(
        "editor.tab_error_reading".into(),
        "ข้อผิดพลาดการอ่าน {}: {}".into(),
    );

    m.insert(
        "editor.tab_max_reached".into(),
        "ถึงจำนวนแท็บสูงสุด ({}) แล้ว".into(),
    );

    m.insert("editor.tab_opened".into(), "เปิด {} แล้ว".into());

    m.insert("editor.term_title".into(), "เทอร์มินัล".into());

    m.insert("editor.tmpl_last_saved".into(), "บันทึกล่าสุด: {}".into());

    m.insert(
        "editor.tmpl_recover_prompt".into(),
        "กู้คืนการเปลี่ยนแปลงที่ยังไม่บันทึก? [Y/N]: ".into(),
    );

    m.insert(
        "editor.tmpl_recovery_found".into(),
        "พบไฟล์กู้คืนหลังขัดข้องสำหรับ: {}".into(),
    );

    m.insert("editor.touch_missing".into(), "touch: ขาดอาร์กิวเมนต์".into());

    m.insert("editor.tp_add_missing".into(), "tp-add: ขาดชื่อ".into());

    m.insert("editor.undo".into(), "เลิกทำ".into());

    m.insert("editor.unknown_cmd".into(), "คำสั่งไม่รู้จัก: {}".into());

    m.insert("editor.untitled".into(), "untitled".into());

    m.insert("editor.untitled_label".into(), "untitled".into());

    m.insert(
        "editor.visual_hint".into(),
        "โหมด Visual \u{2014} ใช้ปุ่มลูกศรเพื่อเลือก Esc เพื่อออก".into(),
    );

    m.insert(
        "editor.welcome.title".into(),
        "ตัวแก้ไขโค้ดแบบเทอร์มินัลที่รวดเร็ว เขียนด้วย Rust.".into(),
    );

    m.insert(
        "editor.zoom_hint".into(),
        "ซูม: Ctrl++ / Ctrl+- ในโปรแกรมจำลองเทอร์มินัล".into(),
    );

    m.insert(
        "editor.nfe_confirm".into(),
        "ล้าง '{}' (y) ยืนยัน, (n) ยกเลิก? ".into(),
    );

    m.insert(
        "editor.nfe_rm_root".into(),
        "ไม่สามารถล้าง root ของ sidebar ได้.".into(),
    );
}
