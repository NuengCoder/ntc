#!/usr/bin/env node
// Pure-JS installer: no system `tar` / `unzip` required (Node built-ins only).
const fs = require('fs');
const path = require('path');
const zlib = require('zlib');

const pkg = require('../package.json');
const VERSION = pkg.version;
const REPO = 'NuengCoder/ntc';

function isAndroidTermux() {
  // Termux's own Node build reports process.platform === 'android' (npm needs
  // "android" in the package `os` field for this reason). Some Node builds
  // running under Termux report 'linux' instead, so ALSO detect Android via
  // the kernel release string, the Termux PREFIX env, or the install path.
  if (process.platform === 'android') return true;
  try {
    const os = require('os');
    if (/android/i.test(os.release() || '')) return true;
  } catch {}
  const prefix = process.env.PREFIX || '';
  if (/termux|android/i.test(prefix)) return true;
  if (/termux|android/i.test(__dirname)) return true;
  return false;
}

// Android/Termux assets use Rust target triples and Node's `arch` names.
const ANDROID_ASSET_BY_ARCH = {
  arm64: 'aarch64-linux-android.tar.gz',
  x64:   'x86_64-linux-android.tar.gz',
  arm:   'armv7-linux-androideabi.tar.gz',
};

// Native (non-Android) release assets.
const NATIVE_ASSET_BY_PLATFORM = {
  'darwin-x64':   'macos-x86_64.tar.gz',
  'darwin-arm64': 'macos-aarch64.tar.gz',
  'win32-x64':    'windows-x86_64.zip',
  'win32-arm64':  'windows-arm64.zip',
  'linux-x64':    'linux-x86_64.tar.gz',
  'linux-arm64':  'linux-aarch64.tar.gz',
};

// Same-arch Linux build, used as the Termux fallback when the manually
// uploaded Android asset is missing: degrade to the Linux build instead of
// hard-failing, and say so on stdout.
const LINUX_ASSET_BY_ARCH = {
  arm64: 'linux-aarch64.tar.gz',
  x64:   'linux-x86_64.tar.gz',
};

// Returns { bin, candidates: [filename, ...] } so a missing/renamed manual
// asset can fall back instead of 404ing. `platform`/`arch`/`android` are
// parameters (not read from `process`) purely so scripts/selftest.js can
// exercise every branch — this is the regression that shipped in 2.4.5.
function getPlatformInfo(platform = process.platform, arch = process.arch, android = isAndroidTermux()) {
  const bin = platform === 'win32' ? 'ntc.exe' : 'ntc';

  // Escape hatch for emergencies: force an exact asset name or a full URL,
  // e.g. NTC_NPM_ASSET=ntc-v2.4.6-linux-x86_64.tar.gz
  const forced = process.env.NTC_NPM_ASSET;
  if (forced) return { bin, candidates: [forced] };

  const nativeAsset = NATIVE_ASSET_BY_PLATFORM[`${platform}-${arch}`];
  const androidAsset = android ? ANDROID_ASSET_BY_ARCH[arch] : null;
  const linuxAsset = LINUX_ASSET_BY_ARCH[arch];

  const order = [];
  if (androidAsset) order.push(androidAsset);          // Termux first
  if (nativeAsset) order.push(nativeAsset);
  if (androidAsset && !nativeAsset) order.push(linuxAsset); // Termux -> Linux
  const names = [...new Set(order)].filter(Boolean);

  if (names.length === 0) return null;
  return { bin, candidates: names.map(n => `ntc-v${VERSION}-${n}`) };
}

async function downloadOnce(url, destPath) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 120_000);

  let response;
  try {
    response = await fetch(url, { signal: controller.signal });
  } finally {
    clearTimeout(timer);
  }

  if (!response.ok) {
    const err = new Error(`HTTP ${response.status} for ${url}`);
    err.httpStatus = response.status;
    throw err;
  }

  const len = parseInt(response.headers.get('content-length') || '0', 10);
  let downloaded = 0;

  const reader = response.body.getReader();
  const chunks = [];
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
    downloaded += value.length;
    if (len > 0) process.stdout.write(`\rntc: ${(downloaded / len * 100).toFixed(0)}%`);
  }
  if (len > 0) process.stdout.write('\n');

  const buffer = Buffer.concat(chunks.map(c => Buffer.from(c)));
  if (len > 0 && buffer.length !== len) {
    throw new Error(
      `incomplete download for ${path.basename(destPath)} (got ${buffer.length} of ${len} bytes). ` +
      `The connection may have dropped — retrying usually fixes this.`
    );
  }
  fs.writeFileSync(destPath, buffer);
}

async function download(url, destPath, retries = 3) {
  let lastErr;
  for (let attempt = 1; attempt <= retries; attempt++) {
    try {
      await downloadOnce(url, destPath);
      return;
    } catch (err) {
      lastErr = err;
      try { fs.rmSync(destPath, { force: true }); } catch {}
      // A missing asset will still be missing on attempt 4: let the caller
      // fall back to the next candidate immediately.
      if (err.httpStatus === 404) break;
      if (attempt < retries) {
        const waitMs = attempt * 1500;
        console.log(`ntc: download attempt ${attempt} failed (${err.message}), retrying in ${waitMs}ms...`);
        await new Promise(r => setTimeout(r, waitMs));
      }
    }
  }
  throw lastErr;
}

// ---- Minimal tar parser (ustar, files + dirs) ----
function parseOctal(buf, offset, length) {
  const str = buf.subarray(offset, offset + length).toString('utf8').replace(/\0.*$/, '').trim();
  if (str === '') return 0;
  const n = parseInt(str, 8);
  return Number.isNaN(n) ? 0 : n;
}

function readCString(buf, offset, length) {
  const end = buf.indexOf(0, offset);
  const stop = end === -1 || end > offset + length ? offset + length : end;
  return buf.subarray(offset, stop).toString('utf8');
}

function safeJoin(destDir, name) {
  // Defend against Zip-Slip / Tar-Slip.
  const target = path.resolve(destDir, name);
  const root = path.resolve(destDir) + path.sep;
  if (target !== path.resolve(destDir) && !target.startsWith(root)) {
    throw new Error(`archive entry escapes destination: ${name}`);
  }
  return target;
}

function extractTar(buffer, destDir) {
  let offset = 0;
  let longName = null;
  let count = 0;

  while (offset + 512 <= buffer.length) {
    const header = buffer.subarray(offset, offset + 512);
    const isZero = header.every(b => b === 0);
    if (isZero) break;

    const name = readCString(header, 0, 100);
    const size = parseOctal(header, 124, 12);
    const typeflag = String.fromCharCode(header[156]);
    const prefix = readCString(header, 345, 155);
    offset += 512;

    if (typeflag === 'L' || typeflag === 'K') {
      // GNU long name / long link extension.
      longName = buffer.subarray(offset, offset + size).toString('utf8').replace(/\0.*$/, '');
      offset += Math.ceil(size / 512) * 512;
      continue;
    }

    let fullName = prefix ? `${prefix}/${name}` : name;
    if (longName) { fullName = longName; longName = null; }
    // Normalize ./ prefixes and backslashes from odd packagers.
    fullName = fullName.replace(/^\.\//, '').replace(/\\/g, '/');

    const target = safeJoin(destDir, fullName);

    if (typeflag === '5' || fullName.endsWith('/')) {
      fs.mkdirSync(target, { recursive: true });
    } else if (typeflag === '0' || typeflag === '\0' || typeflag === '' || typeflag === ' ') {
      fs.mkdirSync(path.dirname(target), { recursive: true });
      const fileData = buffer.subarray(offset, offset + size);
      fs.writeFileSync(target, fileData);
      count++;
    }
    // Other types (symlink/hardlink/char/block) are skipped intentionally.

    offset += Math.ceil(size / 512) * 512;
  }

  if (count === 0) throw new Error('no files found in tar archive');
}

// ---- Minimal zip parser (stored + deflated, via central directory) ----
// Parses the central directory so entries using data descriptors
// (flag 0x08, sizes zeroed in local headers — common in real-world zips)
// still extract correctly. No system `unzip` needed.
function findEOCD(buffer) {
  // EOCD is at the end; comment can push it back up to 64KB + 22 bytes.
  const minStart = Math.max(0, buffer.length - (65557 + 22));
  for (let i = buffer.length - 22; i >= minStart; i--) {
    if (buffer.readUInt32LE(i) === 0x06054b50) return i;
  }
  return -1;
}

function extractZip(buffer, destDir) {
  const eocd = findEOCD(buffer);
  if (eocd === -1) throw new Error('not a valid zip file (EOCD not found). The download may be corrupt — try again.');

  const entryCount = buffer.readUInt16LE(eocd + 10);
  const cdOffset = buffer.readUInt32LE(eocd + 16);
  if (entryCount === 0xFFFF || cdOffset === 0xFFFFFFFF) {
    throw new Error(
      'this zip uses Zip64 (large-archive extension) which the pure-JS extractor does not support. ' +
      'The release asset needs re-uploading without Zip64 (zip -X -9, no Zip64). ' +
      'Workaround: install from source with `cargo install ntc`.'
    );
  }

  let ptr = cdOffset;
  let count = 0;

  for (let n = 0; n < entryCount; n++) {
    if (ptr + 46 > buffer.length) throw new Error('truncated zip central directory');
    if (buffer.readUInt32LE(ptr) !== 0x02014b50) throw new Error('corrupt zip central directory');
    const method = buffer.readUInt16LE(ptr + 10);
    // Central-directory layout: comp size @ +20, uncomp size @ +24
    // (see APPNOTE.TXT 4.3.12/13). A previous revision read +24 as the
    // compressed size, overshooting dataEnd on deflated entries and
    // producing bogus `truncated zip data` failures.
    const compSize = buffer.readUInt32LE(ptr + 20);
    const uncompSize = buffer.readUInt32LE(ptr + 24);
    const fileNameLen = buffer.readUInt16LE(ptr + 28);
    const extraLen = buffer.readUInt16LE(ptr + 30);
    const commentLen = buffer.readUInt16LE(ptr + 32);
    const localHeaderOffset = buffer.readUInt32LE(ptr + 42);
    if (compSize === 0xFFFFFFFF || localHeaderOffset === 0xFFFFFFFF || uncompSize === 0xFFFFFFFF) {
      throw new Error(
        `Zip64 entry for ${ptr} is not supported by the pure-JS extractor. ` +
        `The release asset needs re-uploading without Zip64.`
      );
    }
    const rawName = buffer.subarray(ptr + 46, ptr + 46 + fileNameLen).toString('utf8').replace(/\\/g, '/');
    ptr += 46 + fileNameLen + extraLen + commentLen;

    const isDir = rawName.endsWith('/');
    if (isDir) {
      fs.mkdirSync(safeJoin(destDir, rawName.replace(/^\.\//, '')), { recursive: true });
      continue;
    }

    // Local header: skip name + extra to find data start.
    if (localHeaderOffset + 30 > buffer.length) throw new Error(`truncated zip header for ${rawName}`);
    if (buffer.readUInt32LE(localHeaderOffset) !== 0x04034b50) throw new Error(`corrupt zip header for ${rawName}`);
    const lhNameLen = buffer.readUInt16LE(localHeaderOffset + 26);
    const lhExtraLen = buffer.readUInt16LE(localHeaderOffset + 28);
    const dataStart = localHeaderOffset + 30 + lhNameLen + lhExtraLen;
    const dataEnd = dataStart + compSize;
    if (dataEnd > buffer.length) {
      throw new Error(
        `truncated zip data for ${rawName} ` +
        `(need bytes ${dataStart}..${dataEnd}, archive is ${buffer.length} bytes; ` +
        `compSize=${compSize} uncompSize=${uncompSize}). ` +
        `The download was likely cut short — try again, or install from source: cargo install ntc`
      );
    }

    const target = safeJoin(destDir, rawName.replace(/^\.\//, ''));
    fs.mkdirSync(path.dirname(target), { recursive: true });
    const compData = buffer.subarray(dataStart, dataEnd);
    let out;
    if (method === 0) out = Buffer.from(compData);
    else if (method === 8) out = zlib.inflateRawSync(compData);
    else throw new Error(`unsupported zip compression method ${method} for ${rawName}. Please report this version.`);
    fs.writeFileSync(target, out);
    count++;
  }

  if (count === 0) throw new Error('no files found in zip archive');
}

function extractArchive(archivePath, destDir) {
  const buf = fs.readFileSync(archivePath);
  if (archivePath.endsWith('.zip')) {
    extractZip(buf, destDir);
  } else if (archivePath.endsWith('.tar.gz') || archivePath.endsWith('.tgz')) {
    let tarBuf;
    try {
      tarBuf = zlib.gunzipSync(buf);
    } catch (e) {
      throw new Error(`not a valid .tar.gz file (${e.message}). The download may be corrupt — try again.`);
    }
    extractTar(tarBuf, destDir);
  } else {
    throw new Error(`unknown archive format: ${path.basename(archivePath)}`);
  }
}

function ensureExecutable(binaryPath) {
  if (process.platform === 'win32') return;
  try {
    fs.chmodSync(binaryPath, 0o755);
  } catch (err) {
    console.error(`ntc: could not chmod +x ${binaryPath}: ${err.message}`);
    console.error('ntc: fix with: chmod +x "' + binaryPath + '"');
    throw err;
  }
  try {
    fs.accessSync(binaryPath, fs.constants.X_OK);
  } catch {
    console.error(`ntc: binary at ${binaryPath} is not executable.`);
    console.error(`ntc: run: chmod +x "${binaryPath}"`);
    throw new Error('binary not executable after chmod');
  }
}

function printPathHint() {
  // Print-only (never mutate shell rc from postinstall): tell the user how
  // to put the npm global bin dir on PATH when `ntc` is not found.
  try {
    const { execSync } = require('child_process');
    const prefix = execSync('npm prefix -g', { encoding: 'utf8' }).trim();
    if (!prefix) return;
    const binDir = process.platform === 'win32'
      ? prefix
      : path.join(prefix, 'bin');
    const pathVar = process.env.PATH || '';
    const needle = binDir.toLowerCase();
    const onPath = pathVar.split(path.delimiter).some(p => p.toLowerCase() === needle);
    if (onPath) return;
    console.log(`ntc: the npm global bin dir is not on your PATH:`);
    console.log(`ntc:   ${binDir}`);
    if (process.platform === 'win32') {
      console.log(`ntc: add it via Settings > System > About > Advanced system settings > Environment Variables,`);
      console.log(`ntc: or in this shell: $env:Path += ";${binDir}"`);
    } else {
      console.log(`ntc: add it with: export PATH="${binDir}:$PATH"`);
      console.log(`ntc: (append that line to ~/.bashrc or ~/.profile, then restart your shell)`);
    }
  } catch {}
}

function describePlatform() {
  let release = '';
  try { release = require('os').release() || ''; } catch {}
  const prefix = process.env.PREFIX || '(unset)';
  return `platform=${process.platform} arch=${process.arch} android=${isAndroidTermux()} release=${release || '(unknown)'} PREFIX=${prefix}`;
}

function print404Help(info) {
  const primary = info.candidates[0];
  const alts = info.candidates.slice(1);
  console.log(`ntc: binary not found for v${VERSION} on ${process.platform}-${process.arch}${isAndroidTermux() ? ' (Termux/Android)' : ''}.`);
  console.log(`ntc: expected release asset: ${primary}`);
  if (alts.length) console.log(`ntc: also tried: ${alts.join(', ')}`);
  console.log(`ntc: debug: ${describePlatform()}`);
  console.log(`ntc: CI builds macOS assets only; Linux/Windows/Android assets are uploaded manually.`);
  console.log(`ntc: check https://github.com/${REPO}/releases/tag/v${VERSION} — if ${primary} is missing,`);
  console.log('ntc: install from source instead: cargo install ntc');
}

async function main() {
  const pkgDir = path.resolve(__dirname, '..');
  const info = getPlatformInfo();

  if (!info) {
    console.log(`ntc: unsupported platform ${process.platform}-${process.arch}.`);
    console.log(`ntc: debug: ${describePlatform()}`);
    console.log('Install from source: cargo install ntc');
    process.exit(1);
  }

  const binaryPath = path.join(pkgDir, info.bin);

  if (fs.existsSync(binaryPath)) {
    // A previous install may have lost the exec bit (bad umask / copy).
    try { ensureExecutable(binaryPath); } catch {}
    return;
  }

  const tmpDir = fs.mkdtempSync(path.join(pkgDir, 'tmp-'));

  try {
    // Try each candidate asset in order. Only a 404 moves on to the next one;
    // transient network failures are retried inside download().
    let archivePath = null;
    for (let i = 0; i < info.candidates.length; i++) {
      const candidate = info.candidates[i];
      const isUrl = /^https?:\/\//i.test(candidate);
      const archiveName = isUrl
        ? (path.basename(new URL(candidate).pathname) || 'ntc-asset')
        : candidate;
      const url = isUrl
        ? candidate
        : `https://github.com/${REPO}/releases/download/v${VERSION}/${candidate}`;
      const dest = path.join(tmpDir, archiveName);

      console.log(`ntc: downloading ${archiveName}...`);
      if (i === 0) console.log('ntc: (pure-JS extractor — no system tar/unzip required)');

      try {
        await download(url, dest);
        archivePath = dest;
        break;
      } catch (err) {
        const is404 = err.httpStatus === 404 || /HTTP 404|Not Found/i.test(err.message || '');
        if (is404 && i < info.candidates.length - 1) {
          console.log(`ntc: ${archiveName} not found (404) — trying the next asset...`);
          continue;
        }
        throw err;
      }
    }

    extractArchive(archivePath, tmpDir);

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

    try {
      fs.copyFileSync(extracted, binaryPath);
    } catch (err) {
      if (err.code === 'EACCES' || err.code === 'EPERM') {
        console.error(`ntc: permission denied writing ${binaryPath}.`);
        console.error(`ntc: global installs write to your npm prefix, not /usr/local/bin directly. Try:`);
        console.error(`ntc:   sudo chown -R $(whoami) "$(npm config get prefix)/"{lib,bin}`);
        console.error(`ntc: or reinstall with elevated rights, then chmod +x the binary.`);
      }
      throw err;
    }
    ensureExecutable(binaryPath);

    console.log(`ntc: installed to ${binaryPath}`);
    printPathHint();
  } catch (err) {
    const msg = err.message || String(err);
    const status = err.httpStatus;
    if (status === 404 || msg.includes('404') || msg.includes('Not Found')) {
      print404Help(info);
    } else {
      console.error(`ntc: install failed: ${msg}`);
    }
    try { fs.rmSync(binaryPath, { force: true }); } catch {}
    process.exit(1);
  } finally {
    try { fs.rmSync(tmpDir, { recursive: true, force: true }); } catch {}
  }
}

if (require.main === module) {
  main();
}

// Exported for scripts/selftest.js (packager <-> extractor round-trip
// regression tests). main() still only runs on direct execution.
module.exports = {
  downloadOnce,
  download,
  extractTar,
  extractZip,
  extractArchive,
  getPlatformInfo,
  isAndroidTermux,
  describePlatform,
};
