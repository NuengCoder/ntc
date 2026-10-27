use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.view.file_usage".into(), "view <file|@tp/file|@tp/dir/file> [--head N|--tail N|--betw A:B] ## แสดงเนื้อหาไฟล์ (เหมือน txtf)".into());

    m.insert(
        "help.view.example10".into(),
        "view main.rs --tail 30 ## 30 บรรทัดสุดท้ายของ main.rs".into(),
    );

    m.insert(
        "help.view.example11".into(),
        "view main.rs --betw 10:20 ## บรรทัด 10-20 ของ main.rs".into(),
    );

    m.insert(
        "help.view.example12".into(),
        "view @docs/readme.md ## แสดงไฟล์ในจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "help.view.care".into(),
        "view -c, --care ## แสดงต้นไม้พร้อม care ทุกอย่าง".into(),
    );

    m.insert(
        "help.view.cp".into(),
        "view -cp, --copy ## แสดงต้นไม้และคัดลอกไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.view.file_only".into(),
        "view -f, --file ## แสดงเฉพาะไฟล์ (ตัดไดเรกทอรีออก)".into(),
    );

    m.insert(
        "help.view.dir_only".into(),
        "view -dir, --directory ## แสดงเฉพาะไดเรกทอรี (ตัดไฟล์ออก)".into(),
    );

    m.insert(
        "help.view.depth".into(),
        "view -d, --depth <n> ## แสดงต้นไม้พร้อมความลึกกำหนดเอง".into(),
    );

    m.insert(
        "help.view.dirsize".into(),
        "view -ds, --dirsize [ขนาดขั้นต่ำ] ## แสดงขนาดไดเรกทอรี; ใส่ค่าจะซ่อนที่เล็กกว่า".into(),
    );

    m.insert(
        "help.view.head".into(),
        "view --head <n> ## แสดงเพียง n บรรทัดแรกของ tree".into(),
    );

    m.insert(
        "help.view.tail".into(),
        "view --tail <n> ## แสดงเพียง n บรรทัดสุดท้ายของ tree".into(),
    );

    m.insert(
        "help.view.sort".into(),
        "view --sort size|char|ext ## เรียงลูก (ไดเรกทอรีอยู่ก่อน)".into(),
    );

    m.insert(
        "help.view.ext".into(),
        "view -ext, --ext <นามสกุล...> ## แสดงต้นไม้เฉพาะนามสกุลที่ตรง".into(),
    );

    m.insert(
        "help.view.filesize".into(),
        "view -fs, --filesize [ขนาดขั้นต่ำ] ## แสดงขนาดไฟล์; ใส่ค่าจะซ่อนไฟล์ที่เล็กกว่า".into(),
    );

    m.insert(
        "help.view.path".into(),
        "view -p, --path <path|@tp|@tp/sub> ## แสดงต้นไม้ของเส้นทางหรือจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "help.view.size".into(),
        "view -s, --size [ขนาดขั้นต่ำ] ## แสดงขนาด; ใส่ค่าเช่น 10KB, 1.5MB จะซ่อนที่เล็กกว่า".into(),
    );

    m.insert(
        "help.view.time".into(),
        "view --time, -t ## แสดงต้นไม้พร้อมเวลาดำเนินการ".into(),
    );

    m.insert("help.view.title".into(), "คำสั่ง View:".into());

    m.insert(
        "help.view.tree".into(),
        "view [<dir|file|@tp|@tp/file|@tp/dir/>] ## แสดงต้นไม้หรือเนื้อหาไฟล์".into(),
    );

    m.insert(
        "help.view.example1".into(),
        "view ## แสดงต้นไม้ไดเรกทอรี".into(),
    );

    m.insert(
        "help.view.example2".into(),
        "view --size ## แสดงต้นไม้พร้อมขนาด".into(),
    );

    m.insert(
        "help.view.example3".into(),
        "view -d 2 ## แสดงต้นไม้พร้อมความลึก 2".into(),
    );

    m.insert(
        "help.view.example4".into(),
        "view -cp ## แสดงต้นไม้และคัดลอกไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.view.example5".into(),
        "view --time ## แสดงต้นไม้พร้อมเวลาดำเนินการ".into(),
    );

    m.insert(
        "help.view.example6".into(),
        "view -p C:\\code\\project ## แสดงต้นไม้ของเส้นทางที่ระบุ".into(),
    );

    m.insert(
        "help.view.example7".into(),
        "view @web ## แสดงต้นไม้ของจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "help.view.example8".into(),
        "view -s 10MB ## แสดงเฉพาะไฟล์/ไดเรกทอรีอย่างน้อย 10MB".into(),
    );

    m.insert(
        "help.view.example9".into(),
        "view --sort size --head 20 ## เรียงตามขนาดแสดง 20 บรรทัดแรก".into(),
    );
}
