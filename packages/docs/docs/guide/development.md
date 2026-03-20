# 开发指南

本文档面向想要参与 Browser Sync CLI 开发的开发者。

## 项目结构

```
browser-sync-cli/
├── packages/
│   ├── extension/           # 浏览器扩展 (Manifest V3)
│   │   ├── src/
│   │   │   ├── background/
│   │   │   │   └── index.ts    # Service Worker 入口
│   │   │   ├── bookmark-sync.ts    # 收藏夹事件监听
│   │   │   └── native-messaging.ts # Native Messaging 连接
│   │   ├── manifest.json
│   │   └── package.json
│   │
│   ├── native-host/         # Rust Native Messaging Host
│   │   ├── src/
│   │   │   └── main.rs          # STDIO 消息处理
│   │   ├── install.ps1          # Windows 注册脚本
│   │   ├── Cargo.toml
│   │   └── package.json
│   │
│   ├── app/                 # Tauri + Vue 3 应用
│   │   ├── src/                  # Vue 前端
│   │   ├── src-tauri/
│   │   │   ├── src/
│   │   │   │   ├── main.rs      # 文件监听、Tauri 设置
│   │   │   │   └── commands/
│   │   │   │       └── bookmarks.rs  # Tauri 命令
│   │   │   └── tauri.conf.json
│   │   └── package.json
│   │
│   └── docs/                # VuePress 文档
│       └── docs/
│
├── CLAUDE.md                # 项目说明
├── package.json             # Monorepo 根配置
└── pnpm-workspace.yaml
```

## 技术栈

| 组件 | 技术 |
|------|------|
| 浏览器扩展 | TypeScript, Manifest V3 |
| Native Host | Rust, serde, chrono |
| 桌面应用 | Vue 3, Tauri, TypeScript |
| 构建工具 | esbuild, Vite, Cargo |
| 包管理 | pnpm (Monorepo) |

## 开发环境设置

### 前置要求

1. **Node.js** >= 18
2. **pnpm** >= 8
3. **Rust** >= 1.70
4. **Visual Studio Build Tools** (Windows)

### 安装 Rust

```powershell
# 使用 rustup 安装
winget install Rustlang.Rustup

# 或访问 https://rustup.rs/
```

### 安装 Visual Studio Build Tools

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"
```

### 安装 WiX 工具（Windows 安装包构建）

WiX 是 Tauri 在 Windows 上生成 MSI 安装包所必需的工具。

**方法一：通过 winget 安装（推荐）**

需要以管理员身份运行 PowerShell：

```powershell
winget install WiXToolset.WiXToolset
```

**方法二：手动下载安装**

1. 访问 [WiX releases 页面](https://github.com/wixtoolset/wix3/releases/tag/wix3141rtm)
2. 下载 `wix314.exe` 并运行安装

**添加环境变量**

winget 安装 WiX 后可能不会自动添加到 PATH，需要手动配置：

**方法一：命令行添加（推荐）**

在 PowerShell 中执行：

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files (x86)\WiX Toolset v3.14\bin", "User")
```

**方法二：图形界面添加**

1. 按 `Win + R`，输入 `sysdm.cpl`，回车
2. 点击"高级" → "环境变量"
3. 在"用户变量"中找到 `Path`，点击"编辑"
4. 点击"新建"，添加路径：`C:\Program Files (x86)\WiX Toolset v3.14\bin`
5. 点击"确定"保存

::: warning 注意
添加环境变量后，需要**重新打开终端**才能生效。
:::

**验证安装**

```bash
# 检查 WiX 是否正确安装
light -?

# 或检查 candle（WiX 编译器）
candle -?
```

### 安装 NSIS 工具（NSIS 安装包构建）

NSIS 用于生成 Windows 安装程序（.exe 安装包）。构建时 Tauri 会自动从 GitHub 下载，如果网络不通可手动配置。

**下载地址**

1. NSIS 主体：https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip
2. Tauri NSIS 工具库：https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.4.1/nsis_tauri_utils.dll

**手动安装步骤**

1. 下载上述两个文件
2. 解压 `nsis-3.zip`
3. 将解压后的内容和 `nsis_tauri_utils.dll` 放到缓存目录：

```
C:\Users\<用户名>\.cache\tauri\NSIS\
```

最终目录结构应如下：

```
NSIS/
├── makensis.exe
├── Plugins/
│   └── x86-unicode/
│       └── nsis_tauri_utils.dll
├── Include/
├── Stubs/
└── ...其他文件
```

::: tip 提示
`nsis_tauri_utils.dll` 需要同时放在 `NSIS/` 目录和 `NSIS/Plugins/x86-unicode/` 目录下。
:::

### 安装项目依赖

```bash
pnpm install
```

## 开发工作流

### 开发浏览器扩展

```bash
# 进入扩展目录
cd packages/extension

# 开发模式（自动重新编译）
pnpm dev

# 构建
pnpm build
```

修改代码后，在浏览器扩展管理页面点击刷新按钮加载新版本。

### 开发 Native Host

```bash
# 进入 Native Host 目录
cd packages/native-host

# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test
```

修改代码后需要重新编译并重启浏览器。

### 开发桌面应用

```bash
# 进入应用目录
cd packages/app

# 开发模式（热重载）
pnpm tauri dev

# 构建
pnpm build
pnpm tauri build
```

### 开发文档

```bash
# 进入文档目录
cd packages/docs

# 开发模式
pnpm dev

# 构建
pnpm build
```

## 关键实现细节

### Native Messaging 协议

Native Messaging 使用 STDIO 通信，消息格式：

- **消息结构**: 4 字节小端序长度前缀 + UTF-8 JSON 内容
- **双向通信**: 浏览器→host 和 host→浏览器使用相同格式

```rust
// 读取消息
fn read_message() -> Option<IncomingMessage> {
    // 读取消息长度 (4 字节, 小端序)
    let mut length_bytes = [0u8; 4];
    io::stdin().read_exact(&mut length_bytes).ok()?;
    let length = u32::from_le_bytes(length_bytes) as usize;

    // 读取消息内容
    let mut message_bytes = vec![0u8; length];
    io::stdin().read_exact(&mut message_bytes).ok()?;

    // 解析 JSON
    serde_json::from_slice(&message_bytes).ok()
}

// 发送响应
fn send_response(response: &Response) {
    let json = serde_json::to_string(response).unwrap();
    let length = json.len() as u32;

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&length.to_le_bytes()).ok();
    handle.write_all(json.as_bytes()).ok();
    handle.flush().ok();
}
```

### JSON 字段命名约定

所有与 JavaScript 交互的 Rust 结构体必须使用 `#[serde(rename_all = "camelCase")]`：

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookmarkNode {
    id: String,
    title: String,
    url: Option<String>,
    parent_id: Option<String>,  // 序列化为 parentId
    date_added: Option<i64>,    // 序列化为 dateAdded
}
```

**原因**: JavaScript 使用 camelCase，Rust 默认使用 snake_case。

### 文件监听

桌面应用使用 `notify` crate 监听文件变化：

```rust
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

let (tx, rx) = channel();
let mut watcher = watcher(tx, Duration::from_millis(200))?;
watcher.watch(path, RecursiveMode::NonRecursive)?;

// 监听事件
loop {
    match rx.recv() {
        Ok(event) => {
            // 通知前端更新
            app_handle.emit_all("bookmarks-updated", ())?;
        }
        Err(_) => break,
    }
}
```

## 代码规范

### TypeScript

- 使用 ESLint 和 Prettier
- 使用严格模式 (`strict: true`)
- 优先使用 `const` 和箭头函数

### Rust

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 错误处理使用 `Result<T, String>`

## 提交规范

使用约定式提交：

```
feat: 添加新功能
fix: 修复 bug
docs: 文档更新
style: 代码格式调整
refactor: 代码重构
test: 测试相关
chore: 构建/工具链更新
```