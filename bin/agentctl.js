#!/usr/bin/env node

/**
 * agentctl — CLI binary wrapper
 *
 * This JS shim spawns the native Rust binary that was downloaded
 * during `npm install` (postinstall script).
 */
const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const binaryName = 'agentctl';
const binaryPath = path.join(__dirname, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error();
  console.error('  ╔══════════════════════════════════════════════════════╗');
  console.error('  ║  agentctl binary not found                         ║');
  console.error(`  ║  Expected at: ${binaryPath}  ║`);
  console.error('  ║                                                    ║');
  console.error('  ║  Run the agentctl install script again:            ║');
  console.error('  ║    $ node node_modules/agentctl/scripts/install.js ║');
  console.error('  ║                                                    ║');
  console.error('  ║  Or rebuild from source:                           ║');
  console.error('  ║    $ cargo build --release                         ║');
  console.error('  ╚══════════════════════════════════════════════════════╝');
  console.error();
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

process.exit(result.status ?? 1);
