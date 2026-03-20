<template><div><h1 id="介绍" tabindex="-1"><a class="header-anchor" href="#介绍"><span>介绍</span></a></h1>
<p>Browser Sync CLI 是一个浏览器扩展 + 本地应用的组合工具，实现实时收藏夹同步。</p>
<h2 id="它是什么" tabindex="-1"><a class="header-anchor" href="#它是什么"><span>它是什么？</span></a></h2>
<p>Browser Sync CLI 由三个主要组件组成：</p>
<ol>
<li><strong>浏览器扩展</strong> - Chrome/Edge Manifest V3 扩展，监听收藏夹变化</li>
<li><strong>Native Host</strong> - Rust 应用，通过 Native Messaging 接收浏览器消息</li>
<li><strong>Tauri 应用</strong> - Vue 3 + Tauri 桌面应用，用于查看/搜索收藏夹</li>
</ol>
<h2 id="为什么需要它" tabindex="-1"><a class="header-anchor" href="#为什么需要它"><span>为什么需要它？</span></a></h2>
<ul>
<li><strong>备份收藏夹</strong> - 自动保存收藏夹到本地 JSON 文件</li>
<li><strong>跨浏览器管理</strong> - 同时管理 Chrome 和 Edge 的收藏夹</li>
<li><strong>快速搜索</strong> - 使用桌面应用快速搜索和打开收藏夹</li>
<li><strong>隐私保护</strong> - 所有数据存储在本地，不上传到云端</li>
</ul>
<h2 id="工作原理" tabindex="-1"><a class="header-anchor" href="#工作原理"><span>工作原理</span></a></h2>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">┌─────────────────┐     Native Messaging     ┌─────────────────┐</span>
<span class="line">│  浏览器扩展      │ ─────────────────────── │  Native Host    │</span>
<span class="line">│  (Extension)    │     (STDIO)              │  (Rust)         │</span>
<span class="line">└─────────────────┘                         └────────┬────────┘</span>
<span class="line">                                                     │ 写入 JSON</span>
<span class="line">                                                     ▼</span>
<span class="line">┌─────────────────┐                         ┌─────────────────┐</span>
<span class="line">│  Tauri 应用     │ ◄─── 文件监听 ────────  │  bookmarks.json │</span>
<span class="line">│  (Vue 3)        │                         └─────────────────┘</span>
<span class="line">└─────────────────┘</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p><strong>数据流向：</strong></p>
<ol>
<li>浏览器扩展监听收藏夹 CRUD 事件</li>
<li>扩展通过 <code v-pre>chrome.runtime.connectNative()</code> 发送完整收藏夹树到 Native Host</li>
<li>Native Host 写入 JSON 到 <code v-pre>%LOCALAPPDATA%\browser-sync-cli\bookmarks.json</code></li>
<li>Tauri 应用监听文件变化并更新 UI</li>
</ol>
<h2 id="下一步" tabindex="-1"><a class="header-anchor" href="#下一步"><span>下一步</span></a></h2>
<ul>
<li><RouteLink to="/guide/installation.html">安装指南</RouteLink> - 了解如何安装和配置</li>
<li><RouteLink to="/guide/usage.html">使用方法</RouteLink> - 了解如何使用各项功能</li>
<li><RouteLink to="/guide/development.html">开发指南</RouteLink> - 了解如何参与开发</li>
</ul>
</div></template>


