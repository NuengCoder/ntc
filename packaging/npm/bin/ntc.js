#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binaryName = process.platform === 'win32' ? 'ntc.exe' : 'ntc';
const binaryPath = path.join(__dirname, '..', binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(`ntc binary not found at ${binaryPath}.`);
  console.error('ntc: the postinstall download may have failed. Try reinstalling: npm install -g @nuengcoder/ntc');
  console.error('ntc: or install from source: cargo install ntc');
  process.exit(1);
}

// Self-heal a lost exec bit (bad umask / manual copy). No system tar needed.
if (process.platform !== 'win32') {
  try {
    fs.accessSync(binaryPath, fs.constants.X_OK);
  } catch {
    try {
      fs.chmodSync(binaryPath, 0o755);
    } catch (err) {
      console.error(`ntc: binary is not executable: ${binaryPath} (${err.message})`);
      console.error(`ntc: run: chmod +x "${binaryPath}"`);
      process.exit(1);
    }
  }
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true,
});

child.on('error', (err) => {
  if (err.code === 'EACCES') {
    console.error(`ntc: permission denied executing ${binaryPath}.`);
    console.error(`ntc: run: chmod +x "${binaryPath}"`);
  } else {
    console.error(`ntc: failed to launch binary: ${err.message}`);
  }
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code ?? 0);
});
