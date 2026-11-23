<p align="center">
  <img src="https://img.shields.io/badge/version-2.4.9-blue.svg" alt="Version 2.4.9"/>
</p>

<h1 align="center">ntc 2.4.9</h1>

<p align="center">
  <b>Navigate · Toolkit · Center</b><br>
  ເຄື່ອງມືທີ່ເປັນປະໂຫຍດໃນເທີມິນອນຂອງທ່ານ ກັບ NTC
</p>

<p align="center">
  Windows · Linux · macOS · Android (ຜ່ານ Termux arm64)
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.th.md">ไทย</a> ·
  <b>ລາວ</b> ·
  <a href="README.zh.md">中文</a>
</p>

---

<p align="center">
  <img src="assets/ntc.gif" alt="ntc demo" width="700"/>
</p>

---

## ntc ແມ່ນຫຍັງ?

ເຄື່ອງມື CLI ຫຼາຍຢ່າງລວມຢູ່ໃນ CLI ດຽວ ພ້ອມຄຳສັ່ງນຳທາງທີ່ໃຊ້ງ່າຍ

| ຄຳສັ່ງ | ໜ້າທີ່ |
| ------- | ------- |
| `tp` | ຈຸດບັນທຶກເທເລພອດ (ວາບ) |
| `ral` | ອາລິອາດທີ່ມີພາຣາມິເຕີ |
| `ran` | ຕົວຮັນງານ |
| `nfe` | ຕົວສຳຫຼວດໄຟລ໌ |
| `ne` | ໂປຣແກຣມແກ້ໄຂໃນຕົວ |
| `view` | ໂຄງສ້າງໄດເຣກທໍຣີພ້ອມຂະໜາດ, ຕົວກອງ ແລະ ຕົວຢ່າງໄຟລ໌ |
| `size` | ຂະໜາດໄຟລ໌ ແລະ ໂຟນເດີ |
| `sys` | ຕົວກວດກາລະບົບ |
| `spec` | ສະເປັກລະບົບ ແລະ ການໃຊ້ດິສ |
| `gosc` | ການນຳທາງຕໍ່ເນື່ອງ |
| `fake` | ຕົວສ້າງຂໍ້ມູນຈຳລອງ |
| `serve files` | ໃຫ້ບໍລິການໄດຣຟ໌ຂອງທ່ານຜ່ານອຸປະກອນ |

…ແລະ ອື່ນໆ

ພິມ `help` ເພື່ອເບິ່ງຂໍ້ມູນເພີ່ມເຕີມ

ພິມ `tutorial` ເພື່ອທົດລອງໃນແຊນບັອກຂອງ ntc

---

## ມີຫຍັງໃໝ່

- `config env NAME=value` ໃຊ້ໄດ້ຄືກັບ `config env NAME value` ແລ້ວ

---

## ພາບໜ້າຈໍ

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

## ຕິດຕັ້ງ

### npm (ທຸກແພລດຟອມ)

```bash
npm install -g @nuengcoder/ntc
```

### pip (Windows x64 · Linux x64 · macOS x64 · macOS arm64)

```bash
pip install ntc-cli
```

ໝາຍເຫດ: pip wheel ເຜີຍແຜ່ສະເພາະສີ່ແພລດຟອມຂ້າງເທິງເທົ່ານັ້ນ
Android/Termux arm64 ແລະ Windows ARM64 ບໍ່ມີໃນ PyPI — ໃຫ້ໃຊ້ npm ຫຼື
ໄຟລ໌ໄບນາຣີຈາກ GitHub Release ແທນ

### ຊ່ອງທາງອື່ນ

- GitHub Releases — ໄຟລ໌ໄບນາຣີສຳເລັດຮູບສຳລັບທຸກແພລດຟອມທີ່ຮອງຮັບ:
  https://github.com/NuengCoder/ntc/releases
- ຕົວຕິດຕັ້ງ Windows — ເບິ່ງຕາຕະລາງຂ້າງລຸ່ມ

---

## ແພລດຟອມທີ່ຮອງຮັບ

| ແພລດຟອມ | npm | pip | GitHub Release | ໝາຍເຫດ |
| -------- | :-: | :-: | :------------: | -------- |
| Windows 10/11 x64 | yes | yes | yes | wheel `win_amd64` |
| Windows 10/11 ARM64 | yes | no | yes | ຍັງບໍ່ມີ pip wheel |
| Windows 7 x64 | yes | no | yes | ໃຊ້ຕົວຕິດຕັ້ງ / npm |
| Windows 7 ARM64 | yes | no | yes | ຕົວຕິດຕັ້ງ / npm ເທົ່ານັ້ນ |
| Linux x64 (glibc) | yes | yes | yes | wheel `manylinux_2_28` |
| Linux ARM64 (glibc) | yes | no | yes | ຍັງບໍ່ມີ pip wheel |
| macOS x64 | yes | yes | yes | `macosx_*_x86_64` |
| macOS ARM64 | yes | yes | yes | `macosx_*_arm64` |
| Android / Termux arm64 | yes | no | yes | ໃຊ້ asset ຂອງ Termux arm64 ຜ່ານ npm |

ຄຳອະທິບາຍ: yes = ຮອງຮັບ · no = ບໍ່ມີໃຫ້ໃຊ້

---

## ການໃຊ້ງານ

```bash
ntc
```

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.th.md">ไทย</a> ·
  <b>ລາວ</b> ·
  <a href="README.zh.md">中文</a>
</p>

<p align="center">
  <a href="https://github.com/NuengCoder/ntc">GitHub</a> ·
  <a href="https://github.com/NuengCoder/ntc/releases">Releases</a> ·
  <a href="CHANGELOG.md">Changelog</a>
</p>
