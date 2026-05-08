#!/usr/bin/env node

/**
 * agentctl — 源码编译安装器
 *
 * postinstall 脚本。优先从源码编译 (cargo build --release)，
 * 也支持 $AGENTCTL_BIN_PATH 环境变量指定预编译二进制路径。
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const BINARY_NAME = 'agentctl';
const PKG_DIR = path.resolve(__dirname, '..');
const BIN_DIR = path.join(PKG_DIR, 'bin');
const TARGET = path.join(BIN_DIR, BINARY_NAME);
const CARGO_TARGET = path.join(PKG_DIR, 'target', 'release', BINARY_NAME);

function log(...args) { console.log('[agentctl]', ...args); }
function warn(...args) { console.warn('[agentctl] ⚠️', ...args); }
function err(...args) { console.error('[agentctl] ❌', ...args); }

// ── 方式 1: AGENTCTL_BIN_PATH 环境变量 ──────────────────────────────

function tryEnvVar() {
  const customPath = process.env.AGENTCTL_BIN_PATH;
  if (!customPath) return false;
  if (!fs.existsSync(customPath)) {
    warn(`AGENTCTL_BIN_PATH 指定的文件不存在: ${customPath}`);
    return false;
  }
  log('使用 AGENTCTL_BIN_PATH:', customPath);
  fs.mkdirSync(BIN_DIR, { recursive: true });
  fs.copyFileSync(customPath, TARGET);
  fs.chmodSync(TARGET, 0o755);
  return true;
}

// ── 方式 2: 本地已有编译产物 ──────────────────────────────────────────

function tryLocalBuild() {
  // 从 scripts/ 向上找最近的 target/release/agentctl
  let dir = path.resolve(__dirname);
  for (let i = 0; i < 5; i++) {
    dir = path.dirname(dir);
    const binPath = path.join(dir, 'target', 'release', BINARY_NAME);
    if (fs.existsSync(binPath)) {
      log('发现本地编译产物:', binPath);
      fs.mkdirSync(BIN_DIR, { recursive: true });
      fs.copyFileSync(binPath, TARGET);
      fs.chmodSync(TARGET, 0o755);
      return true;
    }
  }
  return false;
}

// ── 方式 3: 从源码编译（主路径） ─────────────────────────────────────

function tryCompileFromSource() {
  // 检查 cargo 是否可用
  try {
    execSync('cargo --version', { stdio: 'pipe' });
  } catch {
    warn('未检测到 Rust 工具链 (cargo)');
    return false;
  }

  log('正在从源码编译 agentctl...');
  log('这可能需要几分钟（首次需要下载 Rust 依赖）');

  try {
    execSync('cargo build --release', {
      cwd: PKG_DIR,
      stdio: 'inherit',
    });

    if (!fs.existsSync(CARGO_TARGET)) {
      err('编译成功但未找到产物:', CARGO_TARGET);
      return false;
    }

    fs.mkdirSync(BIN_DIR, { recursive: true });
    fs.copyFileSync(CARGO_TARGET, TARGET);
    fs.chmodSync(TARGET, 0o755);
    log('编译完成，二进制安装到:', TARGET);
    return true;

  } catch (e) {
    warn('编译失败:', e.message);
    return false;
  }
}

// ── 入口 ─────────────────────────────────────────────────────────────

function main() {
  log('agentctl 安装器');

  fs.mkdirSync(BIN_DIR, { recursive: true });

  // 优先级: 环境变量 > 本地编译产物 > 源码编译
  if (tryEnvVar()) {
    log('✅ 已通过 AGENTCTL_BIN_PATH 安装');
    process.exit(0);
  }

  if (tryLocalBuild()) {
    log('✅ 已使用本地编译产物');
    process.exit(0);
  }

  if (tryCompileFromSource()) {
    log('✅ 源码编译安装成功');
    process.exit(0);
  }

  // 全部失败
  err('');
  err('无法自动安装 agentctl');
  err('');
  err('  请确保已安装 Rust 工具链:');
  err('    $ curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh');
  err('');
  err('  然后重试:');
  err('    $ npm install -g agentctl');
  err('');
  err('  或手动指定预编译二进制路径:');
  err('    $ AGENTCTL_BIN_PATH=/path/to/agentctl npm install -g agentctl');
  err('');
  process.exit(1);
}

main();
