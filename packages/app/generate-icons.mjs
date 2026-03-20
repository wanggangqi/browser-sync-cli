import sharp from 'sharp'
import { readFileSync, mkdirSync, existsSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const iconsDir = join(__dirname, 'src-tauri', 'icons')

// SVG 内容
const svgContent = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <defs>
    <linearGradient id="bgGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#1a3a5c"/>
      <stop offset="100%" stop-color="#2c5282"/>
    </linearGradient>
    <linearGradient id="crossGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#ffd700"/>
      <stop offset="50%" stop-color="#ffb700"/>
      <stop offset="100%" stop-color="#ff8c00"/>
    </linearGradient>
  </defs>
  <rect width="512" height="512" rx="100" fill="url(#bgGradient)"/>
  <rect x="236" y="100" width="40" height="312" rx="8" fill="url(#crossGradient)"/>
  <rect x="100" y="236" width="312" height="40" rx="8" fill="url(#crossGradient)"/>
</svg>`

async function generateIcons() {
  if (!existsSync(iconsDir)) {
    mkdirSync(iconsDir, { recursive: true })
  }

  // 生成 PNG 图标
  const sizes = [32, 128, 256, 512]
  const pngBuffers = []

  for (const size of sizes) {
    const buffer = await sharp(Buffer.from(svgContent))
      .resize(size, size)
      .png()
      .toBuffer()
    pngBuffers.push(buffer)

    const filename = size === 32 ? '32x32.png' : `${size}x${size}.png`
    await sharp(buffer).toFile(join(iconsDir, filename))
    console.log(`Generated ${filename}`)
  }

  // 生成 ICO 文件 (Windows)
  // ICO 需要包含多个尺寸: 16, 32, 48, 256
  const icoSizes = [16, 32, 48, 256]
  const icoPngBuffers = []

  for (const size of icoSizes) {
    const buffer = await sharp(Buffer.from(svgContent))
      .resize(size, size)
      .png()
      .toBuffer()
    icoPngBuffers.push(buffer)
  }

  // 使用 png-to-ico 格式手动构建 ICO
  const icoBuffer = await createIco(icoPngBuffers, icoSizes)
  await import('fs').then(fs => fs.promises.writeFile(join(iconsDir, 'icon.ico'), icoBuffer))
  console.log('Generated icon.ico')

  // 生成 ICNS 文件 (macOS) - 使用 png 序列
  // Tauri 会自动处理，我们只需要提供 512x512 的 PNG
  await sharp(pngBuffers[3]).toFile(join(iconsDir, 'icon.png'))
  console.log('Generated icon.png')

  console.log('All icons generated successfully!')
}

// 简单的 ICO 文件生成
async function createIco(pngBuffers, sizes) {
  // ICO 文件头
  const header = Buffer.alloc(6)
  header.writeUInt16LE(0, 0) // 保留
  header.writeUInt16LE(1, 2) // 类型: 1 = ICO
  header.writeUInt16LE(pngBuffers.length, 4) // 图像数量

  // 目录条目
  const entries = []
  let dataOffset = 6 + pngBuffers.length * 16

  for (let i = 0; i < pngBuffers.length; i++) {
    const entry = Buffer.alloc(16)
    const size = sizes[i]
    entry.writeUInt8(size > 255 ? 0 : size, 0) // 宽度
    entry.writeUInt8(size > 255 ? 0 : size, 1) // 高度
    entry.writeUInt8(0, 2) // 调色板颜色数
    entry.writeUInt8(0, 3) // 保留
    entry.writeUInt16LE(1, 4) // 颜色平面
    entry.writeUInt16LE(32, 6) // 每像素位数
    entry.writeUInt32LE(pngBuffers[i].length, 8) // 图像大小
    entry.writeUInt32LE(dataOffset, 12) // 图像偏移
    entries.push(entry)
    dataOffset += pngBuffers[i].length
  }

  return Buffer.concat([header, ...entries, ...pngBuffers])
}

generateIcons().catch(console.error)