# 安装指南

本指南介绍如何安装 Browser Sync CLI。

## 系统要求

- **操作系统**: Windows 10/11（64位）
- **浏览器**: Chrome 或 Edge（最新版本）
- **磁盘空间**: 约 50MB

## 下载

从 [GitHub Releases](https://github.com/wanggangqi/browser-sync-cli/releases) 下载最新版本：

| 文件 | 说明 |
|------|------|
| `Browser-Sync-CLI_x64-setup.exe` | Windows 安装程序（推荐） |
| `extension.zip` | 浏览器扩展（Chrome/Edge） |

> 注意：解压 `extension.zip` 后，扩展目录为 `dist` 文件夹。

## 安装桌面应用

### 方式一：安装程序（推荐）

1. 下载 `Browser-Sync-CLI_x64-setup.exe`
2. 双击运行
3. 按照向导完成安装
4. 安装完成后会自动启动应用

安装程序会自动完成以下配置：
- 注册 Native Messaging Host
- 创建桌面快捷方式
- 创建开始菜单项

## 安装浏览器扩展

### Chrome 浏览器

1. 解压 `extension.zip` 到一个固定目录
   > 注意：解压后会有一个 `dist` 文件夹，这是扩展的实际目录

2. 打开 Chrome，访问 `chrome://extensions/`

3. 打开右上角的 **开发者模式** 开关

4. 点击 **加载已解压的扩展程序**

5. 选择解压后的 `dist` 目录

6. 安装成功后，记录显示的扩展 ID（类似 `abcdefghijklmnopqrstuvwxyz123456`）

### Edge 浏览器

1. 解压 `extension.zip` 到一个固定目录

2. 打开 Edge，访问 `edge://extensions/`

3. 打开左侧的 **开发人员模式** 开关

4. 点击 **加载解压缩的扩展**

5. 选择解压后的 `dist` 目录

6. 安装成功后，记录显示的扩展 ID

## 配置扩展 ID

为了让浏览器扩展与桌面应用通信，需要配置扩展 ID：

### 使用桌面应用配置（推荐）

1. 启动 Browser Sync CLI 桌面应用
2. 点击右上角 ⚙️ 进入设置
3. 在"扩展 ID"区域填写：
   - Chrome 扩展 ID
   - Edge 扩展 ID（如使用 Edge）
4. 点击"保存"
5. **重启浏览器**

### 手动配置

1. 打开文件：
   ```
   %LOCALAPPDATA%\browser-sync-cli\com.browser_sync.cli.json
   ```

2. 修改 `allowed_origins` 数组：
   ```json
   {
     "name": "com.browser_sync.cli",
     "description": "Browser Sync CLI Native Messaging Host",
     "path": "C:\\path\\to\\browser-sync-native-host.exe",
     "type": "stdio",
     "allowed_origins": [
       "chrome-extension://你的Chrome扩展ID/",
       "chrome-extension://你的Edge扩展ID/"
     ]
   }
   ```

3. 保存文件
4. **重启浏览器**

## 验证安装

### 检查扩展连接状态

1. 打开浏览器扩展管理页面
2. 找到 Browser Sync CLI 扩展
3. 点击"Service Worker"查看控制台
4. 如果显示 "Connected to native host"，说明连接成功

### 测试同步功能

1. 在浏览器中添加一个新收藏夹
2. 打开桌面应用，查看是否显示新收藏夹
3. 或检查文件 `%LOCALAPPDATA%\browser-sync-cli\bookmarks.json` 是否更新

## 下一步

- [使用指南](./usage.md) - 了解如何使用各项功能
- [故障排查](./troubleshooting.md) - 解决常见问题