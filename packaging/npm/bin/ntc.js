#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binaryName = process.platform === 'win32' ? 'ntc.exe' : 'ntc';
const binaryPath = path.join(__dirname, '..', binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error('ntc binary not found. Try reinstalling: npm install -g @nuengcoder/ntc');
  process.exit(1);
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true,
});

child.on('exit', (code) => {
  process.exit(code ?? 0);
});
