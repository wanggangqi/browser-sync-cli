# 故障排查

本文档收集了常见问题及其解决方案。

## Native Messaging 相关

### 错误: "Specified native messaging host not found"

**原因**: Native Host 未正确注册到系统。

**解决方案**:

1. 检查注册表是否正确：

   ```powershell
   # Chrome
   reg query "HKCU\Software\Google\Chrome\NativeMessagingHosts\com.browser_sync.cli"

   # Edge
   reg query "HKCU\Software\Microsoft\Edge\NativeMessagingHosts\com.browser_sync.cli"
   ```

2. 验证 Manifest 文件存在：

   ```powershell
   type "$env:LOCALAPPDATA\browser-sync-cli\com.browser_sync.cli.json"
   ```

3. 确认 `allowed_origins` 包含正确的扩展 ID

4. 重启浏览器

### 错误: "Error when communicating with the native messaging host"

**原因**: Native Host 进程崩溃或消息格式错误。

**解决方案**:

1. 检查 Native Host 可执行文件是否存在

2. 手动运行 Native Host 测试：

   ```powershell
   # 进入目录
   cd packages/native-host/target/release

   # 直接运行（会等待输入）
   .\browser-sync-native-host.exe
   ```

3. 检查浏览器扩展控制台的错误信息

### 扩展 ID 变化导致连接失败

**原因**: 每次加载未打包的扩展，ID 可能会变化。

**解决方案**:

1. 固定扩展 ID：
   - 在 `manifest.json` 中添加 `key` 字段
   - 或使用打包后的扩展

2. 更新 Manifest 文件中的 `allowed_origins`

## 构建相关

### Rust 编译错误: "linker 'link.exe' not found"

**原因**: 未安装 Visual Studio Build Tools。

**解决方案**:

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"
```

安装后重启终端。

### Tauri 构建失败

**原因**: 缺少 WebView2 或其他依赖。

**解决方案**:

1. 安装 WebView2：

   ```powershell
   winget install Microsoft.EdgeWebView2Runtime
   ```

2. 检查 Rust 版本：

   ```bash
   rustc --version  # 需要 >= 1.70
   ```

3. 清理并重新构建：

   ```bash
   cd packages/app
   cargo clean
   pnpm tauri build
   ```

### pnpm install 失败

**原因**: 网络问题或缓存损坏。

**解决方案**:

```bash
# 清理缓存
pnpm store prune

# 删除 node_modules
rm -rf node_modules packages/*/node_modules

# 重新安装
pnpm install
```

## JSON 解析相关

### 错误: "missing field `lastSync`"

**原因**: Rust 结构体缺少 `#[serde(rename_all = "camelCase")]`。

**解决方案**:

确保所有与 JavaScript 交互的 Rust 结构体都有此属性：

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // 必须添加
struct BrowserBookmarks {
    bookmarks: Vec<BookmarkNode>,
    last_sync: Option<String>,  // 会被序列化为 lastSync
}
```

### JSON 格式错误

**解决方案**:

1. 删除现有数据文件重新生成：

   ```powershell
   rm "$env:LOCALAPPDATA\browser-sync-cli\bookmarks.json"
   ```

2. 在浏览器中添加一个新收藏夹触发同步

## 浏览器扩展相关

### Service Worker 不工作

**原因**: Manifest V3 的 Service Worker 可能进入休眠状态。

**解决方案**:

1. 在扩展管理页面点击 "Service Worker" 链接查看控制台

2. 使用 `chrome.alarms` API 保持唤醒

3. 检查 `manifest.json` 中的权限配置

### 收藏夹事件未触发

**原因**: 事件监听器注册失败。

**解决方案**:

1. 检查 `manifest.json` 是否包含 `bookmarks` 权限

2. 在 Service Worker 中添加调试日志

3. 重新加载扩展

## 桌面应用相关

### 应用无法启动

**解决方案**:

1. 检查 WebView2 是否安装

2. 以管理员权限运行

3. 查看日志文件（如果有）

### 文件监听不工作

**原因**: 文件路径错误或权限问题。

**解决方案**:

1. 确认数据文件存在：

   ```powershell
   Test-Path "$env:LOCALAPPDATA\browser-sync-cli\bookmarks.json"
   ```

2. 检查应用是否有读取权限

3. 重启应用

## 调试技巧

### 查看扩展日志

1. 打开 `chrome://extensions/` 或 `edge://extensions/`
2. 找到 Bookmark Sync 扩展
3. 点击 "Service Worker" 链接
4. 在控制台中查看日志

### 查看 Native Host 日志

Native Host 的 `eprintln!` 输出会被浏览器捕获：

1. 打开扩展的 Service Worker 控制台
2. 查看 Native Host 的 stderr 输出

### 手动测试 Native Messaging

创建测试脚本：

```javascript
// 在浏览器控制台中运行
const host = chrome.runtime.connectNative('com.browser_sync.cli');
host.onMessage.addListener(msg => console.log('Received:', msg));
host.onDisconnect.addListener(() => console.log('Disconnected'));
host.postMessage({ type: 'ping', data: {}, browser: 'chrome' });
```

## 获取帮助

如果以上方法都无法解决问题：

1. 查看项目 GitHub Issues
2. 提交新 Issue，包含：
   - 操作系统版本
   - 浏览器版本
   - 错误信息截图
   - 复现步骤