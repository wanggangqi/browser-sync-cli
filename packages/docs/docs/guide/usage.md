# 使用指南

本指南面向最终用户，介绍如何安装和使用 Browser Sync CLI。

## 发布物

Browser Sync CLI 发布以下安装包：

| 产物 | 文件名 | 说明 |
|------|--------|------|
| 桌面应用 (Windows) | `Browser-Sync-CLI_x64-setup.exe` | Windows 安装程序 |
| 桌面应用 (便携版) | `Browser-Sync-CLI_x64.zip` | 免安装便携版 |
| 浏览器扩展 | `extension.zip` | Chrome/Edge 扩展包 |

## 安装步骤

### 1. 安装桌面应用

#### 方式一：安装程序

1. 下载 `Browser-Sync-CLI_x64-setup.exe`
2. 双击运行安装程序
3. 选择安装路径，完成安装
4. 安装程序会自动：
   - 复制 Native Host 程序
   - 注册 Native Messaging Host
   - 创建桌面快捷方式

#### 方式二：便携版

1. 下载 `Browser-Sync-CLI_x64.zip`
2. 解压到任意目录
3. 运行 `install.ps1` 注册 Native Host：
   ```powershell
   powershell -ExecutionPolicy Bypass -File .\install.ps1
   ```
4. 运行 `Browser-Sync-CLI.exe` 启动应用

### 2. 安装浏览器扩展

#### Chrome 浏览器

1. 解压 `extension.zip` 到一个目录
2. 打开 `chrome://extensions/`
3. 启用右上角的 **开发者模式**
4. 点击 **加载已解压的扩展程序**
5. 选择解压的扩展目录
6. 记下显示的扩展 ID

#### Edge 浏览器

1. 解压 `extension.zip` 到一个目录
2. 打开 `edge://extensions/`
3. 启用左侧的 **开发人员模式**
4. 点击 **加载解压缩的扩展**
5. 选择解压的扩展目录
6. 记下显示的扩展 ID

### 3. 配置扩展 ID

安装扩展后，需要配置 Native Messaging 权限：

1. 打开桌面应用
2. 进入 **设置** 页面
3. 输入扩展 ID（Chrome 和 Edge 分别填写）
4. 点击 **保存配置**
5. **重启浏览器**

::: tip 提示
如果使用安装程序，此步骤会自动完成。便携版需要手动配置。
:::

## 使用方法

### 自动同步

安装完成后，收藏夹会自动同步：

- 在浏览器中添加、删除、修改收藏夹
- 所有变更自动保存到本地文件
- 无需手动操作

### 桌面应用功能

#### 搜索收藏夹

1. 启动 Browser Sync CLI 应用
2. 在搜索框输入关键词
3. 支持按标题和 URL 搜索
4. 结果实时显示

#### 打开收藏夹

- 双击搜索结果，在默认浏览器中打开
- 或右键选择"在新标签页打开"

#### 导出/导入

- **导出**：将收藏夹导出为 JSON 文件备份
- **导入**：从 JSON 文件恢复收藏夹

::: warning 注意
导入会覆盖当前数据，请先备份。
:::

#### 切换浏览器

如果同时使用 Chrome 和 Edge：

1. 点击左上角下拉菜单
2. 选择要查看的浏览器
3. 切换收藏夹数据源

## 数据存储

### 文件位置

```
%LOCALAPPDATA%\browser-sync-cli\
├── bookmarks.json          # 收藏夹数据
├── com.browser_sync.cli.json  # Native Messaging 配置
└── browser-sync-native-host.exe  # Native Host 程序
```

### 数据格式

`bookmarks.json` 结构：

```json
{
  "chrome": {
    "bookmarks": [...],
    "lastSync": "2024-01-01T00:00:00Z"
  },
  "edge": {
    "bookmarks": [...],
    "lastSync": "2024-01-01T00:00:00Z"
  }
}
```

## 卸载

### 卸载桌面应用

**安装版：**
- 在 Windows 设置中卸载"Browser Sync CLI"

**便携版：**
- 删除解压目录
- 运行 `uninstall.ps1` 清理注册

### 卸载浏览器扩展

1. 打开浏览器扩展管理页面
2. 找到 Browser Sync CLI 扩展
3. 点击"移除"

## 常见问题

### 扩展显示"无法连接到 Native Host"

1. 确认桌面应用已安装
2. 检查扩展 ID 是否正确配置
3. 重启浏览器

### 收藏夹没有同步

1. 检查 `bookmarks.json` 是否存在
2. 查看浏览器扩展是否有错误信息
3. 重启浏览器和桌面应用

### 桌面应用打不开收藏夹

确保系统已设置默认浏览器。