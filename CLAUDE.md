# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Browser Sync CLI 是一个浏览器扩展 + 本地应用的组合工具，实现实时收藏夹同步。包含三个主要组件：

1. **浏览器扩展** (`packages/extension/`) - Chrome/Edge Manifest V3 扩展，监听收藏夹变化
2. **Native Host** (`packages/native-host/`) - Rust 应用，通过 Native Messaging 接收浏览器消息
3. **Tauri 应用** (`packages/app/`) - Vue 3 + Tauri 桌面应用，用于查看/搜索收藏夹

## 常用命令

```bash
# 安装所有依赖
pnpm install

# 构建所有包
pnpm build

# 单独构建
pnpm build:extension    # 浏览器扩展
pnpm build:native-host  # Rust native host
pnpm build:app          # Tauri 应用

# 开发模式运行 Tauri 应用
# 注意：包名是 browser-sync-app，不是 app
pnpm --filter browser-sync-app tauri dev
# 或者直接进入目录运行
cd packages/app && pnpm tauri dev

# 构建 native host（需要 Rust/Cargo）
cd packages/native-host && cargo build --release

# 安装 native host（Windows）
cd packages/native-host && powershell -ExecutionPolicy Bypass -File .\install.ps1

# 重新构建 Tauri 应用（完整流程）
# 1. 清理旧构建
rm -rf packages/app/dist && rm -rf packages/app/src-tauri/target

# 2. 构建 native host（Tauri 需要它）
cd packages/native-host && cargo build --release

# 3. 复制 native host 到 binaries 目录（开发模式也需要此步骤）
mkdir -p packages/app/src-tauri/binaries
cp packages/native-host/target/release/browser-sync-native-host.exe packages/app/src-tauri/binaries/browser-sync-native-host-x86_64-pc-windows-msvc.exe

# 4. 构建 Tauri 应用
cd packages/app && pnpm tauri build

# 构建产物位置：
# - NSIS: packages/app/src-tauri/target/release/bundle/nsis/shiyi_1.0.0_x64-setup.exe
# - MSI: packages/app/src-tauri/target/release/bundle/msi/shiyi_1.0.0_x64_zh-CN.msi
```

## 架构

```
┌─────────────────┐     Native Messaging     ┌─────────────────┐
│  浏览器扩展      │ ─────────────────────── │  Native Host    │
│  (Extension)    │     (STDIO)              │  (Rust)         │
└─────────────────┘                         └────────┬────────┘
                                                     │ 写入 JSON
                                                     ▼
┌─────────────────┐                         ┌─────────────────┐
│  Tauri 应用     │ ◄─── 文件监听 ────────  │  bookmarks.json │
│  (Vue 3)        │                         └─────────────────┘
└─────────────────┘
```

**数据流向：**
1. 浏览器扩展监听收藏夹 CRUD 事件
2. 扩展通过 `chrome.runtime.connectNative()` 发送完整收藏夹树到 Native Host
3. Native Host 写入 JSON 到 `%LOCALAPPDATA%\browser-sync-cli\bookmarks.json`
4. Tauri 应用监听文件变化并更新 UI

## 关键实现细节

### JSON 字段命名约定

所有与 JavaScript 交互的 Rust 结构体必须使用 `#[serde(rename_all = "camelCase")]`，因为：
- JavaScript 使用 `camelCase`（如 `lastSync`, `parentId`）
- Rust 默认使用 `snake_case`（如 `last_sync`, `parent_id`）

**需要添加此属性的文件：**
- `packages/native-host/src/main.rs`
- `packages/app/src-tauri/src/commands/bookmarks.rs`

### Native Messaging 协议

Native Messaging 使用 STDIO 通信，消息格式：
- 消息结构：4 字节小端序长度前缀 + UTF-8 JSON 内容
- 双向（浏览器→host 和 host→浏览器）使用相同格式

### Native Host 注册（Windows）

Native host 必须在 Windows 注册表中注册：
- 注册表路径：`HKCU\Software\Microsoft\Edge\NativeMessagingHosts\com.browser_sync.cli`
- 值：manifest JSON 文件路径（`%LOCALAPPDATA%\browser-sync-cli\com.browser_sync.cli.json`）
- Manifest 必须包含正确的 `allowed_origins` 和扩展 ID

### 数据文件位置

- **收藏夹数据**：`%LOCALAPPDATA%\browser-sync-cli\bookmarks.json`
- **Manifest 文件**：`%LOCALAPPDATA%\browser-sync-cli\com.browser_sync.cli.json`

## Monorepo 结构

```
packages/
├── extension/           # 浏览器扩展 (Manifest V3)
│   ├── src/
│   │   ├── background/index.ts    # Service Worker 入口
│   │   ├── bookmark-sync.ts       # 收藏夹事件监听
│   │   └── native-messaging.ts    # Native Messaging 连接
│   └── manifest.json
│
├── native-host/         # Rust Native Messaging Host
│   ├── src/main.rs               # STDIO 消息处理
│   └── install.ps1               # Windows 注册脚本
│
└── app/                 # Tauri + Vue 3 应用
    ├── src/                       # Vue 前端
    ├── src-tauri/
    │   ├── src/main.rs           # 文件监听、Tauri 设置
    │   └── src/commands/bookmarks.rs  # Tauri 命令
    └── tauri.conf.json
```

## Tauri 命令

定义在 `packages/app/src-tauri/src/commands/bookmarks.rs`：
- `get_bookmarks` - 读取并解析 bookmarks.json
- `search_bookmarks(query)` - 按标题/URL 过滤收藏夹
- `open_url(url)` - 在默认浏览器中打开 URL
- `export_bookmarks_to_path(path)` - 导出到指定路径
- `import_bookmarks(path)` - 从 JSON 文件导入

## 故障排查

**Native Messaging "host not found" 错误：**
1. 检查注册表：`reg query "HKCU\Software\Microsoft\Edge\NativeMessagingHosts\com.browser_sync.cli"`
2. 验证 manifest 文件存在于注册表指定的路径
3. 确认 `allowed_origins` 包含正确的扩展 ID
4. 修改后重启浏览器

**Rust 构建错误：**
- 需要 Visual Studio Build Tools 和 C++ 工作负载
- 安装命令：`winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"`

**JSON 解析错误 "missing field `last_sync`"：**
- 原因：Rust 结构体缺少 `#[serde(rename_all = "camelCase")]`
- 解决：在相关结构体上添加此属性

**开发模式 "path matching binaries not found" 错误：**
- 原因：Tauri 开发模式也需要 native host 二进制文件在 binaries 目录
- 解决：运行开发模式前先复制 native host：
  ```bash
  mkdir -p packages/app/src-tauri/binaries
  cp packages/native-host/target/release/browser-sync-native-host.exe packages/app/src-tauri/binaries/browser-sync-native-host-x86_64-pc-windows-msvc.exe
  ```

**pnpm filter 找不到 app 包：**
- 原因：app 包名是 `browser-sync-app`，不是 `app`
- 解决：使用正确的包名：
  ```bash
  pnpm --filter browser-sync-app tauri dev
  ```