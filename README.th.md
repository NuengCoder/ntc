<p align="center">
  <img src="https://img.shields.io/badge/version-2.4.9-blue.svg" alt="Version 2.4.9"/>
</p>

<h1 align="center">ntc 2.4.9</h1>

<p align="center">
  <b>Navigate · Toolkit · Center</b><br>
  เครื่องมือที่มีประโยชน์ในเทอร์มินัลของคุณ ด้วย NTC
</p>

<p align="center">
  Windows · Linux · macOS · Android (ผ่าน Termux arm64)
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <b>ไทย</b> ·
  <a href="README.lo.md">ລາວ</a> ·
  <a href="README.zh.md">中文</a>
</p>

---

<p align="center">
  <img src="assets/ntc.gif" alt="ntc demo" width="700"/>
</p>

---

## ntc คืออะไร?

เครื่องมือ CLI หลากหลายรวมไว้ใน CLI เดียว พร้อมคำสั่งนำทางที่ใช้งานง่าย

| คำสั่ง | สิ่งที่ทำ |
| ------ | --------- |
| `tp` | จุดบันทึกเทเลพอร์ต (วาร์ป) |
| `ral` | อะลิอัสแบบมีพารามิเตอร์ |
| `ran` | ตัวรันงาน |
| `nfe` | ตัวสำรวจไฟล์ |
| `ne` | โปรแกรมแก้ไขในตัว |
| `view` | แผนผังไดเรกทอรีพร้อมขนาด ตัวกรอง และตัวอย่างไฟล์ |
| `size` | ขนาดไฟล์และโฟลเดอร์ |
| `sys` | ตัวตรวจสอบระบบ |
| `spec` | สเปกระบบและการใช้พื้นที่ดิสก์ |
| `gosc` | การนำทางต่อเนื่อง |
| `fake` | ตัวสร้างข้อมูลจำลอง |
| `serve files` | ให้บริการไดรฟ์ของคุณผ่านอุปกรณ์ |

…และอื่น ๆ อีกมากมาย

พิมพ์ `help` เพื่อดูข้อมูลเพิ่มเติม

พิมพ์ `tutorial` เพื่อทดลองแซนด์บ็อกซ์ของ ntc

---

## มีอะไรใหม่

- `config env NAME=value` ใช้งานได้เหมือน `config env NAME value` แล้ว

---

## ภาพหน้าจอ

<p align="center">
  <img src="assets/before_tp.png" alt="tp before" width="32%">
  <img src="assets/after_tp.png" alt="tp after" width="32%">
  <img src="assets/ral_info.png" alt="ral info" width="32%">
</p>

<p align="center">
  <img src="assets/after_ral.png" alt="ral after" width="32%">
  <img src="assets/ran_info.png" alt="ran info" width="32%">
  <img src="assets/after_ran.png" alt="ran after" width="32%">
</p>

<p align="center">
  <img src="assets/nfe.png" alt="nfe file explorer" width="32%">
  <img src="assets/ne.png" alt="ne editor" width="32%">
  <img src="assets/sys.png" alt="sys system monitor" width="32%">
</p>

<p align="center">
  <img src="assets/spec.png" alt="spec specs and disks" width="32%">
  <img src="assets/view_and_size.png" alt="view --size" width="32%">
  <img src="assets/view.png" alt="view directory tree" width="32%">
</p>

<p align="center">
  <img src="assets/serve.png" alt="serve services" width="32%">
  <img src="assets/serve_files.png" alt="serve files" width="32%">
  <img src="assets/serve_httpserver.png" alt="serve files HTTP server" width="32%">
</p>

<p align="center">
  <img src="assets/serve_chess.png" alt="serve chess" width="32%">
  <img src="assets/serve_git.png" alt="serve git" width="32%">
  <img src="assets/serve_arc.png" alt="serve arc" width="32%">
</p>

<p align="center">
  <img src="assets/serve_theme.png" alt="serve theme" width="32%">
  <img src="assets/serve_fake.png" alt="serve fake" width="32%">
  <img src="assets/serve_backup.png" alt="serve backup" width="32%">
</p>

---

## ติดตั้ง

### npm (ทุกแพลตฟอร์ม)

```bash
npm install -g @nuengcoder/ntc
```

### pip (Windows x64 · Linux x64 · macOS x64 · macOS arm64)

```bash
pip install ntc-cli
```

หมายเหตุ: pip wheel เผยแพร่เฉพาะสี่แพลตฟอร์มข้างต้นเท่านั้น
Android/Termux arm64 และ Windows ARM64 ไม่มีบน PyPI — ให้ใช้ npm หรือ
ไฟล์ไบนารีจาก GitHub Release แทน

### ช่องทางอื่น

- GitHub Releases — ไฟล์ไบนารีสำเร็จรูปสำหรับทุกแพลตฟอร์มที่รองรับ:
  https://github.com/NuengCoder/ntc/releases
- ตัวติดตั้ง Windows — ดูตารางด้านล่าง

---

## แพลตฟอร์มที่รองรับ

| แพลตฟอร์ม | npm | pip | GitHub Release | หมายเหตุ |
| --------- | :-: | :-: | :------------: | -------- |
| Windows 10/11 x64 | yes | yes | yes | wheel `win_amd64` |
| Windows 10/11 ARM64 | yes | no | yes | ยังไม่มี pip wheel |
| Windows 7 x64 | yes | no | yes | ใช้ตัวติดตั้ง / npm |
| Windows 7 ARM64 | yes | no | yes | ตัวติดตั้ง / npm เท่านั้น |
| Linux x64 (glibc) | yes | yes | yes | wheel `manylinux_2_28` |
| Linux ARM64 (glibc) | yes | no | yes | ยังไม่มี pip wheel |
| macOS x64 | yes | yes | yes | `macosx_*_x86_64` |
| macOS ARM64 | yes | yes | yes | `macosx_*_arm64` |
| Android / Termux arm64 | yes | no | yes | ใช้ asset ของ Termux arm64 ผ่าน npm |

คำอธิบาย: yes = รองรับ · no = ไม่มีให้ใช้

---

## การใช้งาน

```bash
ntc
```

---

<p align="center">
  <a href="README.md">English</a> ·
  <b>ไทย</b> ·
  <a href="README.lo.md">ລາວ</a> ·
  <a href="README.zh.md">中文</a>
</p>

<p align="center">
  <a href="https://github.com/NuengCoder/ntc">GitHub</a> ·
  <a href="https://github.com/NuengCoder/ntc/releases">Releases</a> ·
  <a href="CHANGELOG.md">Changelog</a>
</p>
