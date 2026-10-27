use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nsheet.usage_title".into(), "การใช้งาน:".into());

    m.insert(
        "cmd.nsheet.help_open".into(),
        "nsheet open <ไฟล์> [ตัวเลือก]   ดูแบบเต็มจอ (CSV/TSV/JSON/JSONL)".into(),
    );

    m.insert(
        "cmd.nsheet.help_view".into(),
        "nsheet view <ไฟล์> [ตัวเลือก]   พิมพ์ตาราง (ค่าเริ่มต้น 50 แถว, --all ทั้งหมด)".into(),
    );

    m.insert("cmd.nsheet.options_title".into(), "ตัวเลือก:".into());

    m.insert(
        "cmd.nsheet.help_delimiter".into(),
        "  -d, --delimiter <อักขระ|auto>  ตัวคั่นฟิลด์ (ค่าเริ่มต้น: ตรวจอัตโนมัติ , ; tab |)".into(),
    );

    m.insert(
        "cmd.nsheet.help_header".into(),
        "  --header / --no-header        บังคับมี/ไม่มีแถวหัว (ค่าเริ่มต้น: ตรวจอัตโนมัติ)".into(),
    );

    m.insert(
        "cmd.nsheet.help_limit".into(),
        "  -l, --limit <n> / --all        จำนวนแถวที่พิมพ์ในโหมด view".into(),
    );

    m.insert(
        "cmd.nsheet.help_sort".into(),
        "  --sort <คอลัมน์> [--desc]       เรียงตามชื่อคอลัมน์หรือลำดับที่ (เริ่มที่ 1)".into(),
    );

    m.insert(
        "cmd.nsheet.help_filter".into(),
        "  --filter <เงื่อนไข>           เช่น age>30, city~york, name=bob (!=,>=,<=,&&,||)".into(),
    );

    m.insert(
        "cmd.nsheet.help_export".into(),
        "  --export <json|md>            ส่งออกแทนการพิมพ์".into(),
    );

    m.insert(
        "cmd.nsheet.help_output".into(),
        "  -o, --output <ไฟล์>           ปลายทางส่งออก (ค่าเริ่มต้น: หน้าจอ)".into(),
    );

    m.insert(
        "cmd.nsheet.help_help".into(),
        "  --help, -h                    แสดงความช่วยเหลือนี้".into(),
    );

    m.insert("cmd.nsheet.examples_title".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.nsheet.example_open".into(),
        "nsheet open users.csv".into(),
    );

    m.insert(
        "cmd.nsheet.example_view".into(),
        "nsheet view users.csv -l 20 --sort age --desc".into(),
    );

    m.insert(
        "cmd.nsheet.example_filter".into(),
        "nsheet view data.json --filter \"age>30 && city~york\"".into(),
    );

    m.insert(
        "cmd.nsheet.example_export".into(),
        "nsheet view a.csv --export json -o out.json".into(),
    );

    m.insert(
        "cmd.nsheet.missing_file".into(),
        "ขาดไฟล์ วิธีใช้: nsheet open <ไฟล์> | nsheet view <ไฟล์>".into(),
    );

    m.insert("cmd.nsheet.file_not_found".into(), "ไม่พบไฟล์ '{}'".into());

    m.insert("cmd.nsheet.load_fail".into(), "โหลดตารางไม่ได้: {}".into());

    m.insert(
        "cmd.nsheet.empty_sheet".into(),
        "ตารางว่าง (ไม่พบคอลัมน์)".into(),
    );

    m.insert("cmd.nsheet.no_rows".into(), "ไม่มีแถวที่ตรง".into());

    m.insert("cmd.nsheet.unknown_option".into(), "ไม่รู้จัก: {}".into());

    m.insert(
        "cmd.nsheet.unknown_column".into(),
        "ไม่รู้จักคอลัมน์: '{}'".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_limit".into(),
        "จำนวนไม่ถูกต้อง: '{}'".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_delimiter".into(),
        "ตัวคั่นไม่ถูกต้อง: '{}' (ใช้อักขระเดียวหรือ auto)".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_export".into(),
        "รูปแบบส่งออกไม่ถูกต้อง: '{}' (ใช้ json หรือ md)".into(),
    );

    m.insert(
        "cmd.nsheet.delimiter_usage".into(),
        "วิธีใช้: --delimiter <อักขระ|auto>".into(),
    );

    m.insert(
        "cmd.nsheet.limit_usage".into(),
        "วิธีใช้: --limit <จำนวน>".into(),
    );

    m.insert(
        "cmd.nsheet.sort_usage".into(),
        "วิธีใช้: --sort <คอลัมน์>".into(),
    );

    m.insert(
        "cmd.nsheet.filter_usage".into(),
        "วิธีใช้: --filter <เงื่อนไข> (เช่น age>30)".into(),
    );

    m.insert(
        "cmd.nsheet.export_usage".into(),
        "วิธีใช้: --export <json|md>".into(),
    );

    m.insert(
        "cmd.nsheet.output_usage".into(),
        "วิธีใช้: --output <ไฟล์>".into(),
    );

    m.insert("cmd.nsheet.filter_fail".into(), "เงื่อนไขไม่ถูกต้อง: {}".into());

    m.insert("cmd.nsheet.export_fail".into(), "ส่งออกล้มเหลว: {}".into());

    m.insert("cmd.nsheet.exported".into(), "ส่งออกแล้ว: {}".into());

    m.insert(
        "cmd.nsheet.rows_shown".into(),
        "แสดง {} จาก {} แถว ({} คอลัมน์)".into(),
    );

    m.insert(
        "cmd.nsheet.rows_truncated".into(),
        "ตัดเหลือ {} แถว (ใช้ --all หรือ -l <จำนวน>)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_hint".into(),
        "ลูกศร/hjkl เลื่อน · / ค้นหา · n ถัดไป · s เรียง · f กรอง · c ล้าง · e ส่งออก · Enter ดู · q ออก"
            .into(),
    );

    m.insert(
        "cmd.nsheet.tui_search_prompt".into(),
        "ค้นหา (ว่าง = ล้าง)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_sort_prompt".into(),
        "เรียงตามคอลัมน์ (ท้าย ! = มากไปน้อย, ว่าง = ล้าง)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_filter_prompt".into(),
        "กรอง (ว่าง = ล้าง)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_export_prompt".into(),
        "ส่งออกไปยัง (พาธ .json/.md)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_cell_title".into(),
        "เซลล์ (กดปุ่มใดก็ได้เพื่อปิด)".into(),
    );

    m.insert("cmd.nsheet.tui_popup_hint".into(), "กดปุ่มใดก็ได้เพื่อปิด".into());

    m.insert("cmd.nsheet.tui_no_matches".into(), "ไม่พบที่ตรง".into());

    m.insert(
        "cmd.nsheet.tui_large".into(),
        "ตารางใหญ่ ({} แถว) — อาจเลื่อนช้า".into(),
    );
}
