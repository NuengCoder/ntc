use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "clap.arg.cared.help".into(),
        "หยุดการละเว้นไดเรกทอรีหนึ่งรายการขึ้นไป".into(),
    );

    m.insert(
        "clap.arg.caref.help".into(),
        "สนใจนามสกุลไฟล์หนึ่งรายการขึ้นไป".into(),
    );

    m.insert(
        "clap.arg.caren.help".into(),
        "สนใจไฟล์เฉพาะหนึ่งรายการขึ้นไป".into(),
    );

    m.insert("clap.arg.clear.help".into(), "ล้างหน้าจอเทอร์มินัล".into());

    m.insert(
        "clap.arg.copy.help".into(),
        "คัดลอกรายงานไปยังคลิปบอร์ดแทนการบันทึกเป็นไฟล์".into(),
    );

    m.insert(
        "clap.arg.edit.help".into(),
        "เปิดไฟล์ในตัวแก้ไขข้อความในตัว".into(),
    );

    m.insert(
        "clap.arg.explorer.help".into(),
        "เปิดตัวสำรวจไฟล์ NTC (เส้นทางเริ่มต้นไม่บังคับ)".into(),
    );

    m.insert(
        "clap.arg.format_cli.help".into(),
        "รูปแบบผลลัพธ์: txt, html, json, md".into(),
    );

    m.insert(
        "clap.arg.generate_completions.help".into(),
        "สร้างสคริปต์ความสมบูรณ์ของเชลล์ (bash|zsh|fish|powershell)".into(),
    );

    m.insert(
        "clap.arg.help_arg.help".into(),
        "แสดงความช่วยเหลือโดยละเอียด".into(),
    );

    m.insert(
        "clap.arg.hyper.help".into(),
        "สร้างโครงสร้างโปรเจกต์ hyper (ไลบรารี DSA)".into(),
    );

    m.insert(
        "clap.arg.igcare_export_all.help".into(),
        "ส่งออกการตั้งค่า ignore/care ทั้งหมดไปยัง <name>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_export_select.help".into(),
        "เลือกหมวดหมู่เพื่อส่งออกไปยัง <name>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_import.help".into(),
        "นำเข้าการตั้งค่า ignore/care จากไฟล์ .ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.ignore.help".into(),
        "ละเว้นชื่อไดเรกทอรีหนึ่งรายการขึ้นไป".into(),
    );

    m.insert("clap.arg.ignored.help".into(), "แสดงรายการที่ถูกละเว้น".into());

    m.insert(
        "clap.arg.ignoref.help".into(),
        "ละเว้นนามสกุลไฟล์หนึ่งรายการขึ้นไป".into(),
    );

    m.insert(
        "clap.arg.ignoren.help".into(),
        "ละเว้นไฟล์เฉพาะหนึ่งรายการขึ้นไป".into(),
    );

    m.insert(
        "clap.arg.image.help".into(),
        "แปลงภาพเป็นเอกสาร (JSON, MD, HTML, TXT)".into(),
    );

    m.insert(
        "clap.arg.image_to.help".into(),
        "รูปแบบผลลัพธ์สำหรับการแปลงภาพ: json, md, html, txt".into(),
    );

    m.insert(
        "clap.arg.init.help".into(),
        "สร้างไฟล์เริ่มต้นจากเทมเพลตและเปิดในตัวแก้ไข".into(),
    );

    m.insert(
        "clap.arg.input.help".into(),
        "เส้นทางไฟล์หรือไดเรกทอรีอินพุต".into(),
    );

    m.insert(
        "clap.arg.lang_cli.help".into(),
        "ภาษาสำหรับโปรเจกต์ hyper".into(),
    );

    m.insert(
        "clap.arg.list.help".into(),
        "แสดงรายการฟังก์ชันบรรทัดคำสั่งทั้งหมด".into(),
    );

    m.insert(
        "clap.arg.lsp.help".into(),
        "เริ่มเซิร์ฟเวอร์ LSP ของ ntc-math ผ่าน stdio".into(),
    );

    m.insert(
        "clap.arg.lsp_log.help".into(),
        "เส้นทางสำหรับเขียนบันทึกการดีบัก LSP".into(),
    );

    m.insert("clap.arg.math.help".into(), "ประเมินนิพจน์คณิตศาสตร์".into());

    m.insert(
        "clap.arg.output.help".into(),
        "ชื่อไฟล์ผลลัพธ์สำหรับรายงาน".into(),
    );

    m.insert(
        "clap.arg.ral_export_all.help".into(),
        "ส่งออก run alias ทั้งหมดไปยัง <name>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_export_select.help".into(),
        "เลือก run alias เพื่อส่งออกไปยัง <name>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_import.help".into(),
        "นำเข้า run alias จากไฟล์ .ntc.ral".into(),
    );

    m.insert("clap.arg.say.help".into(), "พิมพ์ข้อความไปยัง stdout".into());

    m.insert(
        "clap.arg.setA.help".into(),
        "แสดงหรือตั้งค่า autosuggest (ข้อความเลือน) (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setC.help".into(),
        "แสดงหรือตั้งค่าสีผลลัพธ์ (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setD.help".into(),
        "แสดงหรือตั้งค่าความลึกการเรียกซ้ำสูงสุด".into(),
    );

    m.insert(
        "clap.arg.setH.help".into(),
        "แสดงหรือตั้งค่าประวัติ (path/ON/OFF/default)".into(),
    );

    m.insert(
        "clap.arg.setL.help".into(),
        "แสดงหรือตั้งค่าการแสดงหมายเลขบรรทัด (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setO.help".into(),
        "แสดงหรือตั้งค่าไดเรกทอรีผลลัพธ์เริ่มต้น".into(),
    );

    m.insert("clap.arg.setT.help".into(), "แสดงหรือตั้งค่าจำนวนเธรด".into());

    m.insert(
        "clap.arg.showcg.help".into(),
        "แสดงภาพรวมการกำหนดค่าปัจจุบัน".into(),
    );

    m.insert(
        "clap.arg.size_cli.help".into(),
        "แสดงขนาดไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "clap.arg.time.help".into(),
        "แสดงเวลาที่ใช้ (ใช้ร่วมกับ --size หรือ --view)".into(),
    );

    m.insert(
        "clap.arg.tp_add.help".into(),
        "บันทึกไดเรกทอรีปัจจุบันเป็นจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "clap.arg.tp_list.help".into(),
        "แสดงรายการจุดเทเลพอร์ตทั้งหมด".into(),
    );

    m.insert("clap.arg.tp_rm.help".into(), "ลบจุดเทเลพอร์ต".into());

    m.insert(
        "clap.arg.upgrade.help".into(),
        "ตรวจสอบอัปเดตและอัปเกรด ntc เป็นเวอร์ชันล่าสุด".into(),
    );

    m.insert("clap.arg.version.help".into(), "แสดงข้อมูลเวอร์ชัน".into());

    m.insert(
        "clap.arg.view_cli.help".into(),
        "ดูโครงสร้างไดเรกทอรีปัจจุบันอย่างรวดเร็ว".into(),
    );

    m.insert(
        "clap.arg.watch_cli.help".into(),
        "แสดงหรือตั้งค่าตัวเฝ้าดูไฟล์ (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.where_cli.help".into(),
        "แสดงตำแหน่งของไฟล์ปฏิบัติการ ntc".into(),
    );

    m.insert("clap.value_name.depth".into(), "DEPTH".into());

    m.insert("clap.value_name.expr".into(), "EXPR".into());

    m.insert("clap.value_name.ext".into(), "EXT".into());

    m.insert("clap.value_name.file".into(), "FILE".into());

    m.insert("clap.value_name.format_cli".into(), "FORMAT".into());

    m.insert("clap.value_name.image".into(), "IMAGE".into());

    m.insert("clap.value_name.lang".into(), "LANG".into());

    m.insert("clap.value_name.name".into(), "NAME".into());

    m.insert("clap.value_name.path".into(), "PATH".into());

    m.insert("clap.value_name.project".into(), "PROJECT".into());

    m.insert("clap.value_name.shell".into(), "SHELL".into());

    m.insert("clap.value_name.state".into(), "STATE".into());

    m.insert("clap.value_name.text".into(), "TEXT".into());

    m.insert("clap.value_name.threads".into(), "THREADS".into());

    m.insert("clap.value_name.value".into(), "VALUE".into());
}
