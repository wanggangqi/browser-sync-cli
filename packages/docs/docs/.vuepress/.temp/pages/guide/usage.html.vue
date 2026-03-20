<template><div><h1 id="使用指南" tabindex="-1"><a class="header-anchor" href="#使用指南"><span>使用指南</span></a></h1>
<p>本指南面向最终用户，介绍如何安装和使用 Browser Sync CLI。</p>
<h2 id="发布物" tabindex="-1"><a class="header-anchor" href="#发布物"><span>发布物</span></a></h2>
<p>Browser Sync CLI 发布以下安装包：</p>
<table>
<thead>
<tr>
<th>产物</th>
<th>文件名</th>
<th>说明</th>
</tr>
</thead>
<tbody>
<tr>
<td>桌面应用 (Windows)</td>
<td><code v-pre>Browser-Sync-CLI_x64-setup.exe</code></td>
<td>Windows 安装程序</td>
</tr>
<tr>
<td>桌面应用 (便携版)</td>
<td><code v-pre>Browser-Sync-CLI_x64.zip</code></td>
<td>免安装便携版</td>
</tr>
<tr>
<td>浏览器扩展</td>
<td><code v-pre>extension.zip</code></td>
<td>Chrome/Edge 扩展包</td>
</tr>
</tbody>
</table>
<h2 id="安装步骤" tabindex="-1"><a class="header-anchor" href="#安装步骤"><span>安装步骤</span></a></h2>
<h3 id="_1-安装桌面应用" tabindex="-1"><a class="header-anchor" href="#_1-安装桌面应用"><span>1. 安装桌面应用</span></a></h3>
<h4 id="方式一-安装程序" tabindex="-1"><a class="header-anchor" href="#方式一-安装程序"><span>方式一：安装程序</span></a></h4>
<ol>
<li>下载 <code v-pre>Browser-Sync-CLI_x64-setup.exe</code></li>
<li>双击运行安装程序</li>
<li>选择安装路径，完成安装</li>
<li>安装程序会自动：
<ul>
<li>复制 Native Host 程序</li>
<li>注册 Native Messaging Host</li>
<li>创建桌面快捷方式</li>
</ul>
</li>
</ol>
<h4 id="方式二-便携版" tabindex="-1"><a class="header-anchor" href="#方式二-便携版"><span>方式二：便携版</span></a></h4>
<ol>
<li>下载 <code v-pre>Browser-Sync-CLI_x64.zip</code></li>
<li>解压到任意目录</li>
<li>运行 <code v-pre>install.ps1</code> 注册 Native Host：<div class="language-powershell line-numbers-mode" data-highlighter="prismjs" data-ext="powershell"><pre v-pre><code class="language-powershell"><span class="line">powershell <span class="token operator">-</span>ExecutionPolicy Bypass <span class="token operator">-</span>File <span class="token punctuation">.</span>\install<span class="token punctuation">.</span>ps1</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div></li>
<li>运行 <code v-pre>Browser-Sync-CLI.exe</code> 启动应用</li>
</ol>
<h3 id="_2-安装浏览器扩展" tabindex="-1"><a class="header-anchor" href="#_2-安装浏览器扩展"><span>2. 安装浏览器扩展</span></a></h3>
<h4 id="chrome-浏览器" tabindex="-1"><a class="header-anchor" href="#chrome-浏览器"><span>Chrome 浏览器</span></a></h4>
<ol>
<li>解压 <code v-pre>extension.zip</code> 到一个目录</li>
<li>打开 <code v-pre>chrome://extensions/</code></li>
<li>启用右上角的 <strong>开发者模式</strong></li>
<li>点击 <strong>加载已解压的扩展程序</strong></li>
<li>选择解压的扩展目录</li>
<li>记下显示的扩展 ID</li>
</ol>
<h4 id="edge-浏览器" tabindex="-1"><a class="header-anchor" href="#edge-浏览器"><span>Edge 浏览器</span></a></h4>
<ol>
<li>解压 <code v-pre>extension.zip</code> 到一个目录</li>
<li>打开 <code v-pre>edge://extensions/</code></li>
<li>启用左侧的 <strong>开发人员模式</strong></li>
<li>点击 <strong>加载解压缩的扩展</strong></li>
<li>选择解压的扩展目录</li>
<li>记下显示的扩展 ID</li>
</ol>
<h3 id="_3-配置扩展-id" tabindex="-1"><a class="header-anchor" href="#_3-配置扩展-id"><span>3. 配置扩展 ID</span></a></h3>
<p>安装扩展后，需要配置 Native Messaging 权限：</p>
<ol>
<li>打开桌面应用</li>
<li>进入 <strong>设置</strong> 页面</li>
<li>输入扩展 ID（Chrome 和 Edge 分别填写）</li>
<li>点击 <strong>保存配置</strong></li>
<li><strong>重启浏览器</strong></li>
</ol>
<div class="custom-container tip"><p class="custom-container-title">提示</p>
<p>如果使用安装程序，此步骤会自动完成。便携版需要手动配置。</p>
</div>
<h2 id="使用方法" tabindex="-1"><a class="header-anchor" href="#使用方法"><span>使用方法</span></a></h2>
<h3 id="自动同步" tabindex="-1"><a class="header-anchor" href="#自动同步"><span>自动同步</span></a></h3>
<p>安装完成后，收藏夹会自动同步：</p>
<ul>
<li>在浏览器中添加、删除、修改收藏夹</li>
<li>所有变更自动保存到本地文件</li>
<li>无需手动操作</li>
</ul>
<h3 id="桌面应用功能" tabindex="-1"><a class="header-anchor" href="#桌面应用功能"><span>桌面应用功能</span></a></h3>
<h4 id="搜索收藏夹" tabindex="-1"><a class="header-anchor" href="#搜索收藏夹"><span>搜索收藏夹</span></a></h4>
<ol>
<li>启动 Browser Sync CLI 应用</li>
<li>在搜索框输入关键词</li>
<li>支持按标题和 URL 搜索</li>
<li>结果实时显示</li>
</ol>
<h4 id="打开收藏夹" tabindex="-1"><a class="header-anchor" href="#打开收藏夹"><span>打开收藏夹</span></a></h4>
<ul>
<li>双击搜索结果，在默认浏览器中打开</li>
<li>或右键选择&quot;在新标签页打开&quot;</li>
</ul>
<h4 id="导出-导入" tabindex="-1"><a class="header-anchor" href="#导出-导入"><span>导出/导入</span></a></h4>
<ul>
<li><strong>导出</strong>：将收藏夹导出为 JSON 文件备份</li>
<li><strong>导入</strong>：从 JSON 文件恢复收藏夹</li>
</ul>
<div class="custom-container warning"><p class="custom-container-title">注意</p>
<p>导入会覆盖当前数据，请先备份。</p>
</div>
<h4 id="切换浏览器" tabindex="-1"><a class="header-anchor" href="#切换浏览器"><span>切换浏览器</span></a></h4>
<p>如果同时使用 Chrome 和 Edge：</p>
<ol>
<li>点击左上角下拉菜单</li>
<li>选择要查看的浏览器</li>
<li>切换收藏夹数据源</li>
</ol>
<h2 id="数据存储" tabindex="-1"><a class="header-anchor" href="#数据存储"><span>数据存储</span></a></h2>
<h3 id="文件位置" tabindex="-1"><a class="header-anchor" href="#文件位置"><span>文件位置</span></a></h3>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">%LOCALAPPDATA%\browser-sync-cli\</span>
<span class="line">├── bookmarks.json          # 收藏夹数据</span>
<span class="line">├── com.browser_sync.cli.json  # Native Messaging 配置</span>
<span class="line">└── browser-sync-native-host.exe  # Native Host 程序</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h3 id="数据格式" tabindex="-1"><a class="header-anchor" href="#数据格式"><span>数据格式</span></a></h3>
<p><code v-pre>bookmarks.json</code> 结构：</p>
<div class="language-json line-numbers-mode" data-highlighter="prismjs" data-ext="json"><pre v-pre><code class="language-json"><span class="line"><span class="token punctuation">{</span></span>
<span class="line">  <span class="token property">"chrome"</span><span class="token operator">:</span> <span class="token punctuation">{</span></span>
<span class="line">    <span class="token property">"bookmarks"</span><span class="token operator">:</span> <span class="token punctuation">[</span>...<span class="token punctuation">]</span><span class="token punctuation">,</span></span>
<span class="line">    <span class="token property">"lastSync"</span><span class="token operator">:</span> <span class="token string">"2024-01-01T00:00:00Z"</span></span>
<span class="line">  <span class="token punctuation">}</span><span class="token punctuation">,</span></span>
<span class="line">  <span class="token property">"edge"</span><span class="token operator">:</span> <span class="token punctuation">{</span></span>
<span class="line">    <span class="token property">"bookmarks"</span><span class="token operator">:</span> <span class="token punctuation">[</span>...<span class="token punctuation">]</span><span class="token punctuation">,</span></span>
<span class="line">    <span class="token property">"lastSync"</span><span class="token operator">:</span> <span class="token string">"2024-01-01T00:00:00Z"</span></span>
<span class="line">  <span class="token punctuation">}</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="卸载" tabindex="-1"><a class="header-anchor" href="#卸载"><span>卸载</span></a></h2>
<h3 id="卸载桌面应用" tabindex="-1"><a class="header-anchor" href="#卸载桌面应用"><span>卸载桌面应用</span></a></h3>
<p><strong>安装版：</strong></p>
<ul>
<li>在 Windows 设置中卸载&quot;Browser Sync CLI&quot;</li>
</ul>
<p><strong>便携版：</strong></p>
<ul>
<li>删除解压目录</li>
<li>运行 <code v-pre>uninstall.ps1</code> 清理注册</li>
</ul>
<h3 id="卸载浏览器扩展" tabindex="-1"><a class="header-anchor" href="#卸载浏览器扩展"><span>卸载浏览器扩展</span></a></h3>
<ol>
<li>打开浏览器扩展管理页面</li>
<li>找到 Browser Sync CLI 扩展</li>
<li>点击&quot;移除&quot;</li>
</ol>
<h2 id="常见问题" tabindex="-1"><a class="header-anchor" href="#常见问题"><span>常见问题</span></a></h2>
<h3 id="扩展显示-无法连接到-native-host" tabindex="-1"><a class="header-anchor" href="#扩展显示-无法连接到-native-host"><span>扩展显示&quot;无法连接到 Native Host&quot;</span></a></h3>
<ol>
<li>确认桌面应用已安装</li>
<li>检查扩展 ID 是否正确配置</li>
<li>重启浏览器</li>
</ol>
<h3 id="收藏夹没有同步" tabindex="-1"><a class="header-anchor" href="#收藏夹没有同步"><span>收藏夹没有同步</span></a></h3>
<ol>
<li>检查 <code v-pre>bookmarks.json</code> 是否存在</li>
<li>查看浏览器扩展是否有错误信息</li>
<li>重启浏览器和桌面应用</li>
</ol>
<h3 id="桌面应用打不开收藏夹" tabindex="-1"><a class="header-anchor" href="#桌面应用打不开收藏夹"><span>桌面应用打不开收藏夹</span></a></h3>
<p>确保系统已设置默认浏览器。</p>
</div></template>


