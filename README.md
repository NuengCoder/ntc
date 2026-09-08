<p align="center">
  <img src="https://img.shields.io/badge/version-2.4.0-blue.svg" alt="Version 2.4.0"/>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20Android-lightgrey.svg" alt="Platforms"/>
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License"/>
</p>

<h1 align="center">ntc 2.4.0</h1>

<p align="center">
  <b>Navigate · Toolkit · Center</b><br>
  One terminal tool to rule them all.
</p>

---

## What is ntc?

ntc is an all-in-one terminal productivity tool that replaces a dozen separate utilities with a single, cohesive experience.

**Navigate** — Move through your filesystem with `cd`, teleport savepoints (`tp`), directory picker (`gos`/`gosc`), and drive navigation.

**Edit** — Built-in multi-tab text editor (`ne`) with syntax highlighting for 45+ languages, hex editor, multi-cursor, LSP, search, undo/redo, and a template engine for code generation.

**Manage** — Package manager (`ntpkg`) to install and switch between 30+ language runtimes (Python, Node.js, Go, Rust, Java, and more). Manage your Windows PATH with `usrpath`/`syspath`.

**Backup** — Project snapshots with `backup create`, restore, undo, and diff.

**Run** — Task runner (`ran`) with dependency resolution. Run aliases (`ral`) for parameterized commands.

**Report** — Generate directory reports in TXT, JSON, Markdown, HTML, PDF, DOCX, and XLSX.

**System** — Monitor CPU/memory/processes with `sys`/`top`. File server with `serve`. Email with `smtp`. QR codes with `qr`/`qsc`.

**Math** — Expression evaluator with built-in functions, constants, user functions, and `.ntc.math` scripts.

**Theme** — Full color system with 61 configurable colors across syntax, shell UI, and editor. Export/import `.ntc_theme` files.

---

## Showcase

<p align="center">
  <img src="assets/showcase240.gif" alt="ntc 2.4.0 showcase" width="700"/>
</p>

---

## Quick Start

```bash
npm install -g @nuengcoder/ntc
```

Or download the latest release from [GitHub Releases](https://github.com/NuengCoder/ntc/releases).

Launch the interactive shell:

```bash
ntc
```

---

## Future Features (v2.4.1 – v2.5.0)

- `config lang <language>` — set UI language (english, chinese, thai, lao, ...)
- `scr` — run `.ntc` automation scripts (variables, interpolation, positional args)
- New sub-features and enhancements
- Bug fixes and improvements

---

<p align="center">
  <a href="https://github.com/NuengCoder/ntc">GitHub</a> ·
  <a href="https://github.com/NuengCoder/ntc/releases">Releases</a> ·
  <a href="CHANGELOG.md">Changelog</a>
</p>
