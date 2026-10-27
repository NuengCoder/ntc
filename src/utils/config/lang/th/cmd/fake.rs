use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.fake.usage_title".into(),
        "fake - สร้างข้อมูลจำลอง/สุ่ม".into(),
    );

    m.insert(
        "cmd.fake.usage_gen".into(),
        "  fake gen <ชนิด> [-n N] [-f รูปแบบ] [-s ซีด] [-o ไฟล์] [-g กลุ่ม] [-l ภาษา] [-u [คอลัมน์]]"
            .into(),
    );

    m.insert(
        "cmd.fake.usage_types".into(),
        "  fake types [-s] [รูปแบบ]       แสดงชนิดทั้งหมด (ค้นหาแบบคลุมเครือด้วย -s)".into(),
    );

    m.insert(
        "cmd.fake.usage_locales".into(),
        "  fake locales                   แสดงรหัส --locale ที่รองรับ".into(),
    );

    m.insert(
        "cmd.fake.usage_group".into(),
        "  fake group <คำสั่งย่อย>        จัดการกลุ่มคอลัมน์ที่บันทึกไว้".into(),
    );

    m.insert(
        "cmd.fake.usage_help".into(),
        "  fake help                      แสดงความช่วยเหลือนี้".into(),
    );

    m.insert("cmd.fake.options_title".into(), "ตัวเลือก (fake gen):".into());

    m.insert(
        "cmd.fake.opt_count".into(),
        "  -n, --count <N>       จำนวนแถวที่จะสร้าง (ค่าเริ่มต้น 1)".into(),
    );

    m.insert(
        "cmd.fake.opt_format".into(),
        "  -f, --format <รูปแบบ> json | txt | sql | csv | xlsx (ค่าเริ่มต้น txt)".into(),
    );

    m.insert(
        "cmd.fake.opt_seed".into(),
        "  -s, --seed <N>        ซีดสำหรับผลลัพธ์ที่ทำซ้ำได้".into(),
    );

    m.insert(
        "cmd.fake.opt_output".into(),
        "  -o, --output <ไฟล์>   เขียนลงไฟล์ (นามสกุลกำหนดรูปแบบ)".into(),
    );

    m.insert(
        "cmd.fake.opt_group".into(),
        "  -g, --group <ชื่อ>    สร้างจากกลุ่มที่บันทึกไว้แทน <ชนิด>".into(),
    );

    m.insert(
        "cmd.fake.opt_locale".into(),
        "  -l, --locale <รหัส>   ข้อมูลตามภาษา: en_US en_GB zh_CN th_TH lo_LA".into(),
    );

    m.insert(
        "cmd.fake.opt_unique".into(),
        "  -u, --unique [คอลัมน์] ทำให้ทั้งแถว (หรือคอลัมน์ที่ระบุ) ไม่ซ้ำ".into(),
    );

    m.insert("cmd.fake.examples_header".into(), "ตัวอย่าง:".into());

    m.insert(
        "cmd.fake.example1".into(),
        "  fake gen name,email,city -n 3".into(),
    );

    m.insert(
        "cmd.fake.example2".into(),
        "  fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "cmd.fake.example3".into(),
        "  fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.example4".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อย fake: '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.wrote".into(),
        "เขียนแล้ว {} แถวไปที่ {} ({})".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_ext".into(),
        "ไม่รู้จักนามสกุลไฟล์ผลลัพธ์ '{}' (ใช้ -f เพื่อกำหนดรูปแบบ)".into(),
    );

    m.insert(
        "cmd.fake.gen.xlsx_needs_output".into(),
        "ผลลัพธ์ xlsx ต้องใช้ -o <ไฟล์.xlsx> (ไม่สามารถพิมพ์สมุดงานไบนารีไปยังเทอร์มินัลได้)".into(),
    );

    m.insert(
        "cmd.fake.gen.count_usage".into(),
        "fake gen: -n/--count ต้องระบุตัวเลข".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_count".into(),
        "fake gen: จำนวนไม่ถูกต้อง '{}' (ต้องเป็นจำนวนเต็มบวก)".into(),
    );

    m.insert(
        "cmd.fake.gen.format_usage".into(),
        "fake gen: -f/--format ต้องระบุค่า".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_format".into(),
        "fake gen: ไม่รู้จักรูปแบบ '{}' (json|txt|sql|csv|xlsx)".into(),
    );

    m.insert(
        "cmd.fake.gen.seed_usage".into(),
        "fake gen: -s/--seed ต้องระบุตัวเลข".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_seed".into(),
        "fake gen: ซีดไม่ถูกต้อง '{}' (ต้องเป็นจำนวนเต็ม)".into(),
    );

    m.insert(
        "cmd.fake.gen.output_usage".into(),
        "fake gen: -o/--output ต้องระบุเส้นทางไฟล์".into(),
    );

    m.insert(
        "cmd.fake.gen.group_usage".into(),
        "fake gen: -g/--group ต้องระบุชื่อกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_flag".into(),
        "fake gen: ตัวเลือกที่ไม่รู้จัก '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.group_and_types".into(),
        "fake gen: <ชนิด> และ -g/--group ใช้ร่วมกันไม่ได้".into(),
    );

    m.insert(
        "cmd.fake.gen.need_input".into(),
        "fake gen: ระบุชนิดอย่างน้อยหนึ่งชนิด หรือใช้ -g/--group".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_type".into(),
        "fake gen: ไม่รู้จักชนิด '{}' (รัน 'fake types' เพื่อดูทั้งหมด)".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_with_group".into(),
        "fake gen: ใช้ --unique ร่วมกับ -g/--group ไม่ได้ (ให้ใส่ '!' ต่อท้ายคอลัมน์ในกลุ่มแทน)".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_unknown_col".into(),
        "fake gen: --unique ระบุคอลัมน์ที่ไม่รู้จัก '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_exhausted".into(),
        "fake gen: ไม่สามารถสร้างค่าที่ไม่ซ้ำสำหรับ '{}' ได้ (พื้นที่ค่าเล็กเกินไป)".into(),
    );

    m.insert(
        "cmd.fake.gen.locale_usage".into(),
        "fake gen: -l/--locale ต้องระบุรหัสภาษา (en_US en_GB zh_CN th_TH lo_LA)".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_locale".into(),
        "fake gen: ไม่รู้จักภาษา '{}' (en_US en_GB zh_CN th_TH lo_LA)".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_cycle".into(),
        "fake gen: การอ้างอิงกลุ่มเป็นวงจรผ่าน '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_unknown_col".into(),
        "fake gen: ไม่พบคอลัมน์ที่อ้างอิง '{}' ในกลุ่ม '{}'".into(),
    );

    m.insert("cmd.fake.types.title".into(), "ชนิด fake ที่ใช้ได้ ({})".into());

    m.insert("cmd.fake.types.none".into(), "ไม่มีชนิดที่ตรงกับ '{}'".into());

    m.insert(
        "cmd.fake.types.matches".into(),
        "ชนิดที่ตรงกับ '{}' ({}):".into(),
    );

    m.insert("cmd.fake.locales.title".into(), "โลแคลที่รองรับ ({}):".into());

    m.insert(
        "cmd.fake.locales.note".into(),
        "ใช้กับ: fake gen <ชนิด> --locale <รหัส>".into(),
    );

    m.insert(
        "cmd.fake.group.usage_title".into(),
        "fake group - จัดการกลุ่มคอลัมน์ที่บันทึกไว้:".into(),
    );

    m.insert(
        "cmd.fake.group.help_add".into(),
        "  fake group add <ชื่อ> [นามแฝง as ชนิด, ...]    สร้างกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.group.help_edit".into(),
        "  fake group edit <ชื่อ> [นามแฝง as ชนิด, ...]   แทนที่คอลัมน์ของกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.group.help_rm".into(),
        "  fake group rm <ชื่อ>                            ลบกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.group.help_rnm".into(),
        "  fake group rnm <เก่า> <ใหม่>                    เปลี่ยนชื่อกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.group.help_ls".into(),
        "  fake group ls [รูปแบบ]                          แสดงกลุ่ม (ไวลด์การ์ด เช่น u*)".into(),
    );

    m.insert(
        "cmd.fake.group.help_info".into(),
        "  fake group info <ชื่อ>                          แสดงคอลัมน์ของกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.group.help_clear".into(),
        "  fake group clear [-f]                           ลบทุกกลุ่ม".into(),
    );

    m.insert(
        "cmd.fake.group.help_export".into(),
        "  fake group export <ชื่อ>|--all [-o ไฟล์]        ส่งออกกลุ่มเป็น .ntc_fake".into(),
    );

    m.insert(
        "cmd.fake.group.help_import".into(),
        "  fake group import <ไฟล์> [--force] [--prefix p]  นำเข้ากลุ่มจากไฟล์".into(),
    );

    m.insert(
        "cmd.fake.group.example1".into(),
        "  fake group add users [username as name, email, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.example2".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_sub".into(),
        "ไม่รู้จักคำสั่งย่อย 'fake group': '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.no_columns".into(),
        "ระบุคอลัมน์อย่างน้อยหนึ่งคอลัมน์ เช่น [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_column".into(),
        "ไม่สามารถแยกวิเคราะห์คอลัมน์ '{}' (ต้องเป็น 'นามแฝง as ชนิด')".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_type".into(),
        "ไม่รู้จักชนิด '{}' (รัน 'fake types' เพื่อดูทั้งหมด)".into(),
    );

    m.insert(
        "cmd.fake.group.add_usage".into(),
        "วิธีใช้: fake group add <ชื่อ> [นามแฝง as ชนิด, ...]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_name".into(),
        "ชื่อกลุ่มไม่ถูกต้อง '{}' (ใช้ตัวอักษร ตัวเลข '_' หรือ '-')".into(),
    );

    m.insert(
        "cmd.fake.group.not_found".into(),
        "ไม่พบกลุ่ม fake ชื่อ '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.exists".into(),
        "มีกลุ่ม fake '{}' อยู่แล้ว".into(),
    );

    m.insert("cmd.fake.group.scope_local".into(), "ภายในเครื่อง".into());

    m.insert("cmd.fake.group.scope_global".into(), "ส่วนกลาง".into());

    m.insert(
        "cmd.fake.group.added".into(),
        "เพิ่มกลุ่ม '{}' พร้อม {} คอลัมน์ไปยังการตั้งค่า{}แล้ว".into(),
    );

    m.insert(
        "cmd.fake.group.updated".into(),
        "อัปเดตกลุ่ม '{}' พร้อม {} คอลัมน์ในการตั้งค่า{}แล้ว".into(),
    );

    m.insert(
        "cmd.fake.group.rm_usage".into(),
        "วิธีใช้: fake group rm <ชื่อ>".into(),
    );

    m.insert("cmd.fake.group.removed".into(), "ลบกลุ่ม '{}' แล้ว".into());

    m.insert(
        "cmd.fake.group.rnm_usage".into(),
        "วิธีใช้: fake group rnm <เก่า> <ใหม่>".into(),
    );

    m.insert(
        "cmd.fake.group.renamed".into(),
        "เปลี่ยนชื่อกลุ่ม '{}' เป็น '{}' แล้ว".into(),
    );

    m.insert(
        "cmd.fake.group.ls_empty".into(),
        "ยังไม่มีกลุ่ม fake ที่บันทึกไว้".into(),
    );

    m.insert("cmd.fake.group.ls_none".into(), "ไม่มีกลุ่มที่ตรงกับ '{}'".into());

    m.insert(
        "cmd.fake.group.info_usage".into(),
        "วิธีใช้: fake group info <ชื่อ>".into(),
    );

    m.insert("cmd.fake.group.info_title".into(), "กลุ่ม '{}'".into());

    m.insert("cmd.fake.group.info_count".into(), "{} คอลัมน์".into());

    m.insert(
        "cmd.fake.group.clear_empty".into(),
        "ไม่มีกลุ่ม fake ที่จะล้าง".into(),
    );

    m.insert(
        "cmd.fake.group.clear_prompt".into(),
        "ลบ {} กลุ่ม fake หรือไม่? [y/N] ".into(),
    );

    m.insert("cmd.fake.group.clear_cancelled".into(), "ยกเลิกแล้ว".into());

    m.insert(
        "cmd.fake.group.cleared".into(),
        "ล้าง {} กลุ่ม fake แล้ว".into(),
    );

    m.insert(
        "cmd.fake.group.bad_ref".into(),
        "การอ้างอิงไม่ถูกต้อง '{}' (ต้องเป็น ref:<กลุ่ม>.<คอลัมน์>)".into(),
    );

    m.insert(
        "cmd.fake.group.ref_col_missing".into(),
        "ไม่พบคอลัมน์ที่อ้างอิง '{}' ในกลุ่ม '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.export_usage".into(),
        "วิธีใช้: fake group export <ชื่อ>|--all [-o ไฟล์]".into(),
    );

    m.insert(
        "cmd.fake.group.import_usage".into(),
        "วิธีใช้: fake group import <ไฟล์> [--force] [--prefix p]".into(),
    );

    m.insert(
        "cmd.fake.group.exported".into(),
        "ส่งออก {} กลุ่มไปที่ {} แล้ว".into(),
    );

    m.insert("cmd.fake.group.import_ok".into(), "นำเข้า {} กลุ่มแล้ว".into());

    m.insert(
        "cmd.fake.group.import_exists".into(),
        "มีกลุ่มอยู่แล้ว: {} (ใช้ --force เพื่อเขียนทับ)".into(),
    );

    m.insert(
        "cmd.fake.group.import_bad_file".into(),
        "นำเข้ากลุ่มไม่สำเร็จ: {}".into(),
    );
}
