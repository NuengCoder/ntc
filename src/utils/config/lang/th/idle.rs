use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "idle.tp".into(),
        "ลอง: tp add <ชื่อโฟลเดอร์>  แล้ว: @<ชื่อโฟลเดอร์>   (บันทึกและกลับมาที่นี่)".into(),
    );

    m.insert(
        "idle.explorer".into(),
        "ลอง: nfe   (เปิดดูโฟลเดอร์ด้วย file explorer; f ค้นหาไฟล์)".into(),
    );

    m.insert(
        "idle.ne".into(),
        "ลอง: ne --init <ไฟล์>   (สร้างไฟล์จากเทมเพลต)".into(),
    );

    m.insert(
        "idle.view".into(),
        "ลอง: view -d 2   (แสดงโครงสร้างสองระดับ)".into(),
    );

    m.insert(
        "idle.search".into(),
        "ลอง: gs <ข้อความ>   (ค้นข้อความนั้นในไฟล์ของโฟลเดอร์นี้)".into(),
    );

    m.insert(
        "idle.todo".into(),
        "ลอง: todo add \"ชื่องาน\"   (โฟลเดอร์นี้มีรายการงานอยู่แล้ว)".into(),
    );

    m.insert(
        "idle.git".into(),
        "ลอง: git status   (นี่คือ git repository)".into(),
    );

    m.insert(
        "idle.serve".into(),
        "ลอง: serve files --lock   (แชร์โฟลเดอร์นี้แบบอ่านอย่างเดียวผ่าน HTTP)".into(),
    );

    m.insert(
        "idle.tutorial".into(),
        "ลอง: tutorial   (ฝึกใช้จริงใน sandbox ที่ลบทิ้งได้)".into(),
    );

    m.insert(
        "idle.ral".into(),
        "ลอง: ral add <ชื่อ> \"<คำสั่ง>\"   (พิมพ์ <ชื่อ> เพื่อรันอีกครั้ง)".into(),
    );

    m.insert(
        "idle.ran".into(),
        "ลอง: ran list   (โฟลเดอร์นี้มี NTCRANFILE.toml กำหนดงานไว้)".into(),
    );

    m.insert(
        "idle.pipe".into(),
        "ลอง: gs <ข้อความ> | sort -u | head 10   (ส่งผลลัพธ์ต่อไปยังคำสั่งอื่น)".into(),
    );

    m.insert(
        "idle.math".into(),
        "ลอง: math 3+4*5   (เครื่องคิดเลขพร้อมฟังก์ชัน ไม่ต้องใช้โปรแกรมอื่น)".into(),
    );

    m.insert(
        "idle.qr".into(),
        "ลอง: qr <ข้อความ|url>   (พิมพ์ QR code ให้สแกนได้)".into(),
    );

    m.insert(
        "idle.theme".into(),
        "ลอง: theme list   (แล้ว: theme <ชื่อ> - เปลี่ยนสีของ shell)".into(),
    );

    m.insert(
        "idle.backup".into(),
        "ลอง: backup create   (สำรองโฟลเดอร์นี้ก่อนแก้ risky)".into(),
    );

    m.insert(
        "idle.empty".into(),
        "ลอง: mkf notes.md   (โฟลเดอร์ว่าง - เริ่มจากไฟล์ก่อน)".into(),
    );

    m.insert(
        "idle.help".into(),
        "ลอง: help   (ดูคำสั่งทั้งหมด หรือ: help <หัวข้อ>)".into(),
    );

    m.insert(
        "idle.tl".into(),
        "ลอง: tl dir .   (นับจำนวนบรรทัดของไฟล์ที่รองรับในโฟลเดอร์นี้)".into(),
    );

    m.insert(
        "idle.size".into(),
        "ลอง: size   (ขนาดรวมของโฟลเดอร์นี้)".into(),
    );

    m.insert(
        "idle.report".into(),
        "ลอง: report project md   (สร้างรายงานโปรเจกต์เป็น markdown ไว้ที่นี่)".into(),
    );

    m.insert(
        "idle.scr".into(),
        "ลอง: scr init demo   (เริ่มสคริปต์ .ntc ที่ใช้ซ้ำได้)".into(),
    );

    m.insert(
        "idle.time".into(),
        "ลอง: time bench --runs 3 <คำสั่ง>   (วัดเวลาเฉลี่ยของคำสั่ง)".into(),
    );

    m.insert(
        "idle.encode".into(),
        "ลอง: encode b64 \"hello\"   (base64, hex, url และอื่น ๆ)".into(),
    );

    m.insert(
        "idle.fmt".into(),
        "ลอง: fmt Cargo.toml to json   (แปลงรูปแบบไฟล์ config)".into(),
    );

    m.insert(
        "idle.arc".into(),
        "ลอง: arc list <ไฟล์.zip>   (ดูรายการใน archive โดยไม่แตกไฟล์)".into(),
    );

    m.insert(
        "idle.scan".into(),
        "ลอง: scan ext rs   (แสดงไฟล์ .rs ทั้งหมดที่นี่)".into(),
    );

    m.insert(
        "idle.sys".into(),
        "ลอง: sys   (มอนิเตอร์ CPU, หน่วยความจำ และโปรเซสแบบสด)".into(),
    );

    m.insert(
        "idle.showcg".into(),
        "ลอง: showcg   (ดูการตั้งค่าปัจจุบันทั้งหมดในครั้งเดียว)".into(),
    );

    m.insert(
        "idle.ntpkg".into(),
        "ลอง: ntpkg list   (runtime ที่ติดตั้งผ่าน ntc)".into(),
    );

    m.insert(
        "idle.ndb".into(),
        "ลอง: ndb   (เปิดดูฐานข้อมูล SQLite ในโฟลเดอร์นี้)".into(),
    );

    m.insert(
        "idle.nsheet".into(),
        "ลอง: nsheet open <ไฟล์.csv>   (เปิดดูสเปรดชีต)".into(),
    );

    m.insert(
        "idle.fgo".into(),
        "ลอง: fgo task.txt   (ค้นไฟล์ตามชื่อแล้วกระโดดไป)".into(),
    );

    m.insert(
        "idle.back".into(),
        "ลอง: back   (ขึ้นหนึ่งโฟลเดอร์; gos, gosc, godrive ก็เป็นคำเดียว)".into(),
    );

    m.insert(
        "idle.viewf".into(),
        "ลอง: viewf .   (ดูเฉพาะไฟล์) · viewd . ดูเฉพาะโฟลเดอร์".into(),
    );

    m.insert(
        "idle.views".into(),
        "ลอง: views .   (รายการโฟลเดอร์พร้อม --size และ --sort)".into(),
    );

    m.insert(
        "idle.txtf".into(),
        "ลอง: txtf task.txt --head 20   (แสดงไฟล์) · txtc คัดลอกลงคลิปบอร์ด".into(),
    );

    m.insert(
        "idle.text".into(),
        "ลอง: head 5 task.txt   (มี tail, betw 2:4, sort, rev)".into(),
    );

    m.insert(
        "idle.count".into(),
        "ลอง: view task.txt | count ntc   (นับคำที่ตรงในข้อความที่ส่งต่อ) · trim ลบ".into(),
    );

    m.insert(
        "idle.files".into(),
        "ลอง: mkf notes.md   (แล้ว mkd, cp, mv, rnmf, rmf, rmd)".into(),
    );

    m.insert(
        "idle.config".into(),
        "ลอง: config output   (ดูค่าหนึ่ง; showcg ดูทั้งหมด)".into(),
    );

    m.insert(
        "idle.opencg".into(),
        "ลอง: opencg   (แก้ config.toml) · resetcg และ restorecg แก้คืนได้".into(),
    );

    m.insert(
        "idle.local".into(),
        "ลอง: config local init   (ตั้งค่าเฉพาะโฟลเดอร์นี้)".into(),
    );

    m.insert(
        "idle.project".into(),
        "ลอง: init   (สร้าง NTCRANFILE.toml ที่นี่) · deinit ลบออก".into(),
    );

    m.insert(
        "idle.tpb".into(),
        "ลอง: tpb history   (เส้นทางที่เคยกระโดดไป)".into(),
    );

    m.insert(
        "idle.qur".into(),
        "ลอง: qur *.txt -s \"old\" -dp   (เลือกก่อน แล้ว -rp เพื่อแทนที่)".into(),
    );

    m.insert(
        "idle.fsc".into(),
        "ลอง: fsc task.txt   (หาไฟล์แล้วเปิด) · fs หาไฟล์, ds หาโฟลเดอร์".into(),
    );

    m.insert(
        "idle.ignore".into(),
        "ลอง: ignore dir target   (ข้ามโฟลเดอร์นี้ตอนสแกน) · care ยกเลิก".into(),
    );

    m.insert(
        "idle.igcare".into(),
        "ลอง: igcared   (ดูกฎ ignore และ care ทั้งหมด)".into(),
    );

    m.insert(
        "idle.http".into(),
        "ลอง: http --help   (HTTP client: method, header, body)".into(),
    );

    m.insert(
        "idle.dl".into(),
        "ลอง: dl --help   (ดาวน์โหลดต่อได้ด้วย --resume)".into(),
    );

    m.insert(
        "idle.dns".into(),
        "ลอง: dns example.com -t MX   (ค้นระเบียน DNS)".into(),
    );

    m.insert(
        "idle.up".into(),
        "ลอง: up --help   (อัปโหลดไฟล์ผ่าน HTTP)".into(),
    );

    m.insert(
        "idle.ping".into(),
        "ลอง: ping 127.0.0.1   (ตรวจว่าโฮสต์ตอบหรือไม่)".into(),
    );

    m.insert(
        "idle.trace".into(),
        "ลอง: trace 127.0.0.1   (เส้นทางทีละฮอป)".into(),
    );

    m.insert(
        "idle.smtp".into(),
        "ลอง: smtp --help   (ส่งอีเมล; รหัสผ่านเก็บใน deadlock)".into(),
    );

    m.insert(
        "idle.ipcfg".into(),
        "ลอง: ipcfg   (อะแดปเตอร์และที่อยู่ของเครื่องนี้)".into(),
    );

    m.insert(
        "idle.wi".into(),
        "ลอง: wi .   (เฝ้าดูการเปลี่ยนแปลงของโฟลเดอร์นี้; q ออก)".into(),
    );

    m.insert(
        "idle.spec".into(),
        "ลอง: spec   (สรุป CPU, RAM, ดิสก์ และระบบ)".into(),
    );

    m.insert(
        "idle.tmprt".into(),
        "ลอง: tmprt city   (ตั้งเมืองเริ่มต้น) · tmprt com ดูเซนเซอร์".into(),
    );

    m.insert(
        "idle.clip".into(),
        "ลอง: clip   (ช่องคลิปบอร์ด) · clip 2 เรียกช่องที่ 2".into(),
    );

    m.insert(
        "idle.qsc".into(),
        "ลอง: qsc status   (daemon คีย์ลัดทำงานอยู่ไหม)".into(),
    );

    m.insert(
        "idle.pathman".into(),
        "ลอง: usrpath show   (PATH ของผู้ใช้) · syspath show ของระบบ".into(),
    );

    m.insert(
        "idle.autoup".into(),
        "ลอง: autoup list   (โปรแกรมที่เริ่มพร้อมเครื่อง)".into(),
    );

    m.insert(
        "idle.mode".into(),
        "ลอง: mode   (โหมดผู้ดูแลหรือปกติปัจจุบัน)".into(),
    );

    m.insert(
        "idle.python".into(),
        "ลอง: showenv   (ตัวแปรสภาพแวดล้อม) · py_activate เปิด venv".into(),
    );

    m.insert(
        "idle.decode".into(),
        "ลอง: decode b64 \"aGVsbG8=\"   (ย้อน encode) · cvcode แปลงรูปแบบ".into(),
    );

    m.insert(
        "idle.image".into(),
        "ลอง: image --help   (แสดงภาพในเทอร์มินัล)".into(),
    );

    m.insert(
        "idle.nssh".into(),
        "ลอง: nssh ls   (โปรไฟล์ SSH ที่บันทึกไว้)".into(),
    );

    m.insert(
        "idle.nms".into(),
        "ลอง: nms scan .   (หาไฟล์เพลงและวิดีโอที่นี่)".into(),
    );

    m.insert(
        "idle.fake".into(),
        "ลอง: fake gen name,email -n 3   (ข้อมูลทดสอบ; fake types ดูทั้งหมด)".into(),
    );

    m.insert(
        "idle.deadlock".into(),
        "ลอง: deadlock   (ตู้รหัสผ่านที่เข้ารหัส)".into(),
    );

    m.insert(
        "idle.ntpx".into(),
        "ลอง: ntpx   (รัน runtime ที่ ntpkg ติดตั้ง)".into(),
    );

    m.insert(
        "idle.pathcp".into(),
        "ลอง: pathcp   (คัดลอกเส้นทางปัจจุบันลงคลิปบอร์ด)".into(),
    );

    m.insert(
        "idle.say".into(),
        "ลอง: say hello --loop 3   (พิมพ์ข้อความ; print เหมือนกัน)".into(),
    );

    m.insert(
        "idle.ui".into(),
        "ลอง: ui   (ดูโหมด UI) · version ดูเวอร์ชัน".into(),
    );
}
