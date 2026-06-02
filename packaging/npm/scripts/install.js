#!/usr/bin/env node
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const pkg = require('../package.json');
const VERSION = pkg.version;
const REPO = 'NuengCoder/ntc';

function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;

  const map = {
    'darwin-x64':     { file: `ntc-v${VERSION}-macos-x86_64.tar.gz`,     bin: 'ntc' },
    'darwin-arm64':   { file: `ntc-v${VERSION}-macos-universal.tar.gz`,  bin: 'ntc' },
    'win32-x64':      { file: `ntc-v${VERSION}-windows-x86_64.zip`,      bin: 'ntc.exe' },
    'linux-x64':      { file: `ntc-v${VERSION}-linux-x86_64.tar.gz`,     bin: 'ntc' },
    'linux-arm64':    { file: `ntc-v${VERSION}-linux-aarch64.tar.gz`,    bin: 'ntc' },
  };

  return map[`${platform}-${arch}`] || null;
}

async function download(url, destPath) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  const buffer = Buffer.from(await response.arrayBuffer());
  fs.writeFileSync(destPath, buffer);
}

async function main() {
  const pkgDir = path.resolve(__dirname, '..');
  const info = getPlatformInfo();

  if (!info) {
    console.log(`ntc: unsupported platform ${process.platform}-${process.arch}.`);
    console.log('Install from source: cargo install ntc');
    process.exit(1);
  }

  const binaryPath = path.join(pkgDir, info.bin);

  if (fs.existsSync(binaryPath)) {
    return;
  }

  const url = `https://github.com/${REPO}/releases/download/v${VERSION}/${info.file}`;
  console.log(`ntc: downloading ${info.file}...`);

  const tmpDir = fs.mkdtempSync(path.join(pkgDir, 'tmp-'));
  const archivePath = path.join(tmpDir, info.file);

  try {
    await download(url, archivePath);

    if (info.file.endsWith('.zip')) {
      if (process.platform === 'win32') {
        execSync(`tar -xf "${archivePath}" -C "${tmpDir}"`, { stdio: 'pipe' });
      } else {
        execSync(`unzip -o "${archivePath}" -d "${tmpDir}"`, { stdio: 'pipe' });
      }
    } else {
      execSync(`tar -xzf "${archivePath}" -C "${tmpDir}"`, { stdio: 'pipe' });
    }

    const walk = (dir) => {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      for (const entry of entries) {
        const full = path.join(dir, entry.name);
        if (entry.name === info.bin) return full;
        if (entry.isDirectory()) {
          const found = walk(full);
          if (found) return found;
        }
      }
      return null;
    };

    const extracted = walk(tmpDir);
    if (!extracted) throw new Error('binary not found in archive');

    fs.copyFileSync(extracted, binaryPath);
    try { fs.chmodSync(binaryPath, 0o755); } catch {}

    console.log(`ntc: installed to ${binaryPath}`);
  } catch (err) {
    const msg = err.message || String(err);
    if (msg.includes('404') || msg.includes('Not Found')) {
      console.log(`ntc: binary not found for v${VERSION} on ${process.platform}-${process.arch}.`);
      console.log('Install from source: cargo install ntc');
    } else {
      console.error(`ntc: install failed: ${msg}`);
    }
    process.exit(1);
  } finally {
    try { fs.rmSync(tmpDir, { recursive: true, force: true }); } catch {}
  }
}

main();
