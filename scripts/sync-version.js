#!/usr/bin/env node

/**
 * 将 Cargo.toml 中的版本号同步到 package.json
 *
 * 用法: node scripts/sync-version.js
 *
 * 发布流程:
 *   make release VERSION=x.y.z   → 改 Cargo.toml + 打 tag
 *   git push origin --tags       → GitHub Actions 自动 npm publish
 *
 * CI 中 publish.yml 会先跑此脚本再 npm publish，
 * 确保 package.json 的 version 与 git tag 一致。
 */

const fs = require('fs');
const path = require('path');

const cargoPath = path.join(__dirname, '..', 'Cargo.toml');
const pkgPath = path.join(__dirname, '..', 'package.json');

// 从 Cargo.toml 读取版本
const cargoRaw = fs.readFileSync(cargoPath, 'utf-8');
const match = cargoRaw.match(/^version\s*=\s*"([^"]+)"/m);
if (!match) {
  console.error('❌ 无法从 Cargo.toml 中解析版本号');
  process.exit(1);
}

const cargoVersion = match[1];

// 读取 package.json
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));

if (pkg.version === cargoVersion) {
  console.log(`✅ 版本一致: ${cargoVersion}`);
  process.exit(0);
}

// 同步版本
pkg.version = cargoVersion;
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
console.log(`✅ package.json 版本已同步: ${cargoVersion}`);
