# 介绍

Browser Sync CLI 是一个浏览器扩展 + 本地应用的组合工具，实现实时收藏夹同步。

## 它是什么？

Browser Sync CLI 由三个主要组件组成：

1. **浏览器扩展** - Chrome/Edge Manifest V3 扩展，监听收藏夹变化
2. **Native Host** - Rust 应用，通过 Native Messaging 接收浏览器消息
3. **Tauri 应用** - Vue 3 + Tauri 桌面应用，用于查看/搜索收藏夹

## 为什么需要它？

- **备份收藏夹** - 自动保存收藏夹到本地 JSON 文件
- **跨浏览器管理** - 同时管理 Chrome 和 Edge 的收藏夹
- **快速搜索** - 使用桌面应用快速搜索和打开收藏夹
- **隐私保护** - 所有数据存储在本地，不上传到云端

## 工作原理

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

## 下一步

- [安装指南](./installation.md) - 了解如何安装和配置
- [使用方法](./usage.md) - 了解如何使用各项功能
- [开发指南](./development.md) - 了解如何参与开发