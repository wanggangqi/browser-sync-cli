<template><div><h1 id="开发指南" tabindex="-1"><a class="header-anchor" href="#开发指南"><span>开发指南</span></a></h1>
<p>本文档面向想要参与 Browser Sync CLI 开发的开发者。</p>
<h2 id="项目结构" tabindex="-1"><a class="header-anchor" href="#项目结构"><span>项目结构</span></a></h2>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">browser-sync-cli/</span>
<span class="line">├── packages/</span>
<span class="line">│   ├── extension/           # 浏览器扩展 (Manifest V3)</span>
<span class="line">│   │   ├── src/</span>
<span class="line">│   │   │   ├── background/</span>
<span class="line">│   │   │   │   └── index.ts    # Service Worker 入口</span>
<span class="line">│   │   │   ├── bookmark-sync.ts    # 收藏夹事件监听</span>
<span class="line">│   │   │   └── native-messaging.ts # Native Messaging 连接</span>
<span class="line">│   │   ├── manifest.json</span>
<span class="line">│   │   └── package.json</span>
<span class="line">│   │</span>
<span class="line">│   ├── native-host/         # Rust Native Messaging Host</span>
<span class="line">│   │   ├── src/</span>
<span class="line">│   │   │   └── main.rs          # STDIO 消息处理</span>
<span class="line">│   │   ├── install.ps1          # Windows 注册脚本</span>
<span class="line">│   │   ├── Cargo.toml</span>
<span class="line">│   │   └── package.json</span>
<span class="line">│   │</span>
<span class="line">│   ├── app/                 # Tauri + Vue 3 应用</span>
<span class="line">│   │   ├── src/                  # Vue 前端</span>
<span class="line">│   │   ├── src-tauri/</span>
<span class="line">│   │   │   ├── src/</span>
<span class="line">│   │   │   │   ├── main.rs      # 文件监听、Tauri 设置</span>
<span class="line">│   │   │   │   └── commands/</span>
<span class="line">│   │   │   │       └── bookmarks.rs  # Tauri 命令</span>
<span class="line">│   │   │   └── tauri.conf.json</span>
<span class="line">│   │   └── package.json</span>
<span class="line">│   │</span>
<span class="line">│   └── docs/                # VuePress 文档</span>
<span class="line">│       └── docs/</span>
<span class="line">│</span>
<span class="line">├── CLAUDE.md                # 项目说明</span>
<span class="line">├── package.json             # Monorepo 根配置</span>
<span class="line">└── pnpm-workspace.yaml</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="技术栈" tabindex="-1"><a class="header-anchor" href="#技术栈"><span>技术栈</span></a></h2>
<table>
<thead>
<tr>
<th>组件</th>
<th>技术</th>
</tr>
</thead>
<tbody>
<tr>
<td>浏览器扩展</td>
<td>TypeScript, Manifest V3</td>
</tr>
<tr>
<td>Native Host</td>
<td>Rust, serde, chrono</td>
</tr>
<tr>
<td>桌面应用</td>
<td>Vue 3, Tauri, TypeScript</td>
</tr>
<tr>
<td>构建工具</td>
<td>esbuild, Vite, Cargo</td>
</tr>
<tr>
<td>包管理</td>
<td>pnpm (Monorepo)</td>
</tr>
</tbody>
</table>
<h2 id="开发环境设置" tabindex="-1"><a class="header-anchor" href="#开发环境设置"><span>开发环境设置</span></a></h2>
<h3 id="前置要求" tabindex="-1"><a class="header-anchor" href="#前置要求"><span>前置要求</span></a></h3>
<ol>
<li><strong>Node.js</strong> &gt;= 18</li>
<li><strong>pnpm</strong> &gt;= 8</li>
<li><strong>Rust</strong> &gt;= 1.70</li>
<li><strong>Visual Studio Build Tools</strong> (Windows)</li>
</ol>
<h3 id="安装-rust" tabindex="-1"><a class="header-anchor" href="#安装-rust"><span>安装 Rust</span></a></h3>
<div class="language-powershell line-numbers-mode" data-highlighter="prismjs" data-ext="powershell"><pre v-pre><code class="language-powershell"><span class="line"><span class="token comment"># 使用 rustup 安装</span></span>
<span class="line">winget install Rustlang<span class="token punctuation">.</span>Rustup</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 或访问 https://rustup.rs/</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h3 id="安装-visual-studio-build-tools" tabindex="-1"><a class="header-anchor" href="#安装-visual-studio-build-tools"><span>安装 Visual Studio Build Tools</span></a></h3>
<div class="language-powershell line-numbers-mode" data-highlighter="prismjs" data-ext="powershell"><pre v-pre><code class="language-powershell"><span class="line">winget install Microsoft<span class="token punctuation">.</span>VisualStudio<span class="token punctuation">.</span>2022<span class="token punctuation">.</span>BuildTools <span class="token operator">--</span>override <span class="token string">"--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h3 id="安装-wix-工具-windows-安装包构建" tabindex="-1"><a class="header-anchor" href="#安装-wix-工具-windows-安装包构建"><span>安装 WiX 工具（Windows 安装包构建）</span></a></h3>
<p>WiX 是 Tauri 在 Windows 上生成 MSI 安装包所必需的工具。</p>
<p><strong>方法一：通过 winget 安装（推荐）</strong></p>
<p>需要以管理员身份运行 PowerShell：</p>
<div class="language-powershell line-numbers-mode" data-highlighter="prismjs" data-ext="powershell"><pre v-pre><code class="language-powershell"><span class="line">winget install WiXToolset<span class="token punctuation">.</span>WiXToolset</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p><strong>方法二：手动下载安装</strong></p>
<ol>
<li>访问 <a href="https://github.com/wixtoolset/wix3/releases/tag/wix3141rtm" target="_blank" rel="noopener noreferrer">WiX releases 页面<ExternalLinkIcon/></a></li>
<li>下载 <code v-pre>wix314.exe</code> 并运行安装</li>
</ol>
<p><strong>添加环境变量</strong></p>
<p>winget 安装 WiX 后可能不会自动添加到 PATH，需要手动配置：</p>
<p><strong>方法一：命令行添加（推荐）</strong></p>
<p>在 PowerShell 中执行：</p>
<div class="language-powershell line-numbers-mode" data-highlighter="prismjs" data-ext="powershell"><pre v-pre><code class="language-powershell"><span class="line"><span class="token namespace">[Environment]</span>::SetEnvironmentVariable<span class="token punctuation">(</span><span class="token string">"Path"</span><span class="token punctuation">,</span> <span class="token variable">$env</span>:Path <span class="token operator">+</span> <span class="token string">";C:\Program Files (x86)\WiX Toolset v3.14\bin"</span><span class="token punctuation">,</span> <span class="token string">"User"</span><span class="token punctuation">)</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p><strong>方法二：图形界面添加</strong></p>
<ol>
<li>按 <code v-pre>Win + R</code>，输入 <code v-pre>sysdm.cpl</code>，回车</li>
<li>点击&quot;高级&quot; → &quot;环境变量&quot;</li>
<li>在&quot;用户变量&quot;中找到 <code v-pre>Path</code>，点击&quot;编辑&quot;</li>
<li>点击&quot;新建&quot;，添加路径：<code v-pre>C:\Program Files (x86)\WiX Toolset v3.14\bin</code></li>
<li>点击&quot;确定&quot;保存</li>
</ol>
<div class="custom-container warning"><p class="custom-container-title">注意</p>
<p>添加环境变量后，需要<strong>重新打开终端</strong>才能生效。</p>
</div>
<p><strong>验证安装</strong></p>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh"><pre v-pre><code class="language-bash"><span class="line"><span class="token comment"># 检查 WiX 是否正确安装</span></span>
<span class="line">light -?</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 或检查 candle（WiX 编译器）</span></span>
<span class="line">candle -?</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h3 id="安装-nsis-工具-nsis-安装包构建" tabindex="-1"><a class="header-anchor" href="#安装-nsis-工具-nsis-安装包构建"><span>安装 NSIS 工具（NSIS 安装包构建）</span></a></h3>
<p>NSIS 用于生成 Windows 安装程序（.exe 安装包）。构建时 Tauri 会自动从 GitHub 下载，如果网络不通可手动配置。</p>
<p><strong>下载地址</strong></p>
<ol>
<li>NSIS 主体：https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip</li>
<li>Tauri NSIS 工具库：https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.4.1/nsis_tauri_utils.dll</li>
</ol>
<p><strong>手动安装步骤</strong></p>
<ol>
<li>下载上述两个文件</li>
<li>解压 <code v-pre>nsis-3.zip</code></li>
<li>将解压后的内容和 <code v-pre>nsis_tauri_utils.dll</code> 放到缓存目录：</li>
</ol>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">C:\Users\&lt;用户名>\.cache\tauri\NSIS\</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><p>最终目录结构应如下：</p>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">NSIS/</span>
<span class="line">├── makensis.exe</span>
<span class="line">├── Plugins/</span>
<span class="line">│   └── x86-unicode/</span>
<span class="line">│       └── nsis_tauri_utils.dll</span>
<span class="line">├── Include/</span>
<span class="line">├── Stubs/</span>
<span class="line">└── ...其他文件</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><div class="custom-container tip"><p class="custom-container-title">提示</p>
<p><code v-pre>nsis_tauri_utils.dll</code> 需要同时放在 <code v-pre>NSIS/</code> 目录和 <code v-pre>NSIS/Plugins/x86-unicode/</code> 目录下。</p>
</div>
<h3 id="安装项目依赖" tabindex="-1"><a class="header-anchor" href="#安装项目依赖"><span>安装项目依赖</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh"><pre v-pre><code class="language-bash"><span class="line"><span class="token function">pnpm</span> <span class="token function">install</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div></div></div><h2 id="开发工作流" tabindex="-1"><a class="header-anchor" href="#开发工作流"><span>开发工作流</span></a></h2>
<h3 id="开发浏览器扩展" tabindex="-1"><a class="header-anchor" href="#开发浏览器扩展"><span>开发浏览器扩展</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh"><pre v-pre><code class="language-bash"><span class="line"><span class="token comment"># 进入扩展目录</span></span>
<span class="line"><span class="token builtin class-name">cd</span> packages/extension</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 开发模式（自动重新编译）</span></span>
<span class="line"><span class="token function">pnpm</span> dev</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 构建</span></span>
<span class="line"><span class="token function">pnpm</span> build</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p>修改代码后，在浏览器扩展管理页面点击刷新按钮加载新版本。</p>
<h3 id="开发-native-host" tabindex="-1"><a class="header-anchor" href="#开发-native-host"><span>开发 Native Host</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh"><pre v-pre><code class="language-bash"><span class="line"><span class="token comment"># 进入 Native Host 目录</span></span>
<span class="line"><span class="token builtin class-name">cd</span> packages/native-host</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 开发构建</span></span>
<span class="line"><span class="token function">cargo</span> build</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 发布构建</span></span>
<span class="line"><span class="token function">cargo</span> build <span class="token parameter variable">--release</span></span>
<span class="line"></span>
<span class="line"><span class="token comment"># 运行测试</span></span>
<span class="line"><span class="token function">cargo</span> <span class="token builtin class-name">test</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p>修改代码后需要重新编译并重启浏览器。</p>
<h3 id="开发桌面应用" tabindex="-1"><a class="header-anchor" href="#开发桌面应用"><span>开发桌面应用</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh"><pre v-pre><code class="language-bash"><span class="line"><span class="token comment"># 进入应用目录</span></span>
<span class="line"><span class="token builtin class-name">cd</span> packages/app</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 开发模式（热重载）</span></span>
<span class="line"><span class="token function">pnpm</span> tauri dev</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 构建</span></span>
<span class="line"><span class="token function">pnpm</span> build</span>
<span class="line"><span class="token function">pnpm</span> tauri build</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h3 id="开发文档" tabindex="-1"><a class="header-anchor" href="#开发文档"><span>开发文档</span></a></h3>
<div class="language-bash line-numbers-mode" data-highlighter="prismjs" data-ext="sh"><pre v-pre><code class="language-bash"><span class="line"><span class="token comment"># 进入文档目录</span></span>
<span class="line"><span class="token builtin class-name">cd</span> packages/docs</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 开发模式</span></span>
<span class="line"><span class="token function">pnpm</span> dev</span>
<span class="line"></span>
<span class="line"><span class="token comment"># 构建</span></span>
<span class="line"><span class="token function">pnpm</span> build</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="关键实现细节" tabindex="-1"><a class="header-anchor" href="#关键实现细节"><span>关键实现细节</span></a></h2>
<h3 id="native-messaging-协议" tabindex="-1"><a class="header-anchor" href="#native-messaging-协议"><span>Native Messaging 协议</span></a></h3>
<p>Native Messaging 使用 STDIO 通信，消息格式：</p>
<ul>
<li><strong>消息结构</strong>: 4 字节小端序长度前缀 + UTF-8 JSON 内容</li>
<li><strong>双向通信</strong>: 浏览器→host 和 host→浏览器使用相同格式</li>
</ul>
<div class="language-rust line-numbers-mode" data-highlighter="prismjs" data-ext="rs"><pre v-pre><code class="language-rust"><span class="line"><span class="token comment">// 读取消息</span></span>
<span class="line"><span class="token keyword">fn</span> <span class="token function-definition function">read_message</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">-></span> <span class="token class-name">Option</span><span class="token operator">&lt;</span><span class="token class-name">IncomingMessage</span><span class="token operator">></span> <span class="token punctuation">{</span></span>
<span class="line">    <span class="token comment">// 读取消息长度 (4 字节, 小端序)</span></span>
<span class="line">    <span class="token keyword">let</span> <span class="token keyword">mut</span> length_bytes <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token number">0u8</span><span class="token punctuation">;</span> <span class="token number">4</span><span class="token punctuation">]</span><span class="token punctuation">;</span></span>
<span class="line">    <span class="token namespace">io<span class="token punctuation">::</span></span><span class="token function">stdin</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">read_exact</span><span class="token punctuation">(</span><span class="token operator">&amp;</span><span class="token keyword">mut</span> length_bytes<span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">ok</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token operator">?</span><span class="token punctuation">;</span></span>
<span class="line">    <span class="token keyword">let</span> length <span class="token operator">=</span> <span class="token keyword">u32</span><span class="token punctuation">::</span><span class="token function">from_le_bytes</span><span class="token punctuation">(</span>length_bytes<span class="token punctuation">)</span> <span class="token keyword">as</span> <span class="token keyword">usize</span><span class="token punctuation">;</span></span>
<span class="line"></span>
<span class="line">    <span class="token comment">// 读取消息内容</span></span>
<span class="line">    <span class="token keyword">let</span> <span class="token keyword">mut</span> message_bytes <span class="token operator">=</span> <span class="token macro property">vec!</span><span class="token punctuation">[</span><span class="token number">0u8</span><span class="token punctuation">;</span> length<span class="token punctuation">]</span><span class="token punctuation">;</span></span>
<span class="line">    <span class="token namespace">io<span class="token punctuation">::</span></span><span class="token function">stdin</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">read_exact</span><span class="token punctuation">(</span><span class="token operator">&amp;</span><span class="token keyword">mut</span> message_bytes<span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">ok</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token operator">?</span><span class="token punctuation">;</span></span>
<span class="line"></span>
<span class="line">    <span class="token comment">// 解析 JSON</span></span>
<span class="line">    <span class="token namespace">serde_json<span class="token punctuation">::</span></span><span class="token function">from_slice</span><span class="token punctuation">(</span><span class="token operator">&amp;</span>message_bytes<span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">ok</span><span class="token punctuation">(</span><span class="token punctuation">)</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span>
<span class="line"><span class="token comment">// 发送响应</span></span>
<span class="line"><span class="token keyword">fn</span> <span class="token function-definition function">send_response</span><span class="token punctuation">(</span>response<span class="token punctuation">:</span> <span class="token operator">&amp;</span><span class="token class-name">Response</span><span class="token punctuation">)</span> <span class="token punctuation">{</span></span>
<span class="line">    <span class="token keyword">let</span> json <span class="token operator">=</span> <span class="token namespace">serde_json<span class="token punctuation">::</span></span><span class="token function">to_string</span><span class="token punctuation">(</span>response<span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">unwrap</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line">    <span class="token keyword">let</span> length <span class="token operator">=</span> json<span class="token punctuation">.</span><span class="token function">len</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token keyword">as</span> <span class="token keyword">u32</span><span class="token punctuation">;</span></span>
<span class="line"></span>
<span class="line">    <span class="token keyword">let</span> stdout <span class="token operator">=</span> <span class="token namespace">io<span class="token punctuation">::</span></span><span class="token function">stdout</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line">    <span class="token keyword">let</span> <span class="token keyword">mut</span> handle <span class="token operator">=</span> stdout<span class="token punctuation">.</span><span class="token function">lock</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line">    handle<span class="token punctuation">.</span><span class="token function">write_all</span><span class="token punctuation">(</span><span class="token operator">&amp;</span>length<span class="token punctuation">.</span><span class="token function">to_le_bytes</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">ok</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line">    handle<span class="token punctuation">.</span><span class="token function">write_all</span><span class="token punctuation">(</span>json<span class="token punctuation">.</span><span class="token function">as_bytes</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">ok</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line">    handle<span class="token punctuation">.</span><span class="token function">flush</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">.</span><span class="token function">ok</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h3 id="json-字段命名约定" tabindex="-1"><a class="header-anchor" href="#json-字段命名约定"><span>JSON 字段命名约定</span></a></h3>
<p>所有与 JavaScript 交互的 Rust 结构体必须使用 <code v-pre>#[serde(rename_all = &quot;camelCase&quot;)]</code>：</p>
<div class="language-rust line-numbers-mode" data-highlighter="prismjs" data-ext="rs"><pre v-pre><code class="language-rust"><span class="line"><span class="token attribute attr-name">#[derive(Debug, Serialize, Deserialize)]</span></span>
<span class="line"><span class="token attribute attr-name">#[serde(rename_all = <span class="token string">"camelCase"</span>)]</span></span>
<span class="line"><span class="token keyword">struct</span> <span class="token type-definition class-name">BookmarkNode</span> <span class="token punctuation">{</span></span>
<span class="line">    id<span class="token punctuation">:</span> <span class="token class-name">String</span><span class="token punctuation">,</span></span>
<span class="line">    title<span class="token punctuation">:</span> <span class="token class-name">String</span><span class="token punctuation">,</span></span>
<span class="line">    url<span class="token punctuation">:</span> <span class="token class-name">Option</span><span class="token operator">&lt;</span><span class="token class-name">String</span><span class="token operator">></span><span class="token punctuation">,</span></span>
<span class="line">    parent_id<span class="token punctuation">:</span> <span class="token class-name">Option</span><span class="token operator">&lt;</span><span class="token class-name">String</span><span class="token operator">></span><span class="token punctuation">,</span>  <span class="token comment">// 序列化为 parentId</span></span>
<span class="line">    date_added<span class="token punctuation">:</span> <span class="token class-name">Option</span><span class="token operator">&lt;</span><span class="token keyword">i64</span><span class="token operator">></span><span class="token punctuation">,</span>    <span class="token comment">// 序列化为 dateAdded</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p><strong>原因</strong>: JavaScript 使用 camelCase，Rust 默认使用 snake_case。</p>
<h3 id="文件监听" tabindex="-1"><a class="header-anchor" href="#文件监听"><span>文件监听</span></a></h3>
<p>桌面应用使用 <code v-pre>notify</code> crate 监听文件变化：</p>
<div class="language-rust line-numbers-mode" data-highlighter="prismjs" data-ext="rs"><pre v-pre><code class="language-rust"><span class="line"><span class="token keyword">use</span> <span class="token namespace">notify<span class="token punctuation">::</span></span><span class="token punctuation">{</span><span class="token class-name">Watcher</span><span class="token punctuation">,</span> <span class="token class-name">RecursiveMode</span><span class="token punctuation">,</span> watcher<span class="token punctuation">}</span><span class="token punctuation">;</span></span>
<span class="line"><span class="token keyword">use</span> <span class="token namespace">std<span class="token punctuation">::</span>sync<span class="token punctuation">::</span>mpsc<span class="token punctuation">::</span></span>channel<span class="token punctuation">;</span></span>
<span class="line"></span>
<span class="line"><span class="token keyword">let</span> <span class="token punctuation">(</span>tx<span class="token punctuation">,</span> rx<span class="token punctuation">)</span> <span class="token operator">=</span> <span class="token function">channel</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span></span>
<span class="line"><span class="token keyword">let</span> <span class="token keyword">mut</span> watcher <span class="token operator">=</span> <span class="token function">watcher</span><span class="token punctuation">(</span>tx<span class="token punctuation">,</span> <span class="token class-name">Duration</span><span class="token punctuation">::</span><span class="token function">from_millis</span><span class="token punctuation">(</span><span class="token number">200</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token operator">?</span><span class="token punctuation">;</span></span>
<span class="line">watcher<span class="token punctuation">.</span><span class="token function">watch</span><span class="token punctuation">(</span>path<span class="token punctuation">,</span> <span class="token class-name">RecursiveMode</span><span class="token punctuation">::</span><span class="token class-name">NonRecursive</span><span class="token punctuation">)</span><span class="token operator">?</span><span class="token punctuation">;</span></span>
<span class="line"></span>
<span class="line"><span class="token comment">// 监听事件</span></span>
<span class="line"><span class="token keyword">loop</span> <span class="token punctuation">{</span></span>
<span class="line">    <span class="token keyword">match</span> rx<span class="token punctuation">.</span><span class="token function">recv</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span></span>
<span class="line">        <span class="token class-name">Ok</span><span class="token punctuation">(</span>event<span class="token punctuation">)</span> <span class="token operator">=></span> <span class="token punctuation">{</span></span>
<span class="line">            <span class="token comment">// 通知前端更新</span></span>
<span class="line">            app_handle<span class="token punctuation">.</span><span class="token function">emit_all</span><span class="token punctuation">(</span><span class="token string">"bookmarks-updated"</span><span class="token punctuation">,</span> <span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token operator">?</span><span class="token punctuation">;</span></span>
<span class="line">        <span class="token punctuation">}</span></span>
<span class="line">        <span class="token class-name">Err</span><span class="token punctuation">(</span>_<span class="token punctuation">)</span> <span class="token operator">=></span> <span class="token keyword">break</span><span class="token punctuation">,</span></span>
<span class="line">    <span class="token punctuation">}</span></span>
<span class="line"><span class="token punctuation">}</span></span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="代码规范" tabindex="-1"><a class="header-anchor" href="#代码规范"><span>代码规范</span></a></h2>
<h3 id="typescript" tabindex="-1"><a class="header-anchor" href="#typescript"><span>TypeScript</span></a></h3>
<ul>
<li>使用 ESLint 和 Prettier</li>
<li>使用严格模式 (<code v-pre>strict: true</code>)</li>
<li>优先使用 <code v-pre>const</code> 和箭头函数</li>
</ul>
<h3 id="rust" tabindex="-1"><a class="header-anchor" href="#rust"><span>Rust</span></a></h3>
<ul>
<li>使用 <code v-pre>cargo fmt</code> 格式化代码</li>
<li>使用 <code v-pre>cargo clippy</code> 检查代码</li>
<li>错误处理使用 <code v-pre>Result&lt;T, String&gt;</code></li>
</ul>
<h2 id="提交规范" tabindex="-1"><a class="header-anchor" href="#提交规范"><span>提交规范</span></a></h2>
<p>使用约定式提交：</p>
<div class="language-text line-numbers-mode" data-highlighter="prismjs" data-ext="text"><pre v-pre><code class="language-text"><span class="line">feat: 添加新功能</span>
<span class="line">fix: 修复 bug</span>
<span class="line">docs: 文档更新</span>
<span class="line">style: 代码格式调整</span>
<span class="line">refactor: 代码重构</span>
<span class="line">test: 测试相关</span>
<span class="line">chore: 构建/工具链更新</span>
<span class="line"></span></code></pre>
<div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div></div></template>


