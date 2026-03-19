---
name: dev-log
description: This skill should be used when the user asks to "record log", "记录日志", "use log", "查看日志", "使用日志", or when AI determines the task involves significant code changes that should be documented.
argument-hint: [描述]
---

# 开发日志追踪

记录和查询开发过程中的变更日志，支持语义检索。

## 触发条件

### 用户主动触发
当用户明确说以下短语时，立即执行对应操作：
- "record log" / "记录日志" → 记录本次任务的变更日志
- "use log" / "查看日志" / "使用日志" → 先查看历史日志，再执行任务

### 斜杠命令
用户也可以通过斜杠命令快速记录日志：
```
/dev-log 修改密钥生成逻辑，从 27 位改为 32 位
/dev-log 修复订单查询时积分计算错误的 bug
```

### AI 智能判断触发
当满足以下条件时，AI 应主动询问用户是否需要记录日志：
- 本次任务涉及功能性代码变更（新功能、Bug 修复、重构等）
- 变更可能影响现有功能

询问话术：
> "本次任务已完成。是否需要记录开发日志？如需记录，请说 'record log' 或 '记录日志'。"

注意：仅记录日志场景需要 AI 主动询问，查看日志仅在用户明确要求时触发。

## 功能

### 记录日志
将变更描述写入 `.dev-log/YYYY-MM-DD.md`，同时生成向量嵌入用于语义搜索。

**示例**：
- "record log 修改密钥生成逻辑，从 27 位改为 32 位"
- "记录日志：修复订单查询时积分计算错误的 bug"
- "/dev-log 添加商品上下架功能"

### 搜索日志
支持语义搜索和关键词搜索，自动选择可用的方式。

**示例**：
- "use log 查找密钥相关的变更"
- "查看日志里关于积分比例的记录"

### 查看日志列表
**示例**：
- "查看最近 7 天的开发日志"

## 实现

调用脚本 `.claude/skills/dev-log/scripts/record-log.js` 记录日志，调用 `.claude/skills/dev-log/scripts/search-logs.js` 搜索日志。

## 数据存储

- **日志文件**: `.dev-log/YYYY-MM-DD.md` (人类可读的 Markdown)
- **向量数据**: `.dev-log/.vector-store/` (用于语义搜索)
- **模型缓存**: `~/.dev-log-cache/` (用户级别，全局共享)