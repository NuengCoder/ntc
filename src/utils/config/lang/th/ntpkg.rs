use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ntpkg.add_to_path".into(), "เพิ่มไปยัง PATH ของคุณ:".into());

    m.insert(
        "ntpkg.add_to_sys_path".into(),
        "เพิ่มไปยัง PATH ของระบบของคุณ (ต้องใช้สิทธิ์ผู้ดูแลระบบ):".into(),
    );

    m.insert(
        "ntpkg.add_to_user_path".into(),
        "เพิ่มไปยัง PATH ของผู้ใช้ของคุณ:".into(),
    );

    m.insert("ntpkg.already_installed".into(), "{} {} ติดตั้งแล้วที่ {}".into());

    m.insert(
        "ntpkg.available_hint".into(),
        " ใช้ 'ntpkg available <language>' เพื่อแสดงรายการเวอร์ชัน.".into(),
    );

    m.insert(
        "ntpkg.configuring".into(),
        "กำลังกำหนดค่า (./configure)...".into(),
    );

    m.insert("ntpkg.downloading".into(), "กำลังดาวน์โหลด {}".into());

    m.insert(
        "ntpkg.enter_number".into(),
        "ป้อนหมายเลข (1-{}) หรือ 0: ".into(),
    );

    m.insert("ntpkg.extracting".into(), "กำลังแตกไฟล์...".into());

    m.insert("ntpkg.extracting_indent".into(), " กำลังแตกไฟล์...".into());

    m.insert(
        "ntpkg.extracting_source".into(),
        "กำลังแตก tarball ซอร์สโค้ด...".into(),
    );

    m.insert("ntpkg.filter_label".into(), "(กรอง: \"{}\")".into());

    m.insert(
        "ntpkg.git_not_found".into(),
        "ไม่พบ Git ใน PATH. กำลังดาวน์โหลด MinGit...".into(),
    );

    m.insert(
        "ntpkg.help_usage".into(),
        "ntpkg \u{2014} ตัวจัดการแพ็กเกจของ ntc สำหรับ language runtimes".into(),
    );

    m.insert("ntpkg.help_usage_title".into(), "  การใช้งาน:".into());

    m.insert(
        "ntpkg.help_install".into(),
        "    ntpkg install <language> [version]   ดาวน์โหลดและติดตั้ง runtime".into(),
    );

    m.insert(
        "ntpkg.help_uninstall".into(),
        "    ntpkg uninstall <language> [version]  ลบ runtime ที่ติดตั้ง".into(),
    );

    m.insert(
        "ntpkg.help_list".into(),
        "    ntpkg list                           แสดง runtimes ที่ติดตั้ง".into(),
    );

    m.insert(
        "ntpkg.help_use".into(),
        "    ntpkg use <language> <version>        เปิดใช้งานเวอร์ชันเฉพาะ".into(),
    );

    m.insert(
        "ntpkg.help_which".into(),
        "    ntpkg which <language>                แสดงเส้นทางไปยัง runtime ที่ใช้งานอยู่".into(),
    );

    m.insert(
        "ntpkg.help_available".into(),
        "    ntpkg available [language]             แสดงภาษาหรือเวอร์ชันที่ดาวน์โหลดได้".into(),
    );

    m.insert(
        "ntpkg.help_help".into(),
        "    ntpkg help                            แสดงความช่วยเหลือนี้".into(),
    );

    m.insert(
        "ntpkg.ntpx_header".into(),
        "  ntpx \u{2014} เรียกไบนารีของแพ็กเกจที่ติดตั้ง".into(),
    );

    m.insert(
        "ntpkg.ntpx_usage_line".into(),
        "    ntpx <package> [binary] [args...]".into(),
    );

    m.insert("ntpkg.help_examples_title".into(), "  ตัวอย่าง:".into());

    m.insert(
        "ntpkg.help_ex_ntpx_go".into(),
        "    ntpx go version              # เรียก 'go version'".into(),
    );

    m.insert(
        "ntpkg.help_ex_ntpx_py".into(),
        "    ntpx python --version        # เรียก 'python --version'".into(),
    );

    m.insert(
        "ntpkg.help_ex_ntpx_msys2".into(),
        "    ntpx msys2 gcc --version     # เรียก 'gcc --version' (ระบุไบนารีชัดเจน)".into(),
    );

    m.insert("ntpkg.available_title".into(), "แพ็กเกจที่มี".into());

    m.insert("ntpkg.package_info_title".into(), "ข้อมูลแพ็กเกจ: {}".into());

    m.insert("ntpkg.active_label".into(), "ใช้งานอยู่:".into());

    m.insert("ntpkg.installed_label".into(), "ติดตั้งแล้ว:".into());

    m.insert("ntpkg.location_label".into(), "ตำแหน่ง:".into());

    m.insert("ntpkg.binaries_label".into(), "ไบนารี:".into());

    m.insert("ntpkg.not_set".into(), "(ไม่ได้ตั้งค่า)".into());

    m.insert("ntpkg.none".into(), "(ไม่มี)".into());

    m.insert("ntpkg.active_marker".into(), " (ใช้งานอยู่)".into());

    m.insert(
        "ntpkg.info_usage".into(),
        "การใช้งาน: ntpkg info <language>".into(),
    );

    m.insert("ntpkg.install_failed".into(), "การติดตั้งล้มเหลว: {}".into());

    m.insert("ntpkg.install_loc".into(), "ตำแหน่งติดตั้ง: {}".into());

    m.insert(
        "ntpkg.install_usage".into(),
        "การใช้งาน: ntpkg install <language> [version]".into(),
    );

    m.insert("ntpkg.installed_msg".into(), "{} {} ติดตั้งแล้ว!".into());

    m.insert(
        "ntpkg.installing".into(),
        "กำลังติดตั้ง (make install)...".into(),
    );

    m.insert("ntpkg.latest_suffix".into(), "(ล่าสุด)".into());

    m.insert(
        "ntpkg.launch_installer".into(),
        "กำลังเปิดตัวติดตั้ง. ทำการตั้งค่าให้เสร็จสิ้นแล้วปิดหน้าต่างตัวติดตั้ง...".into(),
    );

    m.insert("ntpkg.magick_macos".into(), "ImageMagick บน macOS:".into());

    m.insert(
        "ntpkg.magick_manual_reg".into(),
        "หากติดตั้งแล้ว ลงทะเบียนด้วยตนเอง: ntpkg use imagemagick <version>".into(),
    );

    m.insert(
        "ntpkg.magick_not_found".into(),
        "ไม่สามารถตรวจจับ magick.exe อัตโนมัติหลังติดตั้ง.".into(),
    );

    m.insert(
        "ntpkg.msys2_gcc_incomplete".into(),
        "การติดตั้ง GCC อาจยังไม่เสร็จสมบูรณ์. คุณติดตั้งทีหลังผ่าน pacman ได้.".into(),
    );

    m.insert(
        "ntpkg.msys2_gcc_not_found".into(),
        "ยังไม่พบ GCC. เรียก pacman เพื่อติดตั้ง:".into(),
    );

    m.insert(
        "ntpkg.msys2_installed".into(),
        "ติดตั้ง MSYS2 แล้ว! ติดตั้งแพ็กเกจเสริมผ่าน pacman:".into(),
    );

    m.insert(
        "ntpkg.msys2_linux_gcc".into(),
        "บนแพลตฟอร์มนี้ ติดตั้ง GCC ด้วยตัวจัดการแพ็กเกจของคุณ:".into(),
    );

    m.insert(
        "ntpkg.msys2_running_pacman".into(),
        "กำลังเรียก: pacman -S mingw-w64-ucrt-x86_64-gcc".into(),
    );

    m.insert(
        "ntpkg.no_active_version".into(),
        "ไม่มีเวอร์ชันที่ใช้งานอยู่สำหรับ '{}' และไม่มีการระบุเวอร์ชัน".into(),
    );

    m.insert("ntpkg.no_match".into(), "ไม่มีแพ็กเกจตรงกับ '{}'".into());

    m.insert(
        "ntpkg.no_packages".into(),
        "ไม่มีแพ็กเกจที่ติดตั้ง. ใช้ 'ntpkg install <language>'".into(),
    );

    m.insert(
        "ntpkg.no_versions".into(),
        "ไม่มีเวอร์ชันที่ถูกติดตามสำหรับ '{}'".into(),
    );

    m.insert("ntpkg.no_versions_for".into(), "ไม่มีเวอร์ชันสำหรับ '{}'".into());

    m.insert("ntpkg.not_installed".into(), "{} {} ยังไม่ติดตั้ง".into());

    m.insert("ntpkg.now_using".into(), "กำลังใช้ {} {} อยู่".into());

    m.insert(
        "ntpkg.npm_bin_in_path".into(),
        "npm global bin อยู่ใน PATH".into(),
    );

    m.insert("ntpkg.npm_detected".into(), "ตรวจพบ Node.js และ npm".into());

    m.insert(
        "ntpkg.npm_not_detected".into(),
        "ไม่ตรวจพบ Node.js/npm.".into(),
    );

    m.insert(
        "ntpkg.npm_section_node_first".into(),
        "  ── ติดตั้ง Node.js ก่อน ──".into(),
    );

    m.insert(
        "ntpkg.npm_node_found_at".into(),
        "  ✓ พบ Node.js ที่: {}".into(),
    );

    m.insert("ntpkg.npm_node_not_found".into(), "  ✗ ไม่พบ Node.js".into());

    m.insert("ntpkg.npm_install_via_pkg".into(), "  ติดตั้งผ่าน pkg:".into());

    m.insert(
        "ntpkg.npm_install_via_ntpkg".into(),
        "  ติดตั้งผ่าน ntpkg:".into(),
    );

    m.insert("ntpkg.npm_ntpkg_cmd".into(), "    ntpkg install {}".into());

    m.insert("ntpkg.npm_cmd".into(), "    npm i -g {}".into());

    m.insert(
        "ntpkg.npm_or_download_nodejs".into(),
        "  หรือดาวน์โหลดจาก: https://nodejs.org".into(),
    );

    m.insert(
        "ntpkg.npm_or_fnm_nvm_volta".into(),
        "  หรือติดตั้งผ่าน fnm/nvm/volta".into(),
    );

    m.insert("ntpkg.npm_found_at".into(), "  ✓ พบ npm ที่: {}".into());

    m.insert("ntpkg.npm_not_found".into(), "  ✗ ไม่พบ npm".into());

    m.insert("ntpkg.npm_then_run".into(), "  จากนั้นรัน:".into());

    m.insert("ntpkg.npm_then_rerun".into(), "  จากนั้นรันอีกครั้ง:".into());

    m.insert(
        "ntpkg.npm_bin_not_in_path".into(),
        "npm global bin ไม่พบใน PATH ({})".into(),
    );

    m.insert("ntpkg.path_add_label".into(), "  เพิ่มลง PATH:".into());

    m.insert("ntpkg.npm_install_section".into(), "  ── ติดตั้ง {} ──".into());

    m.insert("ntpkg.npm_run_cmd_label".into(), "  รันคำสั่งนี้:".into());

    m.insert(
        "ntpkg.ntpx_usage".into(),
        "การใช้งาน: ntpx <package> [binary] [args...]".into(),
    );

    m.insert("ntpkg.package_count".into(), "{} แพ็กเกจ".into());

    m.insert(
        "ntpkg.php_skip_register".into(),
        "ข้ามการลงทะเบียน PHP กับ ntpkg.".into(),
    );

    m.insert("ntpkg.removed_msg".into(), "{} {} ถูกลบแล้ว".into());

    m.insert(
        "ntpkg.select_jdk_version".into(),
        "เลือกเวอร์ชัน JDK {}".into(),
    );

    m.insert("ntpkg.select_version".into(), "เลือกเวอร์ชัน {}".into());

    m.insert(
        "ntpkg.skip_register".into(),
        "ข้ามการลงทะเบียน {} กับ ntpkg.".into(),
    );

    m.insert(
        "ntpkg.termux_use_pkg".into(),
        "บน Termux ควรติดตั้งผ่าน pkg แทน ntpkg:".into(),
    );

    m.insert("ntpkg.termux_cmd".into(), "    pkg install {}".into());

    m.insert(
        "ntpkg.termux_no_pkg".into(),
        "ไม่มีแพ็กเกจ pkg ของ Termux สำหรับ '{}'.".into(),
    );

    m.insert(
        "ntpkg.src_act".into(),
        "act - เรียก GitHub Actions ในเครื่อง".into(),
    );

    m.insert(
        "ntpkg.src_cloudflare".into(),
        "cloudflared - ไคลเอ็นต์ Cloudflare Tunnel".into(),
    );

    m.insert(
        "ntpkg.src_devops_tool".into(),
        "เครื่องมือ CLI ของ Azure DevOps".into(),
    );

    m.insert("ntpkg.src_docker".into(), "Docker - รันไทม์คอนเทนเนอร์".into());

    m.insert(
        "ntpkg.src_docker_compose".into(),
        "Docker Compose - การจัดวงคอนเทนเนอร์หลายตัว".into(),
    );

    m.insert(
        "ntpkg.src_dotnet".into(),
        ".NET SDK - การพัฒนาข้ามแพลตฟอร์ม".into(),
    );

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - ตัวแปลงสื่อ".into());

    m.insert(
        "ntpkg.src_freebuff".into(),
        "Freebuff - ตัวแทนเขียนโค้ด AI ฟรี".into(),
    );

    m.insert("ntpkg.src_flutter".into(), "Flutter - ชุดเครื่องมือ UI".into());

    m.insert(
        "ntpkg.src_gh".into(),
        "GitHub CLI - GitHub จากเทอร์มินัล".into(),
    );

    m.insert("ntpkg.src_git".into(), "Git - ระบบควบคุมเวอร์ชัน".into());

    m.insert("ntpkg.src_go".into(), "Go - ภาษาโปรแกรม".into());

    m.insert(
        "ntpkg.src_helm".into(),
        "Helm - ตัวจัดการแพ็กเกจ Kubernetes".into(),
    );

    m.insert(
        "ntpkg.src_imagemagick".into(),
        "ImageMagick - การจัดการภาพ".into(),
    );

    m.insert("ntpkg.src_java".into(), "Java - สภาพแวดล้อมรันไทม์".into());

    m.insert("ntpkg.src_kilocode".into(), "Kiloc - ผู้ช่วยโค้ด AI".into());

    m.insert(
        "ntpkg.src_kind".into(),
        "kind - Kubernetes ใน Docker".into(),
    );

    m.insert("ntpkg.src_kotlin".into(), "Kotlin - ภาษาโปรแกรม".into());

    m.insert(
        "ntpkg.src_kubectl".into(),
        "kubectl - CLI ของ Kubernetes".into(),
    );

    m.insert(
        "ntpkg.src_kustomize".into(),
        "Kustomize - การปรับแต่งการกำหนดค่า Kubernetes".into(),
    );

    m.insert("ntpkg.src_lua".into(), "Lua - ภาษาสคริปต์".into());

    m.insert(
        "ntpkg.src_mimocode".into(),
        "Mimic - เอเจนต์เขียนโค้ด AI".into(),
    );

    m.insert(
        "ntpkg.src_msys2".into(),
        "MSYS2 - แพลตฟอร์มการแจกจ่ายและการสร้างซอฟต์แวร์".into(),
    );

    m.insert("ntpkg.src_nim".into(), "Nim - ภาษาโปรแกรม".into());

    m.insert("ntpkg.src_node".into(), "Node.js - รันไทม์ JavaScript".into());

    m.insert(
        "ntpkg.src_omniroute".into(),
        "OmniRoute - เครื่องมือสร้างภาพ".into(),
    );

    m.insert(
        "ntpkg.src_opencode".into(),
        "opencode - เอเจนต์เขียนโค้ด CLI".into(),
    );

    m.insert("ntpkg.src_php".into(), "PHP - ภาษาสคริปต์".into());

    m.insert("ntpkg.src_python".into(), "Python - ภาษาโปรแกรม".into());

    m.insert("ntpkg.src_ruby".into(), "Ruby - ภาษาโปรแกรม".into());

    m.insert("ntpkg.src_rust".into(), "Rust - ภาษาโปรแกรมระบบ".into());

    m.insert("ntpkg.src_scala".into(), "Scala - ภาษาโปรแกรม".into());

    m.insert("ntpkg.src_sqlite".into(), "SQLite - ฐานข้อมูลฝังตัว".into());

    m.insert(
        "ntpkg.src_sqlitestudio".into(),
        "SQLiteStudio - ตัวจัดการ SQLite".into(),
    );

    m.insert("ntpkg.src_swift".into(), "Swift - ภาษาโปรแกรม".into());

    m.insert("ntpkg.src_vscode".into(), "VS Code - ตัวแก้ไขโค้ด".into());

    m.insert("ntpkg.src_zig".into(), "Zig - ภาษาโปรแกรม".into());

    m.insert(
        "ntpkg.src_tesseract".into(),
        "Tesseract - เครื่องมือ OCR".into(),
    );

    m.insert(
        "ntpkg.tesseract_running_installer".into(),
        "กำลังเรียกตัวติดตั้ง Tesseract แบบเงียบ (ยกระดับสิทธิ์)...".into(),
    );

    m.insert("ntpkg.tesseract_not_found".into(), "ไม่สามารถตรวจจับ tesseract.exe อัตโนมัติหลังติดตั้ง. เพิ่มโฟลเดอร์ไปยัง PATH ด้วย: usrpath add <dir> (แล้ว 'mode admin' + 'syspath add <dir>').".into());

    m.insert(
        "ntpkg.tesseract_linux".into(),
        "ติดตั้ง Tesseract บน Linux ผ่านตัวจัดการแพ็กเกจของคุณ:".into(),
    );

    m.insert(
        "ntpkg.tesseract_macos".into(),
        "ติดตั้ง Tesseract บน macOS ผ่าน Homebrew:".into(),
    );

    m.insert(
        "ntpkg.tesseract_unsupported".into(),
        "ไม่รองรับการติดตั้ง Tesseract บนแพลตฟอร์มนี้.".into(),
    );

    m.insert(
        "ntpkg.src_nginx".into(),
        "nginx - เว็บเซิร์ฟเวอร์ / reverse proxy".into(),
    );

    m.insert(
        "ntpkg.nginx_unsupported".into(),
        "ไม่รองรับการติดตั้ง nginx บนแพลตฟอร์มนี้.".into(),
    );

    m.insert(
        "ntpkg.nginx_system_hint_linux".into(),
        "ติดตั้ง nginx บน Linux ผ่านตัวจัดการแพ็กเกจของคุณ:".into(),
    );

    m.insert(
        "ntpkg.nginx_system_hint_macos".into(),
        "ติดตั้ง nginx บน macOS ผ่าน Homebrew:".into(),
    );

    m.insert(
        "ntpkg.nginx_termux".into(),
        "ติดตั้ง nginx บน Termux ผ่านตัวจัดการแพ็กเกจ:".into(),
    );

    m.insert(
        "ntpkg.nginx_choose_method".into(),
        "ไม่มีไบนารี prebuilt อย่างเป็นทางการสำหรับ Linux/macOS. เลือกวิธีการติดตั้ง:".into(),
    );

    m.insert(
        "ntpkg.nginx_option_source".into(),
        "สร้างจากซอร์สโค้ด (ดาวน์โหลด nginx source tarball)".into(),
    );

    m.insert(
        "ntpkg.nginx_option_system".into(),
        "ติดตั้งผ่านตัวจัดการแพ็กเกจของระบบ".into(),
    );

    m.insert(
        "ntpkg.nginx_run_commands_system".into(),
        "ติดตั้ง nginx ด้วยคำสั่งตัวจัดการแพ็กเกจที่แสดงด้านบน (ไม่ได้ติดตามโดย ntpkg).".into(),
    );

    m.insert("ntpkg.nginx_build_confirm".into(), "สร้าง nginx {} จากซอร์สโค้ดไปยังไดเรกทอรีแพ็กเกจของ ntpkg? ต้องใช้ make/gcc + แพ็กเกจ dev ของ PCRE/Zlib/OpenSSL.".into());

    m.insert("ntpkg.nginx_build_deps".into(), "คำใบ้แพ็กเกจสำหรับการสร้าง: build-essential libpcre2-dev zlib1g-dev libssl-dev (Debian/Ubuntu).".into());

    m.insert(
        "ntpkg.nginx_configuring".into(),
        "กำลังเรียก ./configure...".into(),
    );

    m.insert("ntpkg.nginx_compiling".into(), "กำลังคอมไพล์...".into());

    m.insert("ntpkg.supported_langs".into(), "  ภาษาที่รองรับ:".into());

    m.insert(
        "ntpkg.uninstall_failed".into(),
        "การถอนการติดตั้งล้มเหลว: {}".into(),
    );

    m.insert(
        "ntpkg.uninstall_usage".into(),
        "การใช้งาน: ntpkg uninstall <language> [version]".into(),
    );

    m.insert(
        "ntpkg.unknown_subcmd".into(),
        "ไม่รู้จักคำสั่งย่อยของ ntpkg: '{}'. ใช้ 'ntpkg help'".into(),
    );

    m.insert(
        "ntpkg.use_usage".into(),
        "การใช้งาน: ntpkg use <language> <version>".into(),
    );

    m.insert(
        "ntpkg.which_usage".into(),
        "การใช้งาน: ntpkg which <language> [version]".into(),
    );
}
