#!/usr/bin/env node
// Self-test for the pure-JS extractor in install.js (no system tar/unzip).
// No dependencies, no network. Exit non-zero on any failure.
//
// Covers:
//   1. zip: stored + deflated entries, dir entry, ./ prefix, backslashes,
//      data-descriptor entry (flag 0x08, zeroed local sizes).
//   2. tar.gz: hand-built ustar archive through the full extractArchive path.
//   3. round-trip with ntc's own `arc create` (the release packager):
//      skipped unless an ntc binary is found (NTC_BIN or repo target/debug).
const fs = require('fs');
const os = require('os');
const path = require('path');
const zlib = require('zlib');
const { execFileSync } = require('child_process');
const { extractArchive } = require('./install.js');

let failures = 0;
function check(name, cond, extra) {
  if (cond) {
    console.log(`ntc selftest: PASS ${name}`);
  } else {
    failures++;
    console.error(`ntc selftest: FAIL ${name}${extra ? ` — ${extra}` : ''}`);
  }
}

function crc32(buf) {
  let table = crc32.table;
  if (!table) {
    table = crc32.table = new Int32Array(256);
    for (let n = 0; n < 256; n++) {
      let c = n;
      for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
      table[n] = c;
    }
  }
  let crc = -1;
  for (const b of buf) crc = table[(crc ^ b) & 0xff] ^ (crc >>> 8);
  return (crc ^ -1) >>> 0;
}

// Minimal zip builder: local headers + central directory + EOCD.
// method 0 = stored, 8 = deflated. descriptor = streaming style
// (GP flag 0x08, sizes zeroed in the local header + trailing descriptor).
function buildZip(entries) {
  const localParts = [];
  const centralParts = [];
  let offset = 0;
  for (const e of entries) {
    if (e.dir) {
      const nameBuf = Buffer.from(e.name, 'utf8');
      const cd = Buffer.alloc(46);
      cd.writeUInt32LE(0x02014b50, 0);
      cd.writeUInt16LE(20, 4);
      cd.writeUInt16LE(20, 6);
      cd.writeUInt16LE(0, 8);
      cd.writeUInt16LE(0, 10);
      cd.writeUInt16LE(0, 12);
      cd.writeUInt16LE(0, 14);
      cd.writeUInt32LE(0, 16);
      cd.writeUInt32LE(0, 20);
      cd.writeUInt32LE(0, 24);
      cd.writeUInt16LE(nameBuf.length, 28);
      cd.writeUInt16LE(0, 30);
      cd.writeUInt16LE(0, 32);
      cd.writeUInt16LE(0, 34);
      cd.writeUInt16LE(0, 36);
      cd.writeUInt32LE(0, 38);
      cd.writeUInt32LE(offset, 42);
      centralParts.push(cd, nameBuf);
      continue;
    }
    const nameBuf = Buffer.from(e.name, 'utf8');
    const data = Buffer.from(e.data);
    const comp = e.method === 8 ? zlib.deflateRawSync(data) : data;
    const crc = crc32(data);
    const gp = e.descriptor ? 0x08 : 0x00;
    const lh = Buffer.alloc(30);
    lh.writeUInt32LE(0x04034b50, 0);
    lh.writeUInt16LE(20, 4);
    lh.writeUInt16LE(gp, 6);
    lh.writeUInt16LE(e.method, 8);
    lh.writeUInt16LE(0, 10);
    lh.writeUInt16LE(0, 12);
    lh.writeUInt32LE(e.descriptor ? 0 : crc, 14);
    lh.writeUInt32LE(e.descriptor ? 0 : comp.length, 18);
    lh.writeUInt32LE(e.descriptor ? 0 : data.length, 22);
    lh.writeUInt16LE(nameBuf.length, 26);
    lh.writeUInt16LE(0, 28);
    localParts.push(lh, nameBuf, comp);
    if (e.descriptor) {
      const dd = Buffer.alloc(12);
      dd.writeUInt32LE(crc, 0);
      dd.writeUInt32LE(comp.length, 4);
      dd.writeUInt32LE(data.length, 8);
      localParts.push(dd);
    }
    const cd = Buffer.alloc(46);
    cd.writeUInt32LE(0x02014b50, 0);
    cd.writeUInt16LE(20, 4);
    cd.writeUInt16LE(20, 6);
    cd.writeUInt16LE(gp, 8);
    cd.writeUInt16LE(e.method, 10);
    cd.writeUInt16LE(0, 12);
    cd.writeUInt16LE(0, 14);
    cd.writeUInt32LE(crc, 16);
    cd.writeUInt32LE(comp.length, 20);
    cd.writeUInt32LE(data.length, 24);
    cd.writeUInt16LE(nameBuf.length, 28);
    cd.writeUInt16LE(0, 30);
    cd.writeUInt16LE(0, 32);
    cd.writeUInt16LE(0, 34);
    cd.writeUInt16LE(0, 36);
    cd.writeUInt32LE(0, 38);
    cd.writeUInt32LE(offset, 42);
    centralParts.push(cd, nameBuf);
    offset += 30 + nameBuf.length + comp.length + (e.descriptor ? 12 : 0);
  }
  const central = Buffer.concat(centralParts);
  const eocd = Buffer.alloc(22);
  eocd.writeUInt32LE(0x06054b50, 0);
  eocd.writeUInt16LE(0, 4);
  eocd.writeUInt16LE(0, 6);
  eocd.writeUInt16LE(entries.length, 8);
  eocd.writeUInt16LE(entries.length, 10);
  eocd.writeUInt32LE(central.length, 12);
  eocd.writeUInt32LE(offset, 16);
  eocd.writeUInt16LE(0, 20);
  return Buffer.concat([...localParts, central, eocd]);
}

// Minimal ustar builder: regular files only.
function buildTar(entries) {
  const parts = [];
  for (const e of entries) {
    const data = Buffer.from(e.data);
    const h = Buffer.alloc(512, 0);
    h.write(e.name, 0, 100, 'utf8');
    h.write('0000777\0', 100, 8, 'utf8');
    h.write('0000000\0', 108, 8, 'utf8');
    h.write('0000000\0', 116, 8, 'utf8');
    h.write(data.length.toString(8).padStart(11, '0') + '\0', 124, 12, 'utf8');
    h.write('00000000000\0', 136, 12, 'utf8');
    h.write('        ', 148, 8, 'utf8');
    h.write('0', 156, 1, 'utf8');
    h.write('ustar\0', 257, 6, 'utf8');
    h.write('00', 263, 2, 'utf8');
    let sum = 0;
    for (const b of h) sum += b;
    h.write(sum.toString(8).padStart(6, '0') + '\0 ', 148, 8, 'utf8');
    parts.push(h, data, Buffer.alloc((512 - (data.length % 512)) % 512));
  }
  parts.push(Buffer.alloc(1024));
  return Buffer.concat(parts);
}

function testZip() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'ntc-selftest-zip-'));
  try {
    const payload = Buffer.from('hello ntc round-trip\n'.repeat(200));
    const buf = buildZip([
      { name: 'ntc-test.txt', data: payload, method: 8 },
      { name: 'stored.bin', data: Buffer.from([0, 1, 2, 3, 250]), method: 0 },
      { name: './dot-slash.txt', data: Buffer.from('dot slash'), method: 8 },
      { name: 'back\\slash.txt', data: Buffer.from('backslash'), method: 8 },
      { name: 'streamed.txt', data: payload, method: 8, descriptor: true },
      { name: 'sub/', dir: true },
      { name: 'sub/nested.txt', data: Buffer.from('nested'), method: 8 },
    ]);
    const archive = path.join(dir, 't.zip');
    fs.writeFileSync(archive, buf);
    const out = path.join(dir, 'out');
    fs.mkdirSync(out);
    extractArchive(archive, out);
    check('zip deflated', fs.readFileSync(path.join(out, 'ntc-test.txt')).equals(payload));
    check('zip stored', fs.readFileSync(path.join(out, 'stored.bin')).equals(Buffer.from([0, 1, 2, 3, 250])));
    check('zip ./ prefix', fs.readFileSync(path.join(out, 'dot-slash.txt'), 'utf8') === 'dot slash');
    check('zip backslash', fs.readFileSync(path.join(out, 'back/slash.txt'), 'utf8') === 'backslash');
    check('zip data-descriptor', fs.readFileSync(path.join(out, 'streamed.txt')).equals(payload));
    check('zip nested', fs.readFileSync(path.join(out, 'sub/nested.txt'), 'utf8') === 'nested');
  } catch (err) {
    check('zip extract', false, err.message);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

function testTarGz() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'ntc-selftest-tgz-'));
  try {
    const payload = Buffer.from('tarball content\n'.repeat(100));
    const tarBuf = buildTar([
      { name: 'ntc-test.txt', data: payload },
      { name: './prefixed.txt', data: Buffer.from('prefixed') },
    ]);
    const archive = path.join(dir, 't.tar.gz');
    fs.writeFileSync(archive, zlib.gzipSync(tarBuf));
    const out = path.join(dir, 'out');
    fs.mkdirSync(out);
    extractArchive(archive, out);
    check('tar.gz file', fs.readFileSync(path.join(out, 'ntc-test.txt')).equals(payload));
    check('tar.gz ./ prefix', fs.readFileSync(path.join(out, 'prefixed.txt'), 'utf8') === 'prefixed');
  } catch (err) {
    check('tar.gz extract', false, err.message);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

function findNtcBinary() {
  if (process.env.NTC_BIN && fs.existsSync(process.env.NTC_BIN)) return process.env.NTC_BIN;
  const exe = process.platform === 'win32' ? 'ntc.exe' : 'ntc';
  const candidates = [
    path.resolve(__dirname, '..', '..', '..', 'target', 'debug', exe),
    path.resolve(process.cwd(), 'target', 'debug', exe),
  ];
  for (const c of candidates) {
    try {
      if (fs.existsSync(c)) return c;
    } catch {}
  }
  return null;
}

// The release packagers are ntc's own `arc create` (.zip and .tar.gz).
// Pack with it, unpack with install.js, byte-compare: the pair can never
// drift apart silently.
function testArcRoundTrip() {
  const bin = findNtcBinary();
  if (!bin) {
    console.log('ntc selftest: SKIP arc round-trip (no ntc binary found; set NTC_BIN)');
    return;
  }
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'ntc-selftest-arc-'));
  try {
    const payload = Buffer.from('arc round-trip payload\n'.repeat(500));
    const src = path.join(dir, 'ntc.exe');
    fs.writeFileSync(src, payload);
    for (const archiveName of ['pkg.zip', 'pkg.tar.gz']) {
      const archive = path.join(dir, archiveName);
      execFileSync(bin, ['arc', 'create', archive, src], { stdio: 'pipe' });
      const out = path.join(dir, `out-${archiveName}`);
      fs.mkdirSync(out);
      extractArchive(archive, out);
      check(
        `arc create ${archiveName} -> extract round-trip`,
        fs.existsSync(path.join(out, 'ntc.exe')) &&
          fs.readFileSync(path.join(out, 'ntc.exe')).equals(payload)
      );
    }
  } catch (err) {
    check('arc round-trip', false, err.message);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

testZip();
testTarGz();
testArcRoundTrip();

if (failures > 0) {
  console.error(`ntc selftest: ${failures} failure(s)`);
  process.exit(1);
}
console.log('ntc selftest: all passed');
