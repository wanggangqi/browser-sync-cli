<template><div><h1 id="安装指南" tabindex="-1"><a class="header-anchor" href="#安装指南"><span>安装指南</span></a></h1>
<p>本指南介绍如何安装 Browser Sync CLI。</p>
<h2 id="系统要求" tabindex="-1"><a class="header-anchor" href="#系统要求"><span>系统要求</span></a></h2>
<ul>
<li><strong>操作系统</strong>: Windows 10/11（64位）</li>
<li><strong>浏览器</strong>: Chrome 或 Edge（最新版本）</li>
<li><strong>磁盘空间</strong>: 约 50MB</li>
</ul>
<h2 id="下载" tabindex="-1"><a class="header-anchor" href="#下载"><span>下载</span></a></h2>
<p>从 <a href="https://github.com/wanggangqi/browser-sync-cli/releases" target="_blank" rel="noopener noreferrer">GitHub Releases<ExternalLinkIcon/></a> 下载最新版本：</p>
<table>
<thead>
<tr>
<th>文件</th>
<th>说明</th>
</tr>
</thead>
<tbody>
<tr>
<td><code v-pre>Browser-Sync-CLI_x64-setup.exe</code></td>
<td>Windows 安装程序（推荐）</td>
</tr>
<tr>
<td><code v-pre>extension.zip</code></td>
<td>浏览器扩展（Chrome/Edge）</td>
</tr>
</tbody>
</table>
<blockquote>
<p>注意：解压 <code v-pre>extension.zip</code> 后，扩展目录为 <code v-pre>dist</code> 文件夹。</p>
</blockquote>
<h2 id="安装桌面应用" tabindex="-1"><a class="header-anchor" href="#安装桌面应用"><span>安装桌面应用</span></a></h2>
<h3 id="方式一-安装程序-推荐" tabindex="-1"><a class="header-anchor" href="#方式一-安装程序-推荐"><span>方式一：安装程序（推荐）</span></a></h3>
<ol>
<li>下载 <code v-pre>Browser-Sync-CLI_x64-setup.exe</code></li>
<li>双击运行</li>
<li>按照向导完成安装</li>
<li>安装完成后会自动启动应用</li>
</ol>
<p>安装程序会自动完成以下配置：</p>
<ul>
<li>注册 Native Messaging Host</li>
<li>创建桌面快捷方式</li>
<li>创建开始菜单项</li>
</ul>
<h2 id="安装浏览器扩展" tabindex="-1"><a class="header-anchor" href="#安装浏览器扩展"><span>安装浏览器扩展</span></a></h2>
<h3 id="chrome-浏览器" tabindex="-1"><a class="header-anchor" href="#chrome-浏览器"><span>Chrome 浏览器</span></a></h3>
<ol>
<li>
<p>解压 <code v-pre>extension.zip</code> 到一个固定目录</p>
<blockquote>
<p>注意：解压后会有一个 <code v-pre>dist</code> 文件夹，这是扩展的实际目录</p>
</blockquote>
</li>
<li>
<p>打开 Chrome，访问 <code v-pre>chrome://extensions/</code></p>
</li>
<li>
<p>打开右上角的 <strong>开发者模式</strong> 开关</p>
</li>
<li>
<p>点击 <strong>加载已解压的扩展程序</strong></p>
</li>
<li>
<p>选择解压后的 <code v-pre>dist</code> 目录</p>
</li>
<li>
<p>安装成功后，记录显示的扩展 ID（类似 <code v-pre>abcdefghijklmnopqrstuvwxyz123456</code>）</p>
</li>
</ol>
<h3 id="edge-浏览器" tabindex="-1"><a class="header-anchor" href="#edge-浏览器"><span>Edge 浏览器</span></a></h3>
<ol>
<li>
<p>解压 <code v-pre>extension.zip</code> 到一个固定目录</p>
</li>
<li>
<p>打开 Edge，访问 <code v-pre>edge://extensions/</code></p>
</li>
<li>
<p>打开左侧的 <strong>开发人员模式</strong> 开关</p>
</li>
<li>
<p>点击 <strong>加载解压缩的扩展</strong></p>
</li>
<li>
<p>选择解压后的 <code v-pre>dist</code> 目录</p>
</li>
<li>
<p>安装成功后，记录显示的扩展 ID</p>
</li>
</ol>
<h2 id="配置扩展-id" tabindex="-1"><a class="header-anchor" href="#配置扩展-id"><span>配置扩展 ID</span></a></h2>
<p>为了让浏览器扩展与桌面应用通信，需要配置扩展 ID：</p>
<h3 id="使用桌面应用配置-推荐" tabindex="-1"><a class="header-anchor" href="#使用桌面应用配置-推荐"><span>使用桌面应用配置（推荐）</span></a></h3>
<ol>
<li>启动 Browser Sync CLI 桌面应用</li>
<li>点击右上角 ⚙️ 进入设置</li>
<li>在&quot;扩展 ID&quot;区域填写：
<ul>
<li>Chrome 扩展 ID</li>
<li>Edge 扩展 ID（如使用 Edge）</li>
</ul>
</li>
<li>点击&quot;保存&quot;</li>
<li><strong>重启浏览器</strong></li>
</ol>
<h3 id="手动配置" tabindex="-1"><a class="header-anchor" href="#手动配置"><span>手动配置</span></a></h3>
<ol>
<li>
<p>打开文件：</p>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">%LOCALAPPDATA%\browser-sync-cli\com.browser_sync.cli.json</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div></li>
<li>
<p>修改 <code v-pre>allowed_origins</code> 数组：</p>
<div class="language-json line-numbers-mode" data-highlighter="prismjs" data-ext="json"><pre v-pre><code class="language-json"><span class="line"><span class="token punctuation">{</span></span>
<span class="line">  <span class="token property">"name"</span><span class="token operator">:</span> <span class="token string">"com.browser_sync.cli"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"description"</span><span class="token operator">:</span> <span class="token string">"Browser Sync CLI Native Messaging Host"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"path"</span><span class="token operator">:</span> <span class="token string">"C:\\path\\to\\browser-sync-native-host.exe"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"type"</span><span class="token operator">:</span> <span class="token string">"stdio"</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"allowed_origins"</span><span class="token operator">:</span> <span class="token punctuation">[</span></span>
<span class="line">    <span class="token string">"chrome-extension://你的Chrome扩展ID/"</span><span class="token punctuation">,</span></span>
<span class="line">    <span class="token string">"chrome-extension://你的Edge扩展ID/"</span></span>
<span class="line">  <span class="token punctuation">]</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></li>
<li>
<p>保存文件</p>
</li>
<li>
<p><strong>重启浏览器</strong></p>
</li>
</ol>
<h2 id="验证安装" tabindex="-1"><a class="header-anchor" href="#验证安装"><span>验证安装</span></a></h2>
<h3 id="检查扩展连接状态" tabindex="-1"><a class="header-anchor" href="#检查扩展连接状态"><span>检查扩展连接状态</span></a></h3>
<ol>
<li>打开浏览器扩展管理页面</li>
<li>找到 Browser Sync CLI 扩展</li>
<li>点击&quot;Service Worker&quot;查看控制台</li>
<li>如果显示 &quot;Connected to native host&quot;，说明连接成功</li>
</ol>
<h3 id="测试同步功能" tabindex="-1"><a class="header-anchor" href="#测试同步功能"><span>测试同步功能</span></a></h3>
<ol>
<li>在浏览器中添加一个新收藏夹</li>
<li>打开桌面应用，查看是否显示新收藏夹</li>
<li>或检查文件 <code v-pre>%LOCALAPPDATA%\browser-sync-cli\bookmarks.json</code> 是否更新</li>
</ol>
<h2 id="下一步" tabindex="-1"><a class="header-anchor" href="#下一步"><span>下一步</span></a></h2>
<ul>
<li><RouteLink to="/guide/usage.html">使用指南</RouteLink> - 了解如何使用各项功能</li>
<li><RouteLink to="/guide/troubleshooting.html">故障排查</RouteLink> - 解决常见问题</li>
</ul>
</div></template>


