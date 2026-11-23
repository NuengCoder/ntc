use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "ภาพรวมคำสั่ง".into());

    m.insert(
        "help.compact.footer".into(),
        "พิมพ์ 'help <คำสั่ง>' เพื่อดูรายละเอียด, 'help --all' เพื่อดูทั้งหมด.".into(),
    );

    m.insert("help.compact.arc".into(), "สร้างไฟล์บีบอัด".into());

    m.insert(
        "help.compact.autoup".into(),
        "แสดงรายการเริ่มต้นทั้งหมด (CU/LM/SF) [Windows Only]".into(),
    );

    m.insert(
        "help.compact.back".into(),
        "กลับไปยังไดเรกทอรีหลัก (หรือ n ระดับ)".into(),
    );

    m.insert(
        "help.compact.backup".into(),
        "สร้างข้อมูลสำรองของโปรเจกต์ปัจจุบัน".into(),
    );

    m.insert(
        "help.compact.betw".into(),
        "แสดงบรรทัดที่ A ถึง B (เริ่มที่ 1 รวมปลาย)".into(),
    );

    m.insert("help.compact.care".into(), "หยุดการละเว้นไดเรกทอรี".into());

    m.insert("help.compact.cd".into(), "นำทางไปยังไดเรกทอรี".into());

    m.insert("help.compact.clear".into(), "ล้างหน้าจอ".into());

    m.insert("help.compact.clip".into(), "แสดงประวัติคลิปบอร์ด".into());

    m.insert("help.compact.config".into(), "แสดง/ตั้งค่าเส้นทางผลลัพธ์".into());

    m.insert(
        "help.compact.count".into(),
        "นับจำนวนครั้งที่ <needle> ปรากฏ".into(),
    );

    m.insert("help.compact.cp".into(), "คัดลอกไฟล์หรือไดเรกทอรี".into());

    m.insert("help.compact.cvcode".into(), "แปลงระหว่างรูปแบบ".into());

    m.insert(
        "help.compact.deadlock".into(),
        "  deadlock                          เปิด vault (ถามรหัสผ่านหลัก)".into(),
    );

    m.insert("help.compact.decode".into(), "ถอดรหัสไบนารี".into());

    m.insert(
        "help.compact.deinit".into(),
        "ลบไฟล์โปรเจกต์ (ถามยืนยันก่อนลบทุกครั้ง)".into(),
    );

    m.insert("help.compact.dl".into(), "ดาวน์โหลดไฟล์".into());

    m.insert("help.compact.dns".into(), "ค้นหา DNS".into());

    m.insert(
        "help.compact.doctor".into(),
        "ตรวจ config, PATH, ranfile และ ntpkg".into(),
    );

    m.insert("help.compact.ds".into(), "ค้นหาไดเรกทอรีตามชื่อ".into());

    m.insert("help.compact.encode".into(), "เข้ารหัสไบนารี (0101...)".into());

    m.insert("help.compact.exit".into(), "ออกจาก ntc".into());

    m.insert(
        "help.compact.explorer".into(),
        "เปิดตัวสำรวจไฟล์แบบเต็มหน้าจอ".into(),
    );

    m.insert(
        "help.compact.fake".into(),
        "  fake gen <ชนิด> [-n N] [-f รูปแบบ] [-s ซีด] [-o ไฟล์]   สร้างแถว".into(),
    );

    m.insert(
        "help.compact.fgo".into(),
        "ค้นหาไฟล์ นำทางไปยังไดเรกทอรีหลักของไฟล์ที่เลือก".into(),
    );

    m.insert(
        "help.compact.fmt".into(),
        "แปลง JSON เป็น YAML หรือ TOML".into(),
    );

    m.insert("help.compact.fs".into(), "ค้นหาไฟล์ตามชื่อ".into());

    m.insert(
        "help.compact.fsc".into(),
        "ค้นหาไฟล์ เลือกหนึ่งเพื่อแสดงและคัดลอกได้".into(),
    );

    m.insert("help.compact.git".into(), "แสดงสถานะ repo แบบอ่านง่าย".into());

    m.insert(
        "help.compact.godrive".into(),
        "สลับไดรฟ์หรือเมานต์ /mnt".into(),
    );

    m.insert(
        "help.compact.gos".into(),
        "แสดงรายการไดเรกทอรีย่อยและเลือกหนึ่ง".into(),
    );

    m.insert("help.compact.gosc".into(), "นำทางต่อเนื่อง".into());

    m.insert("help.compact.gs".into(), "ค้นหาเนื้อหาไฟล์ (grep ง่าย ๆ)".into());

    m.insert(
        "help.compact.head".into(),
        "แสดง N บรรทัดแรก (ค่าเริ่มต้น 10)".into(),
    );

    m.insert("help.compact.help".into(), "แสดงความช่วยเหลือนี้".into());

    m.insert("help.compact.http".into(), "ส่งคำขอ HTTP".into());

    m.insert(
        "help.compact.igcare".into(),
        "ส่งออกการตั้งค่าทั้งหมดไปยัง <ชื่อ>.ntc.igcare".into(),
    );

    m.insert(
        "help.compact.igcared".into(),
        "แสดงการตั้งค่า ignore/care ทั้งหมด".into(),
    );

    m.insert("help.compact.ignore".into(), "ละเว้นไดเรกทอรีตามชื่อ".into());

    m.insert(
        "help.compact.image".into(),
        "เลือกภาพจากไดเรกทอรีปัจจุบันแล้วแสดงในเทอร์มินัล".into(),
    );

    m.insert("help.compact.init".into(), "เริ่มต้นไฟล์โปรเจกต์".into());

    m.insert(
        "help.compact.ipcfg".into(),
        "แสดงอะแดปเตอร์เครือข่ายแบบ ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.compact.local".into(),
        "สร้างเทมเพลต ntconfig.toml (แบบ comment)".into(),
    );

    m.insert("help.compact.locate".into(), "ค้นหาทั้งไฟล์และไดเรกทอรี".into());

    m.insert("help.compact.math".into(), "ประเมินนิพจน์คณิตศาสตร์".into());

    m.insert("help.compact.mkd".into(), "สร้างไดเรกทอรีใหม่".into());

    m.insert("help.compact.mkf".into(), "สร้างไฟล์ใหม่".into());

    m.insert(
        "help.compact.mode".into(),
        "ออกและเปิด ntc ใหม่ในฐานะ Administrator".into(),
    );

    m.insert("help.compact.mv".into(), "ย้าย/เปลี่ยนชื่อไฟล์หรือไดเรกทอรี".into());

    m.insert(
        "help.compact.ndb".into(),
        "เปิด ndb database shell (.db browser) ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.compact.ne".into(),
        "เปิดไฟล์ที่มีอยู่แล้วในตัวแก้ไขข้อความในตัว".into(),
    );

    m.insert("help.compact.nms".into(), "เล่นเสียงในเทอร์มินัล".into());

    m.insert(
        "help.compact.nsheet".into(),
        "ดูตารางแบบเต็มจอ (CSV/TSV/JSON)".into(),
    );

    m.insert("help.compact.nssh".into(), "บันทึกโปรไฟล์ SSH".into());

    m.insert(
        "help.compact.ntpkg".into(),
        "ดาวน์โหลดและติดตั้ง runtime".into(),
    );

    m.insert("help.compact.ntpx".into(), "เรียกไบนารีของแพ็กเกจที่ติดตั้ง".into());

    m.insert(
        "help.compact.opencg".into(),
        "เปิด config.toml ในตัวแก้ไข".into(),
    );

    m.insert(
        "help.compact.pathcp".into(),
        "คัดลอกเส้นทางไดเรกทอรีปัจจุบันไปยังคลิปบอร์ด".into(),
    );

    m.insert("help.compact.ping".into(), "Ping โฮสต์".into());

    m.insert(
        "help.compact.pipe".into(),
        "ส่งเอาต์พุตของ a ไปเป็นอินพุตของ b [NON-COMMAND]".into(),
    );

    m.insert(
        "help.compact.py_activate".into(),
        "เปิดใช้งานสภาพแวดล้อม virtual Python (ตรวจจับอัตโนมัติ)".into(),
    );

    m.insert(
        "help.compact.py_deactivate".into(),
        "ปิดใช้งานสภาพแวดล้อม virtual ปัจจุบัน".into(),
    );

    m.insert(
        "help.compact.qr".into(),
        "เข้ารหัสข้อความและแสดง QR ในเทอร์มินัล".into(),
    );

    m.insert(
        "help.compact.qsc".into(),
        "เริ่มเดม่อน QSC (โพรเซสเบื้องหลัง)".into(),
    );

    m.insert("help.compact.qur".into(), "ค้นหาไฟล์ เลือกเนื้อหา แทนที่".into());

    m.insert("help.compact.ral".into(), "สร้าง alias".into());

    m.insert(
        "help.compact.ran".into(),
        "เรียก target 'standalone' (ค่าเริ่มต้น)".into(),
    );

    m.insert(
        "help.compact.report".into(),
        "บันทึกรายงานโปรเจกต์ไว้ที่ <file>".into(),
    );

    m.insert(
        "help.compact.resetcg".into(),
        "รีเซ็ตการกำหนดค่าเป็นค่าเริ่มต้น (มีตัวเลือกสำรอง)".into(),
    );

    m.insert(
        "help.compact.restorecg".into(),
        "คืนค่าการกำหนดค่าจากข้อมูลสำรอง".into(),
    );

    m.insert("help.compact.rev".into(), "กลับตัวอักษรของแต่ละบรรทัด".into());

    m.insert(
        "help.compact.rmd".into(),
        "ลบไดเรกทอรี; wildcard ลบทุกที่ตรงกันหลังยืนยัน".into(),
    );

    m.insert("help.compact.rmf".into(), "ลบไฟล์ (ถามยืนยัน)".into());

    m.insert("help.compact.rnmd".into(), "เปลี่ยนชื่อไดเรกทอรี".into());

    m.insert("help.compact.rnmf".into(), "เปลี่ยนชื่อไฟล์".into());

    m.insert("help.compact.run".into(), "เรียก alias ด้วย 'run'".into());

    m.insert(
        "help.compact.say".into(),
        "พิมพ์ข้อความ (ไม่จัดเส้นทางไฟล์ใหม่)".into(),
    );

    m.insert(
        "help.compact.nprintf".into(),
        "เอาต์พุตจัดรูปแบบสไตล์ Bash".into(),
    );

    m.insert(
        "help.compact.scan".into(),
        "แสดงไฟล์ตามนามสกุล (ไม่นับบรรทัด)".into(),
    );

    m.insert("help.compact.scr".into(), "เรียกใช้สคริปต์อัตโนมัติ".into());

    m.insert("help.compact.serve".into(), "เลือกบริการ".into());

    m.insert(
        "help.compact.showcg".into(),
        "แสดงภาพรวมการกำหนดค่าปัจจุบัน".into(),
    );

    m.insert(
        "help.compact.showenv".into(),
        "แสดงตัวแปรสภาพแวดล้อม (PATH 5 รายการแรก)".into(),
    );

    m.insert("help.compact.size".into(), "แสดงขนาดไดเรกทอรีปัจจุบัน".into());

    m.insert("help.compact.skill".into(), "ดาวน์โหลดสกิลจาก GitHub".into());

    m.insert("help.compact.skip".into(), "ข้ามบรรทัดแรกหรือบรรทัดที่เลือก".into());

    m.insert("help.compact.smtp".into(), "ส่งอีเมล (ไคลเอนต์ SMTP)".into());

    m.insert("help.compact.sort".into(), "เรียงบรรทัด".into());

    m.insert("help.compact.spec".into(), "แสดงสเปกคอมพิวเตอร์/ระบบ".into());

    m.insert("help.compact.sys".into(), "ตัวตรวจสอบระบบแบบสด".into());

    m.insert(
        "help.compact.syspath".into(),
        "แสดงรายการ PATH ระดับระบบ".into(),
    );

    m.insert(
        "help.compact.tail".into(),
        "แสดง N บรรทัดสุดท้าย (ค่าเริ่มต้น 10)".into(),
    );

    m.insert(
        "help.compact.theme".into(),
        "ตัวจัดการธีมแบบโต้ตอบ (เมนู)".into(),
    );

    m.insert(
        "help.compact.time".into(),
        "  time run <คำสั่ง>                       เรียก NTC command, alias หรือคำสั่งภายนอกและรายงานเวลา".into(),
    );

    m.insert("help.compact.tl".into(), "นับบรรทัดในไฟล์เดียว".into());

    m.insert(
        "help.compact.tmprt".into(),
        "ตรวจสอบอุณหภูมิฮาร์ดแวร์แบบสด".into(),
    );

    m.insert("help.compact.todo".into(), "แสดงงานทั้งหมด".into());

    m.insert("help.compact.tp".into(), "เมนูแบบโต้ตอบ".into());

    m.insert(
        "help.compact.tpb".into(),
        "เทเลพอร์ตกลับไปยังตำแหน่งก่อนหน้า (เลิกทำ tp)".into(),
    );

    m.insert("help.compact.trace".into(), "ติดตามเส้นทางทีละฮอป".into());

    m.insert(
        "help.compact.trim".into(),
        "ลบทุกครั้งที่พบ <pattern> (จากข้อความหรืออินพุตไปป์)".into(),
    );

    m.insert(
        "help.compact.tutorial".into(),
        "บทเรียนลงมือทำในแซนด์บ็อกซ์".into(),
    );

    m.insert(
        "help.compact.txtc".into(),
        "คัดลอกเนื้อหาไปยังคลิปบอร์ด (เปิด/ปิดเลขบรรทัด)".into(),
    );

    m.insert(
        "help.compact.txtf".into(),
        "แสดงเนื้อหาไฟล์หรือเลือกไฟล์จากไดเรกทอรี (เลขบรรทัดเดิม)".into(),
    );

    m.insert("help.compact.ui".into(), "แสดงโหมด UI ปัจจุบัน".into());

    m.insert("help.compact.up".into(), "อัปโหลดไฟล์".into());

    m.insert(
        "help.compact.usrpath".into(),
        "แสดงรายการ PATH ระดับผู้ใช้".into(),
    );

    m.insert("help.compact.version".into(), "แสดงเวอร์ชัน".into());

    m.insert("help.compact.view".into(), "แสดงต้นไม้หรือเนื้อหาไฟล์".into());

    m.insert("help.compact.viewd".into(), "แสดงต้นไม้เฉพาะไดเรกทอรี".into());

    m.insert(
        "help.compact.viewf".into(),
        "แสดงต้นไม้เฉพาะไฟล์ที่ความลึก 1".into(),
    );

    m.insert(
        "help.compact.views".into(),
        "แสดงหนึ่งระดับของไดเรกทอรี แบบแบนเหมือน ls".into(),
    );

    m.insert(
        "help.compact.where".into(),
        "แสดงพาธของ ntc หรือค้นหาโปรแกรมใน PATH".into(),
    );

    m.insert(
        "help.compact.wi".into(),
        "ฟีดการเปลี่ยนแปลงไฟล์แบบสด (q=ออก)".into(),
    );

    m.insert(
        "help.compact.line".into(),
        "ใส่เลขทุกบรรทัด".into(),
    );
}
