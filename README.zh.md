<p align="center">
  <img src="https://img.shields.io/badge/version-2.4.9-blue.svg" alt="Version 2.4.9"/>
</p>

<h1 align="center">ntc 2.4.9</h1>

<p align="center">
  <b>Navigate · Toolkit · Center</b><br>
  在终端中使用的实用工具，尽在 NTC。
</p>

<p align="center">
  Windows · Linux · macOS · Android（通过 Termux arm64）
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.lo.md">ລາວ</a> ·
  <b>中文</b>
</p>

---

<p align="center">
  <img src="assets/ntc.gif" alt="ntc demo" width="700"/>
</p>

---

## 什么是 ntc？

将多种 CLI 工具集于一身，并配有便捷的导航命令。

| 命令 | 功能 |
| ---- | ---- |
| `tp` | 传送保存点 |
| `ral` | 带参数的别名 |
| `ran` | 任务运行器 |
| `nfe` | 文件浏览器 |
| `ne` | 内置编辑器 |
| `view` | 带大小、过滤和文件预览的目录树 |
| `size` | 文件和文件夹大小 |
| `sys` | 系统监视器 |
| `spec` | 系统规格与磁盘使用情况 |
| `gosc` | 连续导航 |
| `fake` | 模拟数据生成器 |
| `serve files` | 通过你的设备共享自己的硬盘 |

……以及更多。

输入 `help` 查看更多信息。

输入 `tutorial` 试用 ntc 沙盒。

---

## 最新变化

- `config env NAME=value` 现在与 `config env NAME value` 等效。

---

## 截图

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

## 安装

### npm（所有平台）

```bash
npm install -g @nuengcoder/ntc
```

### pip（Windows x64 · Linux x64 · macOS x64 · macOS arm64）

```bash
pip install ntc-cli
```

注意：pip wheel 仅为上述四个平台发布。
Android/Termux arm64 和 Windows ARM64 未发布到 PyPI —— 请使用 npm 或
GitHub Release 二进制文件。

### 其他渠道

- GitHub Releases —— 适用于所有支持平台的预编译二进制文件：
  https://github.com/NuengCoder/ntc/releases
- Windows 安装程序 —— 见下表

---

## 平台支持

| 平台 | npm | pip | GitHub Release | 备注 |
| ---- | :-: | :-: | :------------: | ---- |
| Windows 10/11 x64 | yes | yes | yes | `win_amd64` wheel |
| Windows 10/11 ARM64 | yes | no | yes | 尚未发布 pip wheel |
| Windows 7 x64 | yes | no | yes | 使用安装程序 / npm |
| Windows 7 ARM64 | yes | no | yes | 仅安装程序 / npm |
| Linux x64 (glibc) | yes | yes | yes | `manylinux_2_28` wheel |
| Linux ARM64 (glibc) | yes | no | yes | 尚未发布 pip wheel |
| macOS x64 | yes | yes | yes | `macosx_*_x86_64` |
| macOS ARM64 | yes | yes | yes | `macosx_*_arm64` |
| Android / Termux arm64 | yes | no | yes | 通过 npm 回退使用 Termux arm64 asset |

图例：yes = 支持 · no = 不可用

---

## 使用

```bash
ntc
```

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.lo.md">ລາວ</a> ·
  <b>中文</b>
</p>

<p align="center">
  <a href="https://github.com/NuengCoder/ntc">GitHub</a> ·
  <a href="https://github.com/NuengCoder/ntc/releases">Releases</a> ·
  <a href="CHANGELOG.md">Changelog</a>
</p>
