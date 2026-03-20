const fs = require('fs');
const path = require('path');

const rootDir = path.join(__dirname, '..');

// 确保 dist 目录存在
const distDir = path.join(rootDir, 'dist');
if (!fs.existsSync(distDir)) {
  fs.mkdirSync(distDir, { recursive: true });
}

// 读取并修改 manifest.json
const manifestPath = path.join(rootDir, 'manifest.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));

// 修改 service_worker 路径：dist/background/index.js -> background/index.js
if (manifest.background && manifest.background.service_worker) {
  manifest.background.service_worker = manifest.background.service_worker.replace('dist/', '');
}

// 写入到 dist 目录
fs.writeFileSync(
  path.join(distDir, 'manifest.json'),
  JSON.stringify(manifest, null, 2)
);

console.log('manifest.json processed and copied to dist/');